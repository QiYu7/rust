extern crate clap;

use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use std::process::exit;

fn main() {
    //let mut test = KvStore::new();

    let matches = App::new("kvs")
        .about("a key-value store tool")
        .author("QiYu7")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").required(true))
                .arg(Arg::with_name("VALUE").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .get_matches();

    /*
    if let Some(sub_m) = matches.subcommand_matches("set") {
        if let Some(key) = sub_m.value_of("KEY") {
            if let Some(val) = sub_m.value_of("VALUE") {
                test.set(key, val);
            }
        }
    }
    */

    //println!("{:?}", test);
    match matches.subcommand() {
        ("set", Some(sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => {
            exit(1);
        }
    }
}
