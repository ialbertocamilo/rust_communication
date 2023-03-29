use std::io::{Read, Write};
use interprocess::local_socket::LocalSocketStream;

fn main() {


let mut sock =LocalSocketStream::connect("/tmp/localsocket.v1").unwrap();

        let mut buff= String::new();
        sock.write(b"camiloooo HOLAAAA").unwrap();

}
