//! Routing table information + functionality

/*use futures::{Future, Stream};

#[path = "./nodes.rs"] mod nodes;

pub fn add_node_entry(main_node: nodes::Node, zip_node: nodes::NodeZip, i: u64) -> bool {
    //1. Check if there is room to add a ZipNode || if oldest of 20 nodes is dead
    let ret = false;
    if (main_node.kbuckets[i].len() < nodes::BUCKET_SIZE ||
        check_node(main_node.kbuckets[&i].back()) == false) {
        ret = true;
        if main_node.kbuckets.contains_key(&i) {
            if let Some(x)  = main_node.kbuckets.get_mut(&i) {
                x.push_back(zip_node);
            }
        } else {
            let mut q = LinkedList::new();
            q.push_back(zip_node);
            main_node.kbuckets.entry(i).or_insert(q);
        }
    }
    ret
}

pub fn check_node(zip_node: nodes::NodeZip) -> bool {
    let addr = zip_node.ip;

    //Pings the node in question to check if it is alive
    let pinger = tokio_ping::Pinger::new();
    let stream = pinger.and_then(move |pinger| Ok(pinger.chain(addr).stream()));
    let future = stream.and_then(|stream| {
        stream.take(3).for_each(|mb_time| {
            let pinged = true;
            match mb_time {
                Some(time) => !pinged,
                None => !pinged,
            };
            pinged
        })
    });

    tokio::run(future.map_err(|err| {
        eprintln!("Error: {}", err);
        false
    }))
}*/
