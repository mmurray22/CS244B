# CS244B
CS 244B project

![](https://github.com/mmurray22/CS244B/workflows/Rust/badge.svg?event=push)

Objective: Recreate Kademlia DHT in Rust

## Running the testing harness and our rust implementation:
 After cloning, run `cargo run` from terminal. This will start up 100 nodes which will each add themselves to the network correctly. At this point you should be able to input commands to the testing harness. Each node is identified by its "IP" string. Nodes with "IP" 0 through 99 will be created.

#### Testing Harness Commands:
  `cstore`: This command sends a client store request of a key value pair to a particular node. Usage: `cstore 1 2 3` sends a client store request of key, value pair (2,3) to node 1.
  
  `cget`: This command sends a client get request for a particular key to a particular node. Usage: `cget 1 2` sends a client get request for key 2 to node 1. An indication of a successful lookup is a terminal output of at least one statement that starts with "Value: " and then is followed by some descriptive information about where the value was found.
  
  `add`: This command adds a new node to the network. Its "IP" string will be initialized to a counter value based on the number of nodes already in the system. So if nodes 0 through 99 are already created, the first `add` command creates node 100.
  
  `remove`: This command removes a node from the network. Usage: `remove 3` will remove node with "IP" string 3 from the network.
