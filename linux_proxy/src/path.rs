use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Path {
    absolute: bool,
    elements: Vec<String>,
}

impl Path {
    pub fn new(s: &str) -> Self {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut absolute = false;
        let mut elements = Vec::new();
        let sb = s.as_bytes();

        while end < s.len() {
            // we can use as_bytes here, since '/' is a single byte
            if sb[end] == b'/' {
                if end == 0 {
                    absolute = true;
                    end = end + 1
                }
                elements.push(s[start..end].to_string());
                start = end;
            }
            end = end + 1;
        }
        Path { absolute, elements }
    }

    pub fn to_string(&self) -> String {
        let mut res = self.elements.join("/");
        if self.absolute {
            res = "/".to_string() + &res;
        }
        res
    }

    pub fn join(&self, other: &Path) -> Path {
        // we kinda .. really.. dont want other to be absolute?
        let mut ret: Path = Path {
            absolute: self.absolute,
            elements: vec_with_capacity(self.elements.len() + other.elements.len()),
        };
        ret.elements.extend(self.elements);
        ret.elements.extend(other.elements);
        ret
    }
}
