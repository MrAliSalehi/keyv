use crate::engine::parser;
use crate::instruction::NetworkObject;
use crate::instruction::serialize::SerializeInstruction;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub trait WriteInstruction {
    async fn write_instruction<'a, T: NetworkObject + SerializeInstruction<'a>>(&mut self, t: T);
}
impl WriteInstruction for TcpStream {
    async fn write_instruction<'a, T: NetworkObject + SerializeInstruction<'a>>(&mut self, t: T) {
        let data = parser::serialize_resp(t);
        _ = self.write_all(&data).await;
        _ = self.flush().await;
    }
}
