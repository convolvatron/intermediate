use alloc::vec::Vec;


#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Path {
    elements: Vec<String>,    
}

impl Path {
    pub fn new(s: String) -> &Self {
        unsafe { &*(s.as_ref() as *const str as *const Path) }
    }

    pub fn to_str(&self) -> &str {
        // join
        &self.inner
    }
    
    pub fn join(&self, other: &Path) -> PathBuf {
        let mut ret: Path = Path{elements:vec_with_capacity(self.inner.len() + other.inner.len())}
        ret.push(self);
        ret.push(other);
        ret
    }
}

