// mod libs;

// extern crate yaml_rust;
extern crate strum;
#[macro_use]
extern crate strum_macros;

// use std::env;  // To be able to add arguments, I will expand on this later
use std::fs::File;
use strum_macros::{Display, EnumIter};

fn main() {
    // let args: Vec<String> = env::args().collect();  // collects args for future features
    // println!("{:?}", args[0]);     // For debugging purposes
    let yaml_config = File::open("config/config.yml");


    println!("{:?}",yaml_config);
}
