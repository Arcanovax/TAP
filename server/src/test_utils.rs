use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc::{self, UnboundedReceiver};

use crate::{
    error::ErrorCode,
    game::World,
    protocol::Message,
    state::{ServerInfo, SharedServer},
};

pub(crate) fn test_server() -> SharedServer {
    Arc::new(Mutex::new(ServerInfo::new(World::new())))
}

pub(crate) fn addr(n: u16) -> SocketAddr {
    ([127, 0, 0, 1], n).into()
}

pub(crate) fn connect(
    server: &SharedServer,
    addr: SocketAddr,
    name: &str,
) -> UnboundedReceiver<Message> {
    let (tx, rx) = mpsc::unbounded_channel::<Message>();
    server
        .lock()
        .unwrap()
        .try_add_player(name.to_string(), addr, &tx)
        .expect("text connexion");
    rx
}

pub(crate) fn err(code: ErrorCode) -> Message {
    Message::Response {
        error: code,
        data: None,
    }
}
