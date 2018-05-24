#[macro_use]
extern crate typed_key;

use std::{
    collections::HashMap,
    any::Any,
};

use typed_key::Key;
use std::net::IpAddr;

// http://geekandpoke.typepad.com/.a/6a00d8341d3df553ef0167673db2b5970b-800wi
struct Ffu {
    data: HashMap<&'static str, Box<Any>>
}

impl Ffu {
    fn new() -> Ffu {
        Ffu { data: HashMap::new() }
    }

    fn insert<T: Any>(&mut self, key: Key<T>, value: T) {
        self.data.insert(key.name(), Box::new(value));
    }

    fn get<T: Any>(&self, key: Key<T>) -> Option<&T> {
        let value = self.data.get(key.name())?;
        let value = value.downcast_ref()
            .expect("Corrupted Ffu");
        Some(value)
    }
}


const ADDR: Key<IpAddr> = typed_key!("addr");
const PORT: Key<u32> = typed_key!("port");
const HOST: Key<String> = typed_key!("host");


fn main() {
    let localhost = "127.0.0.1".parse().unwrap();

    let mut ffu = Ffu::new();
    ffu.insert(ADDR, localhost);
    ffu.insert(PORT, 80);
    println!("addr = {:?}", ffu.get(ADDR));
    println!("port = {:?}", ffu.get(PORT));
    println!("host = {:?}", ffu.get(HOST));
}
