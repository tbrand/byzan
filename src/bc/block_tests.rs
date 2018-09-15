use bc::mem::Mem;
use bc::BlockChain;
use bc::block::*;
use proto::*;

#[test]
fn block_new() {
    let b = Block::new(
        0,
        String::from("id"),
        String::from("key"),
        String::from("value"),
        String::from("prev_hash"),
    );
    assert_eq!(b.idx, 0);
    assert_eq!(b.id, String::from("id"));
    assert_eq!(b.key, String::from("key"));
    assert_eq!(b.value, String::from("value"));
    assert_eq!(
        b.get_hash(),
        String::from("7dc5bd97731fb94efbb8bbe434a9f29d64b3c38084d7818690c9b720c86f3c20")
    );
}

#[test]
fn into() {
    let b = Block::new(
        1,
        String::from("id"),
        String::from("key"),
        String::from("value"),
        String::from("prev_hash"),
    );
    let b: byzan::Block = b.into();
    assert_eq!(b.get_idx(), 1);
    assert_eq!(b.id, String::from("id"));
    assert_eq!(b.get_key(), String::from("key"));
    assert_eq!(b.get_value(), String::from("value"));
    assert_eq!(b.get_prev_hash(), String::from("prev_hash"));
    assert_eq!(
        b.get_self_hash(),
        String::from("a5e83640dfb8513f0e3b066d73c34a0ee057a9420ac83de49751f11dd8c1699c")
    );
}

#[test]
fn from() {
    let mut b = byzan::Block::new();
    b.set_idx(1);
    b.set_id(String::from("id"));
    b.set_key(String::from("key"));
    b.set_value(String::from("value"));
    b.set_prev_hash(String::from("prev_hash"));
    b.set_self_hash(String::from("self_hash"));
    let b = Block::from(b);
    assert_eq!(b.idx, 1);
    assert_eq!(b.id, String::from("id"));
    assert_eq!(b.key, String::from("key"));
    assert_eq!(b.value, String::from("value"));
    assert_eq!(b.prev_hash, String::from("prev_hash"));
    assert_eq!(b.self_hash, String::from("self_hash"));
}

#[test]
fn block_genesis() {
    let b = Block::genesis();
    assert_eq!(b.idx, 0);
    assert_eq!(b.id, String::from("genesis"));
    assert_eq!(b.key, String::from("genesis"));
    assert_eq!(b.value, String::from("genesis"));
    assert_eq!(
        b.self_hash,
        String::from("495298f3856aca18c2e7f8420829df42b24dd43047b49e474bbebe875e41aa00"),
    );
}

#[test]
fn new_block_new() {
    let new_block = NewBlock::new(String::from("id"), String::from("key"), String::from("value"));
    assert_eq!(new_block.id, String::from("id"));
    assert_eq!(new_block.key, String::from("key"));
    assert_eq!(new_block.value, String::from("value"));
}

#[test]
fn new_block_from() {
    let mut new_block = byzan::NewBlock::new();
    new_block.set_id(String::from("id"));
    new_block.set_key(String::from("key"));
    new_block.set_value(String::from("value"));

    let new_block = NewBlock::from(new_block);
    assert_eq!(new_block.id, String::from("id"));
    assert_eq!(new_block.key, String::from("key"));
    assert_eq!(new_block.value, String::from("value"));
}

#[test]
fn valid_prev_hash() {
    let b = Block::new(0, String::from(""), String::from(""), String::from(""), String::from(""));
    let i = Block::new(1, String::from(""), String::from(""), String::from(""), b.self_hash.clone());
    let r = i.valid_prev_hash(&b);

    assert_eq!(r, Ok(()));
}

#[test]
fn valid_prev_hash_invalid_index() {
    let b = Block::new(0, String::from(""), String::from(""), String::from(""), String::from(""));
    let i = Block::new(2, String::from(""), String::from(""), String::from(""), String::from(""));
    let r = i.valid_prev_hash(&b);

    assert_eq!(r, Err(String::from("invalid index")));
}

#[test]
fn valid_prev_hash_invalid_prev_hash() {
    let b = Block::new(0, String::from(""), String::from(""), String::from(""), String::from(""));
    let i = Block::new(1, String::from(""), String::from(""), String::from(""), String::from(""));
    let r = i.valid_prev_hash(&b);

    assert_eq!(r, Err(String::from("invalid prev hash")));
}

#[test]
fn valid_id() {
    let m = Mem::new();
    let b = Block::new(1, String::from(""), String::from(""), String::from(""), String::from(""));
    let r = b.valid_id(&m);

    assert_eq!(r, Ok(()));
}

#[test]
fn valid_id_already_included() {
    let mut m = Mem::new();

    let h = m.last().unwrap().self_hash.clone();
    let b0 = Block::new(1, String::from("hoge"), String::from(""), String::from(""), h);
    let b1 = Block::new(2, String::from("hoge"), String::from(""), String::from(""), b0.self_hash.clone());

    let _ = m.push_block(b0);
    let r = b1.valid_id(&m);

    assert_eq!(r, Err(String::from("the block is already included")));
}

#[test]
fn valid_hash() {
    let b = Block::new(1, String::from(""), String::from(""), String::from(""), String::from(""));
    let r = b.valid_hash();

    assert_eq!(r, Ok(()));
}

#[test]
fn light_block_new() {
    let light_block = LightBlock::new(1, String::from("hash"));
    assert_eq!(light_block.idx, 1);
    assert_eq!(light_block.self_hash, String::from("hash"));
}

#[test]
fn light_block_from() {
    let b = Block::new(
        1,
        String::from("id"),
        String::from("key"),
        String::from("value"),
        String::from("prev_hash"),
    );

    let light_block: LightBlock = b.clone().into();
    assert_eq!(light_block.idx, 1);
    assert_eq!(light_block.self_hash, b.self_hash);
}
