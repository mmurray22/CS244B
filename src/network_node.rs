// use std::thread;
// use std::net::{TcpListener, TcpStream, Shutdown};
// use std::io::{Read, Write, Error};

// fn handle_client(mut stream: TcpStream) {
//     let mut data = [0 as u8; 50]; // using 50 byte buffer
//     while match stream.read(&mut data) {
//         Ok(size) => {
//             // echo everything!
//             stream.write(&data[0..size]).unwrap();
//             true
//         },
//         Err(_) => {
//             println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
//             stream.shutdown(Shutdown::Both).unwrap();
//             false
//         }
//     } {}
// }

// fn main() -> Result<(), Error> {
//     let loopback = Ipv4Addr::new(127, 0, 0, 1);
//     let socket = SocketAddrV4::new(loopback, 0);
//     let listener = TcpListener::bind(socket)?;
//     let port = listener.local_addr()?;
//     println!("Listening on {}, access this port to end the program", port);
//     let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
//     println!("Connection received! {:?} is sending data.", addr);
//     let mut input = String::new();
//     let _ = tcp_stream.read_to_string(&mut input)?;
//     println!("{:?} says {}", addr, input);
//     Ok(())
// }

// fn main() {
//     let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
//     // accept connections and process them, spawning a new thread for each one
//     println!("Server listening on port 3333");
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 println!("New connection: {}", stream.peer_addr().unwrap());
//                 thread::spawn(move|| {
//                     // connection succeeded
//                     handle_client(stream)
//                 });
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//                 /* connection failed */
//             }
//         }
//     }
//     // close the socket server
//     drop(listener);
// }


// fn main() {
//     match TcpStream::connect("localhost:3333") {
//         Ok(mut stream) => {
//             println!("Successfully connected to server in port 3333");

//             let msg = b"Hello!";

//             stream.write(msg).unwrap();
//             println!("Sent Hello, awaiting reply...");

//             let mut data = [0 as u8; 6]; // using 6 byte buffer
//             match stream.read_exact(&mut data) {
//                 Ok(_) => {
//                     if &data == msg {
//                         println!("Reply is ok!");
//                     } else {
//                         let text = from_utf8(&data).unwrap();
//                         println!("Unexpected reply: {}", text);
//                     }
//                 },
//                 Err(e) => {
//                     println!("Failed to receive data: {}", e);
//                 }
//             }
//         },
//         Err(e) => {
//             println!("Failed to connect: {}", e);
//         }
//     }
//     println!("Terminated.");
// }