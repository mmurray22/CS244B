/*For initial testing communication over threads*/
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

#[path = "./nodes.rs"] mod nodes;

pub fn test_two_nodes_one_client() {
    //Since this doesn't use a network, ip & port will be defaults
    let ip1 = String::from("0.0.0.0");
    let ip2 = String::from("1.1.1.1");
    let port : u64 = 0;
    let mut test_node_one : Box<nodes::Node> =  <nodes::Node as nodes::NodeTrait>::new(ip1, 0);
    let mut test_node_two : Box<nodes::Node> =  <nodes::Node as nodes::NodeTrait>::new(ip2, 0);
    
    //Create the communication channel between client and node1
    let (tx1_cli, rx1_node)  = mpsc::channel();
    let (tx1_node, rx1_cli)  = mpsc::channel();
    //Three threads ("main" current thread for client, 2 spawn threads for nodes)
    let thread_one = thread::spawn(move || {
        let recv = rx1_node.recv().unwrap();
        println!("Spawn thread 1 got value {}!", recv);
        let reply = String::from("Hi back!");
        tx1_node.send(reply).unwrap();
    });
    
    //Create the communication channel between client and node2
    let (tx2_cli, rx2_node) = mpsc::channel();
    let (tx2_node, rx2_cli) = mpsc::channel();
    let thread_two = thread::spawn(move || {
        let recv = rx2_node.recv().unwrap();
        println!("Spawn thread 2 got value {}!", recv);
        let reply = String::from("Hello back!");
        tx2_node.send(reply).unwrap();
    });
    
    let val1 = String::from("Hi");
    let val2 = String::from("Hello");
    //Send val1 and val2 to node1 and node2 respectively
    println!("Main thread sent {} to spawn thread 1!", val1);
    tx1_cli.send(val1).unwrap();
	println!("Main thread sent {} to spawn thread 2!", val2);
	tx2_cli.send(val2).unwrap();

    //Receive the replies from node1 and node2
    let rec1 = rx1_cli.recv().unwrap();
    println!("Main thread received {} from spawn thread 1!", rec1);
    let rec2 = rx2_cli.recv().unwrap();
    println!("Main thread received {} from spawn thread 2!", rec2);
    thread_one.join().expect("thread one panicked!");
    thread_two.join().expect("thread two panicked!");
}
