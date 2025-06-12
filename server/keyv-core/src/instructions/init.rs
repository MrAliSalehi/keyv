use crate::deserialize::{DeserializeBuffer, DeserializeInstruction};
use crate::instructions::Instruction;
use crate::network_object::NetworkObject;
use crate::serialize::SerializeInstruction;

pub struct Init {
    pub master_pwd: String,
}

pub struct InitResult {
    pub success: bool,
}

impl NetworkObject for Init {
    const ID: [u8; 2] = [161, 161]; //a1a1
}

impl DeserializeInstruction for Init {
    fn des(buffer: &mut DeserializeBuffer) -> Option<Self> {
        let pwd = <String>::des(buffer)?;
        Some(Self { master_pwd: pwd })
    }
}

impl<'a> SerializeInstruction<'a> for Init {
    fn ser(&self, buffer: &mut impl Extend<u8>) {
        let p_b = self.master_pwd.as_bytes();
        buffer.extend((p_b.len() as u32).to_le_bytes());
        buffer.extend(p_b.iter().copied());
    }
}

impl NetworkObject for InitResult {
    const ID: [u8; 2] = [177, 177]; //b1b1
}

impl<'a> SerializeInstruction<'a> for InitResult {
    fn ser(&self, buffer: &mut impl Extend<u8>) {
        buffer.extend(InitResult::ID);
        buffer.extend(if self.success { [1] } else { [0] });
    }
}

impl DeserializeInstruction for InitResult {
    fn des(buffer: &mut DeserializeBuffer) -> Option<Self> {
        buffer.read_n(1).get(0).map(|e| Self { success: *e == 1 })
    }
}

impl<'a> Instruction<'a> for Init {
    type Output = InitResult;
}
