use crate::_unsafe::to_sized_unchecked;

#[derive(Debug)]
pub struct RawInstruction {
    pub instr: [u8; 2],
    pub data: Vec<u8>,
}

impl TryFrom<&[u8]> for RawInstruction {
    type Error = ();
    fn try_from(req: &[u8]) -> Result<Self, Self::Error> {
        let Some(inst_id) = req.get(0..2).map(|e| to_sized_unchecked::<_, 2>(e)) else {
            return Err(());
        };
        let Some(body) = req.get(2..) else {
            return Err(());
        };
        Ok(RawInstruction {
            instr: *inst_id,
            data: body.to_vec(),
        })
    }
}
