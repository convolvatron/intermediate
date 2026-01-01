use alloc::{vec::Vec, string::String};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Path {
    absolute : bool,
    elements: Vec<String>,    
}

impl Path {
    pub fn new(s: &str) -> Self {
        let mut start = 0;
        let mut end = 0;        
        let mut absolute = false;
        let mut elements = Vec::new();

        while end < s.len() {
            if s[end] == '/' {
                if end == 0 {
                    absolute = true;
                    end = end+1
                }
                elements.push(s[start..end].to_string());
                start = end;
            }
            end = end + 1;
        }
        Path{absolute, elements}
    }        


    pub fn to_str(&self) -> &str {
        // join
        &self.inner
    }
    
    pub fn join(&self, other: &Path) -> Path {
        let mut ret: Path = Path{elements:vec_with_capacity(self.inner.len() + other.inner.len())};
        ret.push(self);
        ret.push(other);
        ret
    }
}

