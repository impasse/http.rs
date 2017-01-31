use std::net::*;
use std::io::prelude::*;
use request::Request;
use response::Response;
use header::{Headers, Header};
use prelude::Status;
use std::boxed::FnBox;

pub struct Server {
    bind: &'static str,
    handles: Vec<Box<Fn(&mut Request) -> Option<Response>>>,
}

impl Server {
    pub fn new(bind: &'static str) -> Self {
        Server {
            bind: bind,
            handles: Vec::new(),
        }
    }

    pub fn add_boxed_handle(&mut self, f: Box<Fn(&mut Request) -> Option<Response>>) {
        self.handles.push(f);
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind(self.bind).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut req = Request::from_tcp_stream(&stream);
                    for handle in &self.handles {
                        match handle(&mut req) {
                            Some(res) => {
                                res.send(&mut stream);
                                stream.shutdown(Shutdown::Both);
                            }
                            None => (),
                        }
                    }
                }
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}
