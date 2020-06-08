//TODO #![feature(linked_list_remove)]

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

pub mod nodes;
pub mod routing;
pub mod test_harness;
pub mod rpc;
pub mod kademlia;
//pub mod rpc_test_harness;

pub const DEFAULT_PORT: u64 = 444;

fn main () -> () {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Wrong number of params. Try again!");
    }
    let mut test_node : Box<nodes::Node> =  <nodes::Node as nodes::NodeTrait>::new(args[1].clone(), DEFAULT_PORT);
    let base_id : nodes::ID = <nodes::ID>::from_str(&args[3].clone()).unwrap();
    bootstrap(test_node, args[2].clone(), base_id.clone());
    loop {
        run_test_harness();
        break; //TODO: Eventually remove! 
    }
}


fn bootstrap(mut pre_node: Box<nodes::Node>, base_ip: String, base_id: nodes::ID) {
    let default_zip = <nodes::ZipNode as nodes::RoutingTable>::new(base_id, base_ip, DEFAULT_PORT);
    let pre_node_id : [u8; 20] =  <nodes::Node as nodes::NodeTrait>::get_id(&pre_node); 
    let dist = <nodes::Node as nodes::NodeTrait>::key_distance(base_id, nodes::ID{id: pre_node_id});
    <nodes::ZipNode as nodes::RoutingTable>::add_entry(&mut pre_node, default_zip, dist);

    //Invoke Self Find_Node RPC
    //find_node();
}

fn run_test_harness(/*num_nodes: u64, num_keys: u64*/) {
    let num_nodes = 10;
    let num_keys = 10;

    let network = test_harness::Network::new(num_nodes);
    network.send_rpc("0".to_string(), "Hello world!".to_string());
}
