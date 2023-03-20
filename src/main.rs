use accumulate_api::*;
use tokio::test;
extern crate reqwest;

#[tokio::main]
async fn main() {
	env_logger::init();
	let v = get_last_major_block().await;
	//println!("{}", v);
}

async fn get_last_major_block() {
	let client = Client::new_with_version(ACCUMULATE_MAINNET_RPC_URL.to_string(), 3);
	let resp = blocks::get_range(&client, "acc://bvn-Apollo.acme", 1, 100, true).await;

	match resp {
		Err(e) => println!("{}", e),
		Ok(_) => return,
	}

	// if let Err(e) = resp {
	//     println!("{}", e);
	//     handler(e);
	// }
}
