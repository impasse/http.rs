use std::net::*;
use request::Request;
use response::{Response,ResponseState};
use std::io::Error;
use std::thread;
use std::sync::{Arc, RwLock};
use handle::Handle;

pub struct Server {
    bind: &'static str,
    handles: Vec<Box<Handle>>,
}

impl Server {
    pub fn new(bind: &'static str) -> Self {
        Server {
            bind: bind,
            handles: Vec::new(),
        }
    }

    pub fn add_boxed_handle(&mut self, h: Box<Handle>) {
        self.handles.push(h);
    }

    pub fn add_handle<T>(&mut self, h: T)
        where T: Fn(&mut Request) -> Response + Send + Sync + 'static
    {
        self.handles.push(Box::new(h));
    }

    pub fn serve(self) -> Result<(), Error> {
        let listener = try!(TcpListener::bind(self.bind));
        let handles = Arc::new(RwLock::new(self.handles));
        for stream in listener.incoming() {
            let handles = handles.clone();
            thread::spawn(move || match stream {
                Ok(mut stream) => {
                    let mut req = Request::from_tcp_stream(&stream);
                    for handle in &*handles.read().unwrap() {
                        let res = handle(&mut req);
                        match res.state {
                            ResponseState::Show => {
                                res.send(&mut stream).unwrap();
                                break;
                            }
                            ResponseState::Skip => (),
                        }
                    }
                }
                Err(e) => panic!("{:?}", e),
            });
        }
        Ok(())
    }
}
