use bc::block::*;
use bc::BlockChain;

macro_rules! define_tests {
    ($bc:ident) => {
        #[test]
        fn __new() {
            new::<$bc>();
        }

        #[test]
        fn __create_block() {
            create_block::<$bc>();
        }

        #[test]
        fn __push_block() {
            push_block::<$bc>();
        }

        #[test]
        fn __get_by_idx() {
            get_by_idx::<$bc>();
        }

        #[test]
        fn __get_by_key() {
            get_by_key::<$bc>();
        }

        #[test]
        fn __history_by_key() {
            history_by_key::<$bc>();
        }

        #[test]
        fn __last() {
            last::<$bc>();
        }

        #[test]
        fn __last_hash() {
            last_hash::<$bc>();
        }

        #[test]
        fn __range() {
            range::<$bc>();
        }

        #[test]
        fn __return_empty_vec_range() {
            return_empty_vec_range::<$bc>();
        }

        #[test]
        fn __till() {
            till::<$bc>();
        }

        #[test]
        fn __return_empty_vec_till() {
            return_empty_vec_till::<$bc>();
        }

        #[test]
        fn __cut() {
            cut::<$bc>();
        }

        #[test]
        fn __len() {
            len::<$bc>();
        }
    };
}

pub fn new<B: BlockChain>() {
    let bc = B::new();
    assert_eq!(bc.len(), 1);
    assert_eq!(bc.get_by_idx(0).unwrap(), Block::genesis());
}

pub fn create_block<B: BlockChain>() {
    let id = String::from("id");
    let key = String::from("key");
    let value = String::from("value");
    let bc = B::new();
    let nb = NewBlock::new(id.clone(), key.clone(), value.clone());
    let b = bc.create_block(nb);
    assert_eq!(b.idx, 1);
    assert_eq!(b.id, id.clone());
    assert_eq!(b.key, key.clone());
    assert_eq!(b.value, value.clone());
}

pub fn push_block<B: BlockChain>() {
    let id = String::from("id");
    let key = String::from("key");
    let value = String::from("value");
    let mut bc = B::new();
    let nb = NewBlock::new(id.clone(), key.clone(), value.clone());
    let b = bc.create_block(nb);
    let b = bc.push_block(b).unwrap();

    assert_eq!(bc.len(), 2);
    assert_eq!(bc.get_by_idx(1).unwrap(), b);
}

pub fn get_by_idx<B: BlockChain>() {
    let id = String::from("id");
    let key = String::from("key");
    let value = String::from("value");
    let mut bc = B::new();
    let nb = NewBlock::new(id.clone(), key.clone(), value.clone());
    let b = bc.create_block(nb);
    let b = bc.push_block(b).unwrap();

    assert_eq!(bc.get_by_idx(1).unwrap(), b);
}

pub fn get_by_key<B: BlockChain>() {
    let key = String::from("key");
    let mut bc = B::new();

    for i in 0..10 {
        let nb = NewBlock::new(format!("id-{}", i), key.clone(), format!("value-{}", i));
        let b = bc.create_block(nb);

        bc.push_block(b.clone()).unwrap();

        assert_eq!(bc.get_by_key(&key).unwrap(), b);
    }
}

pub fn history_by_key<B: BlockChain>() {
    let key = String::from("key");
    let value = String::from("value");
    let mut bc = B::new();

    for i in 0..3 {
        let id = format!("id-{}", i);
        let nb = NewBlock::new(id, key.clone(), value.clone());
        let b = bc.create_block(nb);
        bc.push_block(b).unwrap();
    }

    let history = bc.history_by_key(&key);

    assert_eq!(
        history,
        vec![
            bc.get_by_idx(1).unwrap(),
            bc.get_by_idx(2).unwrap(),
            bc.get_by_idx(3).unwrap(),
        ]
    );
}

pub fn last<B: BlockChain>() {
    let mut bc = B::new();

    for i in 0..10 {
        let nb = NewBlock::new(format!("id-{}", i), format!("key-{}", i), format!("value-{}", i));
        let b = bc.create_block(nb);
        let b = bc.push_block(b).unwrap();

        assert_eq!(bc.last().unwrap(), b);
    }
}

pub fn last_hash<B: BlockChain>() {
    let mut bc = B::new();

    for i in 0..10 {
        let nb = NewBlock::new(format!("id-{}", i), format!("key-{}", i), format!("value-{}", i));
        let b = bc.create_block(nb);
        let b = bc.push_block(b).unwrap();

        assert_eq!(bc.last_hash().unwrap(), b.self_hash);
    }
}

pub fn range<B: BlockChain>() {
    let mut bc = B::new();

    for i in 0..10 {
        let nb = NewBlock::new(format!("id-{}", i), format!("key-{}", i), format!("value-{}", i));
        let b = bc.create_block(nb);

        bc.push_block(b).unwrap();
    }

    let range = bc.range(1, 8);

    assert_eq!(range.len(), 8);
    assert_eq!(range.first().unwrap().key, "key-0");
    assert_eq!(range.last().unwrap().key, "key-7");
}

pub fn return_empty_vec_range<B: BlockChain>() {
    let bc = B::new();

    let range = bc.range(100, 120);
    assert_eq!(range.len(), 0);

    let range = bc.range(1, 0);
    assert_eq!(range.len(), 0);
}

pub fn till<B: BlockChain>() {
    let mut bc = B::new();

    for i in 0..10 {
        let nb = NewBlock::new(format!("id-{}", i), format!("key-{}", i), format!("value-{}", i));
        let b = bc.create_block(nb);

        bc.push_block(b).unwrap();
    }

    let till = bc.till(5);

    assert_eq!(till.len(), 6);
    assert_eq!(till.first().unwrap().key, "key-4");
    assert_eq!(till.last().unwrap().key, "key-9");
}

pub fn return_empty_vec_till<B: BlockChain>() {
    let bc = B::new();

    let till = bc.till(100);
    assert_eq!(till.len(), 0);
}

pub fn cut<B: BlockChain>() {
    let mut bc = B::new();

    for i in 0..10 {
        let nb = NewBlock::new(format!("id-{}", i), format!("key-{}", i), format!("value-{}", i));
        let b = bc.create_block(nb);
        bc.push_block(b).unwrap();
    }

    let bs = bc.cut(3);

    assert_eq!(bc.len(), 3);
    assert_eq!(bs.len(), 8);
}

pub fn len<B: BlockChain>() {
    let mut bc = B::new();

    for i in 0..10 {
        let nb = NewBlock::new(format!("id-{}", i), format!("key-{}", i), format!("value-{}", i));
        let b = bc.create_block(nb);
        bc.push_block(b).unwrap();
    }

    assert_eq!(bc.len(), 11);
}
