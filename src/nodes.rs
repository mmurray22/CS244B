#[allow(non_snake_case)]
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use queue::*;

const K_CONST: u64 = 20; //Maximum length of kbuckets

struct Pair {
    key: u64,
    value: u64
}

pub struct Node {
    node_id: ID,
    ip: String,
    port: u64,
    storage: Vec<Pair>,
    kbuckets: Vec<Queue<String>>,//TODO: What type should be inside the Vector?
}

pub trait NodeTrait {
    fn new (ip: String, port: u64) -> Box<Node>;
    fn destroy_node(_destroy_node: Node) -> bool;
    fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> ID;
    fn update_node_state (self, args: u64, _ip: String, _port: u64, _value: u64) -> bool;
    fn update_k_bucket (/*what variables go in here?*/) -> bool;
    fn store_value (key: u64, value: u64) -> bool;
}

const BIT_SLICES: usize = 20;   

pub struct ID([u8; BIT_SLICES]);

trait IDTrait {
    fn get_id(self) -> ID; /**/
    fn get_key_hash(key: u64, res: &mut [u8]); /*Sha1 Hashes key*/
    fn xor(id1: ID, id2: ID) -> ID;
    fn get_random_node_id () -> ID;
}

impl IDTrait for ID {
    fn get_id(self) -> ID {
        ID(self.0)  
    }
    fn get_key_hash(key: u64, res: &mut [u8]) {
        let mut hasher = Sha1::new();
        hasher.input(&key.to_ne_bytes());
        hasher.result(res);
    }

    fn xor(id1: ID, id2: ID) -> ID {
       let mut temp_id = [0; BIT_SLICES];
       for i in 0..BIT_SLICES {
            temp_id[i] = id1.0[i]^id2.0[i];
       }
       ID(temp_id)
    }

    fn get_random_node_id() -> ID {
        let array: [u8; BIT_SLICES] = rand::random();
        ID(array)
    }

}

impl NodeTrait for Node {
    fn new (ip: String, port: u64) -> Box<Node> {
        let node = Box::new(Node{
                                 node_id: ID::get_random_node_id(),
                                 ip: ip,
                                 port: port,
                                 storage: Vec::new(),
                                 kbuckets: Vec::new(),
                            });
        node
    }

    fn destroy_node(_destroy_node: Node) -> bool {
        /*TODO LIFETIMES*/
        /*Need to add to this body->I don't think you explicitly destroy nodes*/  
        true
    }

    fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> ID {
        ID::xor(ID(node_id1), ID(node_id2))
    }

    fn update_node_state(mut self, args: u64, _ip: String, _port: u64, _value: u64) -> bool {
        if args == 1 { // 01 = ip changed, port not changed
            self.ip = _ip;
        } else if args == 2 { //10 = port changed, ip not changed
            self.port = _port;
        } else if args == 3 { //11 = ip, port both changed
            self.ip = _ip;
            self.port = _port;
        }
        true
    }

    fn update_k_bucket () -> bool {
        true
        /*Determine whether or not */ 
    }

    fn store_value (key: u64, val: u64) -> bool {
        let pair = Pair{key: key, value: val};
        true
    }
}
