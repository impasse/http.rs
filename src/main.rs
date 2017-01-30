extern crate httprs;

use httprs::server::*;

fn main(){
    let s = Server::new();
    s.serve();
}