#[macro_use]
extern crate httprs;

use httprs::prelude::*;
use httprs::header::*;
use httprs::prelude::status::{Ok, NotFound};
use httprs::server::Server;
use httprs::request::Request;
use httprs::response::Response;

#[allow(unused_variables)]
fn main() {
    let mut s = Server::new("127.0.0.1:3000");
    s.add_boxed_handle(Box::new(|req: &mut Request| if req.request_uri == "/404" {
        Response::new(NotFound,
                      headers!["Content-Type"=>"text/html;charset=utf-8","Server"=>"http.rs"],
                      None)
            .show()
    } else {
        Response::skip()
    }));

    s.add_boxed_handle(Box::new(|req: &mut Request| if req.request_method == Methods::POST {
        Ok.from(Some(Vec::from(r#"{"override":{"boy":"girl"}}"#)))
            .with_headers(headers!["Content-Type"=>"application/json;charset=utf-8"])
            .show()
    } else {
        Response::skip()
    }));

    s.add_boxed_handle(Box::new(|req: &mut Request| {
        Ok.from(Some(Vec::from("<h1>你好，世界</h1>"))).show()
    }));

    s.serve().unwrap();
}