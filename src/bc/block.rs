use bc::BlockChain;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use proto::byzan;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub idx: u32,
    pub id: String,
    pub key: String,
    pub value: String,
    pub prev_hash: String,
    pub self_hash: String,
}

impl From<byzan::Block> for Block {
    fn from(b: byzan::Block) -> Self {
        Block {
            idx: b.get_idx(),
            id: b.get_id().to_string(),
            key: b.get_key().to_string(),
            value: b.get_value().to_string(),
            prev_hash: b.get_prev_hash().to_string(),
            self_hash: b.get_self_hash().to_string(),
        }
    }
}

impl Into<byzan::Block> for Block {
    fn into(self) -> byzan::Block {
        let mut block = byzan::Block::new();
        block.set_idx(self.idx);
        block.set_id(self.id.clone());
        block.set_key(self.key.clone());
        block.set_value(self.value.clone());
        block.set_prev_hash(self.prev_hash.clone());
        block.set_self_hash(self.self_hash.clone());
        block
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.idx == other.idx
            && self.id == other.id
            && self.key == other.key
            && self.value == other.value
            && self.prev_hash == other.prev_hash
            && self.self_hash == other.self_hash
    }
}

impl Block {
    pub fn new(idx: u32, id: String, key: String, value: String, prev_hash: String) -> Block {
        let mut b = Block {
            idx: idx,
            id: id,
            key: key,
            value: value,
            prev_hash: prev_hash,
            self_hash: String::from(""),
        };

        let hash = b.get_hash();

        b.self_hash = hash;
        b
    }

    pub fn genesis() -> Block {
        Block::new(
            0,
            String::from("genesis"),
            String::from("genesis"),
            String::from("genesis"),
            String::from("genesis"),
        )
    }

    pub fn get_hash(&self) -> String {
        let id = self.id.clone();
        let key = self.key.clone();
        let value = self.value.clone();
        let prev_hash = self.prev_hash.clone();
        let content = format!("{}", self.idx) + &id + &key + &value + &prev_hash;

        let mut sha256 = Sha256::new();
        sha256.input_str(&content);
        sha256.result_str()
    }

    pub fn valid<T: BlockChain>(&self, blockchain: &T) -> Result<(), String> {
        if blockchain.len() == 0 {
            return Ok(());
        }

        match blockchain.get_by_idx(self.idx - 1) {
            Some(prev_block) => {
                try!(self.valid_prev_hash(&prev_block));
                try!(self.valid_id(blockchain));
                try!(self.valid_hash());
            }
            None => {
                return Err(format!(
                    "[valid] failed to find local block at {}",
                    self.idx - 1
                ))
            }
        }

        Ok(())
    }

    pub fn valid_prev_hash(&self, prev_block: &Block) -> Result<(), String> {
        if prev_block.idx + 1 != self.idx {
            return Err(String::from("invalid index"));
        }

        if prev_block.self_hash != self.prev_hash {
            return Err(String::from("invalid prev hash"));
        }

        Ok(())
    }

    pub fn valid_id<T: BlockChain>(&self, blockchain: &T) -> Result<(), String> {
        if blockchain.has_id(&self.id) {
            return Err(String::from("the block is already included"));
        }

        Ok(())
    }

    pub fn valid_hash(&self) -> Result<(), String> {
        if self.get_hash() != self.self_hash {
            return Err(String::from("self hash is invalid"));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBlock {
    pub id: String,
    pub key: String,
    pub value: String,
}

impl From<byzan::NewBlock> for NewBlock {
    fn from(b: byzan::NewBlock) -> Self {
        NewBlock::new(
            b.get_id().to_string(),
            b.get_key().to_string(),
            b.get_value().to_string(),
        )
    }
}

impl From<Block> for NewBlock {
    fn from(b: Block) -> Self {
        NewBlock::new(b.id.clone(), b.key.clone(), b.value)
    }
}

impl NewBlock {
    pub fn new(id: String, key: String, value: String) -> NewBlock {
        NewBlock {
            id: id,
            key: key,
            value: value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightBlock {
    pub idx: u32,
    pub self_hash: String,
}

impl From<Block> for LightBlock {
    fn from(b: Block) -> Self {
        LightBlock::new(b.idx, b.self_hash)
    }
}

impl LightBlock {
    pub fn new(idx: u32, self_hash: String) -> LightBlock {
        LightBlock {
            idx: idx,
            self_hash: self_hash,
        }
    }
}
