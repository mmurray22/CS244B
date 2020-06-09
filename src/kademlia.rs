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
    //TODO: pub token: ,
    pub caller: nodes::ZipNode,
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
