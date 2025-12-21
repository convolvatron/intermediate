use crate::Oid;

pub enum Address {
    Entity(Oid, Attribute),    
    Offset(u64),
}
