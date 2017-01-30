use std::io::*;
use std::net::TcpStream;
use header::{Header, Headers};
use prelude::Methods;

struct AbstractRequest<T: Read> {
    request_uri: String,
    headers: Vec<Header>,
    protocol: String,
    body: Option<T>,
    query_string: Option<String>,
    server_port: u16,
    remote_ip: String,
    request_method: Methods,
}

struct RequestBody {}

impl Read for RequestBody {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
}


pub type Request = AbstractRequest<RequestBody>;

impl Request {
    pub fn from_tcp_stream(stream: TcpStream) -> Self {
        let remote_ip = stream.peer_addr().unwrap().ip().to_string();
        let local_port = stream.local_addr().unwrap().port();
        let mut stream = BufReader::new(stream);
        let mut buf = String::new();
        let read = stream.read_line(&mut buf);
        let (method, request_uri, protocol) = read.ok()
            .or_else(|| panic!("error read request line"))
            .and_then(|_| Request::parse_request_line(&buf))
            .unwrap_or_else(|| panic!("error parse request line"));
        buf.clear();
        let mut headers = Headers::new();
        loop {
            let result = stream.read_line(&mut buf);
            if result.is_ok() {
                if result.unwrap() != 0 {
                    break;
                } else {
                    if let Some(valid_header) = Request::parse_header(&buf) {
                        headers.push(valid_header);
                    } else {
                        continue;
                    }
                    buf.clear();
                }
            } else {
                panic!("error parse headers")
            }
        }
        Request {
            request_uri: request_uri,
            headers: headers,
            protocol: protocol,
            body: None,
            query_string: None,
            server_port: local_port,
            remote_ip: remote_ip,
            request_method: method,
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
        if let &[method, request_uri, protocol] = args.as_slice() {
            Some((Request::parse_method(method), request_uri.to_string(), protocol.to_string()))
        } else {
            None
        }
    }

    fn parse_header(line: &str) -> Option<Header> {
        let args: Vec<&str> = line.splitn(2, ':')
            .map(|s| s.trim())
            .collect();
        if let &[name, value] = args.as_slice() {
            Some(Header::new(name, value))
        } else {
            None
        }
    }
}