use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;
use std::time::SystemTime;
use std::{fmt, sync::Arc};

pub mod error;
pub use error::Error;

pub mod utils;
pub use utils::now_millis;

pub mod blocks;
pub mod types;

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
			//.gzip(true)
			.timeout(Duration::from_secs(DEFAULT_TIMEOUT))
			.build()
			.unwrap();
		Self {
			base_url: ACCUMULATE_DEFAULT_RPC_URL.to_string(),
			version: ACCUMULATE_API_VER,
			client,
		}
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
			//.gzip(true)
			.timeout(Duration::from_secs(timeout))
			.build()
			.unwrap();
		Self {
			base_url,
			version,
			client,
		}
	}

	pub fn new_with_version(base_url: String, version: u8) -> Self {
		let client = reqwest::Client::builder()
			//.gzip(true)
			.timeout(Duration::from_secs(DEFAULT_TIMEOUT))
			.build()
			.unwrap();
		Self {
			base_url,
			version,
			client,
		}
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
		println!("1// request URL: {}", request_url);
		let request = self.client.post(&request_url).json(&data);
		let requestV = self.client.post(&request_url).json(&data);
		println!("2//");

		match requestV.build() {
			Ok(req) => {
				// Inspect the request
				println!("{:?}", req);
				//println!("{:?}", &data);

				// Send the request
				//let response = client.execute(req).await?;
				// Handle the response...
			}
			Err(e) => {
				// Handle the error
				eprintln!("Error building the request: {:?}", e);
			}
		}

		let response = request.send().await?;
		println!("3//");
		let body = response.text().await?;
		println!("body: {}", body);
		let v: Response<T> = serde_json::from_str(&body)?;
		match v {
			Response::Data { result, .. } => Ok(result),
			Response::Error { error, .. } => Err(error::Error::ApiError(error.message, error.code)),
		}
	}
}

#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(tag = "method")]
//#[serde(rename_all = "snake_case")]
enum Method {
	#[serde(rename(serialize = "metrics"))]
	Metrics,
	#[serde(rename(serialize = "consensus-status"))]
	ConsensusStatus,
	#[serde(rename(serialize = "find-service"))]
	FindService,
	#[serde(rename(serialize = "query"))]
	MajorBlocksGet { params: QueryBlockMajorListParams },
	#[serde(rename(serialize = "query"))]
	MinorBlocksGet { params: QueryBlockMinorListParams },

	#[serde(rename(serialize = "query"))]
	MajorBlockGet { params: BlockParams },
	#[serde(rename(serialize = "query"))]
	MinorBlockGet { params: BlockParams },

	#[serde(rename(serialize = "query"))]
	Query { params: QueryParams },
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
			id: "0".to_string(), //now_millis(),
			method: request,
		}
	}

	pub(crate) fn query_major_blocks(url: String, start: u64, count: u64, expand: bool, from_end: bool) -> Self {
		Self::new(Method::MajorBlocksGet {
			params: QueryBlockMajorListParams {
				scope: url,
				query: {
					BlockMajorListParams {
						query_type: "block".to_string(),
						major_range: BlockListRangeParams {
							start,
							count,
							expand,
							from_end,
						},
					}
				},
			},
		})
	}

	pub(crate) fn query_major_block(url: String, height: u64) -> Self {
		Self::new(Method::MajorBlockGet {
			params: BlockParams { scope: url, height },
		})
	}

	pub(crate) fn query_minor_blocks(url: String, start: u64, count: u64, expand: bool, from_end: bool) -> Self {
		Self::new(Method::MinorBlocksGet {
			params: QueryBlockMinorListParams {
				scope: url,
				query: {
					BlockMinorListParams {
						query_type: "block".to_string(),
						major_range: BlockListRangeParams {
							start,
							count,
							expand,
							from_end,
						},
					}
				},
			},
		})
	}
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct QueryParams {
	scope: String,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct BlockParams {
	scope: String,
	height: u64,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct QueryBlockMajorListParams {
	scope: String,
	query: BlockMajorListParams,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct BlockMajorListParams {
	#[serde(rename(serialize = "queryType"))]
	query_type: String,
	#[serde(rename(serialize = "majorRange"))]
	major_range: BlockListRangeParams,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct BlockListRangeParams {
	start: u64,
	count: u64,
	expand: bool,
	#[serde(rename(serialize = "fromEnd"))]
	from_end: bool,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct QueryBlockMinorListParams {
	scope: String,
	query: BlockMinorListParams,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct BlockMinorListParams {
	#[serde(rename(serialize = "queryType"))]
	query_type: String,
	#[serde(rename(serialize = "minorRange"))]
	major_range: BlockListRangeParams,
}

#[cfg(test)]
mod test {
	use super::*;
	use tokio::test;

	#[test]
	async fn get_last_major_block() {
		let client = Client::new_with_version(ACCUMULATE_MAINNET_RPC_URL.to_string(), 3);
		let resp = blocks::get_range(&client, "acc://bvn-Apollo.acme", 1, 100, true).await;

		if let Err(err) = resp {
			// can be omitted if
			println!("{}", err);
			panic!("{err}"); // only the Ok result
		}

		assert!(1 > 0);
	}
}
