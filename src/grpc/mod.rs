use bc::{block, BlockChain, BLOCKCHAIN};
use ctx;
use futures::Future;
use grpcio::{Environment, Error, Server, ServerBuilder};
use grpcio::{RpcContext, RpcStatus, RpcStatusCode, UnarySink};
use p2p::{self, api, api::RequestType};
use proto;
use proto::{byzan, byzan_grpc};
use protobuf::RepeatedField;
use std::sync::Arc;

#[derive(Clone)]
pub struct ByzanService;

pub fn server() -> Result<Server, Error> {
    let context = ctx::CONTEXT.read().unwrap();

    let env = Arc::new(Environment::new(context.threads));
    let service = proto::byzan_grpc::create_block_chain(ByzanService);
    let server = ServerBuilder::new(env)
        .register_service(service)
        .bind(context.bind_host.clone(), context.bind_port)
        .build();

    server
}

impl byzan_grpc::BlockChain for ByzanService {
    fn upsert(&self, ctx: RpcContext, req: byzan::NewBlock, sink: UnarySink<byzan::ResponseBlock>) {
        debug!("called upsert");

        let new_block = block::NewBlock::from(req);

        match BLOCKCHAIN.lock().unwrap().push_new_block(new_block) {
            Ok(block) => {
                let lb: block::LightBlock = block.clone().into();
                let res_block: byzan::Block = block.into();

                let mut res = byzan::ResponseBlock::new();
                res.set_status(String::from("ok"));
                res.set_block(res_block);

                let f = sink
                    .success(res)
                    .map_err(|e| error!("error: upsert ({})", e));

                ctx.spawn(f);

                let _ = p2p::CHANNEL
                    .0
                    .lock()
                    .unwrap()
                    .send(api::payload(RequestType::New, lb));
            }
            Err(e) => {
                let rpc_status = RpcStatus::new(RpcStatusCode::Unknown, Some(e));

                let f = sink
                    .fail(rpc_status)
                    .map_err(|e| error!("error: upsert ({})", e));

                ctx.spawn(f);
            }
        }
    }

    fn push(&self, ctx: RpcContext, req: byzan::Block, sink: UnarySink<byzan::ResponseBlock>) {
        debug!("called push");

        let block = block::Block::from(req);

        match BLOCKCHAIN.lock().unwrap().push_block(block) {
            Ok(block) => {
                let res_block: byzan::Block = block.into();

                let mut res = byzan::ResponseBlock::new();
                res.set_status(String::from("ok"));
                res.set_block(res_block);

                let f = sink.success(res).map_err(|e| error!("error: push ({})", e));

                ctx.spawn(f);
            }
            Err(e) => {
                let rpc_status = RpcStatus::new(RpcStatusCode::InvalidArgument, Some(e));

                let f = sink
                    .fail(rpc_status)
                    .map_err(|e| error!("error: push ({})", e));

                ctx.spawn(f);
            }
        }
    }

    fn get_by_key(
        &self,
        ctx: RpcContext,
        req: byzan::BlockKey,
        sink: UnarySink<byzan::ResponseBlock>,
    ) {
        debug!("called get_by_key");

        let key = req.get_key().to_string();

        match BLOCKCHAIN.lock().unwrap().get_by_key(&key) {
            Some(block) => {
                let res_block: byzan::Block = block.into();

                let mut res = byzan::ResponseBlock::new();
                res.set_status(String::from("ok"));
                res.set_block(res_block);

                let f = sink
                    .success(res)
                    .map_err(|e| error!("error: get_by_key ({})", e));

                ctx.spawn(f);
            }
            None => {
                let e = format!("error: get_by_key ({})", key);
                let rpc_status = RpcStatus::new(RpcStatusCode::NotFound, Some(e));

                let f = sink.fail(rpc_status).map_err(|e| error!("{}", e));

                ctx.spawn(f);
            }
        }
    }

    fn get_by_id(
        &self,
        ctx: RpcContext,
        req: byzan::BlockId,
        sink: UnarySink<byzan::ResponseBlock>,
    ) {
        debug!("called get_by_id");

        let id = req.get_id().to_string();

        match BLOCKCHAIN.lock().unwrap().get_by_id(&id) {
            Some(block) => {
                let res_block: byzan::Block = block.into();

                let mut res = byzan::ResponseBlock::new();
                res.set_status(String::from("ok"));
                res.set_block(res_block);

                let f = sink
                    .success(res)
                    .map_err(|e| error!("error: get_by_id ({})", e));

                ctx.spawn(f);
            }
            None => {
                let e = format!("error: get_by_id ({})", id);
                let rpc_status = RpcStatus::new(RpcStatusCode::NotFound, Some(e));

                let f = sink.fail(rpc_status).map_err(|e| error!("{}", e));

                ctx.spawn(f);
            }
        }
    }

