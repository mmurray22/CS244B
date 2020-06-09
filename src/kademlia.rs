//! Kademlia protocol: What exactly needs to go in here?
//! Okay, so what goes here is the API that will be exposed
//! to the user. Since the user cannot know about all the node
//! and XOR metric and etc. shenanigans, this will serve as the 
//! sole interaction with the system the "user" will have with 
//! the Kademlia DHT (and for actual real users another layer will be
//! built on top of this, but that is for later)

//!TODO Questions which need to be answered: Are we little or big endian?
// use serde::{Serialize, Deserialize};
// use serde_derive::{Serialize, Deserialize};
#[path = "./nodes.rs"] pub mod nodes;

const ALPHA : u64 = 3;

#[derive(Clone)]
pub enum RPCType {
    Ping,
    PingReply,
    Store(u64, u64),
    StoreReply,
    FindNode(nodes::ID),
    FindValue(nodes::ID),
    FindReply(nodes::ZipNode),
    Value(u64),
    KillNode,
    Debug,
}

#[derive(Clone)]
pub struct RPCMessage {
    // Purpose of rpc token? It signs all the rpc messages
    pub rpc_token: nodes::ID,
    pub caller_node: nodes::ZipNode,
    pub callee_id: nodes::ID,
    pub payload: RPCType,
}

// Handler functions for all RPCs
impl RPCMessage {
    fn ping(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        println!("Ping from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        let mut replys = Vec::new();
        replys.push((self.caller_node.ip.clone(), RPCMessage {
            rpc_token: nodes::ID {id: [0; 20]},
            caller_node: nodes::ZipNode {
                id: current.get_id(),
                ip: current.get_ip(),
                port: current.get_port()},
            callee_id: self.caller_node.id,
            payload: RPCType::PingReply
        }));
        return replys;
    }

    fn ping_reply(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        println!("PingACk from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        let replys = Vec::new();
        return replys;
    }
/*
	fn find_k_closest_nodes(target_id: nodes::ID, self_id: nodes::ID, kbuckets: Vec<LinkedList<nodes::ZipNode>>) 
						-> Vec<nodes::ZipNode>{
		let mut ret_vec = Vec::with_capacity(BUCKET_SIZE);
		let mut dist = xor(target_id, self_id);
		while true {
			if ret_vec.len() < ALPHA && dist != 0 {
				dist-=1;
			} else {
				break;
			}
			let mut iter = kbuckets[dist].iter();
			while iter.next() != None {
				if ret_vec.len() < ALPHA {
					ret_vec.push_back(iter.next().unwrap());
				} else {
					break;
				}
			}
			
		}
		return ret_vec;
	}
    
    fn lookup(&self, target_id: nodes::ID) -> Box<nodes::Node> {
        //1. Get all k nodes with IDs closest to the target_id
        let closest_k : Vec = find_k_closest_nodes(target_id, self.id, &self.kbuckets);
        //2. Order those k nodes and select the closest ALPHA
        closest_k.sort_by(|a, b| (xor(b.id, target_id)).cmp(&(xor(a.id, target_id))));
        let ret = xor(closest_k[0].id, target_id) > xor(self.id, target_id) &&
				  xor(closest_k[1].id, target_id) > xor(self.id, target_id) &&
				  xor(closest_k[2].id, target_id) > xor(self.id, target_id);
        //2.5 If there is no closest ALPHA, then return!
        if (ret) {
            return self;
        }
        //3. Recursively lookup nodes in those nodes
        let node_one = lookup(closest_k[0], target_id);
        let node_two = lookup(closest_k[1], target_id);
		let node_three = lookup(closest_k[2], target_id);
        //4. OPTIONAL? -> Once this recursive lookup is done on the alpha, investigate the other k-ALPHA
        //5. Once that is done, return the selected node.
		let node_cmp = xor(node_one.id, target_id) < xor(node_two.id, target_id) ? node_one; node_two;
		let node_final = xor(node_cmp.id, target_id) < xor(node_three.id, target_id) ? node_cmp; node_three;
		return node_final;
    }
*/
    fn store(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> { 

        // TODO 
        // add rpc (key,value) to node storage and send StoreReply

        let replys = Vec::new();
        // replys.push()
        return replys;
    }

    fn store_reply(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        let replys = Vec::new();
        return replys;
    }
    
    fn find(&self, is_fnode: bool, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        // TODO 
        // send FindReplys for closest k nodes in current.k_buckets
        // let replys = lookup();

        let replys = Vec::new();
        // replys.push()
        return replys;
    }

    fn find_reply(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        //TODO
        // Update current lookup and possibly send more find rpcs
        let replys = Vec::new();
        // replys.push()
        return replys;
    }

    fn value(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        let replys = Vec::new();
        return replys;
    }

    pub fn recieve_rpc(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        // Add zipnode to kbuckets
        // let dist = nodes::Node::key_distance(current.get_id(), self.caller_node.id);
        // nodes::ZipNode::add_entry(current, self.caller_node.clone(), dist);

        let replys = match &self.payload {
            RPCType::Ping => self.ping(current),
            RPCType::PingReply => self.ping_reply(current),
            RPCType::Store(key, val) => self.store(current),
            RPCType::StoreReply => self.store_reply(current),
            RPCType::FindNode(id) => self.find(true, current),
            RPCType::FindValue(id) => self.find(false, current),
            RPCType::FindReply(node) => self.find_reply(current),
            RPCType::Value(val) => self.value(current),
            _ => Vec::new()
        };

        return replys;
    }
}
