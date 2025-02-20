use std::fmt::Display;

use gauge::house_layout::house::House;

enum Response {
    OK,
    BAD_REQUEST,
    NOT_FOUND,
    SERVER_ERROR
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let http_version = "HTTP/1.1";

        match self {
            Response::OK           => write!(f,"{} 200 OK\r\n\r\n", http_version),
            Response::BAD_REQUEST  => write!(f,"{} 400 Bad Request\r\n\r\n", http_version),
            Response::NOT_FOUND    => write!(f,"{} 404 Not found\r\n\r\n", http_version),
            Response::SERVER_ERROR => write!(f,"{} 500 Internal Server Error\r\n\r\n", http_version),
        }
    }
}

struct AppState {
    house: House,
}

fn main() {
    let mut app = AppState {
        house: House::default()
    };



    
}
