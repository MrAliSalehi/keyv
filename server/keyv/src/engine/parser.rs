use crate::instruction::deserialize::DeserializeInstruction;
use crate::instruction::serialize::SerializeInstruction;
use crate::instruction::{NetworkObject, RawInstruction};

pub fn serialize_resp<'a, T: NetworkObject + SerializeInstruction<'a>>(t: T) -> Vec<u8> {
    let mut buff = vec![];
    buff.copy_from_slice(&T::ID);
    t.ser(&mut buff);
    buff
}

pub fn parse_bytes(req: &[u8]) -> Option<RawInstruction> {
    let Some(inst_id) = req.get(0..4).map(|e| slice_to_array_unchecked::<_, 2>(e)) else {
        return None;
    };
    let Some(body) = req.get(4..) else {
        return None;
    };
    Some(RawInstruction {
        instr: *inst_id,
        data: body,
    })
}

pub fn parse_instruction<'a, T: DeserializeInstruction<'a> + NetworkObject + 'a>(
    mut raw: RawInstruction,
) -> Option<T> {
    if raw.instr.ne(&T::ID) {
        return None;
    }
    let mut cursor = 0;
    T::des(&mut raw.instr, &mut cursor)
}

fn slice_to_array_unchecked<T, const N: usize>(slice: &[T]) -> &[T; N] {
    assert_eq!(slice.len(), N);
    unsafe { &*slice.as_ptr().cast() }
}
