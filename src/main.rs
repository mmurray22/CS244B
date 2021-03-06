extern crate crypto;
extern crate rand;
extern crate queue;
extern crate tokio;
extern crate futures;
extern crate tokio_ping;
extern crate serde; 
extern crate serde_json;
extern crate serde_derive;

use std::env;
use std::io::{self};

pub mod nodes;
pub mod kademlia;
pub mod test_harness;

pub const DEFAULT_PORT: u64 = 444;

fn main () -> () {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Running test harness");
        run_test_harness();
    } 
}


fn run_test_harness() {
    let mut num_nodes = 100;

    // Creates network and adds nodes
    let mut network = test_harness::Network::new();
    for i in 0..num_nodes {
        // port is always 0
        network.client_add_node(i.to_string(),0);
    }

    // Continually recieved stdin, arg 0 is rpc, arg 1 is source, arg 2 is dest
    loop {
        let mut input = String::new();

        // Generates empty rpc
        let mut rpc = test_harness::kademlia::RPCMessage {
            caller_node: test_harness::kademlia::nodes::ZipNode {
                id: 0,
                ip: "".to_string(),
                port: 0,},
            payload: test_harness::kademlia::RPCType::Ping,
        };

        // parses input and sends rpc
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let split = input.split_whitespace();
                let args = split.collect::<Vec<&str>>();
                if args.len() == 0 {break;}

                match args[0] {
                    "ping" => {
                        // Ex. "ping 1 2"
                        if args.len() != 3 {
                            println!("Invalid Command");
                            continue;
                        }
                        rpc.caller_node.ip = args[1].to_string();
                        rpc.payload = test_harness::kademlia::RPCType::Ping;
                        network.send_rpc(args[2].to_string(), rpc);
                    },
                    "store" => {
                        // Ex. "store 1 2 34 34"
                        if args.len() != 5 {
                            println!("Invalid Command");
                            continue
                        }
                        rpc.caller_node.ip = args[1].to_string();
                        rpc.payload = test_harness::kademlia::RPCType::Store(
                            args[3].parse::<u64>().unwrap(),args[4].parse::<u64>().unwrap());
                        network.send_rpc(args[2].to_string(), rpc);
                    },
                    "cstore" => {
                        // Ex. "cstore 1 34 34"
                        if args.len() != 4 {
                            println!("Invalid Command");
                            continue
                        }
                        rpc.caller_node.ip = "client".to_string();
                        rpc.payload = test_harness::kademlia::RPCType::ClientStore(
                            args[2].parse::<u64>().unwrap(),args[3].parse::<u64>().unwrap());
                        network.send_rpc(args[1].to_string(), rpc);
                    },
                    "cget" => {
                        // Ex. "cget 1 34"
                        if args.len() != 3 {
                            println!("Invalid Command");
                            continue
                        }
                        rpc.caller_node.ip = "client".to_string();
                        rpc.payload = test_harness::kademlia::RPCType::ClientGet(
                            args[2].parse::<u64>().unwrap());
                        network.send_rpc(args[1].to_string(), rpc);
                    },
                    "add" => {
                        // Ex. "add"
                        if args.len() != 1 {
                            println!("Invalid Command");
                            continue
                        }

                        network.client_add_node(num_nodes.to_string(),0);
                        num_nodes += 1;

                    },
                    "remove" => {
                        // Ex. "remove 1"
                        if args.len() != 2 {
                            println!("Invalid Command");
                            continue
                        }
                        network.client_remove_node(args[1].to_string())
                    },

                    _ => break
                }  
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
