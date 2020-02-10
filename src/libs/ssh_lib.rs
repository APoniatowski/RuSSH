extern crate ssh2;

use ssh2::Session;
use std::net::TcpStream;
use std::collections::BTreeMap;
use crate::libs::yaml_parser::*;

fn executecommand() {

}

fn connectandrun() {

}

pub fn serial(fileloc: &str) {
    let groups: BTreeMap<String, Group> = parseyaml(fileloc);
    for (groupname, group) in groups {
        println!("Processing {}:", groupname);
        for (servername, servers) in group {
            for info in servers {
                println!("Connecting to {}...", servername);
                println!("{}", info.FQDN);
                println!("{}", info.Username);
                println!("{}", info.Password);
                println!("{}", info.Key_Path);
            };
        };
    };
    // connectandrun();
}