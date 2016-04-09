use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::prelude::*;
use std::str;

fn header(read_data: &str) -> String{
    let mut _read_data = read_data.to_string();     
    //let client_data: Vec<&str> = read_data.split('\r');
    let mut client_data:Vec<&str> = read_data.split("\r\n").collect();
    println!("{:?}",client_data[0]);
    match client_data[0].find("GET").is_some() {
        true => "GET method".to_string(),
        _ => "It is None GET method".to_string(),
        
        }
//    let client_data: Vec<&str> = _read_data;
   // for row in client_data{
   //    // if assert_eq!(row.matches("GET"), "GET"){
   //    //     println!("{}",&row);
   //    // }
   //     println!("{:?}",&row.matches("GET"));
   //     //println!("{}",&row);
   // }    
}



fn handler(mut stream : TcpStream){
    let mut read_data = [0;1024];
    stream.read(&mut read_data);
    let mut response = header(str::from_utf8(&read_data).unwrap());
    stream.write(response.as_bytes());
    //header(&read_data);

}

fn server(){

    let mut listener = TcpListener::bind("127.0.0.1:8876").unwrap();
    for mut stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handler(stream)
                });

            }
            Err(e) =>{
                println!("connect err {:?}",e);
            }
        }
    }
}


fn main() {
    server();
    println!("Hello, world!");
}
