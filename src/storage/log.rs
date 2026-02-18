use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use crate::error::{GoonError, GoonResult};

#[derive(Debug, Clone)]
pub enum LogEntry {
    Set { key: String, value: String },
    Delete { key: String },
}
/**
 * We will goon to this format for log entries
 * "GOON    key     value" to set a value to a key
 * "BUST  key     value" to delete a value given a key
 */
impl LogEntry {
    pub fn serialize(&self) -> String {
        match self {
            LogEntry::Set { key, value } => {
                format!("GOON\t{}\t{}", key, value)
            }
            LogEntry::Delete { key } => {
                format!("BUST\t{}", key)
            }
        }
    }

    pub fn deserialize(line: &str) -> GoonResult<LogEntry> {
        let parts: Vec<&str> = line.split("\t").collect();

        if parts.is_empty() {
            return Err(GoonError::ParseError("Empty line".to_string()));
        }

        match parts[0] {
            // the goon command
            "GOON" => {
                if parts.len() < 3 {
                    return Err(GoonError::ParseError("Invalid GOON entry".to_string()));
                }
                Ok(LogEntry::Set {
                    key: parts[1].to_string(),
                    value: parts[2].to_string(),
                })
            }
            "BUST" => {
                if parts.len() < 2 {
                    return Err(GoonError::ParseError("Invalid BUST entry".to_string()));
                }

                Ok(LogEntry::Delete {
                    key: parts[1].to_string(),
                })
            }
            _ => Err(GoonError::ParseError(format!(
                "Unknown log entry type {}",
                parts[0]
            ))),
        }
    }
}

pub struct WAL {
    file: File,
}

impl WAL {
    pub fn new(path: &str) -> GoonResult<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(WAL { file })
    }

    pub fn append(&mut self, entry: &LogEntry) -> GoonResult<()> {
        writeln!(self.file, "{}", entry.serialize())?;
        self.file.flush()?;
        Ok(())
    }

    pub fn read_all(path: &str) -> GoonResult<Vec<LogEntry>> {
        if !Path::new(path).exists() {
            return Ok(Vec::new());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                entries.push(LogEntry::deserialize(&line)?);
            }
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_set() {
        let entry = LogEntry::Set {
            key: "name".to_string(),
            value: "MasterGooner69".to_string(),
        };
        assert_eq!(entry.serialize(), "GOON\tname\tMasterGooner69");
    }

    #[test]
    fn test_deserialize_set() {
        let entry = LogEntry::deserialize("GOON\tname\tMasterGooner69").unwrap();
        match entry {
            LogEntry::Set { key, value } => {
                assert_eq!(key, "name");
                assert_eq!(value, "MasterGooner69");
            }
            _ => panic!("Wrong entry type"),
        }
    }
}
