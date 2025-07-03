use std::collections::HashMap;
use std::fs::File;
use std::io;
use serde::{Serialize, Deserialize};
// base struct
#[derive(Serialize, Deserialize)]
pub struct Database {
    store: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    } // create db

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    } // command SET

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    } // command GET

    pub fn del(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    } // command DEL

    pub fn exists(&mut self, key: &str) -> bool {
        self.store.contains_key(key)
    } // command EXISTS

    pub fn flush(&mut self) {
        self.store.clear();
    } // command FLUSH

    pub fn scan(&self) -> Vec<String> {
        self.store.keys().cloned().collect() // clone iterator with all keys and transform to a collection
    } // command SCAN

    pub fn save(&self) -> io::Result<()> {
        let file = File::create("saves/dump.json")?;
        let data = &self.store;

        serde_json::to_writer_pretty(file, &data)?;
        Ok(())

    } // command SAVE
}