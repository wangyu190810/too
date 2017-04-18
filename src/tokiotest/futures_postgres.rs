
use std::io;

use futures::{BoxFuture, Future};
use futures_cpupool::CpuPool;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use rand::Rng;
use tokio_minihttp::{Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;
use rustc_serialize;
use tokio_minihttp;

struct Server {
    thread_pool: CpuPool,
}

#[derive(RustcEncodable)]
struct Message {
    id: i32,
    randomNumber: i32,
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Response, io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        println!("{}", &req.path());
        let msg = self.thread_pool.spawn_fn(move || {
            
            Ok(Message {
                id: 123,
                randomNumber: 456,
            })
        });

        msg.map(|msg| {
            let json = rustc_serialize::json::encode(&msg).unwrap();
            let mut response = Response::new();
            response.header("Content-Type", "application/json");
            response.body(&json);
            response
        }).boxed()
    }
}

pub fn http_postgres_run() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let thread_pool = CpuPool::new(10);

    TcpServer::new(tokio_minihttp::Http, addr).serve(move || {
        Ok(Server {
            thread_pool: thread_pool.clone(),
        })
    })
}