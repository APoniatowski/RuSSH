// mod libs;  // Considering dropping this mod, as it is of no use

// use std::env;  // To be able to add arguments, further expansion is planned
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
    // Read the config and parse/deserialize it
    let file = BufReader::new(File::open("config/config.yml")?);
    let groups: BTreeMap<String, Group> = serde_yaml::from_reader(file)?;

    // loop through the parsed data and pass it into SSH client, still planning on running servers in threads
    for (name, group) in groups {
        println!("Starting with {}", name);
        for (names, servers) in group {
            println!("Connecting to {}...", names);
            println!("Placeholder text to follow, these will be passed into the SSH lib, output will be different");
            for info in servers {
                println!("Using FQDN of server [{}]", info.FQDN);
                println!("Using Username of server [{}]", info.Username);
                println!("Using Password of server [{}]", info.Password);
                println!("Using SSH key of server [{}]", info.Key_Path);
            };
        };
    };
    Ok(())
}
