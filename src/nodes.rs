#![feature(alloc)]
use std::mem;
use std::alloc::oom;
use std::boxed::Box;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;

//! TODO Consider if keys will be stored using crypto hashes (they should be)
struct Node {
    node_id: [u8; 20],
    ip: String,
    port: u64,
    value: u64,
    kbuckets: Vec<Vec<Node>> //TODO: What type should be inside the Vector?
}

trait NodeTrait {
    fn get_random_node_id () -> ID;
    fn create(size: u64, ip: String, port: u64) -> Node;
    fn destroy_node() -> Node;
    fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> Bool;
    fn node_state(self, args: u64, _ip: String, _port: u64) -> Node;
    fn update_node_state (self, args: u64, _ip: String, _port: u64) -> Bool;
    fn update_k_bucket () -> Bool;
    fn store_value () -> Bool;
}

const BIT_SLICES: u64 = 20;   

struct ID([u8; BIT_SLICES]); /*TODO: incorporate SHA1 hash/change to bit arrays in rust*/

trait IDTrait {
    fn get_id(self) -> ID;
    fn XOR(id1: ID, id2: ID) -> ID;
}

impl IDTrait for ID {
    fn get_id(self) -> ID {
        self.0  
    }

    fn XOR(id1: ID, id2: ID) -> ID {
       let mut tempID = [0; BIT_SLICES];
       for i in 0..BIT_SLICES {
            tempID[i] = id1[i]^id2[i];
       }
       tempID
    }
}

impl NodeTrait for Node {
    pub fn get_random_node_id() -> ID {
        let array: [u8; BIT_SLICES] = rand::random();
        ID(array)
    }

    pub fn new (size: u64, ip: String, port: u64) -> Node {
        let mut node = Box::<Node>::new_uninit();
        let node = unsafe {
            node.
        };
        Node(get_random_node_id(), ip, port);
    }

    pub fn destroy_node(destroy_node: Node) -> Node {
        /*TODO Deallocate node*/          
    }

    pub fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> Bool {
        ID::XOR(ID(node_id1), ID(node_id2))
    }

    pub fn update_node_state(self, args: u64, _ip: String, _port: u64) -> Bool {
        if (args == 1) { // 01 = ip changed, port not changed
            self.ip = _ip;
        } else if (args == 2) { //10 = port changed, ip not changed
            self.port = _port;
        } else if (args == 3) { //11 = ip, port both changed
            self.ip = _ip;
            self.port = _port;
        }
    }

    pub fn update_k_bucket () -> Bool {
        /*Determine whether or not */ 
    }
}
