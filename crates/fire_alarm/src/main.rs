use std::{net::TcpStream, io::Write};

use gauge::{Gauge, GaugeState, IsUpdated};

#[derive(Debug)]
struct FireAlarm {
    name: String,
    updated: bool,
    state: GaugeState
}

impl FireAlarm {
    pub fn new<T: ToString>(name: T) -> Self {
        FireAlarm { 
            name: name.to_string(),
            state: GaugeState::Disabled,
            updated: false
        }
    }

}

impl Gauge for FireAlarm {

    fn name(&self) -> &str {
        &self.name
    }

    fn state(&self) -> &GaugeState {
        &self.state
    }

    fn set_state(&mut self, state: GaugeState) {
        self.state = state;
        self.updated = true;
    }

    fn is_updated(&self) -> IsUpdated {
        self.updated
    }

    fn set_is_updated(&mut self, is_updated: IsUpdated) {
        self.updated = is_updated
    }
}

fn main() {
    let name = std::env::var("GAUGE_NAME")
        .expect("GAUGE_NAME variable not defined!");
    let server_adress= std::env::var("SERVER_ADRESS")
        .expect("SERVER_ADRESS variable not defined!");

    let mut gauge: FireAlarm = FireAlarm::new(&name);

    println!("Created fire alarm \"{}\"",&name);

    loop {
        // Управление датчиком
        println!("P = enable/disable, F = trigger");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim(){
            "P" | "p" => {
                match gauge.state {
                    GaugeState::Disabled => {
                        gauge.set_state(GaugeState::Enabled);
                        println!("Gauge enabled");
                    },
                    GaugeState::Enabled => {                      
                        gauge.set_state(GaugeState::Disabled);
                        println!("Gauge disabled");
                    },
                    GaugeState::Message(_) => {
                        gauge.set_state(GaugeState::Disabled);
                        println!("Gauge disabled");
                    }
                }
            },
            "F" | "f" => {
                match gauge.state {
                    GaugeState::Disabled => {
                        println!("Gauge disabled")
                    },
                    GaugeState::Enabled => {                      
                        gauge.set_state(
                            GaugeState::Message("Deteced fire!".to_string())
                        );
                        println!("Gague catched fire");
                    },
                    _ => ()
                }
            }
            _ => println!("Invalid input")
        } // Управление датчиком

        // Обработка состояний дадчика
        if gauge.fetch_update() {
            let mut stream = TcpStream::connect(server_adress.clone())
                .expect(format!("Cant connect to listener on {}",server_adress).as_str());

            let request = stream.write_all(&gauge.serialize());

            match request {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}",e)
                },
            }
        }// Обработка состояний дадчика
    }
}
