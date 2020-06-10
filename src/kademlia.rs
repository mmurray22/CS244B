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
    FindReply(Vec<nodes::ZipNode>),
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
    /*Find ALPHA closest nodes*/
    //pub fn lookup_init(key: ID) -> Vec<ZipNode> /*ALPHA Nodes*/
    pub fn lookup_init(&mut self, target_key: u64, curr_node: &mut nodes::Node)
                        -> Vec<nodes::ZipNode> {
        //1. Get all k nodes with IDs closest to the target_id 
        let mut ret_vec = Vec::with_capacity(nodes::BUCKET_SIZE);
        let target_id : nodes::ID = nodes::ID{id : nodes::ID::get_key_hash(target_key)};
        let mut dist = nodes::Node::key_distance(target_id, self.caller_node.id);
        loop {
            if ret_vec.len() < ALPHA && dist != 0 {
                dist-=1;
            } else {
                break;
            }
            let mut iter = curr_node.kbuckets[dist].iter();
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

    pub fn lookup_update(&mut self, target_key: u64, curr_node: &mut nodes::Node, reply: Vec<nodes::ZipNode>) 
                            -> Vec<nodes::ZipNode> {
        //2. Order those k nodes and select the closest ALPHA
        self.lookup_init(target_key, curr_node)
    }

    pub fn lookup_end(&mut self) {
        
    }

    fn ping(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        println!("Ping from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        let mut replys = Vec::new();

        replys.push((self.caller_node.ip.clone(), RPCMessage {
            rpc_token: nodes::ID {id: [0; 20]},
            caller_node: nodes::ZipNode::new(&current),
            callee_id: self.caller_node.id,
            payload: RPCType::PingReply,
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
                        caller_node: nodes::ZipNode::new(&current),
                        callee_id: self.caller_node.id,
                        payload: RPCType::StoreReply,
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
        nodes::ZipNode::add_entry(current, self.caller_node.clone());

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
