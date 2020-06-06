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

pub enum RequestType {
    Ping(nodes::Node),
    Store(u64, u64),
    Find_Node(nodes::ID),
    Find_Value(u64)
}

pub struct Request {
    pub caller: nodes::Node,
    pub callee_id: nodes::ID,
    pub payload: RequestType,
}

pub enum ResponseType {
    Successful(bool),
    TargetNode(nodes::Node),
}

pub struct Response {
    pub request: Request,
    pub callee_id: nodes::ID,
    pub payload: ResponseType,
}

pub trait rpcfxns {
    pub fn send_ping();
    pub fn respond_ping();
    pub fn send_store();
    pub fn respond_store();
    pub fn send_fnode();
    pub fn respond_fnode();
    pub fn send_fvalue();
    pub fn respond_fvalue();
    pub fn send_RPC(node_from: nodes::Node, node_to: nodes::Node);
}

impl rpcfxns {
    pub fn send_ping(node: nodes::Node) {
        send_RPC();
    }

    pub fn respond_ping() -> bool {
        true
    }

    pub fn send_store() {
    }

    pub fn respond_store() {
    }

    pub fn send_fnode() {
    }

    pub fn respond_fnode() {
    }

    pub fn send_fvalue() {
    }

    pub fn respond_fvalue() {
    }

    pub fn send_RPC(node_from: nodes::Node, node_to: nodes::Node) {
        let serve_future = Server::bind(&addr)
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
        socket.async();
    }
}
