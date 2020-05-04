use futures::{
    future::{self, Ready},
    prelude::*,
}

use tarpc::{
    client, context,
    server::{self, Handler},
};

use std::io;

#[tarpc::service]
trait World {
    async fn hello(name: String) -> String;
}

#[derive(Clone)]
struct HelloServer;

impl World for HelloServer {
    type HelloFut = Ready<String>;

    fn hello(self, _: context::Context, name: String) -> Self::HelloFut {
        future::ready(format!("Hello, {}!", name))
    }
}


/// And here are the struct definition
pub struct RPC {
    msg: /*Message*/, 
    sender: IPv6,
    receiver: IPv6,
    token: /*Signature*/,
}


