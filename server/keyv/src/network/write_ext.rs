use crate::prelude::Res;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use keyv_core::network_object::NetworkObject;
use keyv_core::serialize::{serialize, SerializeInstruction};

#[async_trait::async_trait]
pub trait WriteInstruction {
    async fn write_instruction<'a, T: NetworkObject + SerializeInstruction<'a> + Send + Sync>(
        &mut self,
        t: T,
    ) -> Res;
}

#[async_trait::async_trait]
impl WriteInstruction for TcpStream {
    async fn write_instruction<'a, T: NetworkObject + SerializeInstruction<'a> + Send + Sync>(
        &mut self,
        t: T,
    ) -> Res {
        let data = serialize(t);
        self.write_all(&data).await?;
        self.flush().await?;
        Ok(())
    }
}
