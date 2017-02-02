#![feature(slice_patterns)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

pub use prelude::*;

pub use request::Request;

pub use response::Response;

pub use server::Server;

pub mod prelude;

pub mod request;

pub mod response;

pub mod server;

pub mod header;

pub mod handle;
