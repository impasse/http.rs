use header::Header;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

pub struct Response {
    headers: Vec<Header>,
    body: String
}

impl Write for Response {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        String::from_utf8(Vec::from(buf))
            .map_err(|e| Error::from(ErrorKind::Other))
            .map(|s| {
                self.body.push_str(&s);
                s.len()
            })
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}