
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
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::io::{self, Read};

mod nodes;
mod routing;
mod kademlia;
mod test_harness;

pub const DEFAULT_PORT: u64 = 444;

fn main () -> () {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Running test harness");
        run_test_harness();
    } else {
        let test_node : Box<nodes::Node> =  nodes::Node::new(args[1].clone(), DEFAULT_PORT);
        let base_id : nodes::ID = <nodes::ID>::from_str(&args[3].clone()).unwrap();
        bootstrap(test_node, args[2].clone(), base_id.clone());
    }
}


fn bootstrap(mut pre_node: Box<nodes::Node>, base_ip: String, base_id: nodes::ID) {
    let default_zip = nodes::ZipNode::new(base_id, base_ip, DEFAULT_PORT);
    let pre_node_id = pre_node.get_id(); //nodes::Node::get_id(&pre_node); 
    let dist = nodes::Node::key_distance(base_id, pre_node_id);
    nodes::ZipNode::add_entry(&mut pre_node, default_zip, dist);

    //Invoke Self Find_Node RPC
    //find_node();
}

fn run_test_harness() {
    let num_nodes = 10;

    let mut network = test_harness::Network::new();
    for i in 0..num_nodes {
        // port is always 0
        network.client_add_node(i.to_string(),0);
    }

    // Continually recieved stdin, arg 0 is rpc, arg 1 is source, arg 2 is dest
    while true {
        let mut input = String::new();

        // Generates empty rpc
        let mut rpc = test_harness::kademlia::RPCMessage {
            rpc_token: test_harness::kademlia::nodes::ID {id: [0; 20]},
            caller_node: test_harness::kademlia::nodes::ZipNode {
                id: test_harness::kademlia::nodes::ID { id: [0; 20]},
                ip: "1".to_string(),
                port: 0 },
            callee_id: test_harness::kademlia::nodes::ID {id: [0; 20]},
            payload: test_harness::kademlia::RPCType::Ping
        };

        // parses input and sends rpc
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                let split = input.split_whitespace();
                let args = split.collect::<Vec<&str>>();
                if(args.len() == 0) {break;}

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

                    _ => break
                }
                
                
                
            }
            Err(error) => println!("error: {}", error),
        }
    }
    
}
