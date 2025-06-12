use crate::deserialize::DeserializeInstruction;
use crate::network_object::NetworkObject;
use crate::serialize::SerializeInstruction;

pub mod get;
pub mod init;
pub mod instr_result;
pub mod set;

pub trait Instruction<'a, 'b>: SerializeInstruction<'a> + NetworkObject + Send + Sync {
    type Output: DeserializeInstruction<'a, 'b> + NetworkObject;
}
