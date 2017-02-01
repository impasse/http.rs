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

// #[derive(Debug,Eq)]
// pub enum Status {
    // OK(200,"OK"),
    // Created(201,"Created"),
    // Accepted(202,"Accepted"),
    // NoContent(204,"No Content"),
    // MovePermanently(301,"Moved Permanently"),
    // MoveTemporarily(302,"Moved Temporarily"),
    // NotModified(304,"Not Modified"),
    // BadRequest(400,"Bad Request"),
    // Unauthorized(401,"Unauthorized"),
    // Forbidden(403,"Forbidden"),
    // NotFound(404,"Not Found"),
    // InternalServerError(500,"Internal Server Error"),
    // NotImplemented(501,"Not Implemented"),
    // BadGateway(502,"Bad Gateway"),
    // ServiceUnavailable(503,"Service Unavailable")
// }



pub mod status {
    #[derive(Debug,PartialEq,Eq,Copy,Clone)]
    pub struct HttpStatus(pub isize,pub &'static str);
    pub static Ok:HttpStatus = HttpStatus(200,"Ok");
    pub static Created:HttpStatus = HttpStatus(201,"Created");
    pub static Accepted:HttpStatus = HttpStatus(202,"Accepted");
    pub static NoContent:HttpStatus = HttpStatus(204,"No Content");
    pub static MovePermanently:HttpStatus = HttpStatus(301,"Moved Permanently");
    pub static MoveTemporarily:HttpStatus = HttpStatus(302,"Moved Temporarily");
    pub static NotModified:HttpStatus = HttpStatus(304,"Not Modified");
    pub static BadRequest:HttpStatus = HttpStatus(400,"Bad Request");
    pub static Unauthorized:HttpStatus = HttpStatus(401,"Unauthorized");
    pub static Forbidden:HttpStatus = HttpStatus(403,"Forbidden");
    pub static NotFound:HttpStatus = HttpStatus(404,"Not Found");
    pub static InternalServerError:HttpStatus = HttpStatus(500,"Internal Server Error");
    pub static NotImplemented:HttpStatus = HttpStatus(501,"Not Implemented");
    pub static BadGateway:HttpStatus = HttpStatus(502,"Bad Gateway");
    pub static ServiceUnavailable:HttpStatus = HttpStatus(503,"Service Unavailable");
}

pub use response::ToResponse;
