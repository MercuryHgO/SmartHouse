use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};

use gauge::DeserializedGauge;

fn read_gauge(gauge: DeserializedGauge) {
    match gauge.state() {
        gauge::GaugeState::Disabled => {
            println!("Gauge \"{}\" disabled",gauge.name());
        },
        gauge::GaugeState::Enabled => {
            println!("Gauge \"{}\" enabled",gauge.name());
        },
        gauge::GaugeState::Message(m) => {
            println!("Gauge \"{}\" sent message: {}", gauge.name(),m)
        },
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    while match stream.read(&mut buffer) {
        Ok(size) if size > 0=> {
            let recieved_gauge = gauge::deserialize(&buffer);

            match recieved_gauge {
                Ok(gauge) => {read_gauge(gauge)},
                Err(e) => {
                    eprintln!("Error decoding gauge: {e}")
                },
            }

            true
        },
        _ => false,
    } {}
}

fn main() {
    let port = std::env::var("PORT")
        .expect("PORT env not specified!");
    let adress= std::env::var("ADRESS")
        .expect("ADRESS env not specified!");

    let listener = TcpListener::bind( format!("{adress}:{port}") )
        .expect("Error creating Tcp Listener");

    println!("Created listener on {adress} adress {port} port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) => eprintln!("Failed to accept connection: {e}"),
        }
    }
}
