extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate serde_derive;
extern crate colored;
extern crate crypto;
extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate rand;
extern crate rusqlite;
extern crate url;
extern crate ws;

mod bc;
mod cli;
mod ctx;
mod grpc;
mod p2p;
mod proto;

#[cfg(test)]
mod tests;

use clap::App;
use colored::*;
use simplelog::*;
use std::fs::File;

fn main() {
    let matches = App::new("byzd")
        .version("0.1")
        .author("tbrand (Taichiro Suzuki)")
        .about("gRPC server of byzan")
        .arg(cli::arg_bind_host())
        .arg(cli::arg_bind_port())
        .arg(cli::arg_peer_host())
        .arg(cli::arg_peer_port())
        .arg(cli::arg_nodes())
        .arg(cli::arg_database())
        .arg(cli::arg_log())
        .arg(cli::arg_quiet())
        .get_matches();

    {
        let mut context = ctx::CONTEXT.write().unwrap();

        if let Some(bind_host) = matches.value_of("bind_host") {
            context.bind_host = bind_host.to_string();
        }

        if let Some(bind_port) = matches.value_of("bind_port") {
            context.bind_port = bind_port.to_string().parse::<u16>().unwrap();
        }

        if let Some(peer_host) = matches.value_of("peer_host") {
            context.peer_host = peer_host.to_string();
        }

        if let Some(peer_port) = matches.value_of("peer_port") {
            context.peer_port = peer_port.to_string().parse::<u16>().unwrap();
        }

        if let Some(threads) = matches.value_of("threads") {
            context.threads = threads.to_string().parse::<usize>().unwrap();
        }

        if let Some(database) = matches.value_of("db") {
            context.database = Some(database.to_string());
        }

        if let Some(log) = matches.value_of("log") {
            context.log = Some(log.to_string());
        }
    }

    {
        let context = ctx::CONTEXT.read().unwrap();
        let bind = format!("{}:{}", context.bind_host, context.bind_port)
            .yellow()
            .bold();
        let threads = context.threads.to_string().cyan().bold();

        let mut logger = Vec::<std::boxed::Box<simplelog::SharedLogger + 'static>>::new();

        if !matches.is_present("quiet") {
            logger.push(TermLogger::new(LevelFilter::Info, Config::default()).unwrap());
        }

        if let Some(ref log_path) = context.log {
            logger.push(WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create(log_path.clone()).unwrap(),
            ));
        }

        CombinedLogger::init(logger).unwrap();

        info!("start binding gRPC on {}", bind);
        info!("threads: {}", threads);
    }

    let mut s = grpc::server().unwrap();
    s.start();

    if let Some(nodes) = matches.values_of("nodes") {
        p2p::listen(nodes).unwrap();
    } else {
        p2p::listen(clap::Values::default()).unwrap();
    }
}
