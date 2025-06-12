use crate::deserialize::deserialize;
use crate::instructions::Instruction;
use crate::read_ext::{ReadBuffer, ReadInstruction};
use crate::write_ext::WriteInstruction;
use eyre::{OptionExt, eyre};
use tokio::net::TcpStream;

mod _unsafe;
pub mod deserialize;
pub mod engine_specifications;
pub mod instructions;
pub mod network_object;
pub mod raw_instruction;
pub mod read_ext;
pub mod serialize;
pub mod write_ext;

#[async_trait::async_trait]
pub trait NwInvoke {
    async fn invoke_instruction<'a, T: Instruction<'a>>(&mut self, t: T)
    -> eyre::Result<T::Output>;
}

#[async_trait::async_trait]
impl NwInvoke for TcpStream {
    async fn invoke_instruction<'a, T: Instruction<'a>>(
        &mut self,
        t: T,
    ) -> eyre::Result<T::Output> {
        self.write_instruction(t).await?;

        let mut b = ReadBuffer::with_size(100);
        let r = self
            .read_instruction(&mut b)
            .await?
            .ok_or_eyre("invalid result")?;
        deserialize::<T::Output>(r).ok_or_eyre("deserialization failed")
    }
}
