use crate::configuration::config::Config;
use crate::data::aol::Aol;
use crate::engine::Engine;
use crate::network::read_ext::ReadBuffer;
use crate::prelude::{Res, init_logger};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;
use tracing::{info, trace, warn};

mod configuration;
mod data;
mod engine;
pub mod instruction;
pub mod network;
mod prelude;

#[tokio::main]
async fn main() -> Res {
    let config = Config::load_or_create()
        .await
        .inspect_err(|e| println!("{e:?}"))?;

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
        info!("accepted {addr:?}");
        let pwd = config.connection.master_pwd.clone();
        let _i = config._internal.clone();
        tokio::spawn(async move {
            let mut buffer = ReadBuffer::with_size(config.connection.max_incoming_bytes);
            let Some(mut stream) =
                network::handshake::start_handshake(stream, addr, &mut buffer, &pwd, _i).await
            else {
                warn!("invalid handshake");
                return;
            };

            trace!("handshake completed: {addr:?}");
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let mut s = vec![0u8; 100];
                let n = stream.read(&mut s).await.unwrap();
                if n == 0 {
                    continue;
                }
                info!("recv {s:?}");
            }
        });
    }

    Ok(())
}
