use accumulate_api::*;
use tokio::test;
extern crate reqwest;
use types::*;

#[tokio::main]
async fn main() {
	env_logger::init();
	let ov = get_last_major_blocks().await;
	match ov {
		None => println!("no result"),
		Some(v) => println!("{:?}", v),
	}
}

async fn get_last_major_blocks() -> Option<MajorBlocksRangeResponse> {
	let client = Client::new_with_version(ACCUMULATE_MAINNET_RPC_URL.to_string(), 3);
	let resp = blocks::get_range(&client, "acc://bvn-Apollo.acme", 1, 100, true, true).await;

	match resp {
		Err(e) => {
			println!("{}", e);
			return None;
		}
		Ok(r) => return Some(r),
		Ok(_) | Err(_) => {
			println!("error");
			return None;
		}
	}
}

async fn get_first_major_blocks() -> Option<MajorBlocksRangeResponse> {
	let client = Client::new_with_version(ACCUMULATE_MAINNET_RPC_URL.to_string(), 3);
	let resp = blocks::get_range(&client, "acc://bvn-Apollo.acme", 1, 100, true, false).await;

	match resp {
		Err(e) => {
			println!("{}", e);
			return None;
		}
		Ok(r) => return Some(r),
		Ok(_) | Err(_) => {
			println!("error");
			return None;
		}
	}
}
