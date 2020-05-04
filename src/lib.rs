#[tarpc::service]
pub trait RPC {
    async fn hello(name: String) -> String;
}

#[macro_use]
extern crate crypto;
extern crate rand;

