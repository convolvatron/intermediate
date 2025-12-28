use crate::{Attribute, Buffer, Encodable, Error, Oid, err};
use alloc::boxed::Box;

#[repr(u8)]
pub enum Address {
    Entity(Oid, Attribute) = 1,
    Offset(Box<Address>, u64) = 2,
}

// ok, i didn't realize that Discriminants were kind of a non-orthogonal property of
// enums, so we're just going to replicate the bindings in this match, but its not
// great
impl Encodable for Address {
    fn encode(&self, _dest: &mut Buffer) {}

    fn decode(source: &mut Buffer) -> Result<Self, Error> {
        match source.read(1)?[0] {
            1 => Ok(Address::Entity(
                Encodable::decode(source)?,
                Encodable::decode(source)?,
            )),
            2 => Ok(Address::Offset(
                Box::new(Encodable::decode(source)?),
                u64::from_be_bytes(source.read(8)?.try_into().expect("cant")),
            )),
            x => Err(err!("invalid address {}", x)),
        }
    }
}
