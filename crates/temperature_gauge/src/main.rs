use std::{time::{Duration, Instant}, thread, io::{Write,Read, self}, sync::{Arc, Mutex}, net::TcpStream};

use gauge::types::{temperature_gauge::{TemperatureGauge, TemperatureGaugeState}, Gauge};

const INTERVAL: Duration = Duration::from_secs(5);

fn main() {
    let temperature = Arc::new(Mutex::new(36.6));
    let temperature_clone = Arc::clone(&temperature);

    let temperature_gauge = 
        Arc::new(
            Mutex::new(
                TemperatureGauge::new(
                    "Living room".to_string(), 
                    TemperatureGaugeState::Disabled
                )
            )
        );
    let gauge_clone = Arc::clone(&temperature_gauge);

    thread::spawn(move || {
        let mut next_time = Instant::now() + INTERVAL;

        loop {
            let now = Instant::now();

            if next_time <= now {
                let mut gauge = gauge_clone.lock().expect("Error locking mutex");

                match gauge.state() {
                    TemperatureGaugeState::Enabled | TemperatureGaugeState::ReadedTemperarure(_) => {
                        gauge.set_state(
                            TemperatureGaugeState::ReadedTemperarure(*temperature.lock().expect("Error locking mutex"))
                        );

                        send_gauge(&mut gauge);

                        gauge.set_state(TemperatureGaugeState::Enabled);
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
                    TemperatureGaugeState::Disabled => {
                        gauge.set_state(TemperatureGaugeState::Enabled);
                        println!("Gauge enabled");
                        send_gauge(&mut gauge);
                    },
                    TemperatureGaugeState::Enabled => {
                        gauge.set_state(TemperatureGaugeState::Disabled);
                        println!("Gauge disabled");
                        send_gauge(&mut gauge);
                    },
                    _ => ()
                }
            },
             "k"|"j"|"K"|"J" => {
                match gauge.state() {
                    TemperatureGaugeState::Disabled => {
                        println!("Gauge disabled");
                    },
                    _ => {
                        let mut temp = temperature_clone.lock().expect("Error locking mutex");
                        match input.trim() {
                            "k" => { 
                                *temp += 0.1;
                            }
                            "j" => { 
                                *temp -= 0.1;
                            }
                            "K" => { 
                                *temp += 1.0;
                            }
                            "J" => { 
                                *temp -= 1.0;
                            }
                             _ => ()
                        }

                        *temp = (*temp * 10.0).round() / 10.0;

                        println!("Set temperatupe: {}",*temp);
                    },
                }
             }
            _ => println!("Invalid input")
        };

    }
}

fn send_gauge(gauge: &mut TemperatureGauge) {

    let result = || -> Result<(),Box<dyn std::error::Error>> {
        let address = std::env::var("SERVER_ADRESS")
            .map_err(|_e| "SERVER_ADRESS var not specified")?;
        TcpStream::connect(address)?
            .write(&gauge.serialize())?;

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
