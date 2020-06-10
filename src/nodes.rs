use std::collections::LinkedList;
use std::str::FromStr;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub const K_SIZE: usize = 20; //Maximum length of kbuckets
const BIT_SLICES: usize = 20; //8*20 = 160 bits
const ALPHA : usize = 3;

#[allow(dead_code)]
const DISTANCE_POINTS: usize = 160; //160 distance points


#[allow(dead_code)]
pub struct Pair {
    key: u64,
    value: u64
}

#[derive(Copy, Clone, Debug, Hash, Eq)]
pub struct ID{
    pub id: [u8; BIT_SLICES],
}

#[derive(Clone)]
pub struct Node {
    id: ID,
    key: u64,
    ip: String,
    port: u64,
    lookup_map: HashMap<u64,(Option<u64>, HashSet<ZipNode>, HashSet<ZipNode>)>,
    lookup_counter: u64,
    pub storage: HashMap<u64,u64>,
    kbuckets: Vec<LinkedList<ZipNode>>,
}

#[derive(Clone, Hash, Eq)]
pub struct ZipNode {
    pub id: ID,
    pub ip: String,
    pub port: u64,
}

impl PartialEq for ZipNode {
    fn eq(&self, other: &Self) -> bool {
        self.id.id == other.id.id
    }
}

impl PartialEq for ID {
    fn eq(&self, other: &Self) -> bool {
        self.id.iter().zip(other.id.iter()).all(|(a,b)| a == b) 
    }
}

trait IDTrait {
    fn get_id(self) -> ID; /**/
    fn get_key_hash(key: u64) -> [u8; BIT_SLICES]; /*Sha1 Hashes key*/
    fn xor(id1: ID, id2: ID) -> usize;
    fn get_random_node_id () -> ID;
}

impl ID {
    fn get_id(self) -> ID {
        ID{id: self.id}  
    }
    pub fn get_key_hash(key: u64) -> [u8; BIT_SLICES]{
        let mut hasher = Sha1::new();
        let mut array = [0; BIT_SLICES];
        hasher.input(&key.to_ne_bytes());
        hasher.result(&mut array);
        array
    }

    pub fn xor(id1: ID, id2: ID) -> usize {
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

    pub fn get_random_node_id() -> ID {
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
        let key = rand::random();
        // let mut array: [u8; BIT_SLICES] = [0; BIT_SLICES];
        let mut node = Box::new(Node{
            id: ID {id: ID::get_key_hash(key)},
            key,
            ip: ip,
            port: port,
            storage: HashMap::new(),
            lookup_map: HashMap::new(),
            lookup_counter: 0,
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

    pub fn get_id (&self) -> ID { 
        self.id
    }

    pub fn get_key (&self) -> u64 { 
        self.key
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
        let small_node : ZipNode =  ZipNode::new(additional_node);
        ZipNode::add_entry(primary_node, small_node)
    }

    pub fn find_closest(&mut self, target_id: u64, max_size: usize) -> Vec<ZipNode>{
        let mut ret_vec = Vec::with_capacity(K_SIZE);
        let mut dist = Node::key_distance(self.id, ID{id: ID::get_key_hash(target_id)});
        /*for elem in &self.kbuckets {
            println!("SIZE OF LINKED LIST: {:?}", (*elem).len());
        }*/
        loop {
            //println!("DIST: {:?}, ALPHA: {:?}, LEN: {:?}", dist, ALPHA, ret_vec.len());
            if ret_vec.len() == max_size {
                break;
            }
            //println!("DIST: {:?}", dist);
            for elem in self.kbuckets[dist].iter_mut() {
                if ret_vec.len() < max_size {
                    ret_vec.push(elem.clone());
                } else {
                    break;
                }
            }
            if (dist == 0) {
                break;
            }
            dist-=1;
        }
        // println!("SIZE: {:?}", ret_vec.len());
        return ret_vec;
    }

    // Takes in target_id, and possibly a value to later store if store is true
    // Returns a vector of zipnodes to send to, and a lookup key
    pub fn lookup_init(&mut self, target_id: u64, val: u64, store: bool) -> (Vec<ZipNode>, u64) {
        let zips = self.find_closest(target_id, ALPHA);
        // println!("ORIGINAL SIZE: {:?}", zips.len());
        let lookup_key = self.lookup_counter;
        if store {
            self.lookup_map.insert(lookup_key, (Some(val), HashSet::from_iter(zips.iter().cloned()), HashSet::new()));
        } else {
            self.lookup_map.insert(lookup_key, (None, HashSet::from_iter(zips.iter().cloned()), HashSet::new()));
        }

        return (zips,lookup_key);
    }

    // Takes in the zip node it recieved an RPC from, its k_closest nodes, and a lookup key
    // Returns a vector of zipnode replys, possibly a value, 
    pub fn lookup_update(&mut self, rec_zip: ZipNode, k_closest: Vec<ZipNode>, 
        _target_key:u64, lookup_key: u64) -> (Vec<ZipNode>, u64, bool, bool) {
        
        match self.lookup_map.get_mut(&lookup_key) {
            Some((opt_val, sent, rec)) => {
                // If recieve is valid add sent messages
                if sent.contains(&rec_zip) {
                    let mut new_zips = Vec::new();
                    // Add k_closest to sent set
                    for zip in k_closest {
                        if !rec.contains(&zip) && !sent.contains(&zip) {
                            sent.insert(zip.clone());
                            new_zips.push(zip.clone());
                            // Return at most only ALPHA requests
                            if new_zips.len() == ALPHA {
                                break;
                            }
                        }
                    }
                    sent.remove(&rec_zip);
                    rec.insert(rec_zip.clone());

                    match opt_val {
                        // Send store rpcs or continue searching
                        Some(val) => {
                            if sent.len() == 0 {
                                return (new_zips, *val, true, true)
                            } else {
                                return (new_zips, *val, true, false);
                            }
                        },
                        // Stop or continue searching
                        None => {
                            if sent.len() == 0 {
                                return (new_zips, 0, false, true);
                            } else {
                                return (new_zips, 0, false, false);
                            }
                        }
                    }
                } 
                return (Vec::new(), 0, false, false);
            },
            None => return (Vec::new(), 0, false, false)
        }
    }


    pub fn lookup_end(&mut self, lookup_key: u64) {
        self.lookup_map.remove(&lookup_key);
    }
}


impl ZipNode {
    pub fn new(node: &Box<Node>) -> ZipNode {
        let default_zip = ZipNode{
            id: node.id.clone(),
            ip: node.ip.clone(),
            port: node.port.clone(),
        };
        default_zip
    }
    
    pub fn check_zipnode (main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode, i: usize) -> bool {
        //1. Check if there is room to add a ZipNode
        if main_node.kbuckets[i].len() == K_SIZE {
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
        /*let mut counter = 0;
        for element in main_node.kbuckets[i].iter_mut() {
            if *element == zip_node {
                main_node.kbuckets[i].remove(counter);
                break;
            }
            counter+=1;
        }*/
        true
    }
}
