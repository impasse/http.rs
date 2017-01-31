#[macro_use]
extern crate httprs;

use httprs::header::*;
use httprs::prelude::*;
use httprs::server::*;
use httprs::request::Request;
use httprs::response::Response;

fn main() {
    let mut s = Server::new("127.0.0.1:3000");
    s.add_boxed_handle(Box::new(|req: &mut Request| {
        Some(Response::new(Status::Ok,
                      headers!["Connection"=>"close","Content-Type"=>"text/html;charset=utf-8"],
        Some(Vec::from("<h1>你好，世界</h1>"))))
    }));
    s.serve();
}