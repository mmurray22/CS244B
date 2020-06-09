
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

mod nodes;
mod routing;
mod test_harness;
mod kademlia;

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
        loop {

            break;
        }
    }
}


fn bootstrap(mut pre_node: Box<nodes::Node>, base_ip: String, base_id: nodes::ID) {
    let default_zip = nodes::ZipNode::new(base_id, base_ip, DEFAULT_PORT);
    let pre_node_id : [u8; 20] =  nodes::Node::get_id(&pre_node); 
    let dist = nodes::Node::key_distance(base_id, nodes::ID{id: pre_node_id});
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

    let rpc = test_harness::kademlia::RPCMessage {
        caller: test_harness::kademlia::nodes::ZipNode {
            id: test_harness::kademlia::nodes::ID { id: [0; 20]},
            ip: "Client".to_string(),
            port: 0 },
        callee_id: test_harness::kademlia::nodes::ID {id: [0; 20]},
        payload: test_harness::kademlia::RPCType::Debug
    };

    network.send_rpc("0".to_string(), rpc);
}
