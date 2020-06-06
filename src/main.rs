//TODO #![feature(linked_list_remove)]

extern crate crypto;
extern crate rand;
extern crate queue;
extern crate tokio;
extern crate futures;
extern crate tokio_ping;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod nodes;
pub mod routing;
pub mod test_harness;
pub mod rpc;
pub mod kademlia;
//pub mod rpc_test_harness;

fn main () -> () {
    let args: Vec<String> = env::args().collect();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); //TODO: How to get your current IP?
    //run_server(addr).await;
    //if args[1] == "test" {
    run_test_harness(/*&args[1], &args[2]*/);
    /*} else {
        println!("Start of the main function!");
        /*TODO: 1. Make Node with an ID*/
        let ip = String::from("0.0.0.0");
        let mut test_node : Box<nodes::Node> =  <nodes::Node as nodes::NodeTrait>::new(ip, 0);
        let ip_print : String = <nodes::Node as nodes::NodeTrait>::get_ip(&test_node);
        let port_print : u64 = <nodes::Node as nodes::NodeTrait>::get_port(&test_node);
        let id_print : [u8; 20] = <nodes::Node as nodes::NodeTrait>::get_id(&test_node); 
        println!("The node has IP {}", ip_print);
        println!("The node has port {}", port_print);
        println!("The ID is: ");
        for x in &(id_print) {
            print!("{}", x);
        }
        println!("");

        /*TODO 2. Store a value in the node*/
        let key = 20;
        let val = 20;
        //NOTE: The below function should eventually never be used outside of kademlia.rs; below is
        //just a test
        let res : bool = <nodes::Node as nodes::NodeTrait>::store_value(key, val, &mut test_node);
        if res { println!("Successful storage of key-value pair!"); }

        /*TODO 3. Update k buckets*/


        /*TODO What else needs to be tested?*/
        println!("End of the main function!");
    }*/
}

/*async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Always return successfully with a response containing a body with
    // a friendly greeting ;)
    Ok(Response::new(Body::from("hello, world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    // Create a server bound on the provided address
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

    // Wait for the server to complete serving or exit with an error.
    // If an error occurred, print it to stderr.
    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}*/

fn run_test_harness(/*num_nodes: u64, num_keys: u64*/) {
    let num_nodes = 10;
    let num_keys = 10;

    let network = test_harness::Network::new(num_nodes);
    network.send_rpc("0".to_string(), "Hello world!".to_string());
}
