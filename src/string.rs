// 字符串 (String) implementation in Rust

pub struct String {
    value: Vec<u8>,
}

impl String {
    pub fn new() -> Self {
        String { value: Vec::new() }
    }

    pub fn from_str(s: &str) -> Self {
        String { value: s.as_bytes().to_vec() }
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.value).unwrap()
    }
}

// Additional methods can be added here.