use crate::Oid;
use alloc::string::String;

#[derive(Debug)]
pub struct Error {
    pub location: Oid,
    pub cause: String,
    pub syserr: Option<u8>,
    // file and line can we do?
}

    
#[macro_export]
macro_rules! err {
    ($oid:expr, $($arg:tt)*) => {{
        crate::Error{cause:alloc::format!($($arg)*), location:$oid, syserr: None}
    }}
}