    fn get_by_idx(
        &self,
        ctx: RpcContext,
        req: byzan::BlockIdx,
        sink: UnarySink<byzan::ResponseBlock>,
    ) {
        debug!("called get_by_idx");

        let idx = req.get_idx();

        match BLOCKCHAIN.lock().unwrap().get_by_idx(idx) {
            Some(block) => {
                let res_block: byzan::Block = block.into();

                let mut res = byzan::ResponseBlock::new();
                res.set_status(String::from("ok"));
                res.set_block(res_block);

                let f = sink
                    .success(res)
                    .map_err(|e| error!("error: get_by_idx ({})", e));

                ctx.spawn(f);
            }
            None => {
                let e = format!("error: get_by_idx ({})", idx);
                let rpc_status = RpcStatus::new(RpcStatusCode::NotFound, Some(e));

                let f = sink.fail(rpc_status).map_err(|e| error!("{}", e));

                ctx.spawn(f);
            }
        }
    }

    fn history(
        &self,
        ctx: RpcContext,
        req: byzan::BlockKey,
        sink: UnarySink<byzan::ResponseBlocks>,
    ) {
        debug!("called history_by_key");

        let key = req.get_key().to_string();
        let history = BLOCKCHAIN.lock().unwrap().history_by_key(&key);

        let res_history: Vec<byzan::Block> = history.into_iter().map(|b| b.into()).collect();

        let mut res = byzan::ResponseBlocks::new();
        res.set_status(String::from("ok"));
        res.set_blocks(RepeatedField::<byzan::Block>::from(res_history));

        let f = sink
            .success(res)
            .map_err(|e| error!("error: history ({})", e));

        ctx.spawn(f);
    }

    fn last(&self, ctx: RpcContext, _req: byzan::Empty, sink: UnarySink<byzan::ResponseBlock>) {
        debug!("called last");

        match BLOCKCHAIN.lock().unwrap().last() {
            Some(block) => {
                let res_block: byzan::Block = block.into();

                let mut res = byzan::ResponseBlock::new();
                res.set_status(String::from("ok"));
                res.set_block(res_block);

                let f = sink.success(res).map_err(|e| {
                    println!("error {}", e);
                });

                ctx.spawn(f);
            }
            None => {
                let e = format!("error: last()");
                let rpc_status = RpcStatus::new(RpcStatusCode::NotFound, Some(e));

                let f = sink.fail(rpc_status).map_err(|e| error!("{}", e));

                ctx.spawn(f);
            }
        }
    }

    fn len(&self, ctx: RpcContext, _req: byzan::Empty, sink: UnarySink<byzan::ResponseLen>) {
        debug!("called len");

        let mut res = byzan::ResponseLen::new();
        res.set_status(String::from("ok"));
        res.set_len(BLOCKCHAIN.lock().unwrap().len() as u32);

        let f = sink.success(res).map_err(|e| error!("error: len ({})", e));

        ctx.spawn(f);
    }

    fn range(
        &self,
        ctx: RpcContext,
        req: byzan::BlockRange,
        sink: UnarySink<byzan::ResponseBlocks>,
    ) {
        debug!("called range");

        let first = req.get_first();
        let last = req.get_last();

        let res_blocks: Vec<byzan::Block> = BLOCKCHAIN
            .lock()
            .unwrap()
            .range(first, last)
            .into_iter()
            .map(|b| b.into())
            .collect();

        let mut res = byzan::ResponseBlocks::new();
        res.set_status(String::from("ok"));
        res.set_blocks(RepeatedField::<byzan::Block>::from(res_blocks));

        let f = sink
            .success(res)
            .map_err(|e| error!("errory: range ({})", e));

        ctx.spawn(f);
    }

    fn till(&self, ctx: RpcContext, req: byzan::BlockTill, sink: UnarySink<byzan::ResponseBlocks>) {
        debug!("called till");

        let first = req.get_first();

        let res_blocks: Vec<byzan::Block> = BLOCKCHAIN
            .lock()
            .unwrap()
            .till(first)
            .into_iter()
            .map(|b| b.into())
            .collect();

        let mut res = byzan::ResponseBlocks::new();
        res.set_status(String::from("ok"));
        res.set_blocks(RepeatedField::<byzan::Block>::from(res_blocks));

        let f = sink.success(res).map_err(|e| error!("error: till ({})", e));

        ctx.spawn(f);
    }
}
