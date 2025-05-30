use revm::primitives::{Address, Bytes, HashMap, B256};
use serde::Deserialize;

use crate::{transaction::TxPartIndices, AccountInfo};

/// State test indexed state result deserialization.
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Test {
    pub expect_exception: Option<String>,

    /// Indexes
    pub indexes: TxPartIndices,
    /// Post state hash
    pub hash: B256,
    /// Post state
    #[serde(default)]
    pub post_state: HashMap<Address, AccountInfo>,

    /// Logs root
    pub logs: B256,

    /// Output state.
    ///
    /// Note: Not used.
    #[serde(default)]
    state: HashMap<Address, AccountInfo>,

    /// Tx bytes
    pub txbytes: Option<Bytes>,
}
