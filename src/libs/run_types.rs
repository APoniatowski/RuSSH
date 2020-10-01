use std::collections::BTreeMap;
use crate::libs::yaml_parser::*;
use crate::libs::ssh_lib;


pub fn serial(fileloc: &str) {
    let groups: BTreeMap<String, Group> = parseyaml(fileloc);
    for (groupname, group) in groups {
        println!("Processing {}:", groupname);
        for (servername, servers) in group {
            for info in servers {
                println!("Connecting to {}...", servername);
                // println!("{}", info.FQDN);
                // println!("{}", info.Username);
                // println!("{}", info.Password);
                // println!("{}", info.Key_Path);
                ssh_lib::connectandrun(info);
            };
        };
    };
    // connectandrun();
}

pub fn async_all() {


}

pub fn async_groups () {


}

// TODO test the different types of runs