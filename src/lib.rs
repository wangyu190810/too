mod templateEngine;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
pub mod net;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
        
    #[test]
    fn read_html_data(){

        // use templateEngine::read_html;
        use std::fs::File;
        use std::io::prelude::*;
        let filename = "foo.html";
        // let mut f = File::create(&filename)?;
        let mut f = File::create("foo.html")?;
        f.write_all(b"Hello, world!")?;
        // f.close();
        // read_html(&filename);
    }
}