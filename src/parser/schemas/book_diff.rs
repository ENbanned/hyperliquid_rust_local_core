use sonic_rs::{Deserialize, Serialize};
use super::common::Side;

#[derive(Debug, Deserialize, Serialize)]
pub struct BookDiff {
    pub user: String,
    pub oid: u64,
    pub coin: String,
    pub side: Side,
    pub px: String,
    pub raw_book_diff: RawBookDiff,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RawBookDiff {
    New { new: NewOrder },
    Update { update: UpdateOrder },
    Remove(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewOrder {
    pub sz: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateOrder {
    #[serde(rename = "origSz")]
    pub orig_sz: String,
    #[serde(rename = "newSz")]
    pub new_sz: String,
}