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

#[cfg(test)]
mod tests {
    use crate::Keyv;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    #[tokio::test]
    async fn connect_and_handshake() {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 1032));
        let client = Keyv::connect(addr, "toor").await;
        assert!(client.is_ok());
    }
}
