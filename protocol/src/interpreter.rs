use alloc::{collections::BTreeMap, boxed::Box, sync::Arc, vec::Vec};
use async_trait::async_trait;
use crate::{Attribute,
            Bindings,
            DynAllocator,            
            DynEntityHandler,
            DynResolver,
            DynStream,
            Entity,
            Error,
            Oid,
            Stream,
            Value,
            locerr,
            read_stream_with_err};



// scope is the global context for a node in a network of protocols
#[derive(Clone)]
pub struct Scope {
    pub myself: Oid,
    pub allocator: DynAllocator,
    pub resolver: DynResolver,
    pub write_commands: Arc<Vec<SetHandler>>,
}

impl Scope {
    fn resolve(&self, v:Value) -> Result<DynEntityHandler, Error> {
        if let Value::Oid(oid)  = v {
            if let Some(e) = self.resolver.resolve(oid) {
                Ok(e)
            } else {
                // this should have a routing and fork/join function
                Err(locerr!(self.myself, "unknown object {:?}", oid))
            }
        } else {
            Err(locerr!(self.myself, "non-oid in entity position {:?}", v))
        }

    }
}

// this is currently single threaded, which should be fine for short programs.
// otherwise it would require the runtime from linux_proxy
 
// a resover maps an Oid to an ip address (?)
// a translator maps an address from one space into another



struct EvalRoot {
    first: bool,
} 

#[async_trait]
impl Stream<Bindings> for EvalRoot {
    async fn next(&mut self) -> Result<Option<Bindings>, Error> {
        if self.first  {
            self.first = false;
            Ok(Some(Bindings{b:BTreeMap::new()}))
        } else {
            // this shouldn't happen more than once
            Ok(None)
        }
    }    
}

struct NewHandler {
    prev: DynStream<Bindings>,
    scope: Scope,
    slot: Value,
}

#[async_trait]
impl Stream<Bindings> for NewHandler {
    async fn next(&mut self) -> Result<Option<Bindings>, Error> {
        read_stream_with_err!(self.prev, bindings, {        
            let oid = self.scope.allocator.new();
            // assumed that the scheduler is correct
            bindings.assert(self.slot, Value::Oid(oid));
            Ok(Some(bindings))
        })
    }
}

struct SetHandler {
    e: Entity,
    a: Attribute,
    v: Value,
}

struct GetHandler {
    prev: DynStream<Bindings>,
    scope: Scope,
    entity: Entity,
    attribute: Attribute,
    out: Value,
}

#[async_trait]
impl Stream<Bindings> for GetHandler {
    async fn next(&mut self) -> Result<Option<Bindings>, Error> {
        read_stream_with_err!(self.prev, bindings, {
            // we assume the scheduler has done its job, so these must succeed
            let e = self.scope.resolve(bindings.get(self.entity).unwrap())?;
            let a = bindings.get(self.attribute).unwrap();
            
            if let Some(v) = e.get(a)? {
                bindings.assert(self.out, v);
                Ok(Some(bindings))
            } else {
                Ok(None)
            }
        })
    }
}

struct GetKeysHandler {
    scope: Scope,
    prev: DynStream<Bindings>,
    keys: Option<(DynStream<Attribute>, Bindings)>,    
    entity: Value,    
    dest: Value, 
}

#[async_trait]
impl Stream<Bindings> for GetKeysHandler {
    async fn next(&mut self) -> Result<Option<Bindings>, Error> {
        if let Some((existing, bindings)) = self.keys &&
            let Some(k) = existing.next().await? {
                bindings.assert(self.dest, k);
                return Ok(Some(bindings))
           }
        read_stream_with_err!(self.prev, bindings, {
            let e = self.scope.resolve(bindings.get(self.entity).unwrap())?;
            self.keys = Some((e.keys(), bindings));
            self.next().await
        })
    }
}

struct CopyHandler {
    prev: DynStream<Bindings>,
    scope: Scope,
    se: Entity,
    sa: Attribute, soffset:Value,
    de: Entity,
    da: Attribute, doffset:Value,
    length:Value,
}

#[async_trait]
impl Stream<Bindings> for CopyHandler {
    async fn next(&mut self) -> Result<Option<Bindings>, Error> {
        read_stream_with_err!(self.prev, bindings, {
            Ok(Some(bindings))
        })
    }
}

impl Scope {
    
    // the fact that I can't use enum cases as subtypes is pretty annoying
    fn build_get(&self, entity: Entity, attribute: Attribute, out:Value, prev: DynStream<Bindings>) -> Result<DynStream<Bindings>, Error> {
        Ok(Arc::new(GetHandler{prev, scope:self.clone(), entity, attribute, out}))
    }
    
    fn build_new(&self, out: Value, prev: DynStream<Bindings>) -> Result<DynStream<Bindings>, Error> {
        if let Value::Variable(_) = out {
            Ok(Arc::new(NewHandler{prev, scope:self.clone(), slot:out}))
        } else {
            // it would be nice if this included source information, wouldn't it?
            Err(locerr!(self.myself, "new argument must be variable"))
        }
        
    }
    
    fn build_copy(&self,
                  se: Entity, sa: Attribute, soffset:Value,
                  de: Entity, da: Attribute, doffset:Value,
                  length: Value,
                  prev: DynStream<Bindings>) -> Result<DynStream<Bindings>, Error> {
        // validate length, entity, attribute
        Ok(Arc::new(CopyHandler{se, sa, soffset, de, da, doffset, length, prev, scope:self.clone()}))
    }
}

