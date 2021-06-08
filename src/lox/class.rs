use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(class {})", self.name)
    }
}
