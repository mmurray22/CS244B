use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub const K_SIZE: usize = 20; //Maximum length of kbuckets
// const BIT_SLICES: usize = 20; //8*20 = 160 bits
#[allow(dead_code)]
const ALPHA : usize = 3;
const DISTANCE_POINTS: usize = 64;


#[allow(dead_code)]
pub struct Pair {
    key: u64,
    value: u64
}

#[derive(Clone)]
pub struct Node {
    // id: ID,
    id: u64,
    ip: String,
    port: u64,
    lookup_map: HashMap<u64,(Option<u64>, HashSet<ZipNode>, HashSet<ZipNode>)>,
    lookup_counter: u64,
    pub storage: HashMap<u64,u64>,
    kbuckets: Vec<Vec<ZipNode>>,
}

#[derive(Clone, Hash, Eq)]
pub struct ZipNode {
    // pub id: ID,
    pub id: u64,
    pub ip: String,
    pub port: u64,
}

impl PartialEq for ZipNode {
    fn eq(&self, other: &Self) -> bool {
        // self.id.id == other.id.id
        self.id == other.id
    }
}

pub fn get_ip_hash(ip: String) -> u64{
    let mut s = DefaultHasher::new();
    ip.hash(&mut s);
    s.finish()
}


impl Node {
    pub fn new (ip: String, port: u64) -> Box<Node> {
        // let key = rand::random();
        let mut node = Box::new(Node{
            // id: ID {id: ID::get_key_hash(key)},
            // key,
            id: get_ip_hash(ip.clone()),
            ip: ip,
            port: port,
            storage: HashMap::new(),
            lookup_map: HashMap::new(),
            lookup_counter: 0,
            kbuckets: Vec::with_capacity(DISTANCE_POINTS),
        });
        node.kbuckets = vec![Vec::with_capacity(K_SIZE); DISTANCE_POINTS];
        //TODO: Populate kbuckets with default nodes!
        node
    }

    pub fn get_ip (&self) -> String {
        self.ip.clone()
    }

    pub fn get_port (&self) -> u64 {
        self.port
    }

    pub fn get_id (&self) -> u64 { 
        self.id
    }

    pub fn key_distance (node_id1: u64, node_id2: u64) -> usize {
        let xor = node_id1 ^ node_id2;
        if xor == 0 {return 0}
        for i in 0..64 {
            let test = 1 << i;
            if xor / test == 0 {
                return i as usize;
            }  
        }
        return DISTANCE_POINTS-1;
    }



    pub fn find_closest(&mut self, target_id: u64, max_size: usize) -> Vec<ZipNode>{
        let mut ret_vec = Vec::with_capacity(K_SIZE);

        let mut dist = Node::key_distance(self.id, target_id);
        loop {
            let mut bucket = self.kbuckets[dist].clone();
            bucket.sort_by(|a, b| 
                Node::key_distance(target_id,a.id).partial_cmp(&Node::key_distance(target_id, b.id)).unwrap());

            if ret_vec.len() == max_size {
                break;
            }
            for elem in bucket.iter_mut() {
                if ret_vec.len() < max_size {
                    ret_vec.push(elem.clone());
                } else {
                    break;
                }
            }
            if dist == 63 {
                break;
            }
            dist+=1;
        }
        return ret_vec;
    }


    // Takes in target_id, and possibly a value to later store if store is true
    // Returns a vector of zipnodes to send to, and a lookup key
    pub fn lookup_init(&mut self, target_id: u64, val: u64, store: bool) -> (Vec<ZipNode>, u64) {
        let zips = self.find_closest(target_id, ALPHA);
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
                    sent.remove(&rec_zip);
                    rec.insert(rec_zip.clone());

                    let mut new_zips = Vec::new();
                    let mut dist = Node::key_distance(self.id, _target_key);
                    loop {
                        let mut bucket = self.kbuckets[dist].clone();
                        bucket.sort_by(|a, b| 
                            Node::key_distance(_target_key,a.id).partial_cmp(&Node::key_distance(_target_key, b.id)).unwrap());

                        if new_zips.len() == ALPHA {
                            break;
                        }
                        for elem in bucket.iter_mut() {
                            if new_zips.len() < ALPHA && !sent.contains(&elem) && !rec.contains(&elem){
                                sent.insert(elem.clone());
                                new_zips.push(elem.clone());
                            }
                        }
                        if dist == 63 {
                            break;
                        }
                        dist+=1;
                    }
                    // let mut new_zips = Vec::new();
                    // // Add k_closest to sent set
                    // for zip in k_closest {
                    //     if !rec.contains(&zip) && !sent.contains(&zip) {
                    //         sent.insert(zip.clone());
                    //         new_zips.push(zip.clone());
                    //         // Return at most only ALPHA requests
                    //         if new_zips.len() == ALPHA {
                    //             break;
                    //         }
                    //     }
                    // }
                    // sent.remove(&rec_zip);
                    // rec.insert(rec_zip.clone());

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


    pub fn add_entry(main_node: &mut std::boxed::Box<Node>, zip_node: ZipNode) {
        //1. If the above checks all fail, then you can add the ZipNode to the kbucket!
        let i = Node::key_distance(main_node.id, zip_node.id);

        
        for j in 0..main_node.kbuckets[i].len() {
            if main_node.kbuckets[i][j] == zip_node {
                main_node.kbuckets[i].remove(j);
                break;
            }
        }

        // No LRU eviction atmg
        if main_node.kbuckets[i].len() == K_SIZE {
            // main_node.kbuckets[i].pop();
            // main_node.kbuckets[i].push(zip_node);
        } else {
            main_node.kbuckets[i].push(zip_node);
        }
    }
}
