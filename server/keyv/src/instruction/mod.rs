pub mod deserialize;
pub mod serialize;
pub mod get;
pub mod init;
pub mod set;

pub struct RawInstruction<'a> {
    pub instr: [u8; 2],
    pub data: &'a [u8],
}

pub trait NetworkObject {
    const ID: [u8; 2];
}

