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

use crate::common::{ADDR_BYTES_LEN, HASH_BYTES_LEN};
use cita_cloud_proto::blockchain::BlockHeader;
use cita_cloud_proto::client::{CryptoClientTrait, InterceptedSvc};
use cita_cloud_proto::crypto::crypto_service_client::CryptoServiceClient;
use cita_cloud_proto::crypto::{HashDataRequest, RecoverSignatureRequest, SignMessageRequest};
use cita_cloud_proto::retry::RetryClient;
use cita_cloud_proto::status_code::StatusCodeEnum;
use prost::Message;

pub async fn hash_data(
    client: RetryClient<CryptoServiceClient<InterceptedSvc>>,
    data: &[u8],
) -> Result<Vec<u8>, StatusCodeEnum> {
    let data = data.to_vec();
    match client.hash_data(HashDataRequest { data }).await {
        Ok(hash_respond) => {
            let status_code = StatusCodeEnum::from(
                hash_respond
                    .status
                    .ok_or(StatusCodeEnum::NoneStatusCode)?
                    .code,
            );

            if status_code != StatusCodeEnum::Success {
                Err(status_code)
            } else {
                Ok(hash_respond
                    .hash
                    .ok_or(StatusCodeEnum::NoneHashResult)?
                    .hash)
            }
        }
        Err(status) => {
            warn!("hash_data error: {}", status.to_string());
            Err(StatusCodeEnum::CryptoServerNotReady)
        }
    }
}

pub async fn get_block_hash(
    client: RetryClient<CryptoServiceClient<InterceptedSvc>>,
    header: Option<&BlockHeader>,
) -> Result<Vec<u8>, StatusCodeEnum> {
    match header {
        Some(header) => {
            let mut block_header_bytes = Vec::with_capacity(header.encoded_len());
            header.encode(&mut block_header_bytes).map_err(|_| {
                warn!("get_block_hash: encode block header failed");
                StatusCodeEnum::EncodeError
            })?;
            let block_hash = hash_data(client, &block_header_bytes).await?;
            Ok(block_hash)
        }
        None => Err(StatusCodeEnum::NoneBlockHeader),
    }
}

pub async fn pk2address(
    client: RetryClient<CryptoServiceClient<InterceptedSvc>>,
    pk: &[u8],
) -> Result<Vec<u8>, StatusCodeEnum> {
    Ok(hash_data(client, pk).await?[HASH_BYTES_LEN - ADDR_BYTES_LEN..].to_vec())
}

pub async fn sign_message(
    client: RetryClient<CryptoServiceClient<InterceptedSvc>>,
    msg: &[u8],
) -> Result<Vec<u8>, StatusCodeEnum> {
    let smr = client
        .sign_message(SignMessageRequest { msg: msg.to_vec() })
        .await
        .map_err(|e| {
            warn!("sign_message failed: {}", e.to_string());
            StatusCodeEnum::CryptoServerNotReady
        })?;

    let status = StatusCodeEnum::from(smr.status.ok_or(StatusCodeEnum::NoneStatusCode)?);
    if status != StatusCodeEnum::Success {
        Err(status)
    } else {
        Ok(smr.signature)
    }
}

pub async fn recover_signature(
    client: RetryClient<CryptoServiceClient<InterceptedSvc>>,
    signature: &[u8],
    msg: &[u8],
) -> Result<Vec<u8>, StatusCodeEnum> {
    let rsr = client
        .recover_signature(RecoverSignatureRequest {
            msg: msg.to_vec(),
            signature: signature.to_vec(),
        })
        .await
        .map_err(|e| {
            warn!("recover_signature failed: {}", e.to_string());
            StatusCodeEnum::CryptoServerNotReady
        })?;

    let status = StatusCodeEnum::from(rsr.status.ok_or(StatusCodeEnum::NoneStatusCode)?);
    if status != StatusCodeEnum::Success {
        Err(status)
    } else {
        Ok(rsr.address)
    }
}
