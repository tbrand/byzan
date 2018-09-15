use bc::block::Block;
use bc::BlockChain;
use std::collections::HashMap;

pub struct Mem {
    blocks: Vec<Block>,
    caches: HashMap<String, u32>,
}

impl BlockChain for Mem {
    fn new() -> Self {
        let mut blockchain = Mem {
            blocks: Vec::<Block>::new(),
            caches: HashMap::<String, u32>::new(),
        };

        blockchain.push_block(Block::genesis()).unwrap();
        blockchain
    }

    fn push_block(&mut self, b: Block) -> Result<Block, String> {
        try!(b.valid(self));

        let b_idx = b.idx;

        self.caches.insert(b.key.clone(), b_idx);
        self.blocks.push(b.clone());

        Ok(b)
    }

    fn get_by_idx(&self, idx: u32) -> Option<Block> {
        if idx <= self.len() - 1 {
            return Some(self.blocks[idx as usize].clone());
        }

        None
    }

    fn get_by_id(&self, id: &String) -> Option<Block> {
        for b in self.blocks.iter().rev() {
            if &b.id == id {
                return Some(b.clone());
            }
        }

        None
    }

    fn get_by_key(&self, key: &String) -> Option<Block> {
        if let Some(idx) = self.caches.get(key) {
            return self.get_by_idx(*idx);
        }

        for b in self.blocks.iter().rev() {
            if &b.key == key {
                return Some(b.clone());
            }
        }

        None
    }

    fn history_by_key(&self, key: &String) -> Vec<Block> {
        let history: Vec<Block> = self
            .blocks
            .iter()
            .filter(|b| b.key == key.clone())
            .map(|b| b.clone())
            .collect();

        history
    }

    fn last(&self) -> Option<Block> {
        if self.len() == 0 {
            return None;
        }

        Some(self.blocks.last().unwrap().clone())
    }

    fn len(&self) -> u32 {
        self.blocks.len() as u32
    }

    fn range(&self, first: u32, last: u32) -> Vec<Block> {
        if first > last {
            return Vec::<Block>::new();
        }

        if first >= self.len() {
            return Vec::<Block>::new();
        }

        let last = if last >= self.len() {
            self.len() - 1
        } else {
            last
        };

        let subset: Vec<Block> = self.blocks[first as usize..last as usize + 1].to_vec();

        subset
    }

    fn till(&self, first: u32) -> Vec<Block> {
        if first > self.len() - 1 {
            return Vec::<Block>::new();
        }

        let subset: Vec<Block> = self.blocks[first as usize..self.len() as usize].to_vec();

        subset
    }

    fn cut(&mut self, idx: u32) -> Vec<Block> {
        let (remain, removed) = {
            let (left, right) = self.blocks.split_at(idx as usize);
            (left.to_vec(), right.to_vec())
        };

        self.blocks = remain;
        removed
    }

    fn has_id(&self, id: &String) -> bool {
        for b in self.blocks.iter().rev() {
            if &b.id == id {
                return true;
            }
        }

        false
    }
}
