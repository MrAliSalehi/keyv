use crate::raw_instruction::RawInstruction;
use eyre::eyre;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[async_trait::async_trait]
pub trait ReadInstruction {
    async fn read_instruction(
        &mut self,
        buffer: &mut ReadBuffer,
    ) -> eyre::Result<Option<RawInstruction>>;
}

#[async_trait::async_trait]
impl ReadInstruction for TcpStream {
    async fn read_instruction(
        &mut self,
        buffer: &mut ReadBuffer,
    ) -> eyre::Result<Option<RawInstruction>> {
        let n = self.read(buffer.get_mut()).await?;
        if n == 0 {
            return Err(eyre!("read zero"));
        }
        let b = buffer.read_flush(n);
        Ok(RawInstruction::try_from(b).ok())
    }
}

pub struct ReadBuffer {
    pub _buffer: Vec<u8>,
    head: usize,
}

impl ReadBuffer {
    pub fn with_size(size: usize) -> Self {
        Self {
            _buffer: vec![0u8; size],
            head: 0,
        }
    }
    pub fn get_mut(&mut self) -> &mut [u8] {
        self._buffer[self.head..].as_mut()
    }

    pub fn read_flush(&mut self, n: usize) -> &[u8] {
        self.head += n;
        let buffer = &self._buffer[..self.head];
        self.head = 0;
        buffer
    }
}
