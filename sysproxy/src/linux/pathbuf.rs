use super::path::Path;
use alloc::{string::String, vec::Vec};
use core::ops::Deref;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Default)]
pub struct Path {
    elements: Vec<String>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
        }
    }

    /// Creates a new `PathBuf` with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: String::with_capacity(capacity),
        }
    }

    pub fn concat(&mut self, path: Path) {
        let path = path.as_ref();

        if path.is_absolute() {
            self.inner = path.as_str().into();
            return;
        }

        if !self.inner.is_empty() && !self.inner.ends_with('/') {
            self.inner.push('/');
        }

        self.inner.push_str(path.as_str());
    }


}
