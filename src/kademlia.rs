//! Kademlia protocol: What exactly needs to go in here?
//! Okay, so what goes here is the API that will be exposed
//! to the user. Since the user cannot know about all the node
//! and XOR metric and etc. shenanigans, this will serve as the 
//! sole interaction with the system the "user" will have with 
//! the Kademlia DHT (and for actual real users another layer will be
//! built on top of this, but that is for later)

//!TODO Questions which need to be answered: Are we little or big endian?
#[path = "./nodes.rs"] mod nodes;

pub fn ping (node1: nodes::Node) /*-> nodes::Node*/ {
    /*Find Node spot in the dependency tree & check if it's there*/
}

pub fn store (key: u64, value: u64) /*-> nodes::Node*/ {
    /*Inserts (key, value) into the DHT*/
}

pub fn find_node (node_id: nodes::ID) /*-> nodes::Node*/ {
    /*Searches the tree and finds the corresponding Node to the node id*/
}

pub fn find_value (key: u64) /*-> nodes::Node*/ {
    /*Search the tree and find the corresponding value to the key*/
}   
