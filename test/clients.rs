use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::str;

fn client(){
    if let  Ok(mut stream) = TcpStream::connect("127.0.0.1:12345") {
        println!("Connected to the server!");
        let mut msg = "test msg\n";
        let mut msg_buf = Vec::new();
        let _ = stream.write(msg.as_bytes());
        let len = stream.read(&mut msg_buf);
        println!("read msg{:?}", str::from_utf8(&msg_buf[..]).unwrap());
    } else {
        println!("Couldn't connect to server...");
    }
}

fn main() {
    for index in 1..10000{
         client();
         println!("client id is {}", index)
    }
}
