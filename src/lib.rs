use std::{fmt, sync::Arc};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;
use std::time::SystemTime;

pub mod error;
pub use error::{Error};

pub mod utils;
pub use utils::{now_millis};

////////////////////////////////////////////////////////////////////////////////
// USEFUL DEFAULTS

pub const ACCUMULATE_MAINNET_RPC_URL: &str = "https://mainnet.accumulatenetwork.io";
pub const ACCUMULATE_TESTNET_KERMIT_RPC_URL: &str = "https://api-gateway.accumulate.defidevs.io";
pub const ACCUMULATE_TESTNET_FOZZIE_RPC_URL: &str = "https://fozzie.accumulatenetwork.io";
pub const ACCUMULATE_EDGE_RPC_URL: &str = "https://api.accumulate.chainfold.io";
pub const ACCUMULATE_DEFAULT_RPC_URL: &str = "http://127.0.0.1:4467"; // if none speficied, use devnet

/// Default API version number
pub const ACCUMULATE_API_VER: u8 = 3; // "2" for older requests

/// Default timeout for API requests
pub const DEFAULT_TIMEOUT: u64 = 120;

/// JSON RPC version
pub const JSON_RPC: &str = "2.0";

////////////////////////////////////////////////////////////////////////////////
// Client Logic

// Response type
pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    version: u8,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new_with_base_url(ACCUMULATE_DEFAULT_RPC_URL.to_string())
    }
}

impl Client {

    pub fn new() -> Self {
	let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()
            .unwrap();
	Self {base_url: ACCUMULATE_DEFAULT_RPC_URL.to_string(), version: ACCUMULATE_API_VER, client}
    }

    /// Create a new client using a given base URL and a default
    /// timeout. The library will use absoluate paths based on this
    /// base_url.
    pub fn new_with_base_url(base_url: String) -> Self {
        Self::new_with_timeout(base_url, ACCUMULATE_API_VER, DEFAULT_TIMEOUT)
    }

    /// Create a new client using a given base URL, and request
    /// timeout value.  The library will use absoluate paths based on
    /// the given base_url.
    pub fn new_with_timeout(base_url: String, version: u8, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();
        Self { base_url, version, client }
    }

    pub fn new_with_version(base_url: String, version: u8) -> Self {
	 let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs( DEFAULT_TIMEOUT))
            .build()
            .unwrap();
	 Self { base_url, version, client }
    }

    async fn post<T: DeserializeOwned, D: Serialize>(&self, path: &str, data: D) -> Result<T> {
        #[derive(Clone, Serialize, Deserialize, Debug)]
        #[serde(untagged)]
        pub(crate) enum Response<T> {
            Data { result: T, id: String },
            Error { id: String, error: Error },
        }

        #[derive(Clone, Serialize, Deserialize, Debug)]
        pub(crate) struct Error {
            message: String,
            code: isize,
        }

        let request_url = format!("{}{}", self.base_url, path);
        let request = self.client.post(&request_url).json(&data);
        let response = request.send().await?;
        let body = response.text().await?;
        let v: Response<T> = serde_json::from_str(&body)?;
        match v {
            Response::Data { result, .. } => Ok(result),
            Response::Error { error, .. } => {
                Err(error::Error::NodeError(error.message, error.code))
            }
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(tag = "method")]
#[serde(rename_all = "snake_case")]
enum Method {
    Metrics,
    ConsensusStatus,
    FindService,
    MajorBLocksGet {params: BlockListParams},
    MinorBlocksGet {params: BlockListParams},
    MajorBlockGet {params: BlockParams},
    MinorBlockGet {params: BlockParams},
    Query { params: QueryParams}
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub(crate) struct ApiCall {
    jsonrpc: String,
    id: String,
    #[serde(flatten)]
    method: Method,
}


impl ApiCall {
    fn new(request: Method) -> ApiCall {
        ApiCall {
            jsonrpc: JSON_RPC.to_string(),
            id: now_millis(),
            method: request,
        }
    }

   pub(crate) fn query_major_blocks(url: String, start: u64, count: u64, expand: bool) -> Self {
        Self::new(Method::MajorBLocksGet {
            params: BlockListParams { scope: url, start, count, expand },
        })
   }

   pub(crate) fn query_major_block(url: String, height: u64) -> Self {
        Self::new(Method::MajorBlockGet {
            params: BlockParams { scope: url, height },
        })
   }


}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct QueryParams {
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u64>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct BlockParams {
    scope: String,
    height: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct BlockListParams {
    scope: String,
    start: u64,
    count: u64,
    expand: bool
}


#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn get_last_major_block() {
        let client = Client::default();
        let txn = query_major_blocks(&client, "").await;
        let er = match txn {
            Err(e) => format!("{}", e),
            _ => panic!("??"),
        };
        //assert_eq!(er, "error code -100 from node: No Blocks available");
    }
}
