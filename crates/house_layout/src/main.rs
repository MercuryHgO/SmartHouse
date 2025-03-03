use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};

use gauge::house_layout::house::House;
use http::{HttpRequest, HttpRequestBuilder, HttpResponceBuilder};


struct AppState {
    house: House,
}

fn handle_stream(mut stream: TcpStream) {

    let request: Result<HttpRequest,_> = (&stream)
        .try_into()
    ;

    match request {
        Ok(_) => {
            let responce =
                HttpResponceBuilder::default()
                    .content(&"It works!")
                    .build();
            stream.write_all(&responce.serialize().as_bytes()).unwrap();
        },
        Err(e) => {
            let responce =
                HttpResponceBuilder::default()
                    .status(http::Status::BadRequest)
                    .content(&format!("{:?}",e))
                    .build();
            stream.write_all(&responce.serialize().as_bytes()).unwrap();
        },
    }

}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:9000")
        .expect("Error binding TCP stream");
   
    for stream in listener.incoming() {
        handle_stream(stream.unwrap());
    };
}
