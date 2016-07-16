use hyper::net::Fresh;
use request::Request;
use response::Response;

pub trait Middleware: Send + Sync + 'static {
    fn execute(&self, req: Request) -> Result<Response, String>;
}

impl<T> Middleware for T where T: Send + Sync + 'static + Fn(Request) -> Result<Response, String> {
    fn execute(&self, req: Request) -> Result<Response, String> {
        (*self)(req)
    }
}
