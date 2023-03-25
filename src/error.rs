use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request error")]
    Request(#[from] reqwest::Error),
    #[error("unexpected value")]
    Value(serde_json::Value),
    #[error("invalid decimals in {0}, only 8 allowed")]
    Decimals(String),
    #[error("unexpected or invalid number {0}")]
    Number(String),
    #[error("error code {1} from node: {0}")]
    ApiError(String, isize),
    #[error("error deserializing JSON response")]
    JsonDeserialization(#[from] serde_json::Error),
    #[error("error deserializing transaction of type {r#type} and hash {hash}")]
    TransactionProcessing { r#type: String, hash: String },
    #[error("node response with no error but no result")]
    ApiResponseNoResult,
    #[error("invalid transaction")]
    TxnInvalid(String),
}

impl Error {
    pub fn value(value: serde_json::Value) -> Self {
        Self::Value(value)
    }

    pub fn decimals(value: &str) -> Self {
        Self::Decimals(value.to_string())
    }

    pub fn number(value: &str) -> Self {
        Self::Number(value.to_string())
    }
}

// fn handler(e: reqwest::Error) {
// 	if e.is_http() {
// 		match e.url() {
// 			None => println!("No Url given"),
// 			Some(url) => println!("Problem making request to: {}", url),
// 		}
// 	}
// 	// Inspect the internal error and output it
// 	if e.is_serialization() {
// 		let serde_error = match e.get_ref() {
// 			None => return,
// 			Some(err) => err,
// 		};
// 		println!("problem parsing information {}", serde_error);
// 	}
// 	if e.is_redirect() {
// 		println!("server redirecting too many times or making loop");
// 	}
// }
