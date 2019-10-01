extern crate clap;

use clap::{Arg, App, SubCommand};
use kvs::KvStore;

fn main() {
    
    let mut test = KvStore::new();

    let matches = App::new("kvs")
        .about("a key-value store tool")
        .author("QiYu7")
        .version("CARGO_PKG_VERSION")
        .subcommand(SubCommand::with_name("set")
            .about("Set the value of a string key to a string")
            .arg(
                Arg::with_name("KEY")
                    .required(true)
            )
            .arg(
                Arg::with_name("VALUE")
                    .required(true)
            ))
        .get_matches_from(vec![
            "kvs", "set", "213", "333"
        ]);
        
        /*
        if let Some(sub_m) = matches.subcommand_matches("set") {
            if let Some(key) = sub_m.value_of("KEY") {
                if let Some(val) = sub_m.value_of("VALUE") {
                    test.set(key, val);
                }
            }
        }
        */

        println!("{:?}", test);


}