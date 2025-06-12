use crate::_unsafe::to_sized_unchecked;

#[derive(Debug)]
pub struct RawInstruction<'a> {
    pub instr: [u8; 2],
    pub data: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for RawInstruction<'a> {
    type Error = ();
    fn try_from(req: &'a [u8]) -> Result<Self, Self::Error> {
        let Some(inst_id) = req.get(0..2).map(|e| to_sized_unchecked::<_, 2>(e)) else {
            return Err(());
        };
        let Some(body) = req.get(2..) else {
            return Err(());
        };
        Ok(RawInstruction {
            instr: *inst_id,
            data: body,
        })
    }
}