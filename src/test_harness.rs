/*For communicating over threads*/
use std::collections::LinkedList;
use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex, mpsc::*};
use rand::Rng;

// #[path = "./nodes.rs"] pub mod nodes;
#[path = "./kademlia.rs"] pub mod kademlia;
// use rpc;

/* Test harness to test rpc communication between multiple threads */

#[allow(dead_code)]
pub struct Network {
	net_map: Arc<Mutex<HashMap<String, Sender<kademlia::RPCMessage>>>>,
	join_vec: Vec<Option<thread::JoinHandle<()>>>,
	nodes_map: HashMap<String,kademlia::nodes::ZipNode>
}


impl Network {
	pub fn new() -> Box<Network> {
		let network = Box::new(Network {
			net_map: Arc::new(Mutex::new(HashMap::new())), 
			join_vec: Vec::new(),			// Used to wait for all nodes to die 
			nodes_map: HashMap::new(), 		// Used to add new nodes to the network
		});
		network
	}

	pub fn client_add_node(&mut self, ip: String, port: u64) {

		let (key, zip ,tx, thread) = start_network_node(ip.clone(), port, self.net_map.clone());
		self.join_vec.push(Some(thread));
		self.net_map.lock().unwrap().insert(ip.clone(),tx.clone());

		// Start lookup for self to add itself to the network
		if self.nodes_map.len() > 0 {
			let mut rng = rand::thread_rng();
			let index = rng.gen_range(0, self.nodes_map.len());
			let mut i = 0;
				for ozip in self.nodes_map.values() {
				if index == i {
					let add = kademlia::RPCMessage {
						rpc_token: kademlia::nodes::ID {id: [0; 20]},
						lookup_key: 0,
						caller_node: ozip.clone(),
						callee_id: kademlia::nodes::ID {id: [0; 20]},
						payload: kademlia::RPCType::ClientGet(key),
					    lookup_id: Vec::<kademlia::nodes::ZipNode>::new(),
                    };

					tx.send(add).expect("Failed to join network");
					break;
				}
				i += 1;
			}
		}

		// Add
		self.nodes_map.insert(ip,zip);
	}

	pub fn client_remove_node(&mut self, ip: String) {
		let kill = kademlia::RPCMessage {
			rpc_token: kademlia::nodes::ID {id: [0; 20]},
			lookup_key: 0,
            caller_node: kademlia::nodes::ZipNode{
				id: kademlia::nodes::ID { id: [0; 20]},
				ip: "".to_string(),
				port: 0,
                kbuckets: Vec::<LinkedList<kademlia::nodes::ZipNode>>::new()},
			callee_id: kademlia::nodes::ID {id: [0; 20]},
			payload: kademlia::RPCType::KillNode,
            lookup_id: Vec::<kademlia::nodes::ZipNode>::new(),
		};
		self.send_rpc(ip.clone(),kill);
		self.net_map.lock().unwrap().remove(&ip);
		self.nodes_map.remove(&ip);
	}

	// Sends rpc to node with passed ip
	// TODO convert rpc from String to actual RPC struct
	pub fn send_rpc(&mut self, ip: String, msg: kademlia::RPCMessage) {
		match self.net_map.lock().unwrap().get(&ip) {
			Some(node) => {
				node.send(msg).expect("Failed to send");
			},
			None => println!("Can't find node with ip: {:?}", &ip)
		}
	}
}


// Sends all "router" threads kill messages and waits for them to exit
impl Drop for Network {

	fn drop(&mut self) {
		for node in &mut self.net_map.lock().unwrap().values() {
			let kill = kademlia::RPCMessage {
				rpc_token: kademlia::nodes::ID {id: [0; 20]},
				lookup_key: 0,
                caller_node: kademlia::nodes::ZipNode {
					id: kademlia::nodes::ID { id: [0; 20]},
					ip: "".to_string(),
					port: 0,
                    kbuckets: Vec::<LinkedList<kademlia::nodes::ZipNode>>::new()},
				callee_id: kademlia::nodes::ID {id: [0; 20]},
				payload: kademlia::RPCType::KillNode,
			    lookup_id: Vec::<kademlia::nodes::ZipNode>::new(),
            };

			node.send(kill).expect("Failed to kill thread");
		}

		for thread in &mut self.join_vec {
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
		(u64, kademlia::nodes::ZipNode, Sender<kademlia::RPCMessage>, thread::JoinHandle<()>) {
	
	let (tx, rx) = channel::<kademlia::RPCMessage>();
	let node = Box::new(kademlia::nodes::Node::new(ip, port));
	let key = node.get_key();
	let zip = kademlia::nodes::ZipNode::new(&node);

	// Thread continuously waits on its RPC queue until it receives kill msg
	let thread = thread::spawn(move || {

		// Inits threads node, and routing table.
		
		let net = network;
		let mut r_table = Box::new(HashMap::new());
		let mut me = node;
		
		loop {
			let rpc = rx.recv().expect("Error in receiving RPC");
			match rpc.payload {
				kademlia::RPCType::KillNode => {
					println!("received: KILL {:?}", 
						kademlia::nodes::Node::get_ip(&me));
					break;
				},
				_ => handle(&mut me, rpc, &net, &mut r_table)
			}
		}
	});

	return (key,zip,tx,thread);
}


fn handle(current: &mut Box<kademlia::nodes::Node>, rpc: kademlia::RPCMessage, 
		network: &Arc<Mutex<HashMap<String,Sender<kademlia::RPCMessage>>>>,
		r_table: &mut Box<HashMap<String,Sender<kademlia::RPCMessage>>>) {

	let replys = match rpc.payload {
		kademlia::RPCType::Debug => debug(rpc, current),
		kademlia::RPCType::KillNode => debug(rpc, current),
		_ => rpc.recieve_rpc(current),
	};

	// Sends all replys 
	for (ip,reply) in replys {
		match r_table.get(&ip) {
			Some(tx) => {
				tx.send(reply).expect("Failed to send");
			},
			None => {
				match network.lock().unwrap().get(&ip) {
					Some(tx) => {
						// Add tx clone to thread table if it doesn't have it
						r_table.insert(ip.clone(), tx.clone());
						tx.send(reply).expect("Failed to send");
					},
					None => println!("Can't find node with ip: {:?}", &ip)
				}
			}
		}
	}
}

fn debug(rpc: kademlia::RPCMessage, node: &Box<kademlia::nodes::Node>) -> Vec<(String,kademlia::RPCMessage)> {
	match rpc.payload {
		kademlia::RPCType::Debug => {
			println!("Node {:?} recieved debug to {:?} from {:?}", 
			kademlia::nodes::Node::get_id(node),
			rpc.callee_id,
			rpc.caller_node.ip);
		}
		_ => println!("IMPOSSIBLE")
	}
	let replys = Vec::new();
	return replys;
}


