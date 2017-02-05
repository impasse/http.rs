use std::io::{Error, ErrorKind};
use std::ascii::AsciiExt;
use std::str::FromStr;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Header {
    pub key: String,
    pub val: String,
}

impl Header {
    pub fn new(k: &str, v: &str) -> Self {
        Header {
            key: k.to_string(),
            val: v.to_string(),
        }
    }
}

impl FromStr for Header{
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kv: Vec<_> = s.split('=').collect();
        if kv.len() == 2 {
            Ok(Header {
                key: kv[0].to_string(),
                val: kv[1].to_string(),
            })
        } else {
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }
}

pub type Headers = Vec<Header>;

pub trait FindHeader {
    fn find(&self, key: &str) -> Option<Header>;
}

impl FindHeader for Headers {
    fn find(&self, key: &str) -> Option<Header> {
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
