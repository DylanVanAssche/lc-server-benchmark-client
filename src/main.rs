extern crate sse_client;
extern crate clap;
extern crate isahc;

use std::time::Duration;
use std::thread;
use sse_client::EventSource;
use clap::{Arg, App};

const REAL_TIME_PUSHING_URL: &str = "http://lc.dylanvanassche.be/sncb/events/sse";
const REAL_TIME_POLLING_URL: &str = "http://lc.dylanvanassche.be/sncb/events";

fn main() {
    let matches = App::new("lc-server-benchmark-client")
                    .version("0.0.1")
                    .author("Dylan Van Assche")
                    .about("Client to benchmark the real time events handling of the Linked Connections Server")
                    .arg(Arg::with_name("mode")
                    .short("m")
                    .long("mode")
                    .takes_value(true)
                    .help("The operation mode of the client: 'pushing' or 'polling' mode. Other values are rejected."))
                    .arg(Arg::with_name("interval")
                    .short("i")
                    .long("interval")
                    .takes_value(true)
                    .help("The time between 2 polling requests in milliseconds (ms)"))
                    .get_matches();

    // Configure client
    let mode = matches.value_of("mode").unwrap_or("unknown");
    match mode {
        "pushing" => {
            println!("Pushing mode");
            handle_pushing()
        },
        "polling" => { 
            println!("Polling mode");
            let interval = matches.value_of("interval").unwrap_or("");
            match interval.parse::<u64>() {
                Ok(i) => {
                    println!("Polling interval {} ms", i);
                    handle_polling(i);
                },
                Err(_) => panic!("Unknown interval")
            }
        },
        _ => panic!("Unknown operation mode!")
    }
}

fn handle_pushing() {
    let event_source = EventSource::new(REAL_TIME_PUSHING_URL).unwrap();

    for _event in event_source.receiver().iter() {
        println!("Received pushing answer");
        //println!("{}", event.data)
    }
}

fn handle_polling(interval: u64) {
    let client = isahc::HttpClient::builder().redirect_policy(isahc::config::RedirectPolicy::Follow).build().unwrap();
    loop {
        let _response = client.get(REAL_TIME_POLLING_URL).unwrap();
        println!("Received polling answer");
        //println!("{}", response.text().unwrap());
        thread::sleep(Duration::from_millis(interval))
    }
}
