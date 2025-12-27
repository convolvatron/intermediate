use crate::{Attribute, Value, Address, Error, Oid, Variable, DynResolver};
use alloc::{vec::Vec,
            string::String,
            collections::BTreeMap};

#[repr(u8)]
pub enum Status {
    Success = 1,
    Error(String) =2,
}


#[repr(u8)]
pub enum Command  {
    Get(Oid, Attribute, Value, Variable) = 1, 
    Set(Oid, Attribute, Value, Variable) = 2,  
    Copy(Address, Address, u64, Variable) = 3,
    Create(Variable, Status) = 4,  
}

fn unify_variable(working:&mut BTreeMap<u64, Value>, slot:Variable, assertion:Value) -> bool {
    // we are failing to handle an important case where the assertion is also a
    // variable. if it is bound we need to look it up, and if not, then
    // either (a)reorder the evaluation (b) make equivalence classes
    let us = &(slot as u64);
    if let Value::Empty() = working[us] {
        working.insert(*us,assertion);
        true
    } else {
        working[us] == assertion
    }
}


// should this return a Status?
fn unify(working:&mut BTreeMap<u64, Value>, a: Value, b:Value) -> bool {
    if a == b {
        true
    } else {
        if let Value::Variable(v) = a {
            unify_variable(working, v, b)
        } else {
            if let Value::Variable(v) = b {
                unify_variable(working, v, a)
            } else {
                false
            }
        }
    }
}

fn interpret(scope: DynResolver, root:Oid, block:Vec<Command>) -> Result<(), Error> {
    let sets : BTreeMap<Oid, (Attribute, Value)>;
    let working : BTreeMap<u64, Value>;
    for c in block {  
        match c {
            Command::Get(entity, attribute, value, result) => {
                // I want these errors to get merged into status,
                // not be the return value
                let de = scope.resolve(entity)?;
                let v = de.get(attribute)?;
                if unify(&mut working, v, value) {
                    merge_status(Status::Success, result);
                }
            }
            Command::Set(entitry, attribute, value, result) => {
            }
            Command::Copy(source, dest, length, result) => {
            }
            Command::Create(new, result) => {
            }
        }
    }
    for (o, s) in sets {
    }
    Ok(())
}


