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

// tonic does not derive `Eq` for the gRPC message types, which causes a warning from Clippy. The
// current suggestion is to explicitly allow the lint in the module that imports the protos.
// Read more: https://github.com/hyperium/tonic/issues/1056
#![allow(clippy::derive_partial_eq_without_eq)]

pub mod blockchain {
    tonic::include_proto!("blockchain");
}

pub mod common {
    tonic::include_proto!("common");
}

pub mod consensus {
    tonic::include_proto!("consensus");
}

pub mod controller {
    tonic::include_proto!("controller");
}

pub mod executor {
    tonic::include_proto!("executor");
}

pub mod crypto {
    tonic::include_proto!("crypto");
}

pub mod network {
    tonic::include_proto!("network");
}

pub mod storage {
    tonic::include_proto!("storage");
}

pub mod evm {
    tonic::include_proto!("evm");
}

pub mod health_check {
    tonic::include_proto!("grpc.health.v1");
}

pub mod status_code {
    tonic::include_proto!("status_code");
}

pub mod client;
pub mod retry;

impl crate::status_code::StatusCode {
    pub fn is_success(&self) -> Result<(), crate::status_code::StatusCode> {
        if self != &crate::status_code::StatusCode::Success {
            Err(*self)
        } else {
            Ok(())
        }
    }
}

impl ::std::fmt::Display for crate::status_code::StatusCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for crate::status_code::StatusCode {}

impl From<crate::common::StatusCode> for crate::status_code::StatusCode {
    fn from(status: crate::common::StatusCode) -> Self {
        crate::status_code::StatusCode::from_i32(status.code as i32)
            .ok_or(crate::status_code::StatusCode::NoneStatusCode)
            .unwrap()
    }
}

impl From<u32> for crate::status_code::StatusCode {
    fn from(status: u32) -> Self {
        crate::status_code::StatusCode::from_i32(status as i32)
            .ok_or(crate::status_code::StatusCode::NoneStatusCode)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::status_code::StatusCode;

    #[test]
    fn it_works() {
        println!("{:?}", StatusCode::LoadError);
        assert_eq!(606, StatusCode::LoadError as u32);
        assert_eq!(StatusCode::LoadError, StatusCode::from(606));

        let status = crate::common::StatusCode { code: 0 };
        let s = StatusCode::from(status);
        assert_eq!(StatusCode::Success, s);
    }
}
