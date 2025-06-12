#[cfg(test)]
mod tests {
    use keyv_client::Keyv;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    #[tokio::test]
    async fn connect_and_handshake() {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 1032));
        let client = Keyv::connect(addr, "toor").await;
        assert!(client.is_ok());
    }
}