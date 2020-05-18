#![feature(alloc)]
use std::mem;
use std::alloc::oom;
use std::boxed::Box;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use queues::*;

struct Node {
    node_id: [u8; 20],
    ip: String,
    port: u64,
    value: u64,
    kbuckets: Vec<Queue<Node>> //TODO: What type should be inside the Vector?
}

trait NodeTrait {
    fn get_random_node_id () -> ID;
    fn create(size: u64, ip: String, port: u64) -> Node;
    fn destroy_node() -> Node;
    fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> Bool;
    fn update_node_state (self, args: u64, _ip: String, _port: u64, _value: u64) -> Bool;
    fn update_k_bucket () -> Bool;
    fn store_value () -> Bool;
}

const BIT_SLICES: u64 = 20;   

struct ID([u8; BIT_SLICES]); /*TODO: incorporate SHA1 hash/change to bit arrays in rust*/

trait IDTrait {
    fn get_id(self) -> ID; /**/
    fn get_key_hash(key: u64) -> ID; /*Sha1 Hashes key*/
    fn XOR(id1: ID, id2: ID) -> ID;
}

impl IDTrait for ID {
    fn get_id(self) -> ID {
        self.0  
    }
    fn get_key_hash(key: u64) -> ID {
        let mut hasher = Sha1::new();
        hasher.input_str(key);
        hasher.result()
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
            node.node_id.as_mut_ptr().write(get_random_node_id());
            node.ip.as_mut_ptr().write(ip);
            node.port.as_mut_ptr().write(port);
            node.value.as_mut_ptr().write(0);
            node.kbuckets.as_mut_ptr().write(/*How do I define a k bucket?*/);
            node.assume_init()
        };
        node
    }

    pub fn destroy_node(destroy_node: Node) -> Node {
                   
    }

    pub fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> Bool {
        ID::XOR(ID(node_id1), ID(node_id2))
    }

    pub fn update_node_state(self, args: u64, _ip: String, _port: u64, _value: u64) -> Bool {
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
