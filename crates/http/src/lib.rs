// TODO: no .clone()
use std::{collections::HashMap, fmt::Display, io::{BufRead, BufReader, Lines}, net::TcpStream, ops::Not};

pub type Result<T> = std::result::Result<T,crate::Error>;

const HTTP_VERSION: &str = "HTTP/1.1";
const SEPARATOR: &str = "\r\n";

#[derive(Debug,Clone,Copy)]
pub enum Status {
    OK,
    Created,
    InternalServerError,
    BadRequest,
    NotFound,
    MethodNotAllowed
}

impl TryFrom<&str> for Status {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "200" => Ok(Status::OK),
            "201" => Ok(Status::Created),
            "500" => Ok(Status::InternalServerError),
            "400" => Ok(Status::BadRequest),
            "404" => Ok(Status::NotFound),
            "405" => Ok(Status::MethodNotAllowed),
            _ => Err(Error::UnknownStatus)
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Status::OK => "200 OK",
            Status::Created => "201 Created",
            Status::InternalServerError => "500 Internal Server Error",
            Status::BadRequest => "400 Bad Request",
            Status::NotFound => "404 Not Found",
            Status::MethodNotAllowed => "405 Method Not Allowed",
        };

        write!(f,"{message}")
    }
}

pub type Headers = HashMap<String,String>;
pub type Content = Option<String>;

#[derive(Debug)]
pub struct HttpResponceBuilder {
    status: Status,
    headers: Headers,
    content: Content
}

impl Default for HttpResponceBuilder {
    fn default() -> Self {
        Self { status: Status::OK, headers: HashMap::new(), content: None }
    }
}

impl HttpResponceBuilder {
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn content(mut self, content: &dyn ToString) -> Self {
        let content = content.to_string();
        self.headers.insert("Content-Length".to_string(), content.len().to_string());
        self.content = Some(content);
        self
    }

    pub fn build(self) -> HttpResponce {
        HttpResponce { status: self.status, headers: self.headers, content: self.content }
    }
}

#[derive(Debug)]
pub struct HttpResponce {
    status: Status,
    headers: Headers,
    content: Content
}

impl HttpResponce {
    fn ok() -> Self {
        HttpResponce { status: Status::OK, headers: HashMap::new(), content: None }
    }

    fn with_code(status: Status) -> Self {
        HttpResponce { status, headers: HashMap::new(), content: None }
    }

    fn parse(responce: String) -> Result<Self> {
        let splited: Vec<&str> = responce.lines().collect();
        let separator_line = splited
            .iter()
            .position(|e| e.is_empty())
            .ok_or(Error::MalformedResponce)?
        ;

        let mut status_line = splited[0].split(" ");

        status_line.nth(0)
            .ok_or(Error::MalformedResponce)?
            .eq(HTTP_VERSION)
            .then_some(())
            .ok_or(Error::UnsopportedProtocol)?
        ;

        let status: Status = status_line
            .nth(0)
            .ok_or(Error::MalformedStatusLine)?
            .try_into()?
        ;

        let mut headers: Headers = Headers::new();
        for header in splited[1..separator_line].to_vec() {
            let mut split = header.split(": ");
            let (k,v) =
                (
                    split.nth(0).ok_or(Error::MalformedHeader)?,
                    split.nth(0).ok_or(Error::MalformedHeader)?,
                );
            headers.insert(k.into(), v.into());
        };

        let content: Option<String> = 
            (splited.len() > separator_line + 1)
            .then(|| 
                splited[separator_line+1..].join("")
            )
        ;

        Ok(Self { status, headers, content})
    }

    pub fn serialize(self) -> String {
        let stauts_line = format!("HTTP/1.1 {}",self.status);
        let headers = self.headers.into_iter()
            .map(|(k,v)| [SEPARATOR,[k,v].join(": ").as_str()].join(""))
            .collect::<Vec<String>>()
            .join("")
        ;

        let content = self.content.unwrap_or("".to_string());

        format!("{}{}\r\n\r\n{}",
            stauts_line,
            headers,
            content
        )
    }

    const fn headers(&self) -> &Headers { &self.headers }
    const fn content(&self) -> &Content { &self.content }

}

type RequestTarget = String;

#[derive(Debug, Clone, Copy)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",
            match self {
                Method::GET => "GET",
                Method::HEAD => "HEAD",
                Method::POST => "POST",
                Method::PUT => "PUT",
                Method::DELETE => "DELETE",
                Method::CONNECT => "CONNECT",
                Method::OPTIONS => "OPTIONS",
                Method::TRACE => "TRACE",
                Method::PATCH => "PATCH",
            }
        )
    }
}

