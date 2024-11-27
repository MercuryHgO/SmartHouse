use std::net::{TcpListener, TcpStream};
use std::io::Read;

use gauge::helpers::read_gauge_by_id;
use gauge::types::{SerializedGauge, DeserializedGauge};

fn read_gauge(gauge: DeserializedGauge) {
    match read_gauge_by_id(gauge) {
        Ok(reader_gauge) => {
            match reader_gauge {
                gauge::helpers::Gauges::FireAlarm(fire_alarm) => {
                    println!("{}",fire_alarm);
                },
                gauge::helpers::Gauges::Unknown(deserialized_gauge) => {
                    println!("Parsed gauge with unknown id, trying to read...");
                    println!("{}",deserialized_gauge);
                },
                gauge::helpers::Gauges::TemperatureGauge(temperature_gauge) => {
                    println!("{temperature_gauge}")
                },
            };
        },
        Err(e) => {
            eprintln!("Error reading gauge id: {e}")
        },
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    while match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            let recieved_gauge = SerializedGauge::from(buffer.to_vec());
            let deserialized_gauge = DeserializedGauge::parse(recieved_gauge);
            
            match deserialized_gauge {
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
