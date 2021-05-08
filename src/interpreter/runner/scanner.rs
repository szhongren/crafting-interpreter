pub struct Scanner<'a> {
    pub source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn scan_tokens(&self) -> Vec<char> {
        self.source.chars().collect()
    }
}
