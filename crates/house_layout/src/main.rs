use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::sync::{Arc, RwLock};

use gauge::house_layout::house::House;
use http::HttpRequest;

pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type Error = http::Error;
pub(crate) type App = Arc<RwLock<AppState>>;

mod routes;

struct AppState {
    house: House,
}


fn handle_stream(mut stream: TcpStream, app: App) {

    let request: http::Result<HttpRequest> = (&stream)
        .try_into()
    ;

    let responce = match request {
        Ok(r) => {
            match routes::route(r, app) {
                Ok(r) => r,
                Err(e) => routes::error_handler(e),
            }
        },
        Err(e) => routes::error_handler(e),
    };

    stream.write_all(&responce.serialize().as_bytes()).expect("Error writing stream");
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000")
        .expect("Error binding TCP stream");

    let app = Arc::new(RwLock::new(
        AppState {
            house: House::default()
        }
    ));
   
    for stream in listener.incoming() {
        let app = Arc::clone(&app);
        handle_stream(stream.unwrap(),app);
    };
}
