use bc::block::{Block, LightBlock, NewBlock};
use bc::{BlockChain, BLOCKCHAIN};
use colored::*;
use ctx::{remote_context, CONTEXT, RemoteContext};
use grpcio::{ChannelBuilder, EnvBuilder, CallOption, Error};
use proto::byzan::{BlockIdx, BlockTill};
use proto::byzan_grpc::BlockChainClient;
use serde_json;
use std::sync::Arc;
use std::time::Duration;

pub enum RequestType {
    New = 0,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub req_type: u32,
    pub context: RemoteContext,
    pub block: LightBlock,
}

pub fn payload(req_type: RequestType, block: LightBlock) -> String {
    let req = create_req(req_type, block);
    serde_json::to_string(&req).unwrap()
}

pub fn negotiate(req: &Request) -> Result<Option<LightBlock>, String> {
    let req_hash = req.block.self_hash.clone();
    let req_len = req.block.idx + 1;

    let (blockchain_hash, blockchain_len) = try!(local_bc_last());

    if req_len > blockchain_len as u32
        || (req_len == blockchain_len as u32 && req_hash != blockchain_hash)
    {
        for idx in (0..blockchain_len).rev() {
            let b = try!(local_b_by_idx(idx));
            let r = try!(remote_b_by_idx(req, idx));

            if b.self_hash == r.self_hash {
                let tb = try!(till(&req.context, idx + 1));

                if tb.len() > 0 {
                    let mut imported = tb.len();

                    //
                    // blockchain's lock is acquired from here
                    //
                    let mut blockchain = BLOCKCHAIN.lock().unwrap();
                    let c = blockchain.cut(idx + 1);
                    let nb: Vec<NewBlock> = c.iter().map(|b| NewBlock::from(b.clone())).collect();

                    for t in tb {
                        if let Err(e) = blockchain.push_block(t) {
                            warn!("[push] the block is ignored for the reason: {}", e);
                            imported -= 1;
                        }
                    }

                    for n in nb {
                        if let Err(_) = blockchain.push_new_block(n) {
                            // 一個ignoreならそれ以降無視して良さそう
                            // warn!("[push new] the block is ignored for the reason: {}", e);
                            imported -= 1;
                        }
                    }

                    let local = local().unwrap();

                    info!(
                        "---> [{}:{}] imported {} blocks",
                        local.0,
                        local.1,
                        imported.to_string().cyan().bold()
                    );

                    let lb: Option<LightBlock> = match blockchain.last() {
                        Some(b) => Some(b.into()),
                        None => None,
                    };

                    return Ok(lb);
                    //
                    // blockchain's lock is acquired till here
                    //
                }
                break;
            }
        }
    }

    Ok(None)
}

fn create_req(req_type: RequestType, block: LightBlock) -> Request {
    Request {
        req_type: req_type as u32,
        context: remote_context(),
        block: block,
    }
}

fn client(context: &RemoteContext) -> BlockChainClient {
    let remote = format!("{}:{}", &context.bind_host, context.bind_port);
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&remote);
    let client = BlockChainClient::new(ch);
    client
}

fn opt() -> CallOption {
    let opt = CallOption::default();
    opt.timeout(Duration::from_millis(500))
}

fn local() -> Result<(String, u16), String> {
    let ctx = CONTEXT.read().unwrap();
    Ok((ctx.bind_host.clone(), ctx.bind_port))
}

fn local_bc_last() -> Result<(String, u32), String> {
    let bc = BLOCKCHAIN.lock().unwrap().last().unwrap();
    Ok((bc.self_hash.clone(), bc.idx + 1 as u32))
}

fn local_b_by_idx(idx: u32) -> Result<Block, String> {
    match BLOCKCHAIN.lock().unwrap().get_by_idx(idx) {
        Some(b) => {
            return Ok(b);
        }
        None => {
            return Err(format!("failed to get block at {} on localhost", idx));
        }
    }
}

fn remote_b_by_idx(req: &Request, idx: u32) -> Result<Block, String> {
    let mut b_idx = BlockIdx::new();
    b_idx.set_idx(idx);

    match client(&req.context).get_by_idx_opt(&b_idx, opt()) {
        Ok(b) => Ok(Block::from(b.get_block().clone())),
        Err(e) => {
            return Err(handle_error(&format!("failed to get block at {}", idx), &req.context, e));
        },
    }
}

fn till(context: &RemoteContext, idx: u32) -> Result<Vec<Block>, String> {
    let mut block_till = BlockTill::new();
    block_till.set_first(idx as u32);

    match client(&context).till_opt(&block_till, opt()) {
        Ok(blocks) => {
            let blocks: Vec<Block> = blocks
                .get_blocks()
                .to_vec()
                .iter()
                .map(|b| Block::from(b.clone()))
                .collect();

            return Ok(blocks);
        }
        Err(e) => {
            return Err(handle_error(&format!("failed to exec till {}", idx), &context, e));
        }
    }
}

fn handle_error(s: &String, context: &RemoteContext, e: Error) -> String {
    let detail = match e {
        Error::RpcFailure(s) => {
            match s.details {
                Some(d) => d,
                None => String::from("unknown"),
            }
        },
        _ => String::from("unknown")
    };

    format!("{} on {}:{} ({})", s, context.bind_host, context.bind_port, detail)
}
