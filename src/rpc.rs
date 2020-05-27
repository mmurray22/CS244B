// Necessary crates and imports
// pub use crate::trace;
// pub use crate::server::Server;
// use std::io;
// use std::net::{TcpListener, TcpStream};
// use futures::{
//     future::{self, Ready},
//     prelude::*,
// };

//  Declares the essential structs to be used throughout
#[derive(Clone)]
struct Server;
struct Client;

const TIMEOUT:u32 = 100; //TIMEOUT 

// Types to be used in the RPC message 
pub enum RPCType {
    PING,
    STORE,
    FIND_NODE,
    FIND_VALUE,
}

// pub enum SocketAddr {
//     V4(IPv4Addr),
//     V6(IPv6Addr),
// }

/// RPC struct definition
pub struct RPC {
    /*msg_type: RPCType,
    msg: /*Message*/, 
    sender: IPv6,
    receiver: IPv6,
    token: /*Signature*/,*/

}

// impl RPC {
//     fn list_of_ipaddr (self) -> Vec {
//         //returns a list of ip addrs
//     }

//     fn list_of_ports (self) -> Vec {
//         //returns a list of ports
//     }

//     fn listening(self, ipaddr: IPV4Addr) -> Result<(), Error> {
//         let socket = SocketAddrV4::new(ipaddr, 0);
//         let listener = TcpListener::bind(socket);
//         let port = listener.local_addr()?;
//         println!("Listening on {}, access this port to end the program", port);
//         let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
//         println!("Connection received! {:?} is sending data.", addr);
//         let mut input = String::new();
//         let _ = tcp_stream.read_to_string(&mut input)?;
//         println!("{:?} says {}", addr, input);
//         Ok(())
//     }

//     fn send(self, _: context::Context, name: String) -> Self::ReturnFut {
//         future::ready(format!("Hello, {}!", name))
//     }

//     fn receive(self, _: context::Context) -> ReturnFut {
        
//     }

//     fn connecting(self, ipaddr: String, port: String) -> Result<(), Error> {
//         let input = String::from(ipaddr + port);
//         if let Ok(stream) = TcpStream::connect_timeout(input, TIMEOUT) {
//             println!("Connected to the server!");
//             while () {
//                 stream.write(&[1])?; //TODO: send custom RPC type 
//                 stream.read(&mut [0; 128])?;
//             }
//         } else {
//             println!("Couldn't connect to server...");
//         }
//         Ok(())
//     }
// }

