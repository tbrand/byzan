// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_BLOCK_CHAIN_UPSERT: ::grpcio::Method<super::byzan::NewBlock, super::byzan::ResponseBlock> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/upsert",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_PUSH: ::grpcio::Method<super::byzan::Block, super::byzan::ResponseBlock> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/push",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_GET_BY_KEY: ::grpcio::Method<super::byzan::BlockKey, super::byzan::ResponseBlock> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/get_by_key",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_GET_BY_IDX: ::grpcio::Method<super::byzan::BlockIdx, super::byzan::ResponseBlock> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/get_by_idx",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_GET_BY_ID: ::grpcio::Method<super::byzan::BlockId, super::byzan::ResponseBlock> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/get_by_id",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_LAST: ::grpcio::Method<super::byzan::Empty, super::byzan::ResponseBlock> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/last",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_LEN: ::grpcio::Method<super::byzan::Empty, super::byzan::ResponseLen> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/len",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_HISTORY: ::grpcio::Method<super::byzan::BlockKey, super::byzan::ResponseBlocks> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/history",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_RANGE: ::grpcio::Method<super::byzan::BlockRange, super::byzan::ResponseBlocks> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/range",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BLOCK_CHAIN_TILL: ::grpcio::Method<super::byzan::BlockTill, super::byzan::ResponseBlocks> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/byzan.BlockChain/till",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct BlockChainClient {
    client: ::grpcio::Client,
}

