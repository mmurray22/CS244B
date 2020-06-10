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
    FindNode(u64),
    FindValue(u64),
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
    pub lookup_key: usize,
    pub caller_node: nodes::ZipNode,
    pub callee_id: nodes::ID,
    pub payload: RPCType,
    pub lookup_id: Vec<nodes::ZipNode>,
}

// Handler functions for all RPCs
impl RPCMessage {
    /*Find ALPHA closest nodes*/
    // pub fn lookup_init(&mut self, target_id: nodes::ID)
    //                     -> Vec<nodes::ZipNode> {
    //     //1. Get all k nodes with IDs closest to the target_id 
    //     let mut ret_vec = Vec::with_capacity(nodes::BUCKET_SIZE);
    //     let mut dist = nodes::Node::key_distance(target_id, self.caller_node.id);
    //     loop {
    //         if ret_vec.len() < ALPHA && dist != 0 {
    //             dist-=1;
    //         } else {
    //             break;
    //         }
    //         let mut iter = self.caller_node.kbuckets[dist].iter();
    //         while iter.next() != None {
    //             if ret_vec.len() < ALPHA {
    //                 ret_vec.push((iter.next().unwrap()).clone());
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    //     self.lookup_key = ret_vec.len();
    //     self.lookup_id = ret_vec.clone();
    //     return ret_vec;
    // }

    // pub fn lookup_update(&mut self, target_id: nodes::ID, _zip_node : nodes::ZipNode) -> Vec<nodes::ZipNode> {
    //     //2. Order those k nodes and select the closest ALPHA
    //     self.lookup_key -= 1;
    //     self.lookup_init(target_id)
    // }

    fn lookup_init(&self, target_id: u64, current: &mut Box<nodes::Node>) -> (Vec<nodes::ZipNode>, u64) {
        return (Vec::new(),0);
    }


    fn lookup_update(&self, target_id: u64, current: &mut Box<nodes::Node>) -> Vec<nodes::ZipNode> {
        return Vec::new();
    }


    fn lookup_end(&self) {

    }

    fn create_rpc(&self, current: &mut Box<nodes::Node>, payload:RPCType) -> RPCMessage {
        RPCMessage {
            rpc_token: nodes::ID {id: [0; 20]},
            lookup_key: 0,
            caller_node: nodes::ZipNode::new(&current),
            callee_id: self.caller_node.id,
            payload,
            lookup_id: Vec::<nodes::ZipNode>::new(),
        }
    }

    fn ping(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        println!("Ping from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        let mut replys = Vec::new();

        replys.push((self.caller_node.ip.clone(), RPCMessage {
            rpc_token: nodes::ID {id: [0; 20]},
            lookup_key: 0,
            caller_node: nodes::ZipNode::new(&current),
            callee_id: self.caller_node.id,
            payload: RPCType::PingReply,
			lookup_id: Vec::<nodes::ZipNode>::new(),
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

        // Store value
        match self.payload {
            RPCType::Store(key,val) => {
                current.storage.insert(key,val);
                let rpc = self.create_rpc(current, RPCType::StoreReply);

                replys.push((self.caller_node.ip.clone(),rpc));
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
    
    pub fn find(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        // TODO 
        // send FindReplys for closest k nodes in current.k_buckets
        // let replys = lookup();

        match self.payload {
            RPCType::FindValue(key) => {
                match current.storage.get(&key) {
                    Some(val) => {

                    },
                    None => {

                    }
                }
            },
            RPCType::FindNode(key) => {

            },
            _ => {}
        }
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
            RPCType::FindNode(id) => self.find(current),
            RPCType::FindValue(id) => self.find(current),
            RPCType::FindReply(node) => self.find_reply(current),
            RPCType::Value(val) => self.value(current),
            RPCType::ClientStore(key,val) => self.client_store(current),
            RPCType::ClientGet(key) => self.client_get(current),
            _ => Vec::new()
        };

        return replys;
    }
}
