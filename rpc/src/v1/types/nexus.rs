use v1::types::{H160, U256, H256, Bytes};

/// Nexus high-quality log data
#[derive(Debug, Serialize)]
pub struct Log {
  pub address: H160,
  pub topics: Vec<H256>,
  pub data: Bytes,
}

/// Nexus high-quality transaction clues
#[derive(Debug, Serialize)]
pub struct Clue {
  /// Transaction hash
  pub txhash: H256,
  /// Block hash
  pub blockhash: H256,
  /// Transaction index in block
  pub index: U256,
  /// From address
  pub from: H160,
  /// Timestamp
  pub time: U256,
  /// Transaction input
  pub input: Bytes,
  /// Logs
  pub logs: Vec<Log>,
}