#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Methods {
    HEAD,
    GET,
    POST,
    PUT,
    // additinal
    DELETE,
    LINK,
    UNLINK,
    Extension(String)
}

pub mod status {
    #[derive(Debug,PartialEq,Eq,Copy,Clone)]
    pub struct HttpStatus(pub isize,pub &'static str);
    pub const Ok:HttpStatus = HttpStatus(200,"Ok");
    pub const Created:HttpStatus = HttpStatus(201,"Created");
    pub const Accepted:HttpStatus = HttpStatus(202,"Accepted");
    pub const NoContent:HttpStatus = HttpStatus(204,"No Content");
    pub const MovePermanently:HttpStatus = HttpStatus(301,"Moved Permanently");
    pub const MoveTemporarily:HttpStatus = HttpStatus(302,"Moved Temporarily");
    pub const NotModified:HttpStatus = HttpStatus(304,"Not Modified");
    pub const BadRequest:HttpStatus = HttpStatus(400,"Bad Request");
    pub const Unauthorized:HttpStatus = HttpStatus(401,"Unauthorized");
    pub const Forbidden:HttpStatus = HttpStatus(403,"Forbidden");
    pub const NotFound:HttpStatus = HttpStatus(404,"Not Found");
    pub const InternalServerError:HttpStatus = HttpStatus(500,"Internal Server Error");
    pub const NotImplemented:HttpStatus = HttpStatus(501,"Not Implemented");
    pub const BadGateway:HttpStatus = HttpStatus(502,"Bad Gateway");
    pub const ServiceUnavailable:HttpStatus = HttpStatus(503,"Service Unavailable");
}

pub use response::ToResponse;