impl TryFrom<&str> for Method {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self> {
        let method = match value {
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "CONNECT" => Method::CONNECT,
            "OPTIONS" => Method::OPTIONS,
            "TRACE" => Method::TRACE,
            "PATCH" => Method::PATCH,
            _ => Err(Error::UnknownRequestMethod)?
        };

        Ok(method)
    }
}

pub struct HttpRequestBuilder {
    method: Method,
    path: RequestTarget,
    headers: Headers,
    content: Content    
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        Self { method: Method::GET, path: "/".to_string(), headers: Headers::new(), content: None }
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn path(mut self, path: &dyn ToString) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn content(mut self, content: &dyn ToString) -> Self {
        let content = content.to_string();
        self.headers.insert("Content-Length".to_string(), content.len().to_string());
        self.content = Some(content);
        self
    }

    pub fn build(self) -> HttpRequest {
        HttpRequest { method: self.method, path: self.path, headers: self.headers, content: self.content }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    method: Method,
    path: RequestTarget,
    headers: Headers,
    content: Content    
}

impl HttpRequest {
    /// Преобразует строку в [`HttpRequest`].
    /// Поле `content` будет `None`, содержание запроса необходимо обработать самостоятельно
    pub fn parse(request: Vec<String>) -> Result<Self> {
        let mut request = request.into_iter();

        let binding = request
            .next()
            .ok_or(Error::MalformedResponce)?;
        let mut status_line = binding
            .split(" ");

        let method: Method = status_line.nth(0)
            .ok_or(Error::MalformedRequest)?
            .try_into()?
        ;

        let path = status_line.nth(0)
            .ok_or(Error::MalformedRequest)?
            .to_string()
        ;

        status_line.nth(0)
            .ok_or(Error::MalformedRequest)?
            .eq(HTTP_VERSION)
            .then_some(())
            .ok_or(Error::UnsopportedProtocol)?
        ;

        let mut headers: Headers = Headers::new();
        for header in request.take_while(|l| !l.is_empty()) {
            let mut split = header.split(": ");
            let (k,v) =
                (
                    split.nth(0).ok_or(Error::MalformedHeader)?,
                    split.nth(0).ok_or(Error::MalformedHeader)?,
                );
            headers.insert(k.into(), v.into());
        };

        Ok(Self { method, path, headers, content: None})
    }

    pub fn serialize(self) -> String {
        let stauts_line = format!("{} {} {}",self.method, self.path,HTTP_VERSION);
        let headers  = self.headers.into_iter()
            .map(|(k,v)| [SEPARATOR,[k,v].join(": ").as_str()].join(""))
            .collect::<Vec<String>>()
            .join("")
        ;

        let content = self.content.unwrap_or("".to_string());

        format!("{}{}\r\n\r\n{}",
            stauts_line,
            headers,
            content
        )
    }

    pub const fn method(&self) -> &Method { &self.method }
    pub const fn path(&self) -> &RequestTarget { &self.path }
    pub const fn headers(&self) -> &Headers { &self.headers }
    pub const fn content(&self) -> &Content { &self.content }
}

impl TryFrom<&TcpStream> for HttpRequest {
    type Error = crate::Error;

    fn try_from(value: &TcpStream) -> std::result::Result<Self, Self::Error> {
        let mut buf_reader = BufReader::new(value);

        let mut request: Vec<String> = buf_reader
            .lines()
            .map(|s| 
                s.map_err(|_| Error::MalformedRequest)
            ) // Fix this
            .take_while(|line|
                line.as_ref().is_ok_and(|s| !s.is_empty())
            )
            .collect::<Result<Vec<String>>>()?
        ;

        Ok(Self::parse(request)?)
    }

}

#[derive(Debug )]
pub enum Error {
    MalformedResponce,
    MalformedStatusLine,
    UnknownStatus,

    MalformedRequest,
    UnknownRequestMethod,

    UnsopportedProtocol,
    MalformedHeader,

    ResponceError(HttpResponce),
    InternalServerError(Box<dyn std::error::Error>)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Error::MalformedResponce => "Error during parsing repsonce",
            Error::MalformedStatusLine => "Error during parsing status code",
            Error::UnknownStatus => "Unknown status",
            Error::MalformedHeader => "Error during parsing header",
            Error::MalformedRequest => "Error during parsing request",
            Error::UnknownRequestMethod => "Unknown request method",
            Error::UnsopportedProtocol => "Unsopported HTTP protocol",
            Error::InternalServerError(e) => {
                let message = format!("Internal server error: {}",e);

                Box::leak(message.into_boxed_str())
            },
            Error::ResponceError(_) => "Error made into responce",
        };

        write!(f,"{message}")
    }
}

impl std::error::Error for Error { }

#[cfg(test)]
mod tests;
