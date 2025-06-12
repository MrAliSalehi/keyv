use crate::deserialize::{DeserializeBuffer, DeserializeInstruction};
use crate::network_object::NetworkObject;
use crate::serialize::SerializeInstruction;

pub struct Init {
    pub master_pwd: String,
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

pub struct InitResult {
    pub auth_key: Vec<u8>,
}
impl NetworkObject for InitResult {
    const ID: [u8; 2] = [177, 177]; //b1b1
}

impl<'a> SerializeInstruction<'a> for InitResult {
    fn ser(&self, buffer: &mut impl Extend<u8>) {
        buffer.extend(InitResult::ID);
        self.auth_key.as_slice().ser(buffer);
    }
}
