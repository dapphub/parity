//! Nexus high quality extended RPC magic
use jsonrpc_core::Error;

use v1::helpers::auto_args::Wrap;
use v1::types::{H160, H256, H512, U256, Bytes, Transaction, Clue};

build_rpc_trait! {
  /// Nexus high-quality special RPC.
  pub trait Nexus {
    /// Stalk an address using high-quality Nexus techniques.
    #[rpc(name = "nexus_stalk")]
    fn stalk(&self, H256, H160) -> Result<Vec<Clue>, Error>;
  }
}

