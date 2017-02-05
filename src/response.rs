use std::io::prelude::*;
use std::io::Error;
use header::*;
use prelude::*;

pub type Body = Vec<u8>;

#[derive(PartialEq,Eq,Debug,Copy,Clone)]
pub enum ResponseState {
    Show,
    Skip,
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Response {
    pub state: ResponseState,
    status: Option<status::HttpStatus>,
    headers: Option<Headers>,
    body: Option<Body>,
}

pub trait ToResponse {
    fn from<T: Into<Body>>(self, body: T) -> Response;
}

impl ToResponse for status::HttpStatus {
    fn from<T: Into<Body>>(self, body: T) -> Response {
        Response {
            state: ResponseState::Show,
            status: Some(self),
            headers: None,
            body: Some(body.into()),
        }
    }
}

impl Response {
    pub fn new(status: status::HttpStatus, headers: Headers, option_body: Option<Body>) -> Self {
        Response {
            state: ResponseState::Show,
            status: Some(status),
            headers: Some(headers),
            body: option_body,
        }
    }

    pub fn with_headers(self, headers: Headers) -> Self {
        Response {
            state: self.state,
            status: self.status,
            body: self.body,
            headers: Some(headers),
        }
    }

    pub fn skip() -> Self {
        Response {
            state: ResponseState::Skip,
            status: None,
            body: None,
            headers: None,
        }
    }

    pub fn send<T: Write>(self, w: &mut T) -> Result<(), Error> {
        let status = self.status.unwrap(); //cound not be None
        try!(w.write_fmt(format_args!("HTTP/1.0 {} {}\r\n", status.0, status.1)));
        let mut headers = self.headers.unwrap_or_else(|| Headers::new());
        if headers.find("Connection").is_none() {
            headers.push(Header::new("Connection", "close"));
        }
        if headers.find("Content-Type").is_none() {
            headers.push(Header::new("Content-Type", "text/html; charset=utf-8"));
        }
        for header in &headers {
            try!(w.write_fmt(format_args!("{}:{}\r\n", header.key, header.val)))
        }
        try!(w.write_fmt(format_args!("\r\n")));
        match self.body {
            Some(ref body) => Ok(try!(w.write_all(body.as_slice()))),
            None => Ok(()),
        }
    }
}
