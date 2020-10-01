extern crate ssh2;

use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
use crate::libs::yaml_parser::Server;

fn executecommand() {

}

pub fn connectandrun(data: Server){
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



// TODO write test functions later