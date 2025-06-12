use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::network_object::NetworkObject;
use crate::serialize::{serialize, SerializeInstruction};

#[async_trait::async_trait]
pub trait WriteInstruction {
    async fn write_instruction<'a, T: NetworkObject + SerializeInstruction<'a> + Send + Sync>(
        &mut self,
        t: T,
    ) -> eyre::Result<()>;
}

#[async_trait::async_trait]
impl WriteInstruction for TcpStream {
    async fn write_instruction<'a, T: NetworkObject + SerializeInstruction<'a> + Send + Sync>(
        &mut self,
        t: T,
    ) -> eyre::Result<()> {
        let data = serialize(t);
        self.write_all(&data).await?;
        self.flush().await?;
        Ok(())
    }
}
