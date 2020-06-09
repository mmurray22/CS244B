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
use std::collections::LinkedList;
#[path = "./nodes.rs"] pub mod nodes;

const ALPHA : usize = 3;

#[derive(Clone)]
pub enum RPCType {
    Ping,
    PingReply,
    Store(u64, u64),
    StoreReply,
    FindNode(nodes::ID),
    FindValue(nodes::ID),
    FindReply(nodes::ZipNode),
    ClientStore(u64,u64),
    ClientGet(u64),
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
    fn find_k_closest_nodes(target_id: nodes::ID, self_id: nodes::ID, kbuckets: Vec<LinkedList<nodes::ZipNode>>) 
                        -> Vec<nodes::ZipNode>{
        let mut ret_vec = Vec::with_capacity(nodes::BUCKET_SIZE);
        let mut dist = nodes::Node::key_distance(target_id, self_id);
        while true {
            if ret_vec.len() < ALPHA && dist != 0 {
                dist-=1;
            } else {
                break;
            }
            let mut iter = kbuckets[dist].iter();
            while iter.next() != None {
                if ret_vec.len() < ALPHA {
                    ret_vec.push((iter.next().unwrap()).clone());
                } else {
                    break;
                }
            }
            
        }
        return ret_vec;
    }
    
    fn lookup(&self, target_id: nodes::ID, origin_node: nodes::ZipNode, origin_kbuckets: Vec<LinkedList<nodes::ZipNode>>) -> nodes::ZipNode {
        //1. Get all k nodes with IDs closest to the target_id
        let mut closest_k : Vec<nodes::ZipNode> = RPCMessage::find_k_closest_nodes(target_id, origin_node.id, origin_kbuckets.clone());
        //2. Order those k nodes and select the closest ALPHA
        closest_k.sort_by(|a, b| (nodes::Node::key_distance(b.id, target_id)).cmp(&(nodes::Node::key_distance(a.id, target_id))));
        let ret = nodes::Node::key_distance(closest_k[0].id, target_id) > nodes::Node::key_distance(origin_node.id, target_id) &&
                  nodes::Node::key_distance(closest_k[1].id, target_id) > nodes::Node::key_distance(origin_node.id, target_id) &&
                  nodes::Node::key_distance(closest_k[2].id, target_id) > nodes::Node::key_distance(origin_node.id, target_id);
        //2.5 If there is no closest ALPHA, then return!
        if ret {
            return origin_node;
        }
        //3. Recursively lookup nodes in those nodes TODO: Need to change to RPC calls!
        let node_one = RPCMessage::lookup(self, target_id, closest_k[0].clone(), origin_kbuckets.clone());
        let node_two = RPCMessage::lookup(self, target_id, closest_k[1].clone(), origin_kbuckets.clone());
        let node_three = RPCMessage::lookup(self, target_id, closest_k[2].clone(), origin_kbuckets.clone());
        //4. OPTIONAL? -> Once this recursive lookup is done on the alpha, investigate the other k-ALPHA
        //5. Once that is done, return the selected node.
        let node_cmp = if nodes::Node::key_distance(node_one.id, target_id) < nodes::Node::key_distance(node_two.id, target_id) {node_one} else {node_two};
        let node_final = if nodes::Node::key_distance(node_cmp.id, target_id) < nodes::Node::key_distance(node_three.id, target_id) {node_cmp} else {node_three};
        return node_final;
    }


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

    fn store(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> { 

        println!("Store from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        let mut replys = Vec::new();

        match self.payload {
            RPCType::Store(key,val) => {
                if current.store_value(key,val) {
                    replys.push((self.caller_node.ip.clone(), RPCMessage {
                        rpc_token: nodes::ID {id: [0; 20]},
                        caller_node: nodes::ZipNode {
                            id: current.get_id(),
                            ip: current.get_ip(),
                            port: current.get_port()},
                        callee_id: self.caller_node.id,
                        payload: RPCType::StoreReply
                    }));
                }
            },
            _ => println!("Store Failed")
        }
        return replys;
    }

    fn store_reply(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        println!("StoreAck from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        let replys = Vec::new();
        return replys;
    }
    
    pub fn find(&self, is_fnode: bool, current: &mut Box<nodes::Node>) 
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
        // Update ongoing lookup and possibly send more find rpcs or send store
        let replys = Vec::new();
        return replys;
    }

    fn value(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        let replys = Vec::new();

        match self.payload {
            RPCType::Value(val) => {
                // TODO 
                // End ongoing lookup for value
                println!("Value from {:?} to {:?}",self.caller_node.ip, current.get_ip());
            }
            _ => println!("IMPOSSIBLE")
        };
        
        return replys;
    }

    fn client_store(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        // TODO
        // Init client lookup for k closest nodes, send 
        let replys = Vec::new();
        return replys;
    }

    fn client_get(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        println!("ClientGet from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        // TODO
        // Init client lookup for k closest nodes, one of them should have value

        let replys = Vec::new();
        return replys;
    }

    pub fn recieve_rpc(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        //// Add zipnode to kbuckets
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
            RPCType::ClientStore(key,val) => self.client_store(current),
            RPCType::ClientGet(key) => self.client_get(current),
            _ => Vec::new()
        };

        return replys;
    }
}
