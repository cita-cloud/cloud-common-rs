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

use crate::common::StatusCode;
use crate::status_code::StatusCodeEnum;

impl StatusCodeEnum {
    pub fn is_success(&self) -> Result<(), StatusCodeEnum> {
        if self != &StatusCodeEnum::Success {
            Err(*self)
        } else {
            Ok(())
        }
    }
}

impl ::std::fmt::Display for StatusCodeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for StatusCodeEnum {}

impl From<StatusCode> for StatusCodeEnum {
    fn from(status: StatusCode) -> Self {
        StatusCodeEnum::from_i32(status.code as i32)
            .ok_or(StatusCodeEnum::NoneStatusCode)
            .unwrap()
    }
}

#[allow(clippy::from_over_into)]
impl Into<StatusCode> for StatusCodeEnum {
    fn into(self) -> StatusCode {
        StatusCode { code: self as u32 }
    }
}

impl From<u32> for StatusCodeEnum {
    fn from(status: u32) -> Self {
        StatusCodeEnum::from_i32(status as i32)
            .ok_or(StatusCodeEnum::NoneStatusCode)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::status_code::StatusCodeEnum;

    #[test]
    fn it_works() {
        println!("{:?}", StatusCodeEnum::LoadError);
        assert_eq!(606, StatusCodeEnum::LoadError as u32);
        assert_eq!(StatusCodeEnum::LoadError, StatusCodeEnum::from(606));

        let status = crate::common::StatusCode { code: 0 };
        let s = StatusCodeEnum::from(status);
        assert_eq!(StatusCodeEnum::Success, s);
    }
}
