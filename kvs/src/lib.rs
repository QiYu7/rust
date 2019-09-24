
pub struct KvStore {
    key : String,
    value : String,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore{
            key: "".to_owned(), 
            value: "".to_owned()
        }
    }
    pub fn set(&self, k : String, v : String){
        panic!("set!");
    }
    pub fn get(&self, key: String) -> Option<String> {
        panic!("get!");
    }
    pub fn remove(&self, key: String) {
        panic!("rm!");
    }
}