use std::collections::HashMap;

// base struct
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
}