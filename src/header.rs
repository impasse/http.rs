use std::io::{Error, ErrorKind};
use std::ascii::AsciiExt;
use std::str::FromStr;
use std::convert::AsRef;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Header {
    pub key: String,
    pub val: String,
}

impl Header {
    pub fn new<T: AsRef<str>>(k: T, v: T) -> Self {
        Header {
            key: k.as_ref().to_string(),
            val: v.as_ref().to_string(),
        }
    }
}

impl FromStr for Header{
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kv: Vec<_> = s.splitn(2, ':').map(|s| s.trim()).collect();
        match kv.as_slice() {
            &[name,value] => Ok(Header::new(name,value)),
            _ => Err(Error::from(ErrorKind::InvalidInput))
        }
    }
}

pub type Headers = Vec<Header>;

pub trait FindHeader {
    fn find<T: AsRef<str>>(&self, key: T) -> Option<Header>;
}

impl FindHeader for Headers {
    fn find<T: AsRef<str>>(&self, key: T) -> Option<Header> {
        let key = key.as_ref();
        if let Some(b) = self.into_iter().find(|a| key.eq_ignore_ascii_case(a.key.as_str())) {
            Some(b.to_owned())
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! headers {
    ($($a:expr=>$b:expr),*) => {
        {
            let mut tmp_headers = Headers::new();
            $(
                tmp_headers.push(Header::new($a,$b));
            )*
            tmp_headers
        }
    }
}
