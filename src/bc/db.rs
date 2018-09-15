use bc::block::Block;
use bc::BlockChain;
use ctx::CONTEXT;
use rusqlite::Connection;

pub struct Db {
    conn: Connection,
}

impl Db {
    fn new() -> Self {
        let ctx = CONTEXT.read().unwrap();

        let mut db: Db = match &ctx.database {
            Some(path) => Db {
                conn: Connection::open(path).unwrap(),
            },
            None => Db {
                conn: Connection::open_in_memory().unwrap(),
            },
        };

        db.setup_table();
        db
    }

    fn setup_table(&mut self) {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS block (
                   idx UNSIGNED BIG INT,
                   id VARCHAR(64),
                   key VARCHAR(255) NOT NULL,
                   value VARCHAR(2048) NOT NULL,
                   prev_hash VARCHAR(64),
                   self_hash VARHCAR(64)
                 )",
                &[],
            )
            .unwrap();

        self.conn
            .execute(
                "CREATE UNIQUE INDEX IF NOT EXISTS blockidx ON block (idx, id)",
                &[],
            )
            .unwrap();

        self.conn
            .execute("CREATE INDEX IF NOT EXISTS blockidxkey ON block (key)", &[])
            .unwrap();
    }
}

impl BlockChain for Db {
    fn new() -> Self {
        let mut db = Db::new();

        db.setup_table();

        if db.len() == 0 {
            db.push_block(Block::genesis()).unwrap();
        }

        db
    }

    fn push_block(&mut self, b: Block) -> Result<Block, String> {
        try!(b.valid(self));

        self.conn
            .execute(
                "INSERT INTO block (idx, id, key, value, prev_hash, self_hash) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                &[&b.idx, &b.id, &b.key, &b.value, &b.prev_hash, &b.self_hash],
            )
            .unwrap();

        Ok(b)
    }

    fn get_by_idx(&self, idx: u32) -> Option<Block> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT idx, id, key, value, prev_hash, self_hash
                 FROM block WHERE idx = ?1 LIMIT 1",
            )
            .unwrap();

        let mut b_iter =
            stmt.query_map(&[&idx], |row| Block {
                idx: row.get(0),
                id: row.get(1),
                key: row.get(2),
                value: row.get(3),
                prev_hash: row.get(4),
                self_hash: row.get(5),
            }).unwrap()
                .flat_map(|b| b);

        b_iter.next()
    }

    fn get_by_id(&self, id: &String) -> Option<Block> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT idx, id, key, value, prev_hash, self_hash
                 FROM block where id = ?1 LIMIT 1",
            )
            .unwrap();
        let mut b_iter =
            stmt.query_map(&[id], |row| Block {
                idx: row.get(0),
                id: row.get(1),
                key: row.get(2),
                value: row.get(3),
                prev_hash: row.get(4),
                self_hash: row.get(5),
            }).unwrap()
                .flat_map(|b| b);

        b_iter.next()
    }

    fn get_by_key(&self, key: &String) -> Option<Block> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT idx, id, key, value, prev_hash, self_hash
                 FROM block WHERE key = ?1 ORDER BY idx DESC LIMIT 1",
            )
            .unwrap();

        let mut b_iter =
            stmt.query_map(&[key], |row| Block {
                idx: row.get(0),
                id: row.get(1),
                key: row.get(2),
                value: row.get(3),
                prev_hash: row.get(4),
                self_hash: row.get(5),
            }).unwrap()
                .flat_map(|b| b);

        b_iter.next()
    }

    fn history_by_key(&self, key: &String) -> Vec<Block> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT idx, id, key, value, prev_hash, self_hash
                 FROM block WHERE key = ?1",
            )
            .unwrap();

        let b_iter =
            stmt.query_map(&[key], |row| Block {
                idx: row.get(0),
                id: row.get(1),
                key: row.get(2),
                value: row.get(3),
                prev_hash: row.get(4),
                self_hash: row.get(5),
            }).unwrap();

        let b = b_iter.flat_map(|b| b).collect();

        b
    }

    fn last(&self) -> Option<Block> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT idx, id, key, value, prev_hash, self_hash
                 FROM block ORDER BY idx DESC LIMIT 1",
            )
            .unwrap();

        let mut b_iter =
            stmt.query_map(&[], |row| Block {
                idx: row.get(0),
                id: row.get(1),
                key: row.get(2),
                value: row.get(3),
                prev_hash: row.get(4),
                self_hash: row.get(5),
            }).unwrap()
                .flat_map(|b| b);

        b_iter.next()
    }

    fn range(&self, first: u32, last: u32) -> Vec<Block> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT idx, id, key, value, prev_hash, self_hash
                 FROM block WHERE idx >= ?1 AND idx <= ?2",
            )
            .unwrap();

        let b_iter =
            stmt.query_map(&[&first, &last], |row| Block {
                idx: row.get(0),
                id: row.get(1),
                key: row.get(2),
                value: row.get(3),
                prev_hash: row.get(4),
                self_hash: row.get(5),
            }).unwrap();

        let b = b_iter.flat_map(|b| b).collect();

        b
    }

    fn till(&self, first: u32) -> Vec<Block> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT idx, id, key, value, prev_hash, self_hash
                 FROM block WHERE idx >= ?1",
            )
            .unwrap();

        let b_iter =
            stmt.query_map(&[&first], |row| Block {
                idx: row.get(0),
                id: row.get(1),
                key: row.get(2),
                value: row.get(3),
                prev_hash: row.get(4),
                self_hash: row.get(5),
            }).unwrap();

        let b = b_iter.flat_map(|b| b).collect();

        b
    }

    fn cut(&mut self, idx: u32) -> Vec<Block> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT idx, id, key, value, prev_hash, self_hash
                 FROM block WHERE idx >= ?1",
            )
            .unwrap();

        let b_iter =
            stmt.query_map(&[&idx], |row| Block {
                idx: row.get(0),
                id: row.get(1),
                key: row.get(2),
                value: row.get(3),
                prev_hash: row.get(4),
                self_hash: row.get(5),
            }).unwrap();

        let b = b_iter.flat_map(|b| b).collect();

        self.conn
            .execute("DELETE FROM block where idx >= ?1", &[&idx])
            .unwrap();

        b
    }

    fn has_id(&self, id: &String) -> bool {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM block where id = ?1")
            .unwrap();

        let b_iter: Vec<String> = stmt
            .query_map(&[id], |row| row.get(0))
            .unwrap()
            .flat_map(|i| i)
            .collect();

        b_iter.len() > 0
    }
}
