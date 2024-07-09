use axum::routing::get;
use axum::{http::StatusCode, response::IntoResponse, Router};
use hyper::{Request, Response};
use lazy_static::lazy_static;
use prometheus::{
    exponential_buckets, gather, register_counter, register_gauge, register_histogram, Counter,
    Encoder, Gauge, Histogram, TextEncoder,
};
use std::collections::HashMap;
use std::time::Instant;
use std::{
    sync::{Arc, RwLock},
    task::{Context, Poll},
};
use tonic::body::BoxBody;
use tower::{Layer, Service};

lazy_static! {
    static ref METRICS_DATA: Arc<RwLock<HashMap<String, MetricsType>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

#[derive(Debug, Clone)]
pub enum MetricsType {
    Histogram(Option<Histogram>),
    Gauge(Option<Gauge>),
    Counter(Option<Counter>),
}

fn register_metrics(
    metrics_type: &MetricsType,
    key: &str,
    help_info: &str,
    histogram_buckets: Option<Vec<f64>>,
) -> Option<MetricsType> {
    match metrics_type {
        MetricsType::Histogram(_) => {
            match register_histogram!(
                key,
                help_info,
                histogram_buckets.unwrap_or(exponential_buckets(0.0001, 10.0, 8).unwrap()),
            ) {
                Ok(h) => {
                    info!("register histogram: {key} success");
                    Some(MetricsType::Histogram(Some(h)))
                }
                Err(e) => {
                    warn!("register histogram: {key} failed: {e}");
                    None
                }
            }
        }
        MetricsType::Gauge(_) => match register_gauge!(key, help_info,) {
            Ok(g) => {
                info!("register gauge: {key} success");
                Some(MetricsType::Gauge(Some(g)))
            }
            Err(e) => {
                warn!("register gauge: {key} failed: {e}");
                None
            }
        },
        MetricsType::Counter(_) => match register_counter!(key, help_info,) {
            Ok(c) => {
                info!("register counter: {key} success");
                Some(MetricsType::Counter(Some(c)))
            }
            Err(e) => {
                warn!("register counter: {key} failed: {e}");
                None
            }
        },
    }
}

fn is_same(m1: &MetricsType, m2: &MetricsType) -> bool {
    matches!(
        (m1, m2),
        (MetricsType::Histogram(_), MetricsType::Histogram(_))
            | (MetricsType::Gauge(_), MetricsType::Gauge(_))
            | (MetricsType::Counter(_), MetricsType::Counter(_))
    )
}

fn is_key_valid(key: &str) -> bool {
    let mut key_chars = key.chars();
    if key.is_empty() || key_chars.next().unwrap().is_ascii_digit() {
        return false;
    }
    if key_chars.any(|c| !c.is_ascii_alphabetic() && !c.is_ascii_digit() && c != '_') {
        return false;
    }
    true
}

pub fn get_metrics(
    metrics_type: &MetricsType,
    key: &str,
    help_info: &str,
    histogram_buckets: Option<Vec<f64>>,
) -> Option<MetricsType> {
    if !is_key_valid(key) {
        warn!("get_metrics: {key} failed: key invalid");
        return None;
    }
    match METRICS_DATA.write() {
        Ok(mut write) => {
            if !write.contains_key(key) {
                if let Some(metrics) =
                    register_metrics(metrics_type, key, help_info, histogram_buckets)
                {
                    write.insert(key.to_string(), metrics);
                } else {
                    return None;
                }
            }
        }
        Err(e) => {
            warn!("get_metrics: {key} failed: {e}");
            return None;
        }
    };
    match METRICS_DATA.read() {
        Ok(read) => match read.get(key).cloned() {
            Some(m) => {
                if is_same(metrics_type, &m) {
                    Some(m)
                } else {
                    warn!("get_metrics: {key} failed, type:{metrics_type:?}, type not same");
                    None
                }
            }
            None => {
                warn!("get_metrics: {key} failed, type:{metrics_type:?}, not found");
                None
            }
        },
        Err(e) => {
            warn!("get_metrics: {key} failed: {e}");
            None
        }
    }
}

// TODO: match any number of method parameter
macro_rules! impl_metrics {
    ($func_name:ident, $enum_type:ident, $method_name:ident, $data_type:ident) => {
        pub fn $func_name(
            metrics_type: MetricsType,
            key: String,
            help_info: String,
            histogram_buckets: Option<Vec<f64>>,
            data: $data_type,
        ) {
            if let Some(MetricsType::$enum_type(Some(g))) =
                get_metrics(&metrics_type, &key, &help_info, histogram_buckets)
            {
                g.$method_name(data);
            }
        }
    };
}

impl_metrics!(gauge_set, Gauge, set, f64);
impl_metrics!(gauge_add, Gauge, add, f64);
impl_metrics!(gauge_sub, Gauge, sub, f64);
impl_metrics!(histogram_observe, Histogram, observe, f64);
impl_metrics!(counter_inc_by, Counter, inc_by, f64);

// grpc call metrics
fn rpc_info_to_key(client_name: &str, method_name: &str) -> String {
    client_name.to_string() + "_to_" + method_name
}

#[derive(Debug, Clone)]
pub struct MiddlewareLayer {
    buckets: Vec<f64>,
}

impl MiddlewareLayer {
    pub fn new(buckets: Vec<f64>) -> Self {
        MiddlewareLayer { buckets }
    }
}

impl<S> Layer<S> for MiddlewareLayer {
    type Service = RpcMetricsService<S>;

    fn layer(&self, service: S) -> Self::Service {
        RpcMetricsService {
            inner: service,
            buckets: self.buckets.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RpcMetricsService<S> {
    inner: S,
    buckets: Vec<f64>,
}

impl<S> Service<Request<BoxBody>> for RpcMetricsService<S>
where
    S: Service<Request<BoxBody>, Response = Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        // parse client_name and method_name from request
        let client_name = req
            .headers()
            .get("client-name")
            .map(|v| v.to_str().unwrap());
        let uri_string = req.uri().to_string();
        let method_name = uri_string.rsplit_once('/').map(|c| c.1);

        if let (Some(client_name), Some(method_name)) = (client_name, method_name) {
            let key = rpc_info_to_key(client_name, method_name);
            let bucket = Some(self.buckets.clone());
            Box::pin(async move {
                let started = Instant::now();
                let response = inner.call(req).await?;
                let elapsed = started.elapsed().as_secs_f64() * 1000f64;
                histogram_observe(
                    MetricsType::Histogram(None),
                    key,
                    "request latencies in milliseconds(ms)".to_string(),
                    bucket,
                    elapsed,
                );
                Ok(response)
            })
        } else {
            Box::pin(async move {
                let response = inner.call(req).await?;
                Ok(response)
            })
        }
    }
}

// exporter
pub async fn run_metrics_exporter(
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = Router::new().route("/metrics", get(exporter));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await?;
    info!("exporting metrics to http://127.0.0.1:{}/metrics", port);

    Ok(())
}

async fn exporter() -> impl IntoResponse {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = gather();
    let _ = encoder.encode(&metric_families, &mut buffer);

    (StatusCode::OK, String::from_utf8(buffer).unwrap())
}
