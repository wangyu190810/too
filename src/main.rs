use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::prelude::*;
use std::str;
use std::fmt;
use std::time::Duration;
use std::collections::HashMap;


fn get_callback(){
    
    thread::sleep(Duration::new(5,0));
}


fn get(form_arges: &str, callback: fn()) -> String{
    let arges: Vec<&str> = form_arges .split('&').collect();
    for arge in &arges{
        println!("{:?}",arge);
    }
    callback();
    let response  = fmt::format(format_args!("{:},GET",form_arges)); 
    return response
}

fn post(form_arges: &str) -> String{
    let arges: Vec<&str> = form_arges.split('&').collect();
    for arge in &arges{
        println!("{:?}",arge);
    }
    let response  = fmt::format(format_args!("{:},POST",form_arges)); 
    return response
}

fn request(method: HashMap, arges: Vec<&str>){
    if method.contains_key("GET"){
        
        return "GET Method"
    }
    else if method.contains_key("POST"){
        
        return "POST Method"
    }
    else{
    
        return "Other Method"
    }

}

fn roule(url : &str,method: &str){

    let mut methods = HashMap::new();
    let method: HashMap = match method {
        "GET" => methods.insert("GET",)
        "POST" => post(method[1]),
         _ => "other method".to_string()
    };
    request(url,)
    

}



fn response(res: &str) -> String{
    
    

    
}

fn header(read_data: &str) -> String{
    let mut _read_data = read_data.to_string();     
    let mut client_data :Vec<&str> = read_data.split("\r\n").collect();
    println!("{:?}",client_data);
    let method : Vec<&str> = client_data[0].split(' ').collect();
    println!("{:?}",method[0]);

    //let mut response = match method[1] {
    //    "GET" => get(method[1],get_callback),
    //    "POST" => post(method[1]),
    //     _ => "other method".to_string()
    //};
    return response
}



fn handler(mut stream : TcpStream){
    let mut read_data = [0;1024];
    stream.read(&mut read_data);
    let mut response = header(str::from_utf8(&read_data).unwrap());
    stream.write(response.as_bytes());

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
