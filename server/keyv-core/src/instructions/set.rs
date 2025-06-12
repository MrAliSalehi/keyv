use crate::_unsafe;
use crate::deserialize::{DeserializeBuffer, DeserializeInstruction};
use crate::engine_specifications::{Key, MAX_KEY_SIZE};
use crate::instructions::Instruction;
use crate::instructions::instr_result::InstrResult;
use crate::network_object::NetworkObject;
use crate::serialize::SerializeInstruction;

pub struct Set {
    key: Key,
    value: Vec<u8>,
}

impl NetworkObject for Set {
    const ID: [u8; 2] = [170, 170];
}

impl<'a> DeserializeInstruction<'a> for Set {
    fn des(buffer: &mut DeserializeBuffer) -> Option<Self> {
        let len1 = buffer.read_u32() as usize;

        let key = buffer.read_n_copied(len1);
        if key.len() > MAX_KEY_SIZE {
            return None;
        }

        let key = if key.len() == MAX_KEY_SIZE {
            *_unsafe::to_sized_unchecked::<_, MAX_KEY_SIZE>(&key)
        } else {
            let mut tmp = [0u8; MAX_KEY_SIZE];
            tmp.copy_from_slice(&key);
            tmp
        };

        let len2 = buffer.read_u32() as usize;

        let value = buffer.read_n_copied(len2);

        Some(Set { key, value })
    }
}

impl<'a> SerializeInstruction<'a> for Set {
    fn ser(&self, buffer: &mut impl Extend<u8>) {
        let len: [u8; 4] = (self.key.len() as u32).to_le_bytes();
        buffer.extend(len);
        buffer.extend(self.key);

        let len: [u8; 4] = (self.value.len() as u32).to_le_bytes();
        buffer.extend(len);
        buffer.extend(self.value.iter().copied());
    }
}

impl<'a> Instruction<'a> for Set {
    type Output = InstrResult;
}
