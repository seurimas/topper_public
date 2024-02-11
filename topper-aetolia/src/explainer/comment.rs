use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub body: String,
    pub reference_line: usize,
    pub added: i32,
}

impl Comment {
    pub fn new(reference_line: usize, time: i32) -> Self {
        Self {
            body: String::new(),
            reference_line,
            added: time,
        }
    }

    pub fn get_line(&self) -> usize {
        self.reference_line
    }

    pub fn is_for_line(&self, line: usize) -> bool {
        self.reference_line == line
    }

    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    pub fn update_body(&mut self, body: String) {
        self.body = body;
    }
}
