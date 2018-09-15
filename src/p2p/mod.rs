pub mod api;

use bc::block::LightBlock;
use bc::{BlockChain, BLOCKCHAIN};
use clap::Values;
use colored::*;
use ctx::CONTEXT;
use serde_json;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use url;
use ws;

lazy_static! {
    pub static ref CHANNEL: (
        Arc<Mutex<mpsc::Sender<String>>>,
        Arc<Mutex<mpsc::Receiver<String>>>
    ) = {
        let (recv, send) = mpsc::channel();
        (Arc::new(Mutex::new(recv)), Arc::new(Mutex::new(send)))
    };
}

pub fn listen(nodes: Values) -> Result<(), ()> {
    let context = CONTEXT.read().unwrap();
    let mut ws = ws::WebSocket::new(|sender| Handler { sender }).unwrap();

    for n in nodes {
        match url::Url::parse(n) {
            Ok(url) => {
                if let Err(e) = ws.connect(url) {
                    error!("cannot connect to {}. it will be skipped.", e);
                }
            }
            Err(e) => {
                error!("cannot parse {}. it will be skipped.", e);
            }
        }
    }

    let broadcaster = ws.broadcaster();

    thread::spawn(move || loop {
        let res = CHANNEL.1.lock().unwrap().recv().unwrap();
        let _ = broadcaster.send(format!("{}", res));
    });

    ws.listen(format!(
        "{}:{}",
        context.peer_host.clone(),
        context.peer_port
    )).unwrap();

    Ok(())
}

struct Handler {
    sender: ws::Sender,
}

impl ws::Handler for Handler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        if let ws::Message::Text(msg) = msg {
            let req: api::Request = serde_json::from_str(&msg).unwrap();
            let req_len = req.block.idx + 1;

            match req.req_type {
                0 => {
                    let blockchain_len = BLOCKCHAIN.lock().unwrap().len();

                    if req_len >= blockchain_len {
                        match api::negotiate(&req) {
                            Ok(r) => {
                                if let Some(b) = r {
                                    broadcast(b);
                                }
                            }
                            Err(e) => {
                                error!("{}", e);
                            }
                        }
                    }
                }
                _ => {
                    info!("unknown request {}", req.req_type);
                }
            }
        }

        Ok(())
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Ok(Some(addr)) = shake.remote_addr() {
            let addr = addr.to_string().yellow().bold();
            info!("open new connection with {}", addr);
        } else {
            info!("open new connection. (remote addr is unknown.)");
        }

        let b: LightBlock = BLOCKCHAIN.lock().unwrap().last().unwrap().into();

        self.sender
            .send(api::payload(api::RequestType::New, b))
            .unwrap();

        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        warn!(
            "close the connection for the reason: {} ({:?})",
            reason, code
        );
    }

    fn on_error(&mut self, err: ws::Error) {
        error!("error happned on the connection {:?}", err);
    }
}

fn broadcast(b: LightBlock) {
    let _ = CHANNEL
        .0
        .lock()
        .unwrap()
        .send(api::payload(api::RequestType::New, b));
}
