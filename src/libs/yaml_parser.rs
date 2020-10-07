extern crate serde_yaml;
extern crate serde_derive;

use std::collections::BTreeMap;
// use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Server {
    pub FQDN: String,
    pub Username: String,
    pub Password: String,
    pub Key_Path: String,
    pub Port: String
}

pub type Group = BTreeMap<String, Vec<Server>>;

pub fn parseyaml(fileloc: &str) -> BTreeMap<String, Group> {
    let file = BufReader::new(File::open(fileloc).unwrap());
    serde_yaml::from_reader(file).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parseyaml() {
        let result = parseyaml("test_data/test_pool.yaml");
        assert!(result.is_ok());
    }
}