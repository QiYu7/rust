use std::collections::HashMap;

#[derive(Debug)]
pub struct KvStore {
    kvs: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore{
            kvs: HashMap::new(),
        }
    }
    pub fn set(&mut self, k: &str, v: &str){
        self.kvs.insert(k.to_owned(), v.to_owned());
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        self.kvs.get(k)
    }
    pub fn rv(&mut self, k: &str) {
        self.kvs.remove(k);
    }
}