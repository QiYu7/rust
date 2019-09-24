The kvs executable supports the following command line arguments:

kvs set <KEY> <VALUE>

Set the value of a string key to a string



kvs get <KEY>

Get the string value of a given string key



kvs rm <KEY>

Remove a given key



kvs -V

Print the version

The kvs library contains a type, KvStore, that supports the following methods:



KvStore::set(&mut self, key: String, value: String)

Set the value of a string key to a string



KvStore::get(&mut self, key: String) -> Option<String>

Get the string value of the a string key. If the key does not exist, return None.



KvStore::remove(&mut self, key: String)

Remove a given key.


//要使用 clap crate

