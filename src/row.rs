use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub id: usize,
    pub name: Vec<u8>,
    pub email: Vec<u8>,
}

impl Row {
    pub fn primary_key(self: &Self) -> usize {
        self.id
    }
}
