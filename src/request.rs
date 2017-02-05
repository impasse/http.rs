use std::io::*;
use std::net::TcpStream;
use header::{Header, Headers, FindHeader};
use prelude::Methods;
use prelude::status::*;
use std::fmt;
use std::result::Result as FullResult;

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

const MAX_BODY_LENGTH : usize = 1024*1024*16;

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
        write!(f, "{}", String::from_utf8(self.data.to_owned()).unwrap())
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
    pub fn from_tcp_stream(stream: &TcpStream) -> FullResult<Self,HttpStatus> {
        let remote_ip = try!(stream.peer_addr().map_err(|_| InternalServerError)).ip().to_string();
        let local_port = try!(stream.local_addr().map_err(|_| InternalServerError)).port();
        let mut stream = BufReader::new(stream);
        let mut buf = String::new();
        let mut start = false;
        // skip empty line
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
                try!(stream.read_line(&mut buf).map_err(|_| BadRequest));
                if buf == "\r\n" {
                    break;
                } else {
                    headers.push(Request::parse_header(&buf).unwrap());
                }
                buf.clear();
            }
            let body_size = headers.find("Content-Length")
                .and_then(|h| usize::from_str_radix(&h.val, 10).ok())
                .unwrap_or(0);
            if body_size > MAX_BODY_LENGTH {
                return Err(BadRequest);
            }
            let body = match body_size {
                0 => None,
                body_size => {
                    let mut buf = vec![0;body_size];
                    stream.read_exact(buf.as_mut())
                    .ok()
                    .map(move |_| RequestBody::new(buf))
                }
            };
            FullResult::Ok(Request {
                request_uri: request_uri,
                headers: headers,
                protocol: protocol,
                body: body,
                query_string: query_string,
                server_port: local_port,
                remote_ip: remote_ip,
                request_method: method,
            })
        } else {
            Err(BadRequest)
        }
    }

    fn parse_query_string(request_uri: &str) -> Option<String> {
        let path: Vec<&str> = request_uri.splitn(2, '?').map(|s| s.trim()).collect();
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
        line.parse::<Header>().ok()
    }
}