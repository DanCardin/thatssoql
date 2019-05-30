use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {}

impl Table {
    pub fn new() -> Self {
        Self {}
    }
}
