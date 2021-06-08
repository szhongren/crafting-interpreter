use std::fmt::Display;

use super::class::Class;

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    klass: Class,
}

impl Instance {
    pub fn new(klass: Class) -> Self {
        Self { klass }
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(instance {})", self.klass)
    }
}
