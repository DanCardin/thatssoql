use crate::row::Row;
use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pager {
    pub rows: BTreeMap<usize, Row>,
}

impl Pager {
    pub fn open() -> io::Result<Self> {
        let path = Path::new("file.rsqlone");
        if path.exists() {
            let reader = BufReader::new(File::open(path)?);
            if let Ok(result) = serde_json::from_reader(reader) {
                return Ok(result);
            }
        }

        Ok(Self {
            rows: BTreeMap::new(),
        })
    }

    pub fn insert(self: &mut Self, row: Row) {
        println!("insert: {:?}", row);
        self.rows.insert(self.rows.len() + 1, row);
    }

    pub fn write(self: &mut Self) -> io::Result<()> {
        let path = Path::new("file.rsqlone");
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).unwrap();
        Ok(())
    }
}
