pub const MAX_KEY_SIZE: usize = 64;
pub type Key = [u8; MAX_KEY_SIZE];
pub type Val<'a> = &'a [u8];
