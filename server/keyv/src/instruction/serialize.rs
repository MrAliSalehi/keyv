use crate::engine::Key;

pub trait SerializeInstruction<'a>: 'a {
    fn ser(&self, buffer: &mut impl Extend<u8>);
}
impl<'a> SerializeInstruction<'a> for Key<'a> {
    fn ser(&self, buffer: &mut impl Extend<u8>) {
        let len: [u8; 4] = (self.len() as u32).to_le_bytes();
        buffer.extend(len);
        buffer.extend(self.iter().copied());
    }
}
