use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MajorBlocksRangeResponse {
    pub record_type: MajorBlocksRangeResponseRecordType,
    pub records: Vec<Option<Record>>,
    pub start: i64,
    pub total: i64,
    pub last_block_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MajorBlocksRangeResponseRecordType {
    Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub record_type: RecordRecordType,
    pub index: i64,
    pub time: String,
    pub minor_blocks: MinorBlocks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinorBlocks {
    pub record_type: MajorBlocksRangeResponseRecordType,
    pub start: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordRecordType {
    #[serde(rename = "majorBlock")]
    MajorBlock,
}


////////////////////////////////////////////////////////////////////////////////
