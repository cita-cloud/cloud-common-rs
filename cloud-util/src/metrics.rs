use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server,
};
use lazy_static::lazy_static;
use prometheus::{gather, register_histogram, Encoder, Histogram, TextEncoder};
use std::time::Instant;
use std::{collections::HashMap, convert::Infallible};
use std::{
    sync::{Arc, RwLock},
    task::{Context, Poll},
};
use tonic::body::BoxBody;
use tower::{Layer, Service};

lazy_static! {
    static ref METRICS_DATA: Arc<RwLock<HashMap<(String, String), Histogram>>> =
        Arc::new(RwLock::new(HashMap::new()));
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
    type Service = MetricsService<S>;

    fn layer(&self, service: S) -> Self::Service {
        MetricsService {
            inner: service,
            buckets: self.buckets.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsService<S> {
    inner: S,
    buckets: Vec<f64>,
}

impl<S> Service<Request<Body>> for MetricsService<S>
where
    S: Service<Request<Body>, Response = Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        // parse client_name and function_name from request
        let client_name = req
            .headers()
            .get("client-name")
            .map(|v| v.to_str().unwrap());
        let function_name = req
            .uri()
            .to_string()
            .rsplit_once('/')
            .map(|c| c.1.to_string());

        if let (Some(client_name), Some(function_name)) = (client_name, function_name) {
            let key = (client_name.to_string(), function_name.to_string());
            let is_exist = {
                if let Ok(read) = METRICS_DATA.read() {
                    Some(read.contains_key(&key))
                } else {
                    None
                }
            };

            if let Some(is_exist) = is_exist {
                // if key not exist, register it
                if !is_exist {
                    match register_histogram!(
                        format!("{client_name}_to_{function_name}"),
                        "request latencies in milliseconds(ms)",
                        self.buckets.clone(),
                    ) {
                        Ok(histogram) => {
                            info!(
                                "register histogram {} succeeded",
                                format!("{client_name}_to_{function_name}")
                            );
                            {
                                if let Ok(mut write) = METRICS_DATA.write() {
                                    write.insert(key.clone(), histogram);
                                }
                            }
                        }
                        Err(e) => {
                            warn!(
                                "register histogram {} failed with error: {}, ignored metrics",
                                format!("{client_name}_to_{function_name}"),
                                e.to_string()
                            );
                            return Box::pin(async move {
                                let response = inner.call(req).await?;
                                Ok(response)
                            });
                        }
                    }
                }
            } else {
                return Box::pin(async move {
                    let response = inner.call(req).await?;
                    Ok(response)
                });
            }

            if let Ok(read) = METRICS_DATA.read() {
                match read.get(&key).cloned() {
                    Some(histogram) => Box::pin(async move {
                        let started = Instant::now();
                        let response = inner.call(req).await?;
                        let elapsed = started.elapsed().as_secs_f64() * 1000f64;
                        histogram.observe(elapsed);
                        Ok(response)
                    }),
                    None => {
                        warn!(
                            "register histogram {} succeeded but get it failed, ignored metrics",
                            format!("{client_name}_to_{function_name}")
                        );
                        Box::pin(async move {
                            let response = inner.call(req).await?;
                            Ok(response)
                        })
                    }
                }
            } else {
                Box::pin(async move {
                    let response = inner.call(req).await?;
                    Ok(response)
                })
            }
        } else {
            Box::pin(async move {
                let response = inner.call(req).await?;
                Ok(response)
            })
        }
    }
}

pub async fn run_metrics_exporter(
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc =
        make_service_fn(move |_conn| async move { Ok::<_, Infallible>(service_fn(serve_req)) });

    let addr = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&addr).serve(make_svc);
    info!("exporting metrics to http://{}/metrics", addr);

    server.await?;

    Ok(())
}

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = gather();
            let _ = encoder.encode(&metric_families, &mut buffer);

            Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Body::from(buffer))
                .unwrap()
        }
        _ => Response::builder()
            .status(404)
            .body(Body::from(
                "
            default:\n
            /60000/metrics for network\n
            /60001/metrics for consensus\n
            /60002/metrics for executor\n
            /60003/metrics for storage\n
            /60004/metrics for controller\n
            /60005/metrics for crypto\n
            ",
            ))
            .unwrap(),
    };

    Ok(response)
}
