// mod libs;

extern crate yaml_rust;

// use std::env;  // To be able to add arguments, I will expand on this later
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File; 
use std::io::BufReader;
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Deserialize, Debug)]
struct Server {
    FQDN: String,
    Username: String,
    Password: String,
    Key_Path: String
}

type Group = BTreeMap<String, Vec<Server>>;

fn main() -> Result<()> {
    let file = BufReader::new(File::open("config/config.yml")?);
    let groups: BTreeMap<String, Group> = serde_yaml::from_reader(file)?;

    // println!("{:#?}", groups);
    for (name, group) in groups {
        println!("GROUP={} SIZE={}", name, group.len());
        for (names, servers) in group {
            println!("SERVER={} SIZE={}", names, servers.len());
            for info in servers {
                println!("INFO={:#?} ", info);
            };
        };
    }
    // println!{"{}", groups[0].Server[0].FQDN()};
    Ok(())
}
