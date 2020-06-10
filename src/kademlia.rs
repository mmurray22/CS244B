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

const ALPHA : usize = 3;

#[derive(Clone)]
pub enum RPCType {
    Ping,
    PingReply,
    Store(u64, u64),
    StoreReply,
    FindNode(u64, u64),
    FindValue(u64, u64),
    FindReply(u64, Vec<nodes::ZipNode>, u64),
    ClientStore(u64,u64),
    ClientGet(u64),
    Value(u64, u64),
    KillNode,
    Debug,
}

#[derive(Clone)]
pub struct RPCMessage {
    // Purpose of rpc token? It signs all the rpc messages
    pub rpc_token: nodes::ID,
    pub caller_node: nodes::ZipNode,
    pub payload: RPCType,
}

// Handler functions for all RPCs
impl RPCMessage {

    fn create_new_rpc(current: Box<nodes::Node>, payload:RPCType) -> RPCMessage {
        RPCMessage {
            rpc_token: nodes::ID {id: [0; 20]},
            caller_node: nodes::ZipNode::new(&current),
            payload,
        }
    }


    fn ping(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        println!("Ping from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        let mut replys = Vec::new();
        let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::PingReply);
        replys.push((self.caller_node.ip.clone(), rpc));
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
                current.storage.insert(key,val);
                let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::StoreReply);
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
        let mut replys = Vec::new();
        println!("Find from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        match self.payload {
            RPCType::FindValue(target_key, lookup_key) => {
                match current.storage.get(&target_key) {
                    Some(val) => {
                        let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::Value(*val, lookup_key));
                        replys.push((self.caller_node.ip.clone(),rpc));
                    },
                    None => {
                        let k_closest = current.find_closest_k(target_key);
                        let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::FindReply(target_key, k_closest, lookup_key));
                        replys.push((self.caller_node.ip.clone(), rpc));
                    }
                }
            },
            RPCType::FindNode(target_key, lookup_key) => {
                let k_closest = current.find_closest_k(target_key);
                let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::FindReply(target_key, k_closest, lookup_key));
                replys.push((self.caller_node.ip.clone(), rpc));
            },
            _ => println!("IMPOSSIBLE")
        }

        return replys;
    }


    fn find_reply(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        //TODO
        // Update ongoing lookup and possibly send more find rpcs or send store
        let mut replys = Vec::new();
        println!("FindAck from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        match self.payload.clone() {
            RPCType::FindReply(target_key,k_closest,lookup_key) => {

                let (zips, val, val_flag, done_flag) = 
                    current.lookup_update(self.caller_node.clone(), k_closest, target_key, lookup_key);

                if done_flag {
                    if val_flag {
                        let stores = current.find_closest_k(target_key);
                        for zip in stores {
                            let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::Store(target_key, val));
                            replys.push((zip.ip.clone(),rpc));
                            current.lookup_end(lookup_key);
                        }
                    } else {
                        current.lookup_end(lookup_key);
                    }
                } else {
                    if val_flag {
                        for zip in zips {
                            let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::FindValue(target_key, lookup_key));
                            replys.push((zip.ip.clone(),rpc));
                        }
                    } else {
                        for zip in zips {
                            let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::FindNode(target_key, lookup_key));
                            replys.push((zip.ip.clone(),rpc));
                        }
                    }
                }
                
            },
            _ => println!("Store Failed")
        }

        return replys;
    }

    // Handle recieving value, closes lookup, and stores node at current closest
    fn value(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        // TODO cache key,val
        let replys = Vec::new();

        match self.payload {
            RPCType::Value(val, lookup_key) => {
                current.lookup_end(lookup_key);
                println!("Value:{:?} from {:?} at {:?}", val, self.caller_node.ip, current.get_ip());
            }
            _ => println!("IMPOSSIBLE")
        };
        
        return replys;
    }

    fn client_store(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        let mut replys = Vec::new();
        println!("ClientStore from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        match self.payload {
            RPCType::ClientStore(key,val) => {
                let (zips,lookup_key) = current.lookup_init(key,val,true);
                // println!("IMMEDIATE SIZE: {:?}", zips.len());
                for zip in zips {
                    let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::FindNode(key, lookup_key));
                    replys.push((zip.ip.clone(), rpc));
                    // println!("SIZE INSIDE: {:?}", replys.len());
                }
            },
            _ => println!("IMPOSSIBLE")
        }

        return replys;
    }

    fn client_get(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {
        println!("ClientGet from {:?} to {:?}",self.caller_node.ip, current.get_ip());

        let mut replys = Vec::new();
        match self.payload {
            RPCType::ClientGet(key) => {
                let (zips,lookup_key) = current.lookup_init(key,0,false);
                for zip in zips {
                    let rpc = RPCMessage::create_new_rpc((*current).clone(), RPCType::FindValue(key, lookup_key));
                    replys.push((zip.ip.clone(), rpc));
                }
            },
            _ => println!("IMPOSSIBLE")
        }

        return replys;
    }

    // Generic handler function
    pub fn recieve_rpc(&self, current: &mut Box<nodes::Node>) 
            -> Vec<(String,RPCMessage)> {

        //// Add zipnode to kbuckets
        nodes::ZipNode::add_entry(current, self.caller_node.clone());

        let replys = match &self.payload {
            RPCType::Ping => self.ping(current),
            RPCType::PingReply => self.ping_reply(current),
            RPCType::Store(key, val) => self.store(current),
            RPCType::StoreReply => self.store_reply(current),
            RPCType::FindNode(id, lookup_key) => self.find(current),
            RPCType::FindValue(id, lookup_key) => self.find(current),
            RPCType::FindReply(target_key, node, lookup_key) => self.find_reply(current),
            RPCType::Value(val, lookup_key) => self.value(current),
            RPCType::ClientStore(key,val) => self.client_store(current),
            RPCType::ClientGet(key) => self.client_get(current),
            _ => Vec::new()
        };

        return replys;
    }
}
