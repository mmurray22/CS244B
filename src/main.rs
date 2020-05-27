extern crate crypto;
extern crate rand;
extern crate queue;

use std::env;

pub mod routing;
pub mod nodes;
pub mod test_harness;
pub mod rpc;
//pub mod kademlia;
pub mod rpc_test_harness;

fn main () -> () {
    let args: Vec<String> = env::args().collect();

    if args[1] == "test" {
        run_test_harness();
    } else {
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
    }
}


fn run_test_harness() {
    let num_nodes = 10;
    let num_keys = 10;

    let network = test_harness::Network::new(num_nodes);
    network.send_rpc("0".to_string(), "Hello world!".to_string());
}


/*async fn main_test() -> io::Result<()> {
    let (client_transport, server_transport) = tarpc::transport::channel::unbounded();
    
    let server = server::new(server::Config::default())
        .incoming(stream::once(future::ready(server_transport)))
        .respond_with(HelloServer.serve());

    tokio::spawn(spawn);

    let mut client = WorldClient::new(client::Config::default(), client_transport).spawn()?;

    let hello = client.hello(context::current(), "Stim".to_string()).await?;

    println!("{}", hello);

    Ok(())
}*/
