pub mod block;
pub mod db;
pub mod mem;

use self::block::{Block, NewBlock};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref BLOCKCHAIN: Arc<Mutex<db::Db>> = Arc::new(Mutex::new(db::Db::new()));
}

pub trait BlockChain {
    fn new() -> Self;
    fn push_block(&mut self, b: Block) -> Result<Block, String>;
    fn get_by_idx(&self, idx: u32) -> Option<Block>;
    fn get_by_id(&self, id: &String) -> Option<Block>;
    fn get_by_key(&self, key: &String) -> Option<Block>;
    fn history_by_key(&self, key: &String) -> Vec<Block>;
    fn last(&self) -> Option<Block>;
    fn range(&self, first: u32, last: u32) -> Vec<Block>;
    fn till(&self, first: u32) -> Vec<Block>;
    fn cut(&mut self, idx: u32) -> Vec<Block>;
    fn has_id(&self, id: &String) -> bool;

    fn create_block(&self, b: NewBlock) -> Block {
        Block::new(self.len(), b.id, b.key, b.value, self.last_hash().unwrap())
    }

    fn push_new_block(&mut self, b: NewBlock) -> Result<Block, String> {
        let block = self.create_block(b);

        self.push_block(block)
    }

    fn last_hash(&self) -> Option<String> {
        if let Some(b) = self.last() {
            return Some(b.self_hash.clone());
        }

        None
    }

    fn len(&self) -> u32 {
        if let Some(b) = self.last() {
            return b.idx + 1;
        }

        0
    }
}
