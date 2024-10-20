use std::{time::{Duration, Instant}, thread, io::{Write,Read, self}, sync::{Arc, Mutex}, net::TcpStream};


use gauge::{GaugeState, Gauge, IsUpdated};
const INTERVAL: Duration = Duration::from_secs(5);

type Temperature = f32;

struct TemperatureGauge {
    name: String,
    temperature: Temperature,
    state: GaugeState,
    updated: IsUpdated
}

impl TemperatureGauge {
    pub fn new<T: ToString>(gauge_name: T, temperature: Temperature) -> Self {
        TemperatureGauge { 
            name: gauge_name.to_string(),
            temperature,
            state: GaugeState::Disabled,
            updated: false
        }
    }

    pub fn temperature(&self) -> Temperature {
        self.temperature
    }

    pub fn set_temperature(&mut self, temperature: Temperature) {
        self.temperature = (temperature * 10.0).round() / 10.0;
        self.updated = true;
    }

}

impl Gauge for TemperatureGauge {
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
    let temperature_gauge = 
        Arc::new(
            Mutex::new(
                TemperatureGauge::new("Living room temperature", 36.0)
            )
        );
    let gauge_clone = Arc::clone(&temperature_gauge);

    thread::spawn(move || {
        let mut next_time = Instant::now() + INTERVAL;


        loop {
            let now = Instant::now();

            if next_time <= now {
                let mut gauge = gauge_clone.lock().expect("Error locking mutex");

                match gauge.state {
                    GaugeState::Enabled | GaugeState::Message(_) => {
                        let current_temperature = gauge.temperature();

                        gauge.set_state(
                            GaugeState::Message(format!("Temperature updated: {} °C",current_temperature))
                        );

                        send_gauge(&mut gauge);

                        gauge.set_state(GaugeState::Enabled);
                    }
                    _ => ()
                }

                next_time += INTERVAL;
            }


            thread::sleep(Duration::from_millis(100));
        }
    });

    loop {
        let mut input = String::new();

        println!("P - enable/disable, K/J - degrees up/down by 1.0, k/j - degrees up/down by 0.1");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");


        let mut gauge = temperature_gauge.lock().expect("Error locking mutex");
        match input.trim() {
            "P" | "p" => {
                match gauge.state() {
                    GaugeState::Disabled => {
                        gauge.set_state(GaugeState::Enabled);
                        println!("Gauge enabled");
                        send_gauge(&mut gauge);
                    },
                    GaugeState::Enabled => {
                        gauge.set_state(GaugeState::Disabled);
                        println!("Gauge disabled");
                        send_gauge(&mut gauge);
                    },
                    _ => ()
                }
            },
             "k"|"j"|"K"|"J" => {
                match gauge.state() {
                    GaugeState::Disabled => {
                        println!("Gauge disabled");
                    },
                    _ => {
                        let current_temperature = gauge.temperature();

                        match input.trim() {
                            "k" => { 
                                gauge.set_temperature(current_temperature + 0.1);
                            }
                            "j" => { 
                                gauge.set_temperature(current_temperature - 0.1);
                            }
                            "K" => { 
                                gauge.set_temperature(current_temperature + 1.0);
                            }
                            "J" => { 
                                gauge.set_temperature(current_temperature - 1.0);
                            }
                             _ => ()
                        }

                        println!("Set temperature: {}",gauge.temperature());
                    },
                }
             }
            _ => println!("Invalid input")
        };

    }
}

fn send_gauge(gauge: &mut TemperatureGauge) {

    let result = || -> Result<(),Box<dyn std::error::Error>> {
        if gauge.fetch_update() {
            let address = std::env::var("SERVER_ADRESS")
                .map_err(|_e| "SERVER_ADRESS var not specified")?;
            TcpStream::connect(address)?
                .write(&gauge.serialize())?;
        }

        Ok(())
    }();

    match result {
        Ok(_) => {
            println!(">> Successfuly sent data to server")
        },
        Err(e) => {
            eprintln!("Error during proccesing gauge: {}",e)
        },
    }
    

}