impl BlockChainClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        BlockChainClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn upsert_opt(&self, req: &super::byzan::NewBlock, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_UPSERT, req, opt)
    }

    pub fn upsert(&self, req: &super::byzan::NewBlock) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.upsert_opt(req, ::grpcio::CallOption::default())
    }

    pub fn upsert_async_opt(&self, req: &super::byzan::NewBlock, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_UPSERT, req, opt)
    }

    pub fn upsert_async(&self, req: &super::byzan::NewBlock) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.upsert_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn push_opt(&self, req: &super::byzan::Block, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_PUSH, req, opt)
    }

    pub fn push(&self, req: &super::byzan::Block) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.push_opt(req, ::grpcio::CallOption::default())
    }

    pub fn push_async_opt(&self, req: &super::byzan::Block, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_PUSH, req, opt)
    }

    pub fn push_async(&self, req: &super::byzan::Block) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.push_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_by_key_opt(&self, req: &super::byzan::BlockKey, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_GET_BY_KEY, req, opt)
    }

    pub fn get_by_key(&self, req: &super::byzan::BlockKey) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.get_by_key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_by_key_async_opt(&self, req: &super::byzan::BlockKey, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_GET_BY_KEY, req, opt)
    }

    pub fn get_by_key_async(&self, req: &super::byzan::BlockKey) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.get_by_key_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_by_idx_opt(&self, req: &super::byzan::BlockIdx, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_GET_BY_IDX, req, opt)
    }

    pub fn get_by_idx(&self, req: &super::byzan::BlockIdx) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.get_by_idx_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_by_idx_async_opt(&self, req: &super::byzan::BlockIdx, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_GET_BY_IDX, req, opt)
    }

    pub fn get_by_idx_async(&self, req: &super::byzan::BlockIdx) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.get_by_idx_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_by_id_opt(&self, req: &super::byzan::BlockId, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_GET_BY_ID, req, opt)
    }

    pub fn get_by_id(&self, req: &super::byzan::BlockId) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.get_by_id_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_by_id_async_opt(&self, req: &super::byzan::BlockId, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_GET_BY_ID, req, opt)
    }

    pub fn get_by_id_async(&self, req: &super::byzan::BlockId) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.get_by_id_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn last_opt(&self, req: &super::byzan::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_LAST, req, opt)
    }

    pub fn last(&self, req: &super::byzan::Empty) -> ::grpcio::Result<super::byzan::ResponseBlock> {
        self.last_opt(req, ::grpcio::CallOption::default())
    }

    pub fn last_async_opt(&self, req: &super::byzan::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_LAST, req, opt)
    }

    pub fn last_async(&self, req: &super::byzan::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlock>> {
        self.last_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn len_opt(&self, req: &super::byzan::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseLen> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_LEN, req, opt)
    }

    pub fn len(&self, req: &super::byzan::Empty) -> ::grpcio::Result<super::byzan::ResponseLen> {
        self.len_opt(req, ::grpcio::CallOption::default())
    }

    pub fn len_async_opt(&self, req: &super::byzan::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseLen>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_LEN, req, opt)
    }

    pub fn len_async(&self, req: &super::byzan::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseLen>> {
        self.len_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn history_opt(&self, req: &super::byzan::BlockKey, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlocks> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_HISTORY, req, opt)
    }

    pub fn history(&self, req: &super::byzan::BlockKey) -> ::grpcio::Result<super::byzan::ResponseBlocks> {
        self.history_opt(req, ::grpcio::CallOption::default())
    }

    pub fn history_async_opt(&self, req: &super::byzan::BlockKey, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlocks>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_HISTORY, req, opt)
    }

    pub fn history_async(&self, req: &super::byzan::BlockKey) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlocks>> {
        self.history_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn range_opt(&self, req: &super::byzan::BlockRange, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlocks> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_RANGE, req, opt)
    }

    pub fn range(&self, req: &super::byzan::BlockRange) -> ::grpcio::Result<super::byzan::ResponseBlocks> {
        self.range_opt(req, ::grpcio::CallOption::default())
    }

    pub fn range_async_opt(&self, req: &super::byzan::BlockRange, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlocks>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_RANGE, req, opt)
    }

    pub fn range_async(&self, req: &super::byzan::BlockRange) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlocks>> {
        self.range_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn till_opt(&self, req: &super::byzan::BlockTill, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::byzan::ResponseBlocks> {
        self.client.unary_call(&METHOD_BLOCK_CHAIN_TILL, req, opt)
    }

    pub fn till(&self, req: &super::byzan::BlockTill) -> ::grpcio::Result<super::byzan::ResponseBlocks> {
        self.till_opt(req, ::grpcio::CallOption::default())
    }

    pub fn till_async_opt(&self, req: &super::byzan::BlockTill, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlocks>> {
        self.client.unary_call_async(&METHOD_BLOCK_CHAIN_TILL, req, opt)
    }

    pub fn till_async(&self, req: &super::byzan::BlockTill) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::byzan::ResponseBlocks>> {
        self.till_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait BlockChain {
    fn upsert(&self, ctx: ::grpcio::RpcContext, req: super::byzan::NewBlock, sink: ::grpcio::UnarySink<super::byzan::ResponseBlock>);
    fn push(&self, ctx: ::grpcio::RpcContext, req: super::byzan::Block, sink: ::grpcio::UnarySink<super::byzan::ResponseBlock>);
    fn get_by_key(&self, ctx: ::grpcio::RpcContext, req: super::byzan::BlockKey, sink: ::grpcio::UnarySink<super::byzan::ResponseBlock>);
    fn get_by_idx(&self, ctx: ::grpcio::RpcContext, req: super::byzan::BlockIdx, sink: ::grpcio::UnarySink<super::byzan::ResponseBlock>);
    fn get_by_id(&self, ctx: ::grpcio::RpcContext, req: super::byzan::BlockId, sink: ::grpcio::UnarySink<super::byzan::ResponseBlock>);
    fn last(&self, ctx: ::grpcio::RpcContext, req: super::byzan::Empty, sink: ::grpcio::UnarySink<super::byzan::ResponseBlock>);
    fn len(&self, ctx: ::grpcio::RpcContext, req: super::byzan::Empty, sink: ::grpcio::UnarySink<super::byzan::ResponseLen>);
    fn history(&self, ctx: ::grpcio::RpcContext, req: super::byzan::BlockKey, sink: ::grpcio::UnarySink<super::byzan::ResponseBlocks>);
    fn range(&self, ctx: ::grpcio::RpcContext, req: super::byzan::BlockRange, sink: ::grpcio::UnarySink<super::byzan::ResponseBlocks>);
    fn till(&self, ctx: ::grpcio::RpcContext, req: super::byzan::BlockTill, sink: ::grpcio::UnarySink<super::byzan::ResponseBlocks>);
}

pub fn create_block_chain<S: BlockChain + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_UPSERT, move |ctx, req, resp| {
        instance.upsert(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_PUSH, move |ctx, req, resp| {
        instance.push(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_GET_BY_KEY, move |ctx, req, resp| {
        instance.get_by_key(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_GET_BY_IDX, move |ctx, req, resp| {
        instance.get_by_idx(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_GET_BY_ID, move |ctx, req, resp| {
        instance.get_by_id(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_LAST, move |ctx, req, resp| {
        instance.last(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_LEN, move |ctx, req, resp| {
        instance.len(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_HISTORY, move |ctx, req, resp| {
        instance.history(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_RANGE, move |ctx, req, resp| {
        instance.range(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BLOCK_CHAIN_TILL, move |ctx, req, resp| {
        instance.till(ctx, req, resp)
    });
    builder.build()
}
