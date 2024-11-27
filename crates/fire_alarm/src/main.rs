use std::{net::TcpStream, io::Write};

use gauge::types::{fire_alarm::{FireAlarm, FireAlarmState}, Gauge};

// #[derive(Debug)]
// struct FireAlarm {
//     name: String,
//     updated: bool,
//     state: GaugeState
// }

// impl FireAlarm {
//     pub fn new<T: ToString>(name: T) -> Self {
//         FireAlarm { 
//             name: name.to_string(),
//             state: GaugeState::Disabled,
//             updated: false
//         }
//     }

// }

// impl Gauge for FireAlarm {

//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn state(&self) -> &GaugeState {
//         &self.state
//     }

//     fn set_state(&mut self, state: GaugeState) {
//         self.state = state;
//         self.updated = true;
//     }

//     fn is_updated(&self) -> IsUpdated {
//         self.updated
//     }

//     fn set_is_updated(&mut self, is_updated: IsUpdated) {
//         self.updated = is_updated
//     }
// }

fn main() {
    let name = std::env::var("GAUGE_NAME")
        .expect("GAUGE_NAME variable not defined!");
    let server_adress= std::env::var("SERVER_ADRESS")
        .expect("SERVER_ADRESS variable not defined!");

    let mut gauge: FireAlarm = FireAlarm::new(name,FireAlarmState::Disabled);
    let mut updated = false;

    loop {
        // Управление датчиком
        println!("P = enable/disable, F = trigger");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim(){
            "P" | "p" => {
                match gauge.state() {
                    FireAlarmState::Disabled => {
                        gauge.set_state(FireAlarmState::Enabled);
                        updated = true;
                        println!("Gauge enabled");
                    },
                    FireAlarmState::Enabled => {                      
                        gauge.set_state(FireAlarmState::Disabled);
                        updated = true;
                        println!("Gauge disabled");
                    },
                    FireAlarmState::OnAlert => {
                        gauge.set_state(FireAlarmState::Disabled);
                        updated = true;
                        println!("Gauge disabled");
                    }
                }
            },
            "F" | "f" => {
                match gauge.state() {
                    FireAlarmState::Disabled => {
                        println!("Gauge disabled")
                    },
                    FireAlarmState::Enabled => {                      
                        gauge.set_state(FireAlarmState::OnAlert);
                        updated = true;
                        println!("Gague catched fire");
                    },
                    _ => ()
                }
            }
            _ => println!("Invalid input")
        } // Управление датчиком

        // Обработка состояний дадчика
        if updated {
            let mut stream = TcpStream::connect(server_adress.clone())
                .expect(format!("Cant connect to listener on {}",server_adress).as_str());

            let request = stream.write_all(&gauge.serialize());

            match request {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}",e)
                },
            };

            updated = false;
        }// Обработка состояний дадчика
    }
}
