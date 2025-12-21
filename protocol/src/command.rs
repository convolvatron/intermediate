
// error handling
pub enum Operation  {
    Get = 1,  //ear
    Set = 2,  //eavr  
    Copy = 3, //sdlr
    Create = 4,  //r  
}

fn interpret(root:Arc<dyn Command>, b:Buffer) -> Result<(), Error>{
    let sets = HashMap<Oid, (Arc<dyn Entity>, s);
    let scope = HashMap<uint64, Value>;
    loop {
        if b.len() == 0 {
            break;
        }
        match b.get(0) {
            Operation::Get as u64 => {
                resolve(Value::decode)
            }
            Operation::Set as u64 => {
                
            }
            Operation::Copy as u64 => {
            }
            Operation::Create as u64 => {
                finalize(b, new_object());
            }
        }
        println!("{}",k);
    }
    for o, s in sets {
    }
}


