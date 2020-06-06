//! Kademlia protocol: What exactly needs to go in here?
//! Okay, so what goes here is the API that will be exposed
//! to the user. Since the user cannot know about all the node
//! and XOR metric and etc. shenanigans, this will serve as the 
//! sole interaction with the system the "user" will have with 
//! the Kademlia DHT (and for actual real users another layer will be
//! built on top of this, but that is for later)

//!TODO Questions which need to be answered: Are we little or big endian?
#[path = "./nodes.rs"] mod nodes;

const ALPHA : u64 = 3;

pub enum RPCType {
    Ping(nodes::Node),
    Store(u64, u64),
    Find_Node(nodes::ID),
    Find_Value(u64)
}

pub struct RPCMessage {
    pub caller: nodes::Node,
    pub callee_id: nodes::ID,
    pub payload: RPCType,
}

pub enum ResponseType {
    Successful(bool),
    TargetNode(nodes::Node),
}

pub trait rpcfxns {
    fn send_ping(node: nodes::Node);
    fn respond_ping() -> bool;
    fn send_store();
    fn respond_store();
    fn send_fnode();
    fn respond_fnode();
    fn send_fvalue();
    fn respond_fvalue();
    fn send_RPC(node_from: nodes::Node, node_to: nodes::Node);
}

impl rpcfxns for RPCMessage {
    fn send_ping(node: nodes::Node) {
        //send_RPC();
    }

    fn respond_ping() -> bool {
        true
    }

    fn send_store() {
    }

    fn respond_store() {
    }

    fn send_fnode() {
    }

    fn respond_fnode() {
    }

    fn send_fvalue() {
    }

    fn respond_fvalue() {
    }

    fn send_RPC(node_from: nodes::Node, node_to: nodes::Node) {
        /*let serve_future = Server::bind(&addr)
        // Serve requests using our `async serve_req` function.
        // `serve` takes a type which implements the `MakeService` trait.
        // `make_service_fn` converts a closure into a type which
        // implements the `MakeService` trait. That closure must return a
        // type that implements the `Service` trait, and `service_fn`
        // converts a request-response function into a type that implements
        // the `Service` trait.
        .serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(serve_req))
        }));
        socket.async();*/
    }
}
