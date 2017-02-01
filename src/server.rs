use std::net::*;
use request::Request;
use response::ResponseState;
use std::io::Error;

type Handle = Box<Fn(&mut Request) -> ResponseState + Send + Sync>;

pub struct Server {
    bind: &'static str,
    handles: Vec<Handle>,
}

impl Server {
    pub fn new(bind: &'static str) -> Self {
        Server {
            bind: bind,
            handles: Vec::new(),
        }
    }

    pub fn add_boxed_handle(&mut self, f: Handle) {
        self.handles.push(f);
    }

    pub fn serve(&self) -> Result<(), Error> {
        let listener = try!(TcpListener::bind(self.bind));
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut req = Request::from_tcp_stream(&stream);
                    for handle in &self.handles {
                        match handle(&mut req) {
                            ResponseState::Show(res) => {
                                try!(res.send(&mut stream));
                                break;
                                // try!(stream.shutdown(Shutdown::Both));
                            }
                            ResponseState::Skip => (),
                        }
                    }
                }
                Err(e) => panic!("{:?}", e),
            }
        }
        Ok(())
    }
}
