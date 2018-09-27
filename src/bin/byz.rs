#![allow(dead_code)]
extern crate clap;
extern crate colored;
extern crate ring;
extern crate futures;
extern crate grpcio;
#[macro_use]
extern crate lazy_static;
extern crate protobuf;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate num;
extern crate rand;
extern crate rusqlite;

#[path = "../bc/mod.rs"]
mod bc;
#[path = "../cli/mod.rs"]
mod cli;
#[path = "../ctx/mod.rs"]
mod ctx;
#[path = "../proto/mod.rs"]
mod proto;

use bc::block;
use clap::{App, SubCommand};
use colored::*;
use grpcio::{ChannelBuilder, EnvBuilder};
use num::bigint::BigUint;
use num::bigint::RandBigInt;
use proto::byzan::{Block, BlockId, BlockIdx, BlockKey, BlockRange, BlockTill, Empty, NewBlock};
use proto::byzan_grpc::BlockChainClient;
use std::sync::Arc;

struct Config {
    connect_host: String,
    connect_port: String,
}

impl Config {
    fn new() -> Config {
        Config {
            connect_host: String::from("127.0.0.1"),
            connect_port: String::from("10799"),
        }
    }
}

fn create_id() -> String {
    let mut r = rand::thread_rng();
    let n: BigUint = r.gen_biguint(256);

    format!("{:x}", n)
}

fn client(config: &Config) -> BlockChainClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env)
        .connect(&format!("{}:{}", config.connect_host, config.connect_port));
    let client = BlockChainClient::new(ch);
    client
}

fn upsert(config: &Config, matches: &clap::ArgMatches) -> () {
    let id = create_id();
    let key = matches.value_of("key").unwrap();
    let value = matches.value_of("value").unwrap();

    let mut new_block = NewBlock::new();
    new_block.set_id(id);
    new_block.set_key(key.to_string());
    new_block.set_value(value.to_string());

    match client(config).upsert(&new_block) {
        Ok(b) => {
            handle_block(b.get_block().clone());
        }
        Err(e) => {
            handle_error(&e);
        }
    }
}

fn push(config: &Config, matches: &clap::ArgMatches) -> () {
    let idx = matches
        .value_of("idx")
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap();
    let id = matches.value_of("id").unwrap().to_string();
    let key = matches.value_of("key").unwrap().to_string();
    let value = matches.value_of("value").unwrap().to_string();
    let prev_hash = matches.value_of("prev_hash").unwrap().to_string();
    let self_hash = matches.value_of("self_hash").unwrap().to_string();

    let mut block = Block::new();
    block.set_idx(idx);
    block.set_id(id);
    block.set_key(key);
    block.set_value(value);
    block.set_prev_hash(prev_hash);
    block.set_self_hash(self_hash);

    match client(config).push(&block) {
        Ok(b) => {
            handle_block(b.get_block().clone());
        }
        Err(e) => {
            handle_error(&e);
        }
    }
}

fn get(config: &Config, matches: &clap::ArgMatches) -> () {
    if let Some(key) = matches.value_of("key") {
        let mut block_key = BlockKey::new();
        block_key.set_key(key.to_string());

        match client(config).get_by_key(&block_key) {
            Ok(b) => {
                handle_block(b.get_block().clone());
            }
            Err(e) => {
                handle_error(&e);
            }
        }
        return;
    }

    if let Some(idx) = matches.value_of("idx") {
        let mut block_idx = BlockIdx::new();
        block_idx.set_idx(idx.to_string().parse::<u32>().unwrap());

        match client(config).get_by_idx(&block_idx) {
            Ok(b) => {
                handle_block(b.get_block().clone());
            }
            Err(e) => {
                handle_error(&e);
            }
        }
        return;
    }

    if let Some(id) = matches.value_of("id") {
        let mut block_id = BlockId::new();
        block_id.set_id(id.to_string());

        match client(config).get_by_id(&block_id) {
            Ok(b) => {
                handle_block(b.get_block().clone());
            }
            Err(e) => {
                handle_error(&e);
            }
        }
        return;
    }

    println!("please specify key (--key), idx (--idx) or id (--id)");
}

fn history(config: &Config, matches: &clap::ArgMatches) -> () {
    let key = matches.value_of("key").unwrap().to_string();

    let mut block_key = BlockKey::new();
    block_key.set_key(key);

    match client(config).history(&block_key) {
        Ok(h) => {
            handle_blocks(h.get_blocks());
        }
        Err(e) => {
            handle_error(&e);
        }
    }
}

fn last(config: &Config) -> () {
    let empty = Empty::new();

    match client(config).last(&empty) {
        Ok(b) => {
            handle_block(b.get_block().clone());
        }
        Err(e) => {
            handle_error(&e);
        }
    }
}

