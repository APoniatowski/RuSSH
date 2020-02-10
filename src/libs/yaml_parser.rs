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
    pub Key_Path: String
}

pub type Group = BTreeMap<String, Vec<Server>>;

pub fn parseyaml(fileloc: &str) -> BTreeMap<String, Group> {
    let file = BufReader::new(File::open(fileloc).unwrap());
    serde_yaml::from_reader(file).unwrap()
}