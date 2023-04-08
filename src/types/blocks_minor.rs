use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinorBlocksRangeResponse {
    pub record_type: MinorBlocksRangeResponseRecordType,
    pub records: Vec<MinorBlocksRangeResponseRecord>,
    pub start: i64,
    pub total: i64,
    pub last_block_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MinorBlocksRangeResponseRecordType {
    Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinorBlocksRangeResponseRecord {
    pub record_type: StickyRecordType,
    pub index: i64,
    pub time: String,
    pub source: Source,
    pub entries: Entries,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entries {
    pub record_type: MinorBlocksRangeResponseRecordType,
    pub records: Vec<EntriesRecord>,
    pub start: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntriesRecord {
    pub record_type: PurpleRecordType,
    pub account: AccountEnum,
    pub name: Name,
    #[serde(rename = "type")]
    pub purple_type: RecordType,
    pub index: i64,
    pub entry: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountEnum {
    #[serde(rename = "acc://bvn-Apollo.acme/anchors")]
    AccBvnApolloAcmeAnchors,
    #[serde(rename = "acc://c473f235e56dede5c2ad5c8152df70ee1244413dbfaa1cd2")]
    AccC473F235E56Dede5C2Ad5C8152Df70Ee1244413Dbfaa1Cd2,
    #[serde(rename = "acc://c473f235e56dede5c2ad5c8152df70ee1244413dbfaa1cd2/ACME")]
    AccC473F235E56Dede5C2Ad5C8152Df70Ee1244413Dbfaa1Cd2Acme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Name {
    #[serde(rename = "anchor-sequence")]
    AnchorSequence,
    Main,
    Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordType {
    #[serde(rename = "blockAnchor")]
    BlockAnchor,
    Signature,
    #[serde(rename = "signatureRequest")]
    SignatureRequest,
    Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PurpleRecordType {
    #[serde(rename = "chainEntry")]
    ChainEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub record_type: ValueRecordType,
    pub id: String,
    pub message: ValueMessage,
    pub status: Status,
    pub status_no: i64,
    pub result: Result,
    pub received: Option<i64>,
    pub produced: Cause,
    pub cause: Cause,
    pub signatures: ValueSignatures,
    pub sequence: Result,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cause {
    pub record_type: MinorBlocksRangeResponseRecordType,
    pub start: i64,
    pub total: i64,
    pub records: Option<Vec<CauseRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CauseRecord {
    pub record_type: FluffyRecordType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FluffyRecordType {
    #[serde(rename = "txID")]
    TxId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueMessage {
    #[serde(rename = "type")]
    pub message_type: RecordType,
    pub signature: Option<Signature>,
    pub anchor: Option<MessageAnchor>,
    pub authority: Option<AccountEnum>,
    #[serde(rename = "txID")]
    pub tx_id: Option<String>,
    pub cause: Option<String>,
    pub transaction: Option<FluffyTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAnchor {
    #[serde(rename = "type")]
    pub anchor_type: AnchorType,
    pub message: AnchorMessage,
    pub source: Source,
    pub destination: Source,
    pub number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorType {
    Sequenced,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "acc://bvn-Apollo.acme")]
    AccBvnApolloAcme,
    #[serde(rename = "acc://bvn-Chandrayaan.acme")]
    AccBvnChandrayaanAcme,
    #[serde(rename = "acc://dn.acme")]
    AccDnAcme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorMessage {
    #[serde(rename = "type")]
    pub message_type: RecordType,
    pub transaction: PurpleTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurpleTransaction {
    pub header: PurpleHeader,
    pub body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    #[serde(rename = "type")]
    pub body_type: BodyType,
    pub source: Option<Source>,
    pub minor_block_index: Option<i64>,
    pub root_chain_index: Option<i64>,
    pub root_chain_anchor: Option<String>,
    pub state_tree_anchor: Option<String>,
    pub receipts: Option<Vec<Receipt>>,
    pub make_major_block_time: Option<String>,
    pub recipient: Option<String>,
    pub entry: Option<BodyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BodyType {
    #[serde(rename = "blockValidatorAnchor")]
    BlockValidatorAnchor,
    #[serde(rename = "directoryAnchor")]
    DirectoryAnchor,
    #[serde(rename = "writeDataTo")]
    WriteDataTo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub data: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub anchor: ReceiptAnchor,
    pub root_chain_receipt: RootChainReceipt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptAnchor {
    pub source: Source,
    pub minor_block_index: i64,
    pub root_chain_index: i64,
    pub root_chain_anchor: String,
    pub state_tree_anchor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootChainReceipt {
    pub start: String,
    pub start_index: i64,
    pub end: String,
    pub end_index: i64,
    pub anchor: String,
    pub entries: Vec<EntryElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryElement {
    pub hash: String,
    pub right: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurpleHeader {
    pub principal: AccountEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    #[serde(rename = "type")]
    pub signature_type: SignatureType,
    pub public_key: Option<String>,
    pub signature: Option<String>,
    pub signer: Option<Signer>,
    pub signer_version: Option<i64>,
    pub timestamp: Option<i64>,
    pub transaction_hash: Option<String>,
    pub origin: Option<AccountEnum>,
    pub authority: Option<AccountEnum>,
    #[serde(rename = "txID")]
    pub tx_id: Option<String>,
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureType {
    Authority,
    Ed25519,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Signer {
    #[serde(rename = "acc://c473f235e56dede5c2ad5c8152df70ee1244413dbfaa1cd2/ACME")]
    AccC473F235E56Dede5C2Ad5C8152Df70Ee1244413Dbfaa1Cd2Acme,
    #[serde(rename = "acc://dn.acme/network")]
    AccDnAcmeNetwork,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluffyTransaction {
    pub header: FluffyHeader,
    pub body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluffyHeader {
    pub principal: Option<AccountEnum>,
    pub initiator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueRecordType {
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "type")]
    pub result_type: AnchorType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSignatures {
    pub record_type: MinorBlocksRangeResponseRecordType,
    pub start: i64,
    pub total: i64,
    pub records: Option<Vec<PurpleRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleRecord {
    pub record_type: TentacledRecordType,
    pub account: AccountClass,
    pub signatures: RecordSignatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountClass {
    #[serde(rename = "type")]
    pub account_type: AccountType,
    pub url: AccountEnum,
    pub credit_balance: Option<i64>,
    pub last_used_on: Option<i64>,
    pub token_url: Option<String>,
    pub balance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    #[serde(rename = "liteIdentity")]
    LiteIdentity,
    #[serde(rename = "liteTokenAccount")]
    LiteTokenAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TentacledRecordType {
    #[serde(rename = "signatureSet")]
    SignatureSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordSignatures {
    pub record_type: MinorBlocksRangeResponseRecordType,
    pub records: Vec<FluffyRecord>,
    pub start: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyRecord {
    pub record_type: ValueRecordType,
    pub id: String,
    pub message: RecordMessage,
    pub historical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordMessage {
    #[serde(rename = "type")]
    pub message_type: PurpleType,
    pub signature: Option<Signature>,
    #[serde(rename = "txID")]
    pub tx_id: String,
    pub authority: Option<AccountEnum>,
    pub cause: Option<String>,
    pub paid: Option<i64>,
    pub payer: Option<AccountEnum>,
    pub initiator: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PurpleType {
    #[serde(rename = "creditPayment")]
    CreditPayment,
    Signature,
    #[serde(rename = "signatureRequest")]
    SignatureRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Delivered,
    Remote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StickyRecordType {
    #[serde(rename = "minorBlock")]
    MinorBlock,
}
