use crate::configuration::ConfigRef;
use crate::configuration::config::Config;
use crate::data::aol::Aol;
use crate::engine::Engine;
use crate::network::socket_listener::TcpHandler;
use crate::prelude::{Res, init_logger};
use keyv_core::read_ext::ReadBuffer;
use std::net::{SocketAddrV4, TcpListener};
use std::str::FromStr;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tracing::{info, trace, warn};

mod configuration;
mod data;
mod engine;
pub mod network;
mod prelude;

#[tokio::main]
async fn main() -> Res {
    let config = Config::load_or_create()
        .await
        .inspect_err(|e| println!("{e:?}"))?;
    let config = ConfigRef::new(config);

    init_logger(&config.log_level);

    trace!("loaded {config:#?}");

    let engine = Engine::new();

    let aol = Aol::new(&config.storage.data_path).await?;
    aol.load_commits(&config.storage.data_path).await?;

    let s_v4 = SocketAddrV4::from_str(&format!(
        "{}:{}",
        config.connection.listen_addr, config.connection.port
    ))?;

    let listener = tokio::net::TcpListener::bind(s_v4).await?;
    loop {
        let Ok((stream, addr)) = listener.accept().await else {
            continue;
        };
        let handler = TcpHandler::new(config.clone(), stream, addr).await;
        handler.connect_with_handshake().await;
    }

    Ok(())
}
