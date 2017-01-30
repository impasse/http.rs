use std::io::{Error,ErrorKind};
use std::fmt;

pub enum Headers {
    Allow,
    Authorization,
    ContentEncoding,
    ContentLength,
    Date,
    Expires,
    From,
    IfModifiedSince,
    LastModified,
    Location,
    Pragma,
    Referer,
    Server,
    UserAgent,
    WWWAuthenticate,
    // additinal
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    ContentLanguage,
    Link,
    MIMEVersion,
    RetryAfter,
    Title,
    URI
}

impl fmt::Display for Headers {
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        write!(f,"{:?}",self)
    }
}

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
//todo
//    fn from(a:Header)-> Self{    }
}

trait HeaderFind{
    fn find_by_str(&self,key:&str) -> Option<Header>;
//    todo
//    fn find(key:Headers)-> Option<Header>;
}

impl HeaderFind for Vec<Header>{
    fn find_by_str(&self,key:&str) -> Option<Header>{
        if let Some(b) = self.into_iter().find(|a|a.key==key.to_string()){
            Some(b.to_owned())
        }else{
            None
        }
    }
}