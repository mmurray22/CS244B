#[derive(Clone)]
struct Server;
struct Client;

const TIMEOUT:u32 = 100; //TIMEOUT 

pub use crate::trace;
pub use crate::server::Server;
use std::io;

use futures::{
    future::{self, Ready},
    prelude::*,
}

use std::net::{TcpListener, TcpStream};

pub enum SocketAddr {
    V4(IPv4Addr),
    V6(IPv6Addr),
}

impl RPC for Server {
    type ReturnFut = Ready<String>;

    fn connect(self, ipaddr: SocketAddr) {
        connection_timeout(ipaddr, TIMEOUT)
    }

    fn send(self, _: context::Context, name: String) -> Self::ReturnFut {
        future::ready(format!("Hello, {}!", name))
    }

    fn receive(self, _: context::Context) -> ReturnFut {
        
    }
}


/// And here are the struct definition
pub struct RPC {
    msg: /*Message*/, 
    sender: IPv6,
    receiver: IPv6,
    token: /*Signature*/,
}


