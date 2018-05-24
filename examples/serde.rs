#[macro_use]
extern crate typed_key;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate failure;

use serde::de::DeserializeOwned;
use serde_json::Value;
use std::net::IpAddr;
use typed_key::Key;

type Result<T> = ::std::result::Result<T, failure::Error>;

struct Config {
    data: Value,
}

impl Config {
    fn from_str(data: &str) -> Result<Config> {
        let data = serde_json::from_str(data)?;
        Ok(Config { data })
    }

    fn get<T: DeserializeOwned>(&self, key: Key<T>) -> Result<T> {
        let value = self.get_opt(key)?;
        let value = value.ok_or_else(|| format_err!("config value is required: `{}`", key.name()))?;
        Ok(value)
    }

    fn get_opt<T: DeserializeOwned>(&self, key: Key<T>) -> Result<Option<T>> {
        match self.data.get(key.name()) {
            None => Ok(None),
            Some(data) => {
                let data = T::deserialize(data)?;
                Ok(Some(data))
            }
        }
    }
}

/// IP address of the server
const ADDR: Key<IpAddr> = typed_key!("addr");

/// PORT of the server, defaults to 8080
const PORT: Key<u32> = typed_key!("port");

/// Host name of the server
const HOST: Key<String> = typed_key!("host");

fn main() -> Result<()> {
    let config = Config::from_str(
        r#"{
        "addr": "127.0.0.1",
        "host": "hal9000"
    }"#,
    )?;

    println!("addr = {:?}", config.get(ADDR)?);
    println!("port = {:?}", config.get_opt(PORT)?.unwrap_or(8080));
    println!("host = {:?}", config.get(HOST)?);
    Ok(())
}
