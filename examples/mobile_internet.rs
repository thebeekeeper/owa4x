#[macro_use]
extern crate env_logger;

use owa4x::inet::{Inet, InetConfig};

fn main() {
    println!("Starting RTU/IO");
    let mut e = true;
    if let Ok(_) = owa4x::start_rtu() {
        if let Ok(_) = owa4x::start_io() {
            let r =  owa4x::gprs::Gprs::initialize();
            println!("gprs init result: {:?}", r);
            println!("Trying to connect to the internet with a sim card");
            let cfg = InetConfig {
                username: "".to_string(),
                password: "".to_string(),
                dns_1: "8.8.8.8".to_string(),
                dns_2: "8.8.4.4".to_string(),
                ap_name: "hologram".to_string(),
            };
            let inet = Inet::new();
            let r = inet.initialize(cfg);
            println!("Result: {:?}", r);
        }
    }
}
