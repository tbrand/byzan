#![allow(dead_code)]
extern crate clap;
extern crate colored;
extern crate futures;
extern crate grpcio;
extern crate protobuf;

#[path = "../cli/mod.rs"]
mod cli;
#[path = "../proto/mod.rs"]
mod proto;

use clap::App;
use colored::*;
use grpcio::{ChannelBuilder, EnvBuilder, Error, CallOption};
use proto::byzan::{Empty, ResponseBlock, ResponseLen};
use proto::byzan_grpc::BlockChainClient;
use std::process;
use std::sync::Arc;
use std::time::Duration;

fn client(remote: String) -> BlockChainClient {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&remote);
    let client = BlockChainClient::new(ch);
    client
}

fn opt() -> CallOption {
    let opt = CallOption::default();
    opt.timeout(Duration::from_millis(500))
}

fn main() {
    let matches = App::new("byz_checker")
        .version("0.1")
        .author("tbrand (Taichiro Suzuki)")
        .about("Checking blockchain's status for remote nodes")
        .arg(cli::arg_nodes())
        .get_matches();

    if let Some(nodes) = matches.values_of("nodes") {
        let mut res = 0;
        let rs: Vec<(
            String,
            Result<ResponseLen, Error>,
            Result<ResponseBlock, Error>,
        )> = nodes
            .map(|n| {
                let c = client(n.to_string());
                let e = Empty::new();

                (n.to_string(), c.len_opt(&e, opt()), c.last_opt(&e, opt()))
            })
            .collect();

        println!("");
        println!("{}", "checking blockchain's consistency...".bright_green());
        println!("");
        println!("checking length...");

        let mut len = 0;

        for r in &rs {
            match r.1 {
                Ok(ref b) => {
                    println!(
                        "- {} len: {}",
                        r.0.bright_yellow(),
                        b.get_len().to_string().bright_cyan()
                    );

                    if len != 0 && len != b.get_len() {
                        let e = format!(
                            "--> {} error: expected {} but got {}",
                            r.0.bright_yellow(),
                            len.to_string().red().bold(),
                            b.get_len().to_string().red().bold()
                        ).red();
                        println!("{}", e);

                        res = -1;
                    }

                    len = b.get_len();
                }
                Err(ref e) => {
                    if let Error::RpcFailure(e) = e {
                        let e = format!(
                            "--> {} error: {}",
                            r.0.bright_yellow(),
                            e.details.clone().unwrap()
                        ).red()
                            .bold();
                        println!("{}", e);
                    }

                    res = -1;
                }
            }
        }

        println!("");
        println!("checking hash...");

        let mut hash = String::from("");

        for r in &rs {
            match r.2 {
                Ok(ref b) => {
                    let self_hash = b.get_block().clone().self_hash;
                    println!(
                        "- {} hash: {}",
                        r.0.bright_yellow(),
                        self_hash.bright_cyan()
                    );

                    if hash != "" && hash != self_hash {
                        let e = format!(
                            "--> {} error: expected {} but got {}",
                            r.0.bright_yellow(),
                            hash.red().bold(),
                            self_hash.red().bold()
                        );
                        println!("{}", e);

                        res = -1;
                    }

                    hash = self_hash;
                }
                Err(ref e) => {
                    if let Error::RpcFailure(e) = e {
                        let e = format!(
                            "--> {} error: {}",
                            r.0.bright_yellow(),
                            e.details.clone().unwrap()
                        ).red()
                            .bold();
                        println!("{}", e);
                    }

                    res = -1;
                }
            }
        }

        let res_text = match res {
            0 => "OK".cyan().bold(),
            n => format!("ERROR({})", n).red().bold(),
        };

        println!("");
        println!("Result    --- {}", res_text);
        println!("Exit code --- {}", res.to_string().cyan().bold());
        println!("");

        process::exit(res);
    } else {
        println!("please specify remote nodes. You can see the usage by '-h'");
        process::exit(-1);
    }
}
