use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::TcpStream;

mod handshake;
pub struct Keyv {}

impl Keyv {
    pub async fn connect(
        addr: impl Into<SocketAddr>,
        pwd: impl Into<String>,
    ) -> eyre::Result<Self> {
        let mut s = TcpStream::connect(addr.into()).await?;
        handshake::handshake(&mut s, &pwd.into()).await?;
        Ok(Self {})
    }
}
