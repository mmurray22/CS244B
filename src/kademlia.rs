//! Kademlia protocol: What exactly needs to go in here?
//! Okay, so what goes here is the API that will be exposed
//! to the user. Since the user cannot know about all the node
//! and XOR metric and etc. shenanigans, this will serve as the 
//! sole interaction with the system the "user" will have with 
//! the Kademlia DHT (and for actual real users another layer will be
//! built on top of this, but that is for later)


pub fn ping (Node) -> Node {
}

pub fn store ((key, value)) -> Node {
}

pub fn find_node_with_id (Node_ID) -> Node {
}

pub fn find_node_with_key (key) -> Node {
}

pub fn find_value (key) -> Node {
}   
