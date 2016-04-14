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

//fn request(method: HashMap, arges: Vec<&str>){
//    if method.contains_key("GET"){
//        
//        return "GET Method"
//    }
//    else if method.contains_key("POST"){
//        
//        return "POST Method"
//    }
//    else{
//    
//        return "Other Method"
//    }
//
//}
//
//fn roule(url : &str,method: &str){
//
//    let mut methods = HashMap::new();
//    let method: HashMap = match method {
//        "GET" => methods.insert("GET"),
//        "POST" => post(method[1]),
//         _ => "other method".to_string()
//    };
//    request(url,)
//    
//
//}
//
//

fn response(res: &str) -> String{
    
    "Response".to_string()
    
}

fn header(read_data: &str) -> String{
    let mut _read_data = read_data.to_string();     
    let mut client_data :Vec<&str> = read_data.split("\r\n").collect();
    println!("{:?}",client_data);
    let method : Vec<&str> = client_data[0].split(' ').collect();
    println!("{:?}",method[0]);

    let mut response = match method[1] {
        "GET" => get(method[1],get_callback),
        "POST" => post(method[1]),
         _ => "other method".to_string()
    };
    return response
}



fn handler(mut stream : TcpStream){
    //let mut request = vec![""];
    //let mut buf = [0;1024];
    let mut buf = [0;128];
    //request.push(str::from_utf8(&buf).unwrap());

    let mut requests:Vec<u8> = Vec::new();
    let mut request:u8;
    let mut read_data = [0;128];
    //let mut read_data = [u8];
    loop{
        match stream.read(&mut buf) {
            Ok(count) => {
                if (count == 128) || (count > 0){
                    println!("message len: {}",count);
                    read_data = buf;
                    let mut flag = 0;
                    loop{
                    
                        &requests.push(buf[flag]);
                        flag += 1;
                        if flag == count - 1{
                            break;
                        }
                    }
                    if count < 128{
                        break;
                    }
                }
                else {
                    println!("message len: {}",count);
                    break;
                }
            }
             _ => {
                break;
            }

        }
    
    println!("message end data: {:}",str::from_utf8(&requests).unwrap());
    }
    let mut response = str::from_utf8(&requests).unwrap().to_string();
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
