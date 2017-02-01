use std::io::*;
use std::net::TcpStream;
use header::{Header, Headers, FindHeader};
use prelude::Methods;
use std::fmt;

#[derive(Debug,Clone)]
pub struct Request {
    pub request_uri: String,
    pub headers: Headers,
    pub protocol: String,
    pub body: Option<RequestBody>,
    pub query_string: Option<String>,
    pub server_port: u16,
    pub remote_ip: String,
    pub request_method: Methods,
}

#[derive(Clone)]
pub struct RequestBody {
    data: Vec<u8>,
}

impl RequestBody {
    pub fn new<T: Into<Vec<u8>>>(data: T) -> Self {
        RequestBody { data: data.into() }
    }
}

impl fmt::Display for RequestBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", String::from_utf8(self.data.to_owned()).unwrap())
    }
}

impl fmt::Debug for RequestBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "RequestBody({:?})",
               String::from_utf8(self.data.to_owned()))
    }
}

impl Request {
    pub fn from_tcp_stream(stream: &TcpStream) -> Self {
        let remote_ip = stream.peer_addr().unwrap().ip().to_string();
        let local_port = stream.local_addr().unwrap().port();
        let mut stream = BufReader::new(stream);
        let mut buf = String::new();
        let mut start = false;
        while !start {
            buf.clear();
            start = stream.read_line(&mut buf)
                .ok()
                .and_then(|v| if v == 0 { None } else { Some(v) })
                .is_some();
        }
        if let Some((method, request_uri, protocol)) = Request::parse_request_line(&buf) {
            buf.clear();
            let query_string = Request::parse_query_string(&request_uri);
            let mut headers = Headers::new();
            loop {
                stream.read_line(&mut buf).expect("error read request header");
                if buf == "\r\n" {
                    break;
                } else {
                    headers.push(Request::parse_header(&buf).unwrap());
                }
                buf.clear();
            }
            let body = headers.find("Content-Length")
                .and_then(|h| usize::from_str_radix(&h.val, 10).ok())
                .and_then(|len| {
                    let mut buf = vec![0u8;len];
                    stream.read_exact(buf.as_mut())
                        .ok()
                        .map(move |_| buf)
                })
                .map(|vec| RequestBody::new(vec));
            Request {
                request_uri: request_uri,
                headers: headers,
                protocol: protocol,
                body: body,
                query_string: query_string,
                server_port: local_port,
                remote_ip: remote_ip,
                request_method: method,
            }
        } else {
            panic!("Error while read request line");
        }
    }

    fn parse_query_string(request_uri: &str) -> Option<String> {
        let path: Vec<&str> = request_uri.splitn(2, '?').collect();
        match path.as_slice() {
            &[_, query] => Some(query.to_owned()),
            _ => None,
        }
    }

    fn parse_method(method: &str) -> Methods {
        match method.to_uppercase().as_str() {
            "HEAD" => Methods::HEAD,
            "GET" => Methods::GET,
            "POST" => Methods::POST,
            "PUT" => Methods::PUT,
            "DELETE" => Methods::DELETE,
            "LINK" => Methods::LINK,
            "UNLINK" => Methods::UNLINK,
            other @ _ => Methods::Extension(other.to_string()),
        }
    }

    fn parse_request_line(line: &str) -> Option<(Methods, String, String)> {
        let args: Vec<&str> = line.splitn(3, ' ')
            .map(|s| s.trim())
            .collect();
        match args.as_slice() {
            &[method, request_uri, protocol] => {
                Some((Request::parse_method(method), request_uri.to_string(), protocol.to_string()))
            }
            _ => None,
        }
    }

    fn parse_header(line: &str) -> Option<Header> {
        let args: Vec<&str> = line.splitn(2, ':')
            .map(|s| s.trim())
            .collect();
        match args.as_slice() {
            &[name, value] => Some(Header::new(name, value)),
            _ => None,
        }
    }
}