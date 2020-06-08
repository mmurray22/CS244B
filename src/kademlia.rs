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
    Ping(nodes::Node),
    PingReply(bool),
    Store(u64, u64),
    StoreReply(bool),
    FindNode(nodes::ID),
    FindValue(nodes::ID),
    FindReply(nodes::ZipNode),
    KillNode,
    Debug,
}

// #[derive(Serialize, Deserialize, Debug)]
pub struct RPCMessage {
    //TODO: pub token: ,
    pub caller: nodes::ZipNode,
    pub callee_id: nodes::ID,
    pub payload: RPCType,
}

// <<<<<<< HEAD
impl RPCMessage {
    pub fn ping(&self, probe_node: nodes::ZipNode) { 
        //TODO
    }
    
    pub fn store(&self, key: u64, val: u64) { 
        //TODO
    }
    
    pub fn find(&self, id: nodes::ID, is_fnode: bool) {
        //TODO
// =======
// pub trait rpcfxns {
//     fn ping(probe_node: Box<nodes::Node>); 
//     fn store(key: u64, val: u64); 
//     fn find(id: nodes::ID, is_fnode: bool);
//     fn send_RPC(node_from: Box<nodes::Node>, node_to: Box<nodes::Node>, msg_type: u8);
//     fn read_RPC(ser_msg: String);
// }

// impl rpcfxns for RPCMessage {
//     fn ping(probe_node: Box<nodes::Node>) { 
//         //TODO: probes a node to see if it is online
//     }
    
//     fn store(key: u64, val: u64) { 
//         //TODO: Instructs a node to store a <key, value> pair for later retrieval
//     }
    
//     fn find(id: nodes::ID, is_fnode: bool) {
//         //TODO: Find a node, and then either returns the node or the value associated with that
//         //node
// >>>>>>> de57393484fe5b0cdd97f1e25efcfb4adf007aeb
    }

    pub fn send_rpc(&self, node_from: nodes::ZipNode, node_to: nodes::ID, msg_type: u8) {
        // let id_print : [u8; 20] =  <nodes::Node as nodes::NodeTrait>::get_id(&node_to);
        // let smaller_node_from : nodes::ZipNode = <nodes::ZipNode as nodes::RoutingTable>::new(nodes::ID{id:<nodes::Node as nodes::NodeTrait>::get_id(&node_from)},
        //                                                                <nodes::Node as nodes::NodeTrait>::get_ip(&node_from), 
        //                                                                <nodes::Node as nodes::NodeTrait>::get_port(&node_from));
        // let msg = RPCMessage{
        //             caller: smaller_node_from, 
        //             callee_id: nodes::ID{id: id_print}, 
        //             payload: RPCType::Ping_Reply(true)
        //          };
        /*if msg_type == 1 { //Ping
            msg.payload = Ping(nodes_to);
        } else if msg_type == 2 { //Store
            msg.payload = Store();
        } else if msg_type == 3 { //Find_Node
            msg.payload = Find_Node();
        } else if msg_type == 4 { //Find_Value
            msg.payload = Find_Value();
        } else if msg_type == 6 { //Store_Reply
            msg.payload = Store_Reply();
        } else if msg_type == 7 { //Find_Reply
            msg.payload = Find_Reply();
        } //No Ping_Reply cuz it's default
        let serialize = serde_json::to_string().unwrap();*/
    }
    
    pub fn read_rpc(&self, ser_msg: String) {
        //TODO
        //let deserialize: RPCMessage = serde_json::from_str(&ser_msg).unwrap();
    }
}
