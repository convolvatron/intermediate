use crate::Oid;
use alloc::string::String;

// maybe this should be dynentity 
#[derive(Debug)]
pub struct Error {
    // we're still figuring out the plumbing here, this demi-idea is that we'd 
    // rather fill in the principal later than ploumb it down into the libraries. maybe?
    pub location: Option<Oid>,
    pub cause: String,
    pub syserr: Option<u8>,
    // file and line can we do?
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {{
        crate::Error{cause:alloc::format!($($arg)*), location:None, syserr: None}
    }}
}

#[macro_export]
macro_rules! locerr {
    ($oid:expr, $($arg:tt)*) => {{
        crate::Error{cause:alloc::format!($($arg)*), location:Some($oid), syserr: None}
    }}
}
