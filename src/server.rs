use std::net::*;
use std::io::prelude::*;
use request::Request;

pub struct Server {
    bind: &'static str,
}

impl Server {
    pub fn new() -> Self {
        Server { bind: "127.0.0.1:3000" }
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind(self.bind).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    {
                        let req = Request::from_tcp_stream(stream);
                        println!("{:?}", req);
                    }
                    // stream.write_all("HTTP/1.0 200 \
                    //                   OK\r\nConnection:close\r\nContent-Length:\
                    //                   10\r\n\r\nHelloworld"
                    //     .as_bytes());
                }
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}