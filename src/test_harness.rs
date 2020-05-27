/*For communicating over threads*/

use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::*;
use nodes;
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

	pub fn send_rpc(&self, ip: String, msg: String) {
		match self.nodes_map.get(&ip) {
			Some(node) => {
				node.tx.send(msg).expect("Failed to send");
			},
			None => println!("Can't find node with ip: {:?}", &ip)
		}
	}
}

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


// Representation of router within simulated network

#[allow(dead_code)]
pub struct NetworkNode {
	ip: String,
	port: u64,
	tx: Sender<String>,
	kad_node: Box<nodes::Node>,
	thread: Option<thread::JoinHandle<()>>,
}


impl NetworkNode {
	fn new(ip: String, port: u64) -> NetworkNode {
		let (tx, rx) = channel();

		let thread = start_thread(ip.clone(), rx);

		// Creates struct
		NetworkNode {
			ip: ip.clone(),
			port,
			tx,
			kad_node: <nodes::Node as nodes::NodeTrait>::new(ip.clone(), port),
			thread: Some(thread),
		}
	}
}


// Starts router thread that recieves messages over the "network" from its input
// channel and should handle them appropriately
fn start_thread(ip: String, rx: Receiver<String>) -> thread::JoinHandle<()> {
	let builder = thread::Builder::new().name(ip);

	builder.spawn(move || {
		loop {
			let rpc = rx.recv().expect("Thread receive failed");
			println!("recieved: {:?}", rpc);
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