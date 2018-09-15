#![allow(dead_code)]
use clap::Arg;

pub fn arg_connect_host<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("connect_host")
        .long("connect_host")
        .value_name("CONNECT_HOST")
        .required(false)
        .takes_value(true)
        .help("specify a connecting host")
}

pub fn arg_connect_port<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("connect_port")
        .long("connect_port")
        .value_name("CONNECT_PORT")
        .required(false)
        .takes_value(true)
        .help("specify a connecting port")
}

pub fn arg_bind_host<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("bind_host")
        .long("bind_host")
        .value_name("BIND_HOST")
        .required(false)
        .takes_value(true)
        .help("specify a binding host")
}

pub fn arg_bind_port<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("bind_port")
        .long("bind_port")
        .value_name("BIND_PORT")
        .required(false)
        .takes_value(true)
        .help("specify a binding port")
}

pub fn arg_peer_host<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("peer_host")
        .long("peer_host")
        .value_name("PEER_HOST")
        .required(false)
        .takes_value(true)
        .help("specify a peerhost")
}

pub fn arg_peer_port<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("peer_port")
        .long("peer_port")
        .value_name("PEER_PORT")
        .required(false)
        .takes_value(true)
        .help("specify a peer port")
}

pub fn arg_threads<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("threads")
        .long("threads")
        .value_name("THREADS")
        .required(false)
        .takes_value(true)
        .help("specify threads for gRPC")
}

pub fn arg_key<'a, 'b>(required: bool) -> Arg<'a, 'b> {
    Arg::with_name("key")
        .short("k")
        .long("key")
        .value_name("KEY")
        .required(required)
        .takes_value(true)
        .help("specify a key for the block")
}

pub fn arg_value<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("value")
        .short("v")
        .long("value")
        .value_name("VALUE")
        .required(true)
        .takes_value(true)
        .help("specify a value for the block")
}

pub fn arg_id<'a, 'b>(required: bool) -> Arg<'a, 'b> {
    Arg::with_name("id")
        .long("id")
        .value_name("ID")
        .required(required)
        .takes_value(true)
        .help("specify a id for the block")
}

pub fn arg_idx<'a, 'b>(required: bool) -> Arg<'a, 'b> {
    Arg::with_name("idx")
        .long("idx")
        .value_name("IDX")
        .required(required)
        .takes_value(true)
        .help("specify a index for the block")
}

pub fn arg_prev_hash<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("prev_hash")
        .long("prevhash")
        .value_name("PREV_HASH")
        .required(true)
        .takes_value(true)
        .help("specify a prev hash for the block")
}

pub fn arg_self_hash<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("self_hash")
        .long("selfhash")
        .value_name("SELF_HASH")
        .required(true)
        .takes_value(true)
        .help("specify a self hash for the block")
}

pub fn arg_first<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("first")
        .long("first")
        .value_name("FIRST")
        .required(true)
        .takes_value(true)
        .help("specify a first index of the range")
}

pub fn arg_last<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("last")
        .long("last")
        .value_name("LAST")
        .required(true)
        .takes_value(true)
        .help("specify a last index of the range")
}

pub fn arg_nodes<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("nodes")
        .help("Connecting nodes")
        .multiple(true)
}

pub fn arg_database<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("db")
        .long("db")
        .required(false)
        .takes_value(true)
        .help("specify a database path")
}

pub fn arg_log<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("log")
        .long("log")
        .required(false)
        .takes_value(true)
        .help("specify a path of log file")
}

pub fn arg_quiet<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("quiet")
        .long("quiet")
        .short("q")
        .help("do not print logs onto stdout/stderr")
}
