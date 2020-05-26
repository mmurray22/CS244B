/*For testing communication over threads*/
use std::threads;
use std::time::Duration;

#[path = "./nodes.rs"] mod nodes;

fn test_two_nodes_one_client() {
    //Since this doesn't use a network, ip & port will be defaults
    let ip : String::from("0.0.0.0");
    let port : u64 = 0;
    let mut test_node_one : Box<nodes::Node> =  <nodes::Node as nodes::NodeTrait>::new(ip, 0);
    let mut test_node_two : Box<nodes::Node> =  <nodes::Node as nodes::NodeTrait>::new(ip, 0);
    //Three threads (one for the client, one for each nodes)
    thread::spawn(|| {})
}
