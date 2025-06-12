use crate::prelude::Res;
use keyv_core::deserialize::deserialize;
use keyv_core::instructions::init::{Init, InstrResult};
use keyv_core::read_ext::{ReadBuffer, ReadInstruction};
use keyv_core::write_ext::WriteInstruction;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn start_handshake(stream: &mut TcpStream, buff: &mut ReadBuffer, pwd: &str) -> bool {
    let instr = match timeout(Duration::from_secs(10), stream.read_instruction(buff)).await {
        Ok(Ok(Some(instr))) => instr,
        _ => {
            _ = stream.shutdown().await;
            return false;
        }
    };
    let Some(init) = deserialize::<Init>(instr) else {
        return false;
    };

    let success = init.master_pwd.eq(pwd);
    let r = stream.write_instruction(InstrResult { success }).await;

    r.is_ok() && success
}
