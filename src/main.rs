#[tokio::main]
async fn main() -> io::Result<()> {
    let (client_transport, server_transport) = tarpc::transport::channel::unbounded();
    
    let server = server::new(server::Config::default())
        .incoming(stream::once(future::ready(server_transport)))
        .respond_with(HelloServer.serve());

    tokio::spawn(spawn);

    let mut client = WorldClient::new(client::Config::default(), client_transport).spawn()?;

    let hello = client.hello(context::current(), "Stim".to_string()).await?;

    println!("{}", hello);

    Ok(())

}
