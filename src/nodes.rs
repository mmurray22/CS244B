#[allow(non_snake_case)]
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use queue::*;
use std::collections::HashMap;

#[allow(unused_imports)]
use std::ops::{Index,IndexMut};

const BUCKET_SIZE: usize = 20; //Maximum length of kbuckets
const BIT_SLICES: usize = 20; //8*20 = 160 bits

#[allow(dead_code)]
const DISTANCE_POINTS: usize = 160; //160 distance points

#[allow(dead_code)]
struct Pair {
    key: u64,
    value: u64
}

#[derive(Copy, Clone)]
pub struct ID([u8; BIT_SLICES]);

pub struct Node {
    id: ID,
    ip: String,
    port: u64,
    storage: Vec<Pair>,
    kbuckets: HashMap<u64, Queue<NodeZip>>,
}

#[derive(Clone)]
pub struct NodeZip {
    id: ID,
    ip: String,
    port: u64,
}

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

pub trait NodeTrait {
    fn new (ip: String, port: u64) -> Box<Node>;
    fn get_ip(node: &Box<Node>) -> String;
    fn get_port(node: &Box<Node>) -> u64;
    fn get_id(node: &Box<Node>) -> [u8; BIT_SLICES];
    fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> ID;
    fn update_node_state (self, args: u64, _ip: String, _port: u64, _value: u64) -> bool;
    fn update_k_bucket (primary_node: &mut Box<Node>, additional_node: &Box<Node>, i: u64) -> bool;
    fn store_value (key: u64, value: u64, node: &mut Box<Node>) -> bool;
}

impl NodeTrait for Node {
    fn new (ip: String, port: u64) -> Box<Node> {
        let node = Box::new(Node{
                                 id: ID::get_random_node_id(),
                                 ip: ip,
                                 port: port,
                                 storage: Vec::new(),
                                 kbuckets: HashMap::new(),
                            });
        node
    }

    fn get_ip (node: &Box<Node>) -> String {
        (node).ip.clone()
    }

    fn get_port (node: &Box<Node>) -> u64 {
        (node).port
    }

    fn get_id (node: &Box<Node>) -> [u8; BIT_SLICES] {
        (node).id.0
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

    fn update_k_bucket (primary_node: &mut Box<Node>, additional_node: &Box<Node>, i: u64) -> bool {
        let small_node = NodeZip{
                                  id: additional_node.id, 
                                  ip: additional_node.ip.clone(), 
                                  port: additional_node.port,
                        };
        if primary_node.kbuckets.contains_key(&i) {
            if let Some(x)  = primary_node.kbuckets.get_mut(&i) { 
                x.queue(small_node).unwrap();
            }
        } else {
            let mut q = Queue::with_capacity(BUCKET_SIZE);
            q.queue(small_node).unwrap();
            primary_node.kbuckets.entry(i).or_insert(q);
        }
        true
    }

    fn store_value (key: u64, val: u64, node: &mut Box<Node>) -> bool {
        let pair = Pair{key: key, value: val};
        node.storage.push(pair);  
        true
    }
}
