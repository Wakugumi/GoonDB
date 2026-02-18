use std::collections::HashMap;

use crate::{
    error::GoonResult,
    log::{LogEntry, WAL},
};

pub struct GoonEngine {
    data: HashMap<String, String>,
    wal: WAL,
}

impl GoonEngine {
    pub fn open(path: &str) -> GoonResult<Self> {
        let entries = WAL::read_all(path)?;
        let mut data = HashMap::new();

        for entry in entries {
            match entry {
                LogEntry::Set { key, value } => {
                    data.insert(key, value);
                }
                LogEntry::Delete { key } => {
                    data.remove(&key);
                }
            }
        }

        let wal = WAL::new(path)?;

        Ok(GoonEngine { data, wal })
    }

    pub fn set(&mut self, key: String, value: String) -> GoonResult<()> {
        let entry = LogEntry::Set {
            key: key.clone(),
            value: value.clone(),
        };
        self.wal.append(&entry)?;

        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> GoonResult<bool> {
        let existed = self.data.contains_key(key);

        if existed {
            let entry = LogEntry::Delete {
                key: key.to_string(),
            };
            self.wal.append(&entry)?;
            self.data.remove(key);
        }

        Ok(existed)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_set_and_get() {
        let path = "/tmp/test_db_1.log";
        let _ = fs::remove_file(path); // Clean up from previous test

        let mut db = GoonEngine::open(path).unwrap();
        db.set("key1".to_string(), "value1".to_string()).unwrap();

        assert_eq!(db.get("key1"), Some(&"value1".to_string()));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_persistence() {
        let path = "/tmp/test_db_2.log";
        let _ = fs::remove_file(path);

        // Create database and add data
        {
            let mut db = GoonEngine::open(path).unwrap();
            db.set("persistent".to_string(), "data".to_string())
                .unwrap();
        }

        // Reopen database and check data persists
        {
            let db = GoonEngine::open(path).unwrap();
            assert_eq!(db.get("persistent"), Some(&"data".to_string()));
        }

        let _ = fs::remove_file(path);
    }
}
