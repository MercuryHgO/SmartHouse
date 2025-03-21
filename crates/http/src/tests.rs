use crate::{HttpRequest, HttpRequestBuilder, HttpResponce, HttpResponceBuilder};

impl PartialEq for HttpResponce {
    fn eq(&self, other: &Self) -> bool {
        self.status as usize == other.status as usize &&
        self.headers == other.headers &&
        self.content == other.content
    }
}

impl PartialEq for HttpRequest {
    fn eq(&self, other: &Self) -> bool {
        self.method as usize == other.method as usize &&
        self.path == other.path &&
        self.headers == other.headers &&
        self.content == other.content
    }
}

#[test]
fn responce_serialize() {
    let resp = HttpResponceBuilder::default()
        .content(&"Boba boba aboba")
        .build()
        .serialize();

    let expected = "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\nBoba boba aboba".to_string();
    
    assert_eq!(resp,expected);
}

#[test]
fn responce_parse() {
    let resp = "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\nBoba boba aboba".to_string();
    
    let expected = HttpResponceBuilder::default()
        .content(&"Boba boba aboba")
        .build();

    let parsed = HttpResponce::parse(resp).unwrap();

    assert_eq!(expected,parsed);
}

#[test]
fn responce_must_be_invalid() {
    let resp = "HTTP 200 OK\r\nContent-Length: 15\r\n\r\nBoba boba aboba".to_string();
    assert!(HttpResponce::parse(resp).is_err());

    let resp = "200 OK\r\nContent-Length: 15\r\n\r\nBoba boba aboba".to_string();
    assert!(HttpResponce::parse(resp).is_err());

    let resp = "HTTP/1.1 200 OK\r\nContent-Length: 15\r\nBoba boba aboba".to_string();
    assert!(HttpResponce::parse(resp).is_err());

    let resp = "HTTP/1.1 200 OK\r\nContent-Length 15\r\n\r\nBoba boba aboba".to_string();
    assert!(HttpResponce::parse(resp).is_err());
}

#[test]
fn request_serialize() {
    let resp = HttpRequestBuilder::new()
        .build()
        .serialize();

    let expected = "GET / HTTP/1.1\r\n\r\n".to_string();
    
    assert_eq!(resp,expected);

    let resp = HttpRequestBuilder::new()
        .method(crate::Method::POST)
        .content(&"Aboba")
        .path(&"/aboba")
        .build()
        .serialize();

    let expected = "POST /aboba HTTP/1.1\r\nContent-Length: 5\r\n\r\nAboba".to_string();
    
    assert_eq!(resp,expected);
}

#[test]
fn request_parse() {
    let resp = "GET / HTTP/1.1\r\n\r\n".to_string();
    let resp = HttpRequest::parse(
        resp
            .lines()
            .map(|s| s
            .to_string())
            .collect()
    ).unwrap();

    let expected = HttpRequestBuilder::new().build();

    assert_eq!(resp,expected);

    let resp = "POST /aboba HTTP/1.1\r\nContent-Length: 5\r\n\r\n".to_string();
    let resp = HttpRequest::parse(
        resp
        .lines()
        .map(|s| s.to_string())
        .collect()
    ).unwrap();

    let expected = HttpRequestBuilder::new()
        .method(crate::Method::POST)
        .header("Content-Length".to_string(), "5".to_string())
        .path(&"/aboba")
        .build()
    ;
    
    assert_eq!(resp,expected);
}
