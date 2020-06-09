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
    // Purpose of rpc token?
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
