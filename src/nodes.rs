//#[allow(non_snake_case)]
#![feature(linked_list_remove)]
use std::collections::LinkedList;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
//use std::collections::LinkedList;
use futures::{Stream};
use futures::future::Future;
use std::collections::BTreeMap;

const BUCKET_SIZE: usize = 20; //Maximum length of kbuckets
const BIT_SLICES: usize = 20; //8*20 = 160 bits

#[allow(dead_code)]
const DISTANCE_POINTS: usize = 160; //160 distance points

//const DEFAULT_NODES

#[allow(dead_code)]
struct Pair {
    key: u64,
    value: u64
}

#[derive(Copy, Clone)]
pub struct ID{
    pub id: [u8; BIT_SLICES],
}

pub struct Node {
    id: ID,
    ip: String,
    port: u64,
    storage: Vec<Pair>,
    kbuckets: Vec<LinkedList<ZipNode>>,
}

#[derive(Clone)]
pub struct ZipNode {
    id: ID,
    ip: String,
    port: u64,
}

trait IDTrait {
    fn get_id(self) -> ID; /**/
    fn get_key_hash(key: u64, res: &mut [u8]); /*Sha1 Hashes key*/
    fn xor(id1: ID, id2: ID) -> u64;
    fn get_random_node_id () -> ID;
}

impl IDTrait for ID {
    fn get_id(self) -> ID {
        ID{id: self.id}  
    }
    fn get_key_hash(key: u64, res: &mut [u8]) {
        let mut hasher = Sha1::new();
        hasher.input(&key.to_ne_bytes());
        hasher.result(res);
    }

    fn xor(id1: ID, id2: ID) -> u64 {
       let mut temp_id = [0; BIT_SLICES];
       let mut length_of_prefix : u64 = 0;
       for i in 0..BIT_SLICES {
            temp_id[i] = id1.id[i]^id2.id[i];
            if temp_id[i] == 0 {
                length_of_prefix+=1;
            }
       }
       /*What is the closeness factor?*/
       length_of_prefix
    }

    fn get_random_node_id() -> ID {
        let array: [u8; BIT_SLICES] = rand::random();
        ID{id: array}
    }

}

pub trait NodeTrait {
    fn new (ip: String, port: u64) -> Box<Node>;
    fn get_ip(node: &Box<Node>) -> String;
    fn get_port(node: &Box<Node>) -> u64;
    fn get_id(node: &Box<Node>) -> [u8; BIT_SLICES];
    fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> u64;
    fn update_node_state (self, args: u64, _ip: String, _port: u64, _value: u64) -> bool;
    fn update_k_bucket (primary_node: &mut Box<Node>, additional_node: &Box<Node>, i: usize) -> bool;
    fn store_value (key: u64, value: u64, node: &mut Box<Node>) -> bool;
}

impl NodeTrait for Node {
    fn new (ip: String, port: u64) -> Box<Node> {
        let node = Box::new(Node{
                                 id: ID::get_random_node_id(),
                                 ip: ip,
                                 port: port,
                                 storage: Vec::new(),
                                 kbuckets: Vec::with_capacity(DISTANCE_POINTS),
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
        (node).id.id
    }

    fn key_distance (node_id1: [u8; 20], node_id2: [u8; 20]) -> u64 {
        ID::xor(ID{id: node_id1}, ID{id: node_id2})
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

    fn update_k_bucket (primary_node: &mut Box<Node>, additional_node: &Box<Node>, i: usize) -> bool {
        let small_node = ZipNode{
                                  id: additional_node.id, 
                                  ip: additional_node.ip.clone(), 
                                  port: additional_node.port,
                        };
        ZipNode::add_entry(primary_node, small_node, i)
    }

    fn store_value (key: u64, val: u64, node: &mut Box<Node>) -> bool {
        let pair = Pair{key: key, value: val};
        node.storage.push(pair);  
        true
    }
}

pub trait RoutingTable {
    fn check_zipnode(main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize /*ID distance*/) -> bool;
    fn add_entry(main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize /*ID distance*/) -> bool;
    fn remove_entry(main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize) -> bool;
}

impl PartialEq for ZipNode {
    fn eq(&self, other: &Self) -> bool {
        self.id.id == other.id.id
    }
}

impl RoutingTable for ZipNode {
    fn check_zipnode (main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize) -> bool {
        //1. Check if there is room to add a ZipNode
        if main_node.kbuckets[i].len() == BUCKET_SIZE {
            /*Just check if oldest of 20 nodes is dead*/
            if /*TODO check_node(main_node.kbuckets[i].back().unwrap().clone())*/ true {
                return false;
            }
        }
        //2. Check if the ZipNode is already in a kbucket 
        for element in main_node.kbuckets[i].iter_mut() {
            if *element == zip_node {
                return false;
            }
        }
        //maybe add the distance index to the ZipNode struct?
        true
    }

    fn add_entry(main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize /*ID distance*/) -> bool {
        //1. If the above checks all fail, then you can add the ZipNode to the kbucket!
        if !Self::check_zipnode(main_node, zip_node.clone(), i) {
            return true;
        }
        if main_node.kbuckets.len() >= i+1 {
            if let Some(x)  = main_node.kbuckets.get_mut(i) {
                x.push_back(zip_node);
            }
        } else {
            let mut q = LinkedList::new();
            q.push_back(zip_node);
            main_node.kbuckets[i] = q;
        }
        false
    }

    fn remove_entry(main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize) -> bool {
        let mut counter = 0;
        for element in main_node.kbuckets[i].iter_mut() {
            if *element == zip_node {
                //TODO: main_node.kbuckets[i].remove(counter);
                break;
            }
            counter+=1;
        }
        true
    }
}

