//use owa4x::leds::{Leds, PanelLed};
use owa4x::io::{AnalogPin, DigitalPin};

use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    println!("Starting i/o subsystem");
    let owa = Owa4x::new();
    owa.init()?;
    let p = owa.io.read_digital(DigitalPin::Pin0);
    println!("Pin 0 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin1);
    println!("Pin 1 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin2);
    println!("Pin 2 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin3);
    println!("Pin 3 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin4);
    println!("Pin 4 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin5);
    println!("Pin 5 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin6);
    println!("Pin 6 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin7);
    println!("Pin 7 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin8);
    println!("Pin 8 state: {}", p);
    let p = owa.io.read_digital(DigitalPin::Pin9);
    println!("Pin 9 state: {}", p);

    // high range scale -> 30.72 / 4095
    owa.io.set_analog_range(AnalogPin::Ain0, true);
    //let analog = (owa.io.read_analog(AnalogPin::Ain0) as f32) * 0.00750183150183150;
    let analog = owa.io.read_volts(AnalogPin::Ain0);
    println!("Analog input 0 value: {}", analog);
    let analog = owa.io.read_analog(AnalogPin::Ain1);
    println!("Analog input 1 value: {}", analog);
    let analog = owa.io.read_analog(AnalogPin::Ain2);
    println!("Analog input 2 value: {}", analog);
    let analog = owa.io.read_analog(AnalogPin::Ain3);
    println!("Analog input 3 value: {}", analog);

    Ok(())
}
