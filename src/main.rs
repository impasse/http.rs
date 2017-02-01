#[macro_use]
extern crate httprs;

use httprs::prelude::*;
use httprs::header::*;
use httprs::prelude::status::{Ok, NotFound};
use httprs::server::Server;
use httprs::response::Response;

fn main() {
    let mut s = Server::new("127.0.0.1:3000");
    s.add_handle(|req| if req.request_uri == "/404" {
        Response::new(NotFound,
                      headers!["Content-Type"=>"text/html;charset=utf-8","Server"=>"http.rs"],
                      None)
    } else {
        Response::skip()
    });

    s.add_handle(|req| if req.request_method == Methods::POST {
        Ok.from(r#"{"override":{"boy":"girl"}}"#)
            .with_headers(headers!["Content-Type"=>"application/json;charset=utf-8"])
    } else {
        Response::skip()
    });

    s.add_handle(|req| if req.request_uri == "/info" {
        Ok.from(format!("{:?}", req))
    } else {
        Response::skip()
    });

    s.add_handle(|req| Ok.from("<h1>你好，世界</h1>"));

    s.serve().unwrap();
}