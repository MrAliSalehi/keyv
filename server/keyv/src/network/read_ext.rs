use crate::engine::parser;
use crate::instruction::RawInstruction;
use eyre::eyre;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub trait ReadInstruction {
    async fn read_instruction<'a>(
        &'a mut self,
        buffer: &'a mut [u8],
    ) -> eyre::Result<Option<RawInstruction<'a>>>;
}
impl ReadInstruction for TcpStream {
    async fn read_instruction<'a>(
        &mut self,
        buffer: &'a mut [u8],
    ) -> eyre::Result<Option<RawInstruction<'a>>> {
        let n = self.read(buffer).await?;

        if n == 0 {
            return Err(eyre!("read zero"));
        }

        Ok(parser::parse_bytes(buffer))
    }
}
