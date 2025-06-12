use crate::configuration::ConfigRef;
use crate::network;
use keyv_core::read_ext::{ReadBuffer, ReadInstruction};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::{info, trace};

pub struct TcpHandler {
    config: ConfigRef,
    stream: TcpStream,
    addr: SocketAddr,
    read_buff: ReadBuffer,
}

impl TcpHandler {
    pub async fn new(config: ConfigRef, stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            read_buff: ReadBuffer::with_size(config.connection.max_incoming_bytes),
            config,
            stream,
            addr,
        }
    }
    pub async fn connect_with_handshake(mut self) {
        tokio::spawn(async move {
            let success = network::handshake::start_handshake(
                &mut self.stream,
                &mut self.read_buff,
                &self.config.connection.master_pwd,
            )
            .await;
            if !success {
                trace!("invalid handshake: {:?}", self.addr);
                return Ok(());
            };

            trace!("handshake completed: {:?}", self.addr);

            self.event_loop().await;
            eyre::Result::<()>::Ok(())
        });
    }
    async fn event_loop(mut self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let Ok(instr) = self.stream.read_instruction(&mut self.read_buff).await else {
                trace!("client disconnected {:?}", self.addr);
                break;
            };
            let Some(instr) = instr else { continue };

            trace!("received instr: {:?}", instr.instr);
        }
    }
}
