use lazy_static::lazy_static;
use regex::bytes::Regex;

use crate::output::Output;

pub struct Parser {
    buf: Vec<u8>,
}

pub enum StateChange {
    Database(String),
    Table(String),
    None,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(8129),
        }
    }

    pub fn parse(&mut self, line: &[u8]) -> color_eyre::Result<StateChange> {
        self.buf.truncate(0);
        self.buf.extend_from_slice(line);

        if let Some(create_db) = CREATE_DB.captures(line) {
            let db = String::from_utf8(create_db.get(1).unwrap().as_bytes().to_vec())?;
            Ok(StateChange::Database(db))
        } else if let Some(create_table) = DROP_TABLE.captures(&line) {
            let table = String::from_utf8(create_table.get(1).unwrap().as_bytes().to_vec())?;
            Ok(StateChange::Table(table))
        } else {
            Ok(StateChange::None)
        }
    }

    pub fn output(&self, out: &mut Output) -> color_eyre::Result<()> {
        out.write_bytes(&self.buf)
    }
}

lazy_static! {
    static ref CREATE_DB: Regex = Regex::new("^CREATE DATABASE .*`([^`]+)`").unwrap();
    static ref DROP_TABLE: Regex = Regex::new("^DROP TABLE .*`([^`]+)`").unwrap();
}
