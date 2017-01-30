extern crate httprs;

use rustpp::server::*;

fn main(){
    let s = Server::new();
    s.serve();
}