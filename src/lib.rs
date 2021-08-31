#[macro_use]
extern crate jsonrpc_client_core;

extern crate derive_builder;
extern crate serde;
extern crate serde_json;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// ADIURL
///
/// Accumulate ADI URL
///
pub type ADIURL = String;

/// ADIPublicKeyHash
///
/// SHA-256 hash of ADI Public Key
///
pub type ADIPublicKeyHash = String;

/// ADIPublicKey
///
/// ADI Public Key in hex format
///
pub type ADIPublicKey = String;

/// TokenURL
///
/// Token URL
///
pub type TokenURL = String;

/// TokenSymbol
///
/// Token ticker
///
pub type TokenSymbol = String;

/// TokenPrecision
///
/// All transactions will need to specify input/output amounts in integer units × 10^precision
///
pub type TokenPrecision = i64;

/// TokenAccountURL
///
/// Token Account URL
///
pub type TokenAccountURL = String;

/// SenderTokenAccount
///
/// Token Account URL
///
pub type SenderTokenAccount = String;

/// Amount
///
/// Number of tokens
///
pub type Amount = i64;

/// ReceiverTokenAccount
///
/// Token Account URL and amount
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ReceiverTokenAccount {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<TokenAccountURL>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub amount: Option<Amount>,
}

/// ReceiverSTokenAccountS
///
/// Token Account URL
///
pub type ReceiverSTokenAccountS = Vec<ReceiverTokenAccount>;

/// Metadata
///
/// Transaction Metadata
///
pub type Metadata = String;

/// Balance
///
/// Token Account balance
///
pub type Balance = i64;

/// Hash
///
/// Transaction Hash
///
pub type Hash = String;
pub type StringXtH3HHU8 = String;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ADIAccumulateDigitalIdentity {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<ADIURL>,
	#[serde(rename = "publicKeyHash", skip_serializing_if = "Option::is_none")]
	pub public_key_hash: Option<ADIPublicKeyHash>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct Signer {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<ADIURL>,
	#[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
	pub public_key: Option<ADIPublicKey>,
}

pub type Integer2AHOqbcQ = i64;
pub type String7GgoCKrm = String;
pub type StringWcSuVYcM = String;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct Token {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<TokenURL>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub symbol: Option<TokenSymbol>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub precision: Option<TokenPrecision>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct TokenAccount {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<TokenAccountURL>,
	#[serde(rename = "tokenURL", skip_serializing_if = "Option::is_none")]
	pub token_url: Option<TokenURL>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct TokenTx {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub from: Option<SenderTokenAccount>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub to: Option<ReceiverSTokenAccountS>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub meta: Option<Metadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct TokenAccountWithBalance {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<TokenAccountURL>,
	#[serde(rename = "tokenURL", skip_serializing_if = "Option::is_none")]
	pub token_url: Option<TokenURL>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub balance: Option<Balance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyOfStringXtH3HHU8ADIAccumulateDigitalIdentitySignerInteger2AHOqbcQString7GgoCKrmStringWcSuVYcMTokenSignerInteger2AHOqbcQString7GgoCKrmStringWcSuVYcMTokenAccountSignerInteger2AHOqbcQString7GgoCKrmString7GgoCKrmTokenTxSignerInteger2AHOqbcQString7GgoCKrmADIAccumulateDigitalIdentityADIAccumulateDigitalIdentityTokenTokenTokenAccountWithBalanceTokenAccountTokenTxTokenTx
{
	StringXtH3HHU8(StringXtH3HHU8),
	ADIAccumulateDigitalIdentity(ADIAccumulateDigitalIdentity),
	Signer(Signer),
	Integer2AHOqbcQ(Integer2AHOqbcQ),
	String7GgoCKrm(String7GgoCKrm),
	StringWcSuVYcM(StringWcSuVYcM),
	Token(Token),
	TokenAccount(TokenAccount),
	TokenTx(TokenTx),
	TokenAccountWithBalance(TokenAccountWithBalance),
}

jsonrpc_client!(pub struct AccumulateAPI {
  pub fn Adi(&mut self, url: StringXtH3HHU8) -> RpcRequest<ADIAccumulateDigitalIdentity>;
pub fn AdiCreate(&mut self, adi: ADIAccumulateDigitalIdentity, signer: Signer, timestamp: Integer2AHOqbcQ, sig: String7GgoCKrm) -> RpcRequest<ADIAccumulateDigitalIdentity>;
pub fn Token(&mut self, url: StringWcSuVYcM) -> RpcRequest<Token>;
pub fn TokenCreate(&mut self, token: Token, signer: Signer, timestamp: Integer2AHOqbcQ, sig: String7GgoCKrm) -> RpcRequest<Token>;
pub fn TokenAccount(&mut self, url: StringWcSuVYcM) -> RpcRequest<TokenAccountWithBalance>;
pub fn TokenAccountCreate(&mut self, tokenAccount: TokenAccount, signer: Signer, timestamp: Integer2AHOqbcQ, sig: String7GgoCKrm) -> RpcRequest<TokenAccount>;
pub fn TokenTx(&mut self, hash: String7GgoCKrm) -> RpcRequest<TokenTx>;
pub fn TokenTxCreate(&mut self, tokenTx: TokenTx, signer: Signer, timestamp: Integer2AHOqbcQ, sig: String7GgoCKrm) -> RpcRequest<TokenTx>;
});
