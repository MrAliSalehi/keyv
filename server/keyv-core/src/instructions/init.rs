use crate::deserialize::{DeserializeBuffer, DeserializeInstruction};
use crate::instructions::Instruction;
use crate::instructions::instr_result::InstrResult;
use crate::network_object::NetworkObject;
use crate::serialize::SerializeInstruction;

pub struct Init {
    pub master_pwd: String,
}

impl NetworkObject for Init {
    const ID: [u8; 2] = [161, 161]; //a1a1
}

impl<'a, 'b> DeserializeInstruction<'a, 'b> for Init {
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

impl<'a, 'b> Instruction<'a, 'b> for Init {
    type Output = InstrResult;
}
