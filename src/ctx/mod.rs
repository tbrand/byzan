use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Clone)]
pub struct Context {
    pub bind_host: String,
    pub bind_port: u16,
    pub peer_host: String,
    pub peer_port: u16,
    pub threads: usize,
    pub database: Option<String>,
    pub log: Option<String>,
}

impl Context {
    fn new() -> Context {
        Context {
            bind_host: String::from("127.0.0.1"),
            bind_port: 10799,
            peer_host: String::from("127.0.0.1"),
            peer_port: 10800,
            threads: 8,
            database: None,
            log: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoteContext {
    pub bind_host: String,
    pub bind_port: u16,
    pub peer_host: String,
    pub peer_port: u16,
}

impl RemoteContext {
    fn new(bind_host: String, bind_port: u16, peer_host: String, peer_port: u16) -> RemoteContext {
        RemoteContext {
            bind_host: bind_host,
            bind_port: bind_port,
            peer_host: peer_host,
            peer_port: peer_port,
        }
    }
}

impl From<Context> for RemoteContext {
    fn from(ctx: Context) -> RemoteContext {
        RemoteContext::new(ctx.bind_host.clone(), ctx.bind_port, ctx.peer_host.clone(), ctx.peer_port)
    }
}

lazy_static! {
    pub static ref CONTEXT: Arc<RwLock<Context>> = Arc::new(RwLock::new(Context::new()));
}

pub fn remote_context() -> RemoteContext {
    CONTEXT.read().unwrap().clone().into()
}
