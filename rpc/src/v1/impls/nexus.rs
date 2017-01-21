use util::{H256, Address, Bytes};
use rlp::*;

use jsonrpc_core::Error;
use v1::types::{H256 as RpcH256, U256 as RpcU256, H160 as RpcH160, Bytes as RpcBytes, Clue};
use v1::traits::Nexus;

use std::sync::{Arc, Weak};
use ethcore::header::Header;
use ethcore::client::{BlockChainClient, TransactionID, BlockID, CallAnalytics};
use ethcore::views::{HeaderView, BodyView};
use ethcore::transaction::{Action, LocalizedTransaction};

/// High-quality Nexus client context
pub struct NexusClient<C> where C: BlockChainClient {
  client: Weak<C>,
}

impl<C> NexusClient<C> where C: BlockChainClient {
  /// Make a high-quality Nexus client
  pub fn new(client: &Arc<C>) -> Self {
    NexusClient { client: Arc::downgrade(client) }
  }
}

impl<C> Nexus for NexusClient<C> where C: BlockChainClient + 'static {
  fn stalk(&self, origin_tx: RpcH256, account: RpcH160) -> Result<Vec<Clue>, Error> {
    let client = take_weak!(self.client);
    
    let t = client.transaction(
      TransactionID::Hash(RpcH256::into(origin_tx.clone()))
    ).expect("origin tx");
    let receipt = client.transaction_receipt(
      TransactionID::Hash(RpcH256::into(origin_tx.clone()))
    ).expect("origin tx receipt");
    
    let first_block_number = t.block_number;
    let bytes: Bytes = client.block_header(BlockID::Latest).expect("latest block header");
    let header = HeaderView::new(&bytes);
    let mut clues = vec!();

    println!("blocks {} .. {}", first_block_number, header.number());

    let mut n = 0;

    let acct = RpcH160::into(account);

    let txAnalytics = CallAnalytics {
      transaction_tracing: true,
      vm_tracing: false,
      state_diffing: false,
    };

    for number in first_block_number .. header.number() {
      let hash = client.block_hash(BlockID::Number(number)).expect("block hash");
      let body = client.block_body(BlockID::Number(number)).expect("block body");
      for tx in BodyView::new(&body).localized_transactions(&hash, number).iter() {

        if tx.action == Action::Call(acct) {
          n += 1;
          match client.replay(TransactionID::Hash(tx.hash()), txAnalytics) {
            Ok(x) => println!("{:?}", x.trace),
            Err(e) => println!("{}", e),
          }
        }

        // let data = RpcBytes::new(vec!());
        // Clue {
        //   txhash: RpcH256::from(tx.signed.hash()),
        //   blockhash: RpcH256::from(tx.block_hash),
        //   index: RpcU256::from(tx.transaction_index),
        //   from: RpcH160::from(tx.sender().unwrap()),
        //   time: RpcU256::from(0),
        //   input: data,
        //   logs: vec!(),
        // };
        ()
      }
      if number % 1000 == 0 {
        let beg = first_block_number as f64;
        let num = number as f64;
        let top = header.number() as f64;
        println!("XXX {} {}/{} ({}%)", n, num, top, 1000.0 * (num - beg) / (top - beg));
      }
    }

    Ok(clues)
  }
}