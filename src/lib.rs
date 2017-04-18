//mod templateEngine;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate futures_cpupool;
extern crate tokio_timer;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rand;
extern crate rustc_serialize;
extern crate tokio_minihttp;

pub mod net;
pub mod query;
pub mod tokionet;
pub mod queue;
pub mod tokiotest;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
        
    // #[test]
    // fn read_html_data(){

    //     // use templateEngine::read_html;
    //     use std::fs::File;
    //     use std::io::prelude::*;
    //     let filename = "foo.html";
    //     // let mut f = File::create(&filename)?;
    //     let mut f = File::create("foo.html")?;
    //     f.write_all(b"Hello, world!")?;
    //     // f.close();
    //     // read_html(&filename);
    // }
    // #[test]
    // fn queue_test(){
    //     use queue::queueData;
    //     let queue =  queueData::new(5);
    //     //queue_data.put("")
    //     let world = String::from("world");
    //     queue.put(world);
    //     let get_world = queue.get();
    //     assert_eq!(world,get_world );
    // }
}