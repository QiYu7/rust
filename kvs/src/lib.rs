use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
    kvs: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            kvs: HashMap::new(),
        }
    }
    pub fn set(&mut self, k: String, v: String) {
        self.kvs.insert(k, v);
    }
    pub fn get(&self, k: String) -> Option<String> {
        self.kvs.get(&k).cloned()
    }
    pub fn remove(&mut self, k: String) {
        self.kvs.remove(&k);
    }
}
