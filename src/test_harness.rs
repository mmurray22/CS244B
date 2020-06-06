/*For communicating over threads*/

use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::*;

#[path = "./nodes.rs"] mod nodes;
// use rpc;

/* Test harness to test rpc communication between multiple threads */

#[allow(dead_code)]
pub struct Network {
	nodes_map: HashMap<String, Box<NetworkNode>>,
	num_nodes: i32,
}


impl Network {
	pub fn new(num_nodes: i32) -> Box<Network> {
		let mut map = HashMap::new();

		// Currently all ports are set to 0
		for i in 0..num_nodes {
			map.insert(i.to_string(), Box::new(NetworkNode::new(i.to_string(), 0)));
		}

		let network = Box::new(Network {
			nodes_map: map,
			num_nodes: num_nodes,
		});
		network
	}

	pub fn add_kv(ip: String) {
		// TODO add key value pair to node with passed ip
	}

	// Sends rpc to node with passed ip
	// TODO convert rpc from String to actual RPC struct
	pub fn send_rpc(&self, ip: String, msg: String) {
		match self.nodes_map.get(&ip) {
			Some(node) => {
				node.tx.send(msg).expect("Failed to send");
			},
			None => println!("Can't find node with ip: {:?}", &ip)
		}
	}
}


// Sends all "router" threads kill messages and waits for them to exit
impl Drop for Network {
	fn drop(&mut self) {
		for (ip,node) in &mut self.nodes_map {
			node.tx.send("kill".to_string()).expect("Failed to kill thread");

			if let Some(thread) = node.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}


// NetworkNode is essentially a wrapper around nodes::Node, only it additionally
// has a multiple producer single consumer (mpsc) queue to simulate receiving 
// RPCs from the "network"
#[allow(dead_code)]
pub struct NetworkNode {
	tx: Sender<String>,  // Used by network and other threads to send to this node.
	kad_node: Box<nodes::Node>,		
	thread: Option<thread::JoinHandle<()>>, 
}


impl NetworkNode {
	fn new(ip: String, port: u64) -> NetworkNode {

		let (tx, rx) = channel();
		let thread = start_thread(ip.clone(), rx);

		NetworkNode {
			tx,
			kad_node: <nodes::Node as nodes::NodeTrait>::new(ip.clone(), port),
			thread: Some(thread),
		}
	}
}


// Starts "router" thread so that it handles RPCs from its input queue
// Returns thread join handle for use by main thread
fn start_thread(ip: String, rx: Receiver<String>) -> thread::JoinHandle<()> {
	let builder = thread::Builder::new().name(ip);

	// Thread continuously waits on its RPC queue until it receives kill msg
	builder.spawn(move || {
		loop {
			let rpc = rx.recv().expect("Error in receiving RPC");
			println!("received: {:?}", rpc);
			handle();
			if rpc == "kill" {
				break;
			}
		}
	}).unwrap()
}


fn handle() {
	// handle RPC
}
