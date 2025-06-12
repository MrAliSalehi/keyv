use crate::network_object::NetworkObject;

/// wrapper function for `T::ser`
pub fn serialize<'a, T: NetworkObject + SerializeInstruction<'a>>(t: T) -> Vec<u8> {
    let mut buff = vec![0u8; 2];
    buff.copy_from_slice(&T::ID);
    t.ser(&mut buff);
    buff
}

pub trait SerializeInstruction<'a>: 'a {
    fn ser(&self, buffer: &mut impl Extend<u8>);
}
impl<'a> SerializeInstruction<'a> for &'a [u8] {
    fn ser(&self, buffer: &mut impl Extend<u8>) {
        let len: [u8; 4] = (self.len() as u32).to_le_bytes();
        buffer.extend(len);
        buffer.extend(self.iter().copied());
    }
}
