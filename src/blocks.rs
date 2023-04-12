use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

mod types;

pub async fn get_range_major(
	client: &Client,
	scope: &str,
	start: u64,
	count: u64,
	expand: bool,
	from_end: bool,
) -> Result<MajorBlocksRangeResponse> {
	let json = json!(ApiCall::query_major_blocks(
		scope.to_string(),
		start,
		count,
		expand,
		from_end
	));
	println!("{}", json);

	let url_path = &("/v".to_string() + client.version.to_string().as_str());
	client.post(url_path, &json).await
}

pub async fn get_range_minor(
	client: &Client,
	scope: &str,
	start: u64,
	count: u64,
	expand: bool,
	from_end: bool,
) -> Result<types::MajorBlocksRangeResponse> {
	let json = json!(ApiCall::query_minor_blocks(
		scope.to_string(),
		start,
		count,
		expand,
		from_end
	));
	println!("{}", json);

	let url_path = &("/v".to_string() + client.version.to_string().as_str());
	client.post(url_path, &json).await
}
