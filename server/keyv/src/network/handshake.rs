use crate::configuration::internal::InternalConfig;
use keyv_core::instructions::init::Init;
use crate::network::auth_key::AuthKey;
use crate::network::read_ext::{ReadBuffer, ReadInstruction};
use crate::network::write_ext::WriteInstruction;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;
use keyv_core::deserialize::deserialize;

pub async fn start_handshake(
    mut stream: TcpStream,
    s: SocketAddr,
    buff: &mut ReadBuffer,
    pwd: &str,
    ic: InternalConfig,
) -> Option<TcpStream> {
    let instr = match timeout(Duration::from_secs(10), stream.read_instruction(buff)).await {
        Ok(Ok(Some(instr))) => instr,
        _ => {
            _ = stream.shutdown().await;
            drop(stream);
            return None;
        }
    };
    let Some(init) = deserialize::<Init>(instr) else {
        return None;
    };

    if init.master_pwd.eq(pwd) {
        let auth_key = AuthKey::from_ip(&ic, &s);
        stream
            .write_instruction(auth_key.into_init_result())
            .await
            .ok()?;
        Some(stream)
    } else {
        None
    }
}
