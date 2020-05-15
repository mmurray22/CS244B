//! Consider if keys will be stored using crypto hashes (they should be)
struct ID([u8; 20]);

trait IDTrait {
    fn get_id(self) -> ID;

    fn XOR(id1: ID, id2: ID) -> ID;
}

impl IDTrait for ID {
    fn get_id(self) -> ID {
        self.0 //TODO: actually need to think through to print out 
    }

    fn XOR(id1: ID, id2: ID) -> ID {
       id1^id2 
    }
}

pub fn get_random_node_id (/**/) -> ID {
}

pub fn create_node (/*Information necessary for creating a node*/) -> Node {
    /// TODO: Craft Node struct
    /// TODO: Assign Node struct random 160-bit node ID
    /// TODO: Fill in other appropriate details
}

pub fn destroy_node () -> Node {
}

pub fn key_distance (/*Key from pair that is being assigned*/) -> Bool {
    /// TODO: Implement XOR metric 
}

pub fn update_node_state () -> Bool {
}

pub fn update_k_bucket () -> Bool {
}

pub fn store_value () -> Bool {
}
