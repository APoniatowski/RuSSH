extern crate serde_yaml;
extern crate serde_derive;

use std::collections::BTreeMap;
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

        let expected = r#"{"ServerGroup1": {"Server11": [Server { FQDN: "hostname11.whatever.com", Username: "user11", Password: "password11", Key_Path: "/path/to/key", Port: "22" }], "Server12": [Server { FQDN: "hostname12.whatever.com", Username: "user12", Password: "password12", Key_Path: "/path/to/key", Port: "2222" }]}, "ServerGroup2": {"Server21": [Server { FQDN: "hostname21.whatever.com", Username: "user21", Password: "password21", Key_Path: "/path/to/key", Port: "2233" }], "Server22": [Server { FQDN: "hostname22.whatever.com", Username: "user22", Password: "password22", Key_Path: "/path/to/key", Port: "2244" }]}}"#;   
        let result: BTreeMap<String, Group> = parseyaml("test_data/test_pool.yml");
        println!("{:?}", result);
        println!("{:?}", expected);

    }
}