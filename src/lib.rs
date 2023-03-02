

use std::{fmt, sync::Arc};
//use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;
use std::time::SystemTime;

pub mod error;
pub use error::{Error};

////////////////////////////////////////////////////////////////////////////////
// USEFUL DEFAULTS

pub const ACCUMULATE_MAINNET_RPC_URL: &str = "https://mainnet.accumulatenetwork.io";
pub const ACCUMULATE_TESTNET_KERMIT_RPC_URL: &str = "https://api-gateway.accumulate.defidevs.io";
pub const ACCUMULATE_TESTNET_FOZZIE_RPC_URL: &str = "https://fozzie.accumulatenetwork.io";
pub const ACCUMULATE_EDGE_RPC_URL: &str = "https://api.accumulate.chainfold.io";
pub const ACCUMULATE_DEFAULT_RPC_URL: &str = "http://127.0.0.1:4467"; // if none speficied, use devnet

/// The default timeout for API requests
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
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new_with_base_url(ACCUMULATE_DEFAULT_RPC_URL.to_string())
    }
}

impl Client {
    /// Create a new client using a given base URL and a default
    /// timeout. The library will use absoluate paths based on this
    /// base_url.
    pub fn new_with_base_url(base_url: String) -> Self {
        Self::new_with_timeout(base_url, DEFAULT_TIMEOUT)
    }

    /// Create a new client using a given base URL, and request
    /// timeout value.  The library will use absoluate paths based on
    /// the given base_url.
    pub fn new_with_timeout(base_url: String, timeout: u64) -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap();
        Self { base_url, client }
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
