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

use backtrace::Backtrace;
use std::{panic, thread};

use crate::graceful_shutdown::ExitType;

/// Set the panic hook
pub(crate) fn set_panic_handler(tx: flume::Sender<ExitType>) {
    panic::set_hook(Box::new(move |info| {
        let location = info.location();
        let file = location.as_ref().map(|l| l.file()).unwrap_or("<unknown>");
        let line = location.as_ref().map(|l| l.line()).unwrap_or(0);
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };
        let thread = thread::current();
        let name = thread.name().unwrap_or("<unnamed>");
        let backtrace = Backtrace::new();
        let error = format!(
            "\n============================\n\
            {backtrace:?}\n\n\
            position:\n\
            Thread {name} panicked at {msg}, {file}:{line}\n\
            ============================\n\
            "
        );
        eprintln!("{}", error);
        let _ = tx.send(ExitType::Panic);
    }));
}
