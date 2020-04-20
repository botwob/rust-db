use std::collections::HashMap;
use std::process;



#[derive(Default)]
pub struct KvStore {
    map: HashMap<String,String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        panic!();
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.map.get(&key).cloned()    
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

fn cli_no_args() {
    process::exit(1);
}

fn cli_version() {
    // println!("{}", env!("CARGO_PKG_VERSION"))
    panic!();
}

fn cli_get() {
    panic!();
}

fn cli_set() {
    panic!();
}

fn cli_invalid_get() {
    panic!();
}

fn cli_invalid_set() {
    panic!();
}

fn cli_invalid_subcommand() {
    panic!();
}

fn get_stored_value() {
    panic!();
}

fn overwrite_value() {
    panic!();
}

fn get_non_existent_value() {
    panic!();
}

fn remove_key() {
    panic!();
}