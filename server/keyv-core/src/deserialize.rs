use crate::network_object::NetworkObject;
use crate::raw_instruction::RawInstruction;

pub fn deserialize<'a, T>(raw: RawInstruction<'a>) -> Option<T>
where
    T: DeserializeInstruction<'a, 'a> + NetworkObject,
{
    if raw.instr.ne(&T::ID) {
        return None;
    }
    let mut buff = DeserializeBuffer::<'a>::new(raw.data);
    T::des(&mut buff)
}

pub trait DeserializeInstruction<'a, 'b>: Sized {
    fn des(buffer: &'b mut DeserializeBuffer<'a>) -> Option<Self>;
}

impl<'a, 'b> DeserializeInstruction<'a, 'b> for String {
    fn des(buffer: &'b mut DeserializeBuffer<'a>) -> Option<Self> {
        let len = buffer.read_u32() as usize;
        Some(String::from_utf8_lossy(&buffer.read_n(len)).into())
    }
}

pub struct DeserializeBuffer<'a> {
    _buffer: &'a [u8],
    cursor: usize,
}
impl<'a> DeserializeBuffer<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            _buffer: buffer,
            cursor: 0,
        }
    }

    pub fn read_n(&mut self, n: usize) -> &[u8] {
        let buffer = &self._buffer[self.cursor..(self.cursor + n)];
        self.cursor += n;
        buffer
    }

    pub fn read_n_copied(&mut self, n: usize) -> Vec<u8> {
        let buffer = self._buffer[self.cursor..(self.cursor + n)]
            .iter()
            .copied()
            .collect();
        self.cursor += n;
        buffer
    }

    pub fn read_u32(&mut self) -> u32 {
        let n = u32::from_le_bytes([
            self._buffer[self.cursor + 0],
            self._buffer[self.cursor + 1],
            self._buffer[self.cursor + 2],
            self._buffer[self.cursor + 3],
        ]);
        self.cursor += 4;
        n
    }
}
