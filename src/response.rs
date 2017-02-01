use std::io::prelude::*;
use std::io::Error;
use header::*;
use prelude::*;

pub type Body = Option<Vec<u8>>;

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
    fn from<T: Into<Vec<u8>>>(self, body: T) -> Response;
}

impl ToResponse for status::HttpStatus {
    fn from<T: Into<Vec<u8>>>(self, body: T) -> Response {
        Response {
            status: self,
            headers: Headers::new(),
            body: Some(body.into()),
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

    pub fn send<T: Write>(mut self, w: &mut T) -> Result<(), Error> {
        try!(w.write_fmt(format_args!("HTTP/1.0 {} {}\r\n", self.status.0, self.status.1)));
		if self.headers.find("Connection").is_none() {
			self.headers.push(Header::new("Connection","close"));
		}
		if self.headers.find("Content-Type").is_none() {
			self.headers.push(Header::new("Content-Type","text/html; charset=utf-8"));
		}
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
