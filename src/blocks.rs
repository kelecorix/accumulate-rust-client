use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use types::*;

/// Represents a block response from blockchain-node.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BlockRaw {
	pub height: u64,
	pub hash: String,
	pub prev_hash: String,
	pub time: u64,
	//pub transactions: Vec<BlockTransaction>,
}

pub async fn get_range(
	client: &Client,
	scope: &str,
	start: u64,
	count: u64,
	expand: bool,
) -> Result<MajorBlocksRangeResponse> {
	let json = json!(ApiCall::query_major_blocks(scope.to_string(), start, count, expand));
	println!("{}", json);

	let url_path = &("/v".to_string() + client.version.to_string().as_str());
	client.post(url_path, &json).await
}
