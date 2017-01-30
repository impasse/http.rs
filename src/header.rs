use std::io::{Error,ErrorKind};
use std::fmt;

pub struct Header {
    key: String,
    val: String
}

impl Clone for Header{
    fn clone(&self) -> Self{
        Header {
            key: self.key.to_owned(),
            val: self.val.to_owned()
        }
    }
}

impl Header {
    pub fn new(k: &str, v: &str) -> Self {
        Header {
            key: k.to_string(),
            val: k.to_string()
        }
    }

    fn from_str(s: &str) -> Result<Self,Error> {
        let kv:Vec<_> = s.split('=').collect();
        if kv.len() == 2{
            Ok(Header{
                key: kv[0].to_string(),
                val: kv[1].to_string()
            })
        }else{
            Err(Error::from(ErrorKind::InvalidInput))
        }
    }
}

pub type Headers = Vec<Header>;

trait FindHeader{
    fn find_by_str(&self,key:&str) -> Option<Header>;
}

impl FindHeader for Vec<Header>{
    fn find_by_str(&self,key:&str) -> Option<Header>{
        if let Some(b) = self.into_iter().find(|a|a.key==key.to_string()){
            Some(b.to_owned())
        }else{
            None
        }
    }
}