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

use cita_cloud_proto::blockchain::raw_transaction::Tx;
use cita_cloud_proto::blockchain::{
    Block, CompactBlock, CompactBlockBody, RawTransaction, RawTransactions,
};
use cita_cloud_proto::common::Address;
use cita_cloud_proto::status_code::StatusCodeEnum;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml::Value;

pub const ADDR_BYTES_LEN: usize = 20;
pub const HASH_BYTES_LEN: usize = 32;

pub fn h160_address_check(address: Option<&Address>) -> Result<(), StatusCodeEnum> {
    match address {
        Some(addr) => {
            if addr.address.len() == ADDR_BYTES_LEN {
                Ok(())
            } else {
                Err(StatusCodeEnum::ProvideAddressError)
            }
        }
        None => Err(StatusCodeEnum::NoProvideAddress),
    }
}

pub fn get_tx_hash(raw_tx: &RawTransaction) -> Result<&[u8], StatusCodeEnum> {
    match raw_tx.tx {
        Some(Tx::NormalTx(ref normal_tx)) => Ok(&normal_tx.transaction_hash),
        Some(Tx::UtxoTx(ref utxo_tx)) => Ok(&utxo_tx.transaction_hash),
        None => Err(StatusCodeEnum::NoTransaction),
    }
}

pub fn get_tx_hash_list(raw_txs: &RawTransactions) -> Result<Vec<Vec<u8>>, StatusCodeEnum> {
    let mut hashes = Vec::new();
    for raw_tx in &raw_txs.body {
        hashes.push(get_tx_hash(raw_tx)?.to_vec())
    }
    Ok(hashes)
}

pub fn extract_compact(block: Block) -> CompactBlock {
    let mut compact_body = CompactBlockBody { tx_hashes: vec![] };

    if let Some(body) = block.body {
        for raw_tx in body.body {
            match raw_tx.tx {
                Some(Tx::NormalTx(normal_tx)) => {
                    compact_body.tx_hashes.push(normal_tx.transaction_hash)
                }
                Some(Tx::UtxoTx(utxo_tx)) => compact_body.tx_hashes.push(utxo_tx.transaction_hash),
                None => {}
            }
        }
    }

    CompactBlock {
        version: block.version,
        header: block.header,
        body: Some(compact_body),
    }
}

pub fn read_toml<T>(path: impl AsRef<Path>, name: &str) -> T
where
    T: for<'de> Deserialize<'de>,
{
    let s = fs::read_to_string(path).unwrap_or_else(|e| panic!("read_to_string err: {e}"));
    let config: Value = toml::from_str(&s).unwrap_or_else(|e| panic!("toml parse err: {e}"));
    let value = config
        .get(name)
        .unwrap_or_else(|| panic!("config get err: {name}"));
    T::deserialize(value.clone()).unwrap_or_else(|e| panic!("config deserialize err: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    struct ExecutorConfig {
        port: u16,
    }

    #[test]
    fn it_works() {
        let config: ExecutorConfig = read_toml("src/example/sample.toml", "executor");
        assert_eq!(config.port, 50002);
    }
}
