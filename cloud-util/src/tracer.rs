// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::FromStr;

use opentelemetry::{
    global,
    propagation::{Extractor, Injector},
    sdk::propagation::TraceContextPropagator,
};
use serde::{Deserialize, Serialize};
use tonic::Request;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{fmt::format, prelude::*, EnvFilter};

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

struct MutMetadataMap<'a>(&'a mut tonic::metadata::MetadataMap);

impl<'a> Injector for MutMetadataMap<'a> {
    /// Set a key and value in the MetadataMap.  Does nothing if the key or value are not valid inputs
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(val) = std::str::FromStr::from_str(&value) {
                self.0.insert(key, val);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LogConfig {
    max_level: String,
    filter: String,
    service_name: String,
    rolling_file_path: Option<String>,
    agent_endpoint: Option<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            max_level: "info".to_owned(),
            filter: "info".to_owned(),
            service_name: Default::default(),
            rolling_file_path: Default::default(),
            agent_endpoint: Default::default(),
        }
    }
}

pub fn init_tracer(
    node: &str,
    log_config: &LogConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // agent
    let mut agent = None;
    if let Some(agent_endpoint) = &log_config.agent_endpoint {
        global::set_text_map_propagator(TraceContextPropagator::new());
        agent = Some(
            opentelemetry_jaeger::new_agent_pipeline()
                .with_service_name(format!("{node}-{}", &log_config.service_name))
                .with_endpoint(agent_endpoint)
                .install_batch(opentelemetry::runtime::Tokio)?,
        );
    }

    // log
    let mut logfile = None;
    let mut stdout = None;
    if let Some(rolling_file_path) = &log_config.rolling_file_path {
        // logfile
        logfile = Some(tracing_appender::rolling::hourly(
            rolling_file_path,
            &log_config.service_name,
        ));
    } else {
        // stdout
        stdout = Some(
            std::io::stdout
                .with_max_level(tracing::Level::from_str(&log_config.max_level).unwrap()),
        );
    }

    if let Some(agent) = agent {
        if let Some(stdout) = stdout {
            tracing_subscriber::registry()
                .with(EnvFilter::new(&log_config.filter))
                .with(tracing_opentelemetry::layer().with_tracer(agent))
                .with(
                    tracing_subscriber::fmt::layer()
                        .event_format(format().compact())
                        .with_ansi(false)
                        .with_writer(stdout),
                )
                .try_init()?;
        } else {
            tracing_subscriber::registry()
                .with(EnvFilter::new(&log_config.filter))
                .with(tracing_opentelemetry::layer().with_tracer(agent))
                .with(
                    tracing_subscriber::fmt::layer()
                        .event_format(format().compact())
                        .with_ansi(false)
                        .with_writer(logfile.unwrap()),
                )
                .try_init()?;
        }
    } else if let Some(stdout) = stdout {
        tracing_subscriber::registry()
            .with(EnvFilter::new(&log_config.filter))
            .with(
                tracing_subscriber::fmt::layer()
                    .event_format(format().compact())
                    .with_ansi(false)
                    .with_writer(stdout),
            )
            .try_init()?;
    } else {
        tracing_subscriber::registry()
            .with(EnvFilter::new(&log_config.filter))
            .with(
                tracing_subscriber::fmt::layer()
                    .event_format(format().compact())
                    .with_ansi(false)
                    .with_writer(logfile.unwrap()),
            )
            .try_init()?;
    }

    Ok(())
}

pub fn shutdown_tracer() {
    opentelemetry::global::shutdown_tracer_provider();
}

pub fn set_parent<T>(request: &Request<T>) {
    let parent_cx =
        global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
    tracing::Span::current().set_parent(parent_cx);
}

pub fn inject_context<T>(request: &mut Request<T>) {
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(
            &tracing::Span::current().context(),
            &mut MutMetadataMap(request.metadata_mut()),
        )
    });
}
