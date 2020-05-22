extern crate crypto;
extern crate rand;
extern crate queue;

pub mod nodes;
pub mod kademlia;
//mod routing;

use nodes::NodeTrait;

fn main () -> () {
    println!("Start of the main function!");
    /*TODO: 1. Make Node with an ID*/
    let ip = String::from("0.0.0.0");
    let test_node : Box<nodes::Node> =  <nodes::Node as nodes::NodeTrait>::new(ip, 0);
    /**/
    /**/
    /**/
    /**/
    println!("End of the main function!");
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
