use std::io::prelude::*;
use std::io::Error;
use header::*;
use prelude::*;

type Body = Option<Vec<u8>>;

pub enum ResponseState {
    Show(Response),
    Skip,
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Response {
    status: status::HttpStatus,
    headers: Headers,
    body: Body,
}

pub trait ToResponse {
    fn from(self, body: Body) -> Response;
}

impl ToResponse for status::HttpStatus {
    fn from(self, body: Body) -> Response {
        let mut tmp_headers = Headers::new();
        tmp_headers.push(Header::new("Connection", "close"));
        tmp_headers.push(Header::new("Content-Type", "text/html; charset=utf-8"));
        Response {
            status: self,
            headers: tmp_headers,
            body: body,
        }
    }
}

impl Response {
    pub fn new(status: status::HttpStatus, headers: Headers, body: Body) -> Self {
        Response {
            status: status,
            headers: headers,
            body: body,
        }
    }

    pub fn with_headers(self, headers: Headers) -> Self {
        Response {
            status: self.status,
            body: self.body,
            headers: headers,
        }
    }

    pub fn show(self) -> ResponseState {
        ResponseState::Show(self)
    }

    pub fn skip() -> ResponseState {
        ResponseState::Skip
    }

    pub fn send<T>(self, w: &mut T) -> Result<(), Error>
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