use std::net::*;
use request::Request;
use response::{Response,ResponseState};
use std::io::Error;
use std::thread;
use std::sync::{Arc, RwLock};
use handle::Handle;

pub struct Server<B> {
    bind: B,
    handles: Arc<RwLock<Vec<Box<Handle>>>>,
}

impl<B:ToSocketAddrs> Server<B> {
    pub fn new(bind: B) -> Self {
        Server {
            bind: bind,
            handles: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_boxed_handle(&mut self, h: Box<Handle>) {
        let handles = self.handles.clone();
        handles.write().unwrap().push(h);
    }

    pub fn add_handle<T>(&mut self, h: T)
        where T: Fn(&mut Request) -> Response + Send + Sync + 'static
    {
        let handles = self.handles.clone();
        handles.write().unwrap().push(Box::new(h));
    }

    pub fn serve(self) -> Result<(), Error> {
        let listener = try!(TcpListener::bind(self.bind));
        for stream in listener.incoming() {
            let handles = self.handles.clone();
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
