#![feature(test)]

extern crate futures;
extern crate grpcio;
extern crate num;
extern crate protobuf;
extern crate rand;
extern crate test;

#[path = "../src/proto/mod.rs"]
mod proto;

use grpcio::{ChannelBuilder, EnvBuilder};
use node::{node, Node};
use num::bigint::BigUint;
use proto::byzan::NewBlock;
use proto::byzan_grpc::BlockChainClient;
use std::process::{Child, Command};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use test::Bencher;

#[bench]
fn one(b: &mut Bencher) {
    bench::run(b, topology::one());
}

#[bench]
fn ring_3(b: &mut Bencher) {
    bench::run(b, topology::ring(3));
}

#[bench]
fn ring_6(b: &mut Bencher) {
    bench::run(b, topology::ring(6));
}

#[bench]
fn all_to_all_3(b: &mut Bencher) {
    bench::run(b, topology::all_to_all(3));
}

#[bench]
fn all_to_all_6(b: &mut Bencher) {
    bench::run(b, topology::all_to_all(6));
}

mod bench {
    use super::*;

    pub fn run(b: &mut Bencher, mut ns: Vec<Node>) {
        thread::sleep(Duration::from_secs(10));

        b.iter(|| {
            let k = "key";
            let v = "value";

            if let Err(e) = client::upsert(&ns, k, v) {
                panic!("{:?}", e);
            }
        });

        thread::sleep(Duration::from_secs(200));

        if let Err(e) = client::finalize(&mut ns) {
            println!("<--- checker's failure --->");
            println!("{}", e);
        }

        for n in &mut ns {
            if let Err(e) = n.process.kill() {
                panic!("{:?}", e);
            }
        }
    }
}

mod topology {
    use super::*;

    pub fn one() -> Vec<Node> {
        let mut ns = Vec::<Node>::new();
        let ps = Vec::<u32>::new();

        ns.push(node(8000, 8001, ps));
        ns
    }

    pub fn ring(n: u32) -> Vec<Node> {
        let mut ns = Vec::<Node>::new();
        let mut ps = Vec::<u32>::new();

        for i in 0..n {
            let bind_port = 8000 + i * 2;
            let peer_port = 8000 + i * 2 + 1;

            let mut peers = match ps.last() {
                Some(p) => vec![*p],
                None => Vec::<u32>::new(),
            };

            if i == n - 1 && n > 1 {
                peers.push(*ps.first().unwrap());
            }

            ns.push(node(bind_port, peer_port, peers));
            ps.push(peer_port);
        }

        ns
    }

    pub fn all_to_all(n: u32) -> Vec<Node> {
        let mut ns = Vec::<Node>::new();
        let mut ps = Vec::<u32>::new();

        for i in 0..n {
            let bind_port = 8000 + i * 2;
            let peer_port = 8000 + i * 2 + 1;

            ns.push(node(bind_port, peer_port, ps.clone()));
            ps.push(peer_port);
        }

        ns
    }
}

mod client {
    use super::*;
    use num::bigint::RandBigInt;
    use rand::Rng;

    pub fn upsert(ns: &Vec<Node>, key: &str, value: &str) -> Result<(), grpcio::Error> {
        let remote = format!("localhost:{}", random_port(ns));
        let i = create_id();
        let mut new_block = NewBlock::new();
        new_block.set_id(i);
        new_block.set_key(key.to_string());
        new_block.set_value(value.to_string());

        try!(client(&remote).upsert(&new_block));

        Ok(())
    }

    fn random_port(ns: &Vec<Node>) -> u32 {
        rand::thread_rng().choose(ns).unwrap().bind_port
    }

    fn client(remote: &String) -> BlockChainClient {
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(&remote);
        let client = BlockChainClient::new(ch);
        client
    }

    fn create_id() -> String {
        let mut r = rand::thread_rng();
        let n: BigUint = r.gen_biguint(256);

        format!("{:x}", n)
    }

    pub fn finalize(nodes: &mut Vec<Node>) -> Result<(), String> {
        let checks: Vec<String> = nodes
            .iter()
            .map(|n| format!("localhost:{}", n.bind_port))
            .collect();

        match Command::new("target/release/byz_checker")
            .args(&checks)
            .output()
        {
            Ok(c) => {
                if c.status.success() {
                    // for debugging by `--nocapture`
                    println!("{}", String::from_utf8_lossy(&c.stdout));

                    return Ok(());
                } else {
                    // for debugging by `--nocapture`
                    println!("{}", String::from_utf8_lossy(&c.stdout));

                    return Err(String::from("checker's exit code is not zero"));
                }
            }
            Err(_) => {
                return Err(String::from("failed to execute checker"));
            }
        }
    }
}

mod node {
    use super::*;

    #[allow(dead_code)]
    pub struct Node {
        pub process: Child,
        pub bind_port: u32,
        pub peer_port: u32,
    }

    impl Node {
        pub fn new(process: Child, bind_port: u32, peer_port: u32) -> Self {
            Node {
                process: process,
                bind_port: bind_port,
                peer_port: peer_port,
            }
        }
    }

    pub fn node(bind_port: u32, peer_port: u32, peers: Vec<u32>) -> Node {
        let mut args: Vec<String> = vec![
            format!("--bind_port={}", bind_port),
            format!("--peer_port={}", peer_port),
            String::from("-q"),
        ];

        let peers: Vec<String> = peers
            .iter()
            .map(|port| format!("ws://localhost:{}", port))
            .collect();

        for peer in peers {
            args.push(peer)
        }

        let c = Command::new("target/release/byzd")
            .args(&args)
            .spawn()
            .unwrap();

        Node::new(c, bind_port, peer_port)
    }
}
