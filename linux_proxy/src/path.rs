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

        while end < s.len() {
            if s[end] == '/' {
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
        let res = self.elements.join("/");
        if self.absolute {
            res = "/" + res;
        }
        res
    }

    pub fn join(&self, other: &Path) -> Path {
        let mut ret: Path = Path {
            elements: vec_with_capacity(self.elements.len() + other.elementslen()),
        };
        ret.push(self);
        ret.push(other);
        ret
    }
}
