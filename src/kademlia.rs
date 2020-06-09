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

pub enum RPCType {
    Ping(nodes::ZipNode),
    PingReply(bool),
    Store(u64, u64),
    StoreReply(bool),
    FindNode(nodes::ID),
    FindValue(nodes::ID),
    FindReply(nodes::ZipNode),
    KillNode,
    Debug,
    Null,
}

// #[derive(Serialize, Deserialize, Debug)]
pub struct RPCMessage {
    pub rpc_token: nodes::ID,
    pub caller_node: nodes::ZipNode,
    pub callee_id: nodes::ID,
    pub payload: RPCType,
}

// pub fn lookup(node: nodes::Node, sig: u32, target_id: nodes::ID) -> 

impl RPCMessage {
    pub fn ping(&self, probe_node: nodes::ZipNode, current: &Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        // let dist = nodes::key_distance(current.id, probe_node.id);
        // nodes::ZipNode::add_entry(current, self.caller, dist);
        
        // let reply = RPCMessage {
        //     caller: nodes::ZipNode {
        //         id: nodes::ID { id: [0; 20]},
        //         ip: "".to_string(),
        //         port: 0 },
        //     callee_id: nodes::ID {id: [0; 20]},
        //     payload: RPCType::Null
        // };
        let replys = Vec::new();
        // replys.push()
        return replys;
    }

    pub fn ping_reply(&self, success: bool, current: &Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        
        // let reply = RPCMessage {
        //     caller: nodes::ZipNode {
        //         id: nodes::ID { id: [0; 20]},
        //         ip: "".to_string(),
        //         port: 0 },
        //     callee_id: nodes::ID {id: [0; 20]},
        //     payload: RPCType::Null
        // };
        let replys = Vec::new();
        // replys.push()
        return replys;
    }
    
    // pub fn find(&self, id: nodes::ID, is_fnode: bool) {
    //     let mut closest = Vec::with_capacity(nodes::BUCKET_SIZE);
    //     if is_fnode {
            
    //     }
    // }

    //// ! Notes for lookup algoirthm: 
    // pub fn lookup(&self, target_id: nodes::ID, closest: Vec/*can closest be sorted BEFORE we call it?*/) -> Vec{
    //     let dist = xor(target_id, self.id);
    //     let lookup_nodes = Vec::with_capacity(ALPHA);
    //     while lookup_nodes.len() < ALPHA {
    //         lookup_nodes.sort_by(|a, b| (xor(b.id, target_id)).cmp(&(xor(a.id, target_id))));
    //         for i in 0..self.kbuckets[dist].len() {
    //             /*get top three nodes! TODO*/
    //             if lookup_nodes.len() == ALPHA {
    //                 break;
    //             }
    //             lookup_nodes.push_back(self.kbuckets[dist][i]);
    //         }
    //         break;
    //     }
    //     lookup(lookup_nodes[0]);
    //     lookup(lookup_nodes[1]);
    //     lookup(lookup_nodes[2]);
    // }

    pub fn store(&self, key: u64, val: u64, current: &Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> { 
        //TODO
        // let reply = RPCMessage {
        //     caller: nodes::ZipNode {
        //         id: nodes::ID { id: [0; 20]},
        //         ip: "".to_string(),
        //         port: 0 },
        //     callee_id: nodes::ID {id: [0; 20]},
        //     payload: RPCType::Null
        // };
        let replys = Vec::new();
        // replys.push()
        return replys;
    }

    pub fn store_reply(&self, success: bool, current: &Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        //TODO
        // let reply = RPCMessage {
        //     caller: nodes::ZipNode {
        //         id: nodes::ID { id: [0; 20]},
        //         ip: "".to_string(),
        //         port: 0 },
        //     callee_id: nodes::ID {id: [0; 20]},
        //     payload: RPCType::Null
        // };
        let replys = Vec::new();
        // replys.push()
        return replys;
    }
    
    pub fn find(&self, id: nodes::ID, is_fnode: bool, current: &Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        //TODO
        // let reply = RPCMessage {
        //     caller: nodes::ZipNode {
        //         id: nodes::ID { id: [0; 20]},
        //         ip: "".to_string(),
        //         port: 0 },
        //     callee_id: nodes::ID {id: [0; 20]},
        //     payload: RPCType::Null
        // };
        let replys = Vec::new();
        // replys.push()
        return replys;
    }

    pub fn find_reply(&self, reply: nodes::ZipNode, current: &Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        //TODO
        // let reply = RPCMessage {
        //     caller: nodes::ZipNode {
        //         id: nodes::ID { id: [0; 20]},
        //         ip: "".to_string(),
        //         port: 0 },
        //     callee_id: nodes::ID {id: [0; 20]},
        //     payload: RPCType::Null
        // };
        let replys = Vec::new();
        // replys.push()
        return replys;
    }
}
