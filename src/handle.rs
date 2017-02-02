use request::Request;
use response::Response;

pub type Handle = Fn(&mut Request) -> Response + Send + Sync + 'static;