// mod libs;

extern crate yaml_rust;
// extern crate strum;
// #[macro_use] extern crate strum_macros;

// use std::env;  // To be able to add arguments, I will expand on this later
use std::fs::File;
use std::io::Read;
use yaml_rust::{YamlLoader};
// use std::string::ToString;
// use strum_macros::{Display, EnumIter};

// #[derive(Display, Debug)]
// enum ServersYAML {
//     Groups {Server: String {FQDN: String, Username: String, Password: String, SSHKey: String} },

// }

fn main() {
    // let args: Vec<String> = env::args().collect();  // collects args for future features
    // println!("{:?}", args[0]);     // For debugging purposes
    let mut yaml_config = File::open("config/config.yml").expect("File missing");
    let mut yaml_data = String::new();
    yaml_config.read_to_string(&mut yaml_data).expect("Cannot read file");
    let Entries =  YamlLoader::load_from_str(&mut yaml_data).unwrap();

    println!("{}", yaml_data);
    println!("{:?}",Entries);
}
