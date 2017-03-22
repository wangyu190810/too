use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_html(filename: &String) ->String{
    let mut body = String::new();
    match File::open(filename){
        Ok(mut file ) =>{        
            file.read_to_string(&mut body).unwrap();
        },
        Err(mut err) => {
            
        }

    }
    return body
}
