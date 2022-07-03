use std::{io, sync::Arc};

use ricq::{
    device::Device,
    handler::DefaultHandler,
    version::{get_version, Protocol},
    Client,
};
use tokio::net::TcpStream;

pub mod account_pwd; 
mod qr_code;

pub struct UnLoginClient {
    inner: Arc<Client>,
}

impl UnLoginClient {
    pub async fn new(device: Device, protocol: Protocol) -> io::Result<Self> {
        let client = Arc::new(Client::new(device, get_version(protocol), DefaultHandler));
        // Connect to server
        let stream = TcpStream::connect(client.get_address()).await?;
        let client_cloned = client.clone();
        tokio::spawn(async move { client_cloned.start(stream).await });

        Ok(Self { inner: client })
    }
}
