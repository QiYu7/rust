#[derive(Debug)]
pub struct KvStore {
    key : String,
    value : String,
}

impl KvStore {
    pub fn new(k: String, v: String) -> KvStore {
        KvStore{
            key: k, 
            value: v,
        }
    }
    pub fn set(&self, k: String, v: String){
        panic!("set!");
    }
    pub fn get(&self, k: String) -> Option<String> {
        panic!("get!");
    }
    pub fn remove(&self, k: String) {
        panic!("rm!");
    }
}