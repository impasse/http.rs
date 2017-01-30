use std::io::{Read, Error};
use header::Header;

struct AbstractRequest<T: Read> {
    request_uri: String,
    headers: Vec<Header>,
    protocol: String,
    body: Option<T>,
    query_string: Option<String>,
    server_port: u16,
    remote_ip: String
}

struct RequestBody {}

impl Read for RequestBody {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }
}


pub type Request = AbstractRequest<RequestBody>;

impl Default for Request {
    fn default() -> Self {
        Request {
            request_uri: "".to_string(),
            headers: Vec::new(),
            protocol: "HTTP/1.0".to_string(),
            body: None,
            query_string: None,
            server_port: 0,
            remote_ip: "".to_string()
        }
    }
}


impl Request {
    pub fn new() -> Self {
        Default::default()
    }
}