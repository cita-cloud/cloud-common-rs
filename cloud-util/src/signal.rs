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

use futures::stream::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;

pub fn handle_signals() -> flume::Receiver<()> {
    let (tx, rx) = flume::bounded(0);
    tokio::spawn(async move {
        let mut signals = Signals::new([SIGTERM]).unwrap();
        while let Some(signal) = signals.next().await {
            match signal {
                SIGTERM => {
                    info!("exit by signal: {signal}");
                    drop(tx);
                    break;
                }
                _ => warn!("Received signal: {signal}"),
            }
        }
    });
    rx
}

pub async fn grpc_serve_listen_term(rx: flume::Receiver<()>) {
    let _ = rx.recv_async().await;
    info!("grpc server exit!");
}
