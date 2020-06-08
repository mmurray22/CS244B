/*For communicating over threads*/

use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex, mpsc::*};

#[path = "./nodes.rs"] pub mod nodes;
#[path = "./kademlia.rs"] pub mod kademlia;
// use rpc;

/* Test harness to test rpc communication between multiple threads */

#[allow(dead_code)]
pub struct Network {
	nodes_map: Arc<Mutex<HashMap<String, Sender<kademlia::RPCMessage>>>>,
	nodes_vec: Vec<Option<thread::JoinHandle<()>>>,
}


impl Network {
	pub fn new() -> Box<Network> {
		let network = Box::new(Network {
			nodes_map: Arc::new(Mutex::new(HashMap::new())),
			nodes_vec: Vec::new(),
		});
		network
	}

	pub fn client_add_node(&mut self, ip: String, port: u64) {
		let (tx, thread) = start_network_node(ip.clone(), port, self.nodes_map.clone());
		self.nodes_vec.push(Some(thread));
		self.nodes_map.lock().unwrap().insert(ip,tx);
	}

	// Sends rpc to node with passed ip
	// TODO convert rpc from String to actual RPC struct
	pub fn send_rpc(&mut self, ip: String, msg: kademlia::RPCMessage) {
		match self.nodes_map.lock().unwrap().get(&ip) {
			Some(node) => {
				node.send(msg).expect("Failed to send");
			},
			None => println!("Can't find node with ip: {:?}", &ip)
		}
	}

	pub fn client_remove_node(&mut self, ip: String) {
		let kill = kademlia::RPCMessage {
			caller: kademlia::nodes::ZipNode {
				id: kademlia::nodes::ID { id: [0; 20]},
				ip: "".to_string(),
				port: 0 },
			callee_id: kademlia::nodes::ID {id: [0; 20]},
			payload: kademlia::RPCType::KillNode
		};
		self.send_rpc(ip,kill);
	}
}


// Sends all "router" threads kill messages and waits for them to exit
impl Drop for Network {

	fn drop(&mut self) {
		for node in &mut self.nodes_map.lock().unwrap().values() {
			let kill = kademlia::RPCMessage {
				caller: kademlia::nodes::ZipNode {
					id: kademlia::nodes::ID { id: [0; 20]},
					ip: "".to_string(),
					port: 0 },
				callee_id: kademlia::nodes::ID {id: [0; 20]},
				payload: kademlia::RPCType::KillNode
			};

			node.send(kill).expect("Failed to kill thread");
		}

		for thread in &mut self.nodes_vec {
			if let Some(t) = thread.take(){
				t.join().unwrap();
			}
		}
	}
}


// Starts network node  thread so that it handles RPCs from its input queue
// returns a tx for sending by other threads
fn start_network_node(ip: String, port: u64, 
	network: Arc<Mutex<HashMap<String,Sender<kademlia::RPCMessage>>>>) -> 
		(Sender<kademlia::RPCMessage>, thread::JoinHandle<()>) {
	
	let (tx, rx) = channel::<kademlia::RPCMessage>();

	// Thread continuously waits on its RPC queue until it receives kill msg
	let thread = thread::spawn(move || {
		let me = Box::new(<nodes::Node as nodes::NodeTrait>::new(ip, port));
		let net = network;
		loop {
			let rpc = rx.recv().expect("Error in receiving RPC");
			match rpc.payload {
				kademlia::RPCType::KillNode => {
					println!("received: KILL {:?}", <nodes::Node as nodes::NodeTrait>::get_ip(&me));
					break;
				},
				_ => handle(&me,rpc,&net)
			}
		}
	});

	return (tx,thread);
}


fn handle(node: &Box<nodes::Node>, rpc: kademlia::RPCMessage, 
		network: &Arc<Mutex<HashMap<String,Sender<kademlia::RPCMessage>>>>) {
	match rpc.payload {
		kademlia::RPCType::Debug => debug(rpc, node),
		_ => println!("Other recieved")
	}
	
}

fn debug(rpc: kademlia::RPCMessage, node: &Box<nodes::Node>) {
	println!("Node {:?} recieved debug to {:?} from {:?}", 
		<nodes::Node as nodes::NodeTrait>::get_id(node),
		rpc.callee_id,
		rpc.caller.ip);
}



