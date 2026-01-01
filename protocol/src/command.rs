use crate::{Address, Attribute, Buffer, ChangeSet, DynResolver, Error, Oid, Value, Variable, Encodable};
use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

#[repr(u8)]
pub enum Status {
    Success = 1,
    Error(String) = 2,
}

// too bad lambdas are so screwy, but this can maybe be a FnOnce with some trouble
type DynThunk = Arc<dyn Thunk>;
trait Thunk {
    fn apply(&self) -> Result<(), Error>;
}

struct EmptyFinalizer {}
impl Thunk for EmptyFinalizer {
    fn apply(&self) -> Result<(), Error> {
        Ok(())
    }
}

type DynTranslator = Arc<dyn Translator>;
trait Translator {
    // return new address, and some post block actions if necessary,
    // which I would assume needs to know the direciton of data motion?
    // this is always the target?
    fn translate(&self, a: Address) -> Result<(DynThunk, Address), Error>;
}

#[repr(u8)]
pub enum Command {
    Get(Oid, Attribute, Value, Variable) = 1,
    Set(Oid, Attribute, Value, Variable) = 2,
    Copy(Address, Address, usize, Variable) = 3,
    Create(Variable, Variable) = 4,
}

// xxx - discriminant issues resulting in an unforuntate duplication that
// requires manual consistency
impl Command {
    
    pub fn get(b: &mut Buffer, e: Oid, a: Attribute, v: Value, r: Variable) {
        b.write(&[1]);
        e.encode(b);
        a.encode(b);
        v.encode(b);
        r.encode(b);
    }

    pub fn set(b: &mut Buffer, e: Oid, a: Attribute, v: Value, r: Variable) {
        b.write(&[2]);
        e.encode(b);
        a.encode(b);
        v.encode(b);
        r.encode(b);
    }

    pub fn copy(b: &mut Buffer, source: Address, dest: Address, length: usize, r: Variable) {
        b.write(&[3]);
        source.encode(b);
        dest.encode(b);
        b.write(&length.to_be_bytes());
        r.encode(b);
    }

    pub fn create(b: &mut Buffer, _r: Value) {
        b.write(&[4]);
    }
}

fn unify_variable(working: &mut BTreeMap<u64, Value>, slot: Variable, assertion: Value) -> bool {
    // we are failing to handle an important case where the assertion is also a
    // variable. if it is bound we need to look it up, and if not, then
    // either (a)reorder the evaluation (b) make equivalence classes
    let us = &(slot as u64);
    if let Value::Empty() = working[us] {
        working.insert(*us, assertion);
        true
    } else {
        working[us] == assertion
    }
}

// should this return a Status?
fn unify(working: &mut BTreeMap<u64, Value>, a: Value, b: Value) -> bool {
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

// this is a .. maybe temporary gross thing to get us running, see
// the design doc. this should take a result?
fn merge_status(_d: Variable, _s: Status) {}

// no variables
fn get(scope: DynResolver, entity: Oid, attribute: Attribute) -> Result<Value, Error> {
    let de = scope.resolve(entity)?;
    de.get(attribute)
}

//fn copy(r:DynResolver, entity:Oid, attribute:Attribute) -> Result<Value, Error> {
//    let de = scope.resolve(entity)?;
//    de.get(attribute)?;
//}

// we need a per-channel address translator (i.e. this came from the proxy kernel
fn interpret(
    scope: DynResolver,
    _t: DynTranslator,
    _root: Oid,
    block: Vec<Command>,
) -> Result<(), Error> {
    let mut sets: BTreeMap<Oid, ChangeSet> = BTreeMap::new();
    let mut working = BTreeMap::new();
    for c in block {
        match c {
            Command::Get(entity, attribute, value, result) => {
                let de = scope.resolve(entity)?;
                let v = de.get(attribute)?;
                if unify(&mut working, v, value) {
                    merge_status(result, Status::Success);
                }
            }

            Command::Set(entity, attribute, value, result) => {
                let de = scope.resolve(entity)?;
                let v = de.get(attribute.clone())?;
                if unify(&mut working, v.clone(), value) {
                    merge_status(result, Status::Success);
                }
                sets.entry(entity)
                    .or_insert(Vec::new())
                    .push((attribute.clone(), v.clone()));
            }

            Command::Copy(_source, _dest, _length, _result) => {}

            Command::Create(new, _result) => {
                unify_variable(&mut working, new, Value::Oid(Oid(7777)));
            }
        }
    }
    for (e, s) in sets {
        // we already resolved this, but i dont think we can used a DynEntity as
        // a btree key(?)
        let de = scope.resolve(e)?;
        de.set(s)?;
    }
    Ok(())
}
