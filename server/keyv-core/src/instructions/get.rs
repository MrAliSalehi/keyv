use crate::network_object::NetworkObject;

pub struct Get;

impl NetworkObject for Get {
    const ID: [u8; 2] = [193, 193];//c1c1
}
