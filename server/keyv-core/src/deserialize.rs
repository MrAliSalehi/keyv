use crate::network_object::NetworkObject;
use crate::raw_instruction::RawInstruction;

pub fn deserialize<'a, T: DeserializeInstruction + NetworkObject + 'a>(
    raw: RawInstruction<'a>,
) -> Option<T> {
    if raw.instr.ne(&T::ID) {
        return None;
    }
    T::des(&mut DeserializeBuffer::<'a>::new(raw.data))
}

pub trait DeserializeInstruction: Sized {
    fn des(buffer: &mut DeserializeBuffer) -> Option<Self>;
}

impl DeserializeInstruction for String {
    fn des(buffer: &mut DeserializeBuffer) -> Option<Self> {
        let len = buffer.read_u32() as usize;
        Some(String::from_utf8_lossy(&buffer.read_n(len)).into())
    }
}

pub struct DeserializeBuffer<'a> {
    _buffer: &'a [u8],
    cursor: usize,
}
impl<'a> DeserializeBuffer<'a> {
    pub fn new(buffer: &'a [u8]) -> Self{
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
