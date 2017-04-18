mod query;
use query::{Server,Request,Response};


// fn main(){
//     let  server = Server::new("0.0.0.0",5000,"./static");
//     server.start();
// }
extern crate server;

use server::net;
use server::tokionet;
use server::queue::{queueData, thread_test};
use server::tokiotest;

fn main() {
    // net::run();
    // tokionet::run();
  //   use queue::queueData;
    //   thread_test();
    // tokiotest::futures_test::futher_run();
    tokiotest::futures_postgres::http_postgres_run();

}