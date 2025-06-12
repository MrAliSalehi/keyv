use crate::network_object::NetworkObject;

pub struct Set;

impl NetworkObject for Set {
    const ID: [u8; 2] = [170, 170];
}