fn len(config: &Config) -> () {
    let empty = Empty::new();

    match client(config).len(&empty) {
        Ok(len) => {
            println!("{}", json!({ "len": len.get_len() }));
        }
        Err(e) => {
            handle_error(&e);
        }
    }
}

fn range(config: &Config, matches: &clap::ArgMatches) -> () {
    let first = matches
        .value_of("first")
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap();
    let last = matches
        .value_of("last")
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap();

    let mut block_range = BlockRange::new();
    block_range.set_first(first);
    block_range.set_last(last);

    match client(config).range(&block_range) {
        Ok(blocks) => {
            handle_blocks(blocks.get_blocks());
        }
        Err(e) => {
            handle_error(&e);
        }
    }
}

fn till(config: &Config, matches: &clap::ArgMatches) -> () {
    let first = matches
        .value_of("first")
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap();

    let mut block_till = BlockTill::new();
    block_till.set_first(first);

    match client(config).till(&block_till) {
        Ok(blocks) => {
            handle_blocks(blocks.get_blocks());
        }
        Err(e) => {
            handle_error(&e);
        }
    }
}

fn handle_error(e: &grpcio::Error) {
    match e {
        grpcio::Error::RpcFailure(e) => {
            println!(
                "\n {}\n",
                "An error has been occurred for the request.".red().bold(),
            );

            if let Some(reason) = &e.details {
                println!(" the reason is follwing : {}\n", reason.red().bold());
            } else {
                println!(" the reason is {}\n", "unknown".red());
            }
        }
        _ => {
            println!("\n unknown error: {:?}\n", e);
        }
    }
}

fn handle_block(b: Block) {
    match serde_json::to_string_pretty(&block::Block::from(b)) {
        Ok(b_json) => {
            println!("{}", b_json);
        }
        Err(e) => {
            println!("failed to parse the block: {}", e);
        }
    }
}

fn handle_blocks(blocks: &[Block]) {
    let blocks: Vec<block::Block> = blocks
        .to_vec()
        .iter()
        .map(|b| block::Block::from(b.clone()))
        .collect();

    match serde_json::to_string_pretty(&blocks) {
        Ok(b_json) => {
            println!("{}", b_json);
        }
        Err(e) => {
            println!("failed to parse the blocks: {}", e);
        }
    }
}

fn main() {
    let matches = App::new("byz")
        .version("0.1")
        .author("tbrand (Taichiro Suzuki)")
        .about("command line client for byzan server (byzd)")
        .subcommand(
            SubCommand::with_name("upsert")
                .about("create or update new block with key and value")
                .arg(cli::arg_key(true))
                .arg(cli::arg_value()),
        ).subcommand(
            SubCommand::with_name("push")
                .about("push a new block")
                .arg(cli::arg_idx(true))
                .arg(cli::arg_id(true))
                .arg(cli::arg_key(true))
                .arg(cli::arg_value())
                .arg(cli::arg_prev_hash())
                .arg(cli::arg_self_hash()),
        ).subcommand(
            SubCommand::with_name("get")
                .about("get a block for the arg")
                .arg(cli::arg_key(false))
                .arg(cli::arg_id(false))
                .arg(cli::arg_idx(false)),
        ).subcommand(
            SubCommand::with_name("history")
                .about("get a history for the key")
                .arg(cli::arg_key(true)),
        ).subcommand(SubCommand::with_name("last").about("get a last block"))
        .subcommand(SubCommand::with_name("len").about("get a length of the blockchain"))
        .subcommand(
            SubCommand::with_name("range")
                .about("get a subset of the blockchain for the specified range")
                .arg(cli::arg_first())
                .arg(cli::arg_last()),
        ).subcommand(
            SubCommand::with_name("till")
                .about("get a subset of the blockchain from the specified index")
                .arg(cli::arg_first()),
        ).arg(cli::arg_connect_host())
        .arg(cli::arg_connect_port())
        .get_matches();

    let mut config = Config::new();

    if let Some(connect_host) = matches.value_of("connect_host") {
        config.connect_host = connect_host.to_string();
    }

    if let Some(connect_port) = matches.value_of("connect_port") {
        config.connect_port = connect_port.to_string();
    }

    match matches.subcommand() {
        ("upsert", Some(matches)) => upsert(&config, &matches),
        ("push", Some(matches)) => push(&config, &matches),
        ("get", Some(matches)) => get(&config, &matches),
        ("history", Some(matches)) => history(&config, &matches),
        ("last", Some(_)) => last(&config),
        ("len", Some(_)) => len(&config),
        ("range", Some(matches)) => range(&config, &matches),
        ("till", Some(matches)) => till(&config, &matches),
        _ => {
            println!("{}", matches.usage());
        }
    }
}
