use super::method::{Method, MethodError};
use super::query_string::QueryString;
use std::str::FromStr;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{Utf8Error, from_utf8};

#[derive(Debug)]
pub struct Request<'buf> {
    method: Method,
    path: &'buf str,
    query_params: Option<QueryString<'buf>>,
}

impl<'buf> Request<'buf> {
    pub fn new(method: Method, path: &'buf str, query_params: Option<QueryString<'buf>>) -> Self {
        Self {
            method,
            path,
            query_params,
        }
    }
}

impl<'buf> Request<'buf> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &'buf str {
        &self.path
    }

    pub fn query_params(&self) -> Option<&QueryString> {
        self.query_params.as_ref()
    }
}
/*    Message: GET / HTTP/1.1
    Host: localhost:8080
    User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:98.0) Gecko/20100101 Firefox/98.0
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,/;q=0.8
    Accept-Language: en-US,en;q=0.5
    Accept-Encoding: gzip, deflate
    Connection: keep-alive
    Upgrade-Insecure-Requests: 1
    Sec-Fetch-Dest: document
    Sec-Fetch-Mode: navigate
    Sec-Fetch-Site: none
    Sec-Fetch-User: ?1
*/

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;    

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
       let request: &str = from_utf8(buf)?;

       //match get_next_word(request) {
       //    Some((method, request)) => {},
       //    None => return Err(ParseError::InvalidRequest)
       //}

       let (method, request): (&str, &str) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
       let (mut path, request): (&str, &str) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
       let (protocol, _): (&str, &str) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

       if protocol != "HTTP/1.1" {
           return Err(ParseError::InvalidProtocol);
       }

       let method: Method = Method::from_str(method)?;

       let mut query_params: Option<QueryString<'buf>> = None;
       if let Some(i) = path.find("?") {
           query_params = Some(QueryString::from(&path[i + 1..]));
           path = &path[..i];
       }

       Ok(Self {
           method,
           path,
           query_params
       })
    }
}

fn get_next_word(text: &str) -> Option<(&str, &str)> {
    for (i, c) in text.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&text[..i], &text[i + 1 ..]))
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
           Self::InvalidRequest => "Invalid Request",
           Self::InvalidEncoding => "Invalid Encoding",
           Self::InvalidProtocol => "Invalid Protocol",
           Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError{}

