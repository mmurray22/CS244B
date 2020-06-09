use std::collections::LinkedList;
use std::str::FromStr;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
//use std::collections::LinkedList;

pub const BUCKET_SIZE: usize = 20; //Maximum length of kbuckets
const BIT_SLICES: usize = 20; //8*20 = 160 bits
const ALPHA : usize = 3;

#[allow(dead_code)]
const DISTANCE_POINTS: usize = 160; //160 distance points

pub const K: usize = 2;
pub const SIG: usize = 1;

//const DEFAULT_NODES

#[allow(dead_code)]
pub struct Pair {
    key: u64,
    value: u64
}

#[derive(Copy, Clone, Debug)]
pub struct ID{
    pub id: [u8; BIT_SLICES],
}

pub struct Node {
    pub id: ID,
    pub ip: String,
    pub port: u64,
    pub storage: Vec<Pair>,
    pub kbuckets: Vec<LinkedList<ZipNode>>,
}

#[derive(Clone)]
pub struct ZipNode {
    pub id: ID,
    pub ip: String,
    pub port: u64,
}

trait IDTrait {
    fn get_id(self) -> ID; /**/
    fn get_key_hash(key: u64, res: &mut [u8]); /*Sha1 Hashes key*/
    fn xor(id1: ID, id2: ID) -> usize;
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

    fn xor(id1: ID, id2: ID) -> usize {
       let mut temp_id = [0; BIT_SLICES];
       let mut length_of_prefix : usize = 0;
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

impl FromStr for ID {
    type Err = std::num::ParseIntError;
    fn from_str(input_id: &str) -> Result<Self, Self::Err> {
        let mut array: [u8; BIT_SLICES] = rand::random();
        for mut i in 0..input_id.len() {
            let converted: u8 = u8::from_str_radix(&input_id[i..i+3], 10)?;            
            array[i/3] = converted;
            i+=3;
        }
        Ok(ID {id: array})
    }
}

impl Node {
    pub fn new (ip: String, port: u64) -> Box<Node> {
        let mut node = Box::new(Node{
            id: ID::get_random_node_id(),
            ip: ip,
            port: port,
            storage: Vec::new(),
            kbuckets: Vec::with_capacity(DISTANCE_POINTS),
        });
        node.kbuckets = vec![LinkedList::new(); DISTANCE_POINTS];
        //TODO: Populate kbuckets with default nodes!
        node
    }

    pub fn get_ip (&self) -> String {
        self.ip.clone()
    }

    pub fn get_port (&self) -> u64 {
        self.port
    }

    pub fn get_id (&self) -> ID { // [u8; BIT_SLICES] {
        self.id
    }

    pub fn key_distance (node_id1: ID, node_id2: ID) -> usize {
        ID::xor(node_id1, node_id2)
    }

    pub fn update_node_state(mut self, args: u64, _ip: String, _port: u64, _value: u64) -> bool {
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

    pub fn update_k_bucket (primary_node: &mut Box<Node>, additional_node: &Box<Node>) -> bool {
        let small_node = ZipNode{
                                  id: additional_node.id, 
                                  ip: additional_node.ip.clone(), 
                                  port: additional_node.port,
                        };
        ZipNode::add_entry(primary_node, small_node)
    }

    pub fn store_value (&mut self, key: u64, val: u64) -> bool {
        let pair = Pair{key: key, value: val};
        self.storage.push(pair);  
        true
    }
}

impl PartialEq for ZipNode {
    fn eq(&self, other: &Self) -> bool {
        self.id.id == other.id.id
    }
}

impl ZipNode {
    pub fn new(base_id: ID, base_ip: String, base_port: u64) -> ZipNode {
        let default_zip = ZipNode{
            id: base_id,
            ip: base_ip,
            port: base_port,
        };
        default_zip
    }
    
    pub fn check_zipnode (main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize) -> bool {
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

    pub fn add_entry(main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode) -> bool {
        //1. If the above checks all fail, then you can add the ZipNode to the kbucket!
        let i : usize = Node::key_distance(main_node.id, zip_node.id);
        if  main_node.kbuckets.len() >= i+1 &&
            !Self::check_zipnode(main_node, zip_node.clone(), i) {
            return false;
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
        true
    }

    pub fn remove_entry(main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize) -> bool {
        let mut counter = 0;
        for element in main_node.kbuckets[i].iter_mut() {
            if *element == zip_node {
                //main_node.kbuckets[i].remove(counter);
                break;
            }
            counter+=1;
        }
        true
    }

    /*Find ALPHA closest nodes*/
    //pub fn lookup_init(key: ID) -> Vec<ZipNode> /*ALPHA Nodes*/
    pub fn lookup_init(target_id: ID, self_id: ID, kbuckets: Vec<LinkedList<ZipNode>>)
                        -> Vec<ZipNode>{
        //1. Get all k nodes with IDs closest to the target_id 
        let mut ret_vec = Vec::with_capacity(BUCKET_SIZE);
        let mut dist = Node::key_distance(target_id, self_id);
        loop {
            if ret_vec.len() < ALPHA && dist != 0 {
                dist-=1;
            } else {
                break;
            }
            let mut iter = kbuckets[dist].iter();
            while iter.next() != None {
                if ret_vec.len() < ALPHA {
                    ret_vec.push((iter.next().unwrap()).clone());
                } else {
                    break;
                }
            }

        }
        return ret_vec;
    }

    pub fn lookup_update(target_id: ID, zip_node : ZipNode, closest_k: &mut Vec<ZipNode>) -> Vec<ZipNode> {
        //2. Order those k nodes and select the closest ALPHA
        closest_k.sort_by(|a, b| (Node::key_distance(b.id, target_id)).cmp(&(Node::key_distance(a.id, target_id))));
        let ret = Node::key_distance(closest_k[0].id, target_id) > Node::key_distance(zip_node.id, target_id) &&
                  Node::key_distance(closest_k[1].id, target_id) > Node::key_distance(zip_node.id, target_id) &&
                  Node::key_distance(closest_k[2].id, target_id) > Node::key_distance(zip_node.id, target_id);
        //2.5 If there is no closest ALPHA, then return!
        if ret {
            let mut ret_vec = Vec::<ZipNode>::new();
            ret_vec.push(zip_node);
            return ret_vec;
        }
        //3. ?? Recursively lookup nodes in those nodes TODO: Need to change to RPC calls!
        //let node_one = RPCMessage::lookup(self, target_id, closest_k[0].clone(), origin_kbuckets.clone());
        //let node_two = RPCMessage::lookup(self, target_id, closest_k[1].clone(), origin_kbuckets.clone());
        //let node_three = RPCMessage::lookup(self, target_id, closest_k[2].clone(), origin_kbuckets.clone());
        
		let dummy = Vec::<ZipNode>::new();
        dummy
    }

    pub fn lookup_end(key: ID) -> () {
    
    }
}
