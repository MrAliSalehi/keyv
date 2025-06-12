use crate::deserialize::{DeserializeBuffer, DeserializeInstruction};
use crate::network_object::NetworkObject;
use crate::serialize::SerializeInstruction;

pub struct InstrResult {
    pub success: bool,
}

impl NetworkObject for InstrResult {
    const ID: [u8; 2] = [177, 177]; //b1b1
}

impl<'a> SerializeInstruction<'a> for InstrResult {
    fn ser(&self, buffer: &mut impl Extend<u8>) {
        buffer.extend(InstrResult::ID);
        buffer.extend(if self.success { [1] } else { [0] });
    }
}

impl<'a> DeserializeInstruction<'a> for InstrResult {
    fn des(buffer: &mut DeserializeBuffer) -> Option<Self> {
        buffer.read_n(1).get(0).map(|e| Self { success: *e == 1 })
    }
}
