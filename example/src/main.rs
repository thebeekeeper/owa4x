use owa4x::gps::Gps;
use owa4x::leds::{Leds, PanelLed};
use owa4x::io::{Io, DigitalPin, AnalogPin};
use owa4x::inet::{Inet, InetConfig};
use std::{thread, time};

fn main() {
    println!("owa4x sdk test program");

    println!("Starting RTU");
    if let Ok(_) = owa4x::start_rtu() {
        println!("Starting IO");
        if let Ok(_) = owa4x::start_io() {
            //run_gps_test();
            let io = Io { };
            let p = io.read_digital(DigitalPin::Pin2);
            println!("Pin 2 state: {}", p);
            let analog = io.read_analog(AnalogPin::Ain3);
            println!("Analog input 3 value: {}", analog);
            connect_gprs();
        }
        else {
            println!("Error starting IO");
        }
    }
    else {
        println!("Error starting RTU");
    }
}

fn run_gps_test() {
    let gps = Gps::new();
    let leds = Leds::new();
    println!("Initializing GPS");
    gps.initialize();
    let mut led_status = false;
    loop {
        leds.set_state(PanelLed::Green, led_status);
        led_status = !led_status;
        if let Some(pos) = gps.get_position() {
            println!("GPS position: {:?}", pos);
        }
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn connect_gprs() { 
    if let Ok(()) = owa4x::gprs::Gprs::initialize() {
    let inet = Inet::new();
    let cfg = InetConfig {
        username: "".to_string(),
        password: "".to_string(),
        dns_1: String::from("8.8.8.8"),
        dns_2: String::from("8.8.4.4"),
        ap_name: "hologram".to_string(),
    };
    inet.initialize(cfg);
    }
}
