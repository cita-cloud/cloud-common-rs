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

use crate::graceful_shutdown::ExitType;

pub(crate) async fn handle_signals(tx_exit: flume::Sender<ExitType>) {
    let mut signals = Signals::new([SIGTERM]).unwrap();
    while let Some(signal) = signals.next().await {
        match signal {
            SIGTERM => {
                let _ = tx_exit.send(ExitType::Signal(SIGTERM));
                break;
            }
            _ => warn!("Received signal: {signal}"),
        }
    }
}
