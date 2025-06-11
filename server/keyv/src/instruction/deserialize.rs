pub trait DeserializeInstruction<'a>: Sized + 'a {
    fn des(buffer: &[u8], cursor: &mut usize) -> Option<Self>;
}

impl<'a> DeserializeInstruction<'a> for String {
    fn des(buffer: &[u8], cursor: &mut usize) -> Option<Self> {
        let len = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;
        let slf = String::from_utf8(buffer[*cursor..(*cursor + len)].to_vec()).ok();
        *cursor += len;
        slf
    }
}
