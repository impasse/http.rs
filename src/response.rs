use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use header::{Headers, Header};
use prelude::*;

type Body = Option<Vec<u8>>;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Response {
    status: Status::HttpStatus,
    headers: Headers,
    body: Body,
}

impl Response {
    pub fn new(status: Status::HttpStatus, headers: Headers, body: Body) -> Self {
        Response {
            status: status,
            headers: headers,
            body: body,
        }
    }

    pub fn send<T>(&self, w: &mut T) -> Result<(), Error>
        where T: Write
    {
        try!(w.write_fmt(format_args!("HTTP/1.0 {} {}\r\n", self.status.0, self.status.1)));
        for header in &self.headers {
            try!(w.write_fmt(format_args!("{}:{}\r\n", header.key, header.val)))
        }
        try!(w.write_fmt(format_args!("\r\n")));
        match self.body {
            Some(ref body) => Ok(try!(w.write_all(body.as_slice()))),
            None => Ok(()),
        }
    }
}