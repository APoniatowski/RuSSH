extern crate ssh2;

use std::io::prelude::*;
use std::net::{TcpStream};
use std::collections::BTreeMap;
use ssh2::Session;
use crate::libs::yaml_parser::*;
use crate::libs::yaml_parser::Server;

fn executecommand() {

}

fn connectandrun(data: Server){
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent("username").unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap())
}

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
                connectandrun(info);
            };
        };
    };
    // connectandrun();
}