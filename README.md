# http.rs
A http/1.0 implemention for rust

> Too much restriction, don't link it.

```rust
#[macro_use]
extern crate httprs;

use httprs::header::*;
use httprs::prelude::*;
use httprs::server::*;
use httprs::request::Request;
use httprs::response::Response;

fn main() {
    let mut s = Server::new("127.0.0.1:3000");
    s.add_boxed_handle(Box::new(|req: &mut Request| if req.request_uri == "/404" {
        Some(Response::new(Status::NotFound,
                           headers!["Content-Type"=>"text/html;charset=utf-8","Server"=>"http.rs"],
                           Some(Vec::from("<h1>404 Not Found</h1>"))))
    } else {
        None
    }));
    s.add_boxed_handle(Box::new(|req: &mut Request| if req.request_method == Methods::POST {
        Some(Response::new(Status::Ok,
                           headers!["Content-Type"=>"application/json;charset=utf-8"],
                           Some(Vec::from(r#"{"override":{"boy":"girl"}}"#))))
    } else {
        None
    }));
    s.add_boxed_handle(Box::new(|req: &mut Request| {
        Some(Response::new(Status::Ok,
                           headers!["Content-Type"=>"text/html;charset=utf-8"],
                           Some(Vec::from("<h1>你好，世界</h1>"))))
    }));
    s.serve();
}
```