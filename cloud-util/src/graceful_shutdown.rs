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

use crate::{panic_hook::set_panic_handler, signal::handle_signals};

#[derive(Debug)]
pub(crate) enum ExitType {
    Signal(i32),
    Panic,
}

/// Should be called in tokio runtime
pub fn graceful_shutdown() -> flume::Receiver<()> {
    let (tx, rx) = flume::bounded(0);
    let (tx_exit, rx_exit) = flume::bounded(1);
    set_panic_handler(tx_exit.clone());

    #[cfg(not(windows))]
    tokio::spawn(handle_signals(tx_exit));

    tokio::spawn(async move {
        if let Ok(exit_type) = rx_exit.recv_async().await {
            match exit_type {
                ExitType::Signal(_) => info!("exit by: {exit_type:?}"),
                ExitType::Panic => eprintln!("exit by: {exit_type:?}"),
            }
            drop(tx);
        }
    });
    rx
}

pub async fn grpc_serve_listen_term(rx: flume::Receiver<()>) {
    let _ = rx.recv_async().await;
    info!("grpc server exit!");
}
