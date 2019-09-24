extern crate clap;

use clap::{Arg, App};
use kvs::KvStore;

fn main() {
    /*
    let matches = App::new("kvs")
        .version("0.1.0")
        .author("QiYu7")
        .about("a key-value store tool.")
        .arg(Arg::with_name("get")
            .long("get")
            .value_delimiter(","))
        .get_matches();
    
    let args: Vec<&str> = matches.values_of("get").unwrap().collect();

    println!("{:?}", args);
    */

    let test = KvStore::new("key1".to_owned(), "value1".to_owned());

    println!("{:?}", test);

}