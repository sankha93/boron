/*
use std::io::{self, Write};
use std::any::Any;
use hyper::net::{Fresh, Streaming};
use hyper::server::Response as UnwrappedResponse;
*/

use std::convert::From;
use hyper::header::{Headers, ContentLength};
use hyper::status::StatusCode;

pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Option<Vec<u8>>
}

impl From<Vec<u8>> for Response {
    fn from(body: Vec<u8>) -> Response {
        let mut headers = Headers::new();
        headers.set(ContentLength(body.len() as u64));

        Response {
            status: StatusCode::Ok,
            headers: headers,
            body: Some(body)
        }
    }
}

/*
pub struct Response<'a, T: Any = Fresh> {
    res: UnwrappedResponse<'a, T>
}

impl<'a> Response<'a, Fresh> {
    pub fn wrap_response<'b>(res: UnwrappedResponse<'b, Fresh>) -> Response<'b, Fresh> {
        Response { res: res }
    }

    #[inline]
    pub fn status_mut(&mut self) -> &mut StatusCode {
        self.res.status_mut()
    }

    #[inline]
    pub fn headers_mut(&mut self) -> &mut Headers {
        self.res.headers_mut()
    }

    #[inline]
    pub fn send(self, body: &[u8]) -> io::Result<()> {
        self.res.send(body)
    }

    #[inline]
    pub fn start(self) -> io::Result<Response<'a, Streaming>> {
        match self.res.start() {
            Ok(res) => Ok(Response { res: res }),
            Err(e) => Err(e)
        }
    }
}

impl<'a> Response<'a, Streaming> {
    #[inline]
    pub fn end(self) -> io::Result<()> {
        self.res.end()
    }
}

impl<'a> Write for Response<'a, Streaming> {
    #[inline]
    fn write(&mut self, msg: &[u8]) -> io::Result<usize> {
        self.res.write(msg)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.res.flush()
    }
}
*/