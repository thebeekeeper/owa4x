use owa4x::gps::Gps;
use owa4x::{OwaError, Owa4x};
use std::{thread, time};

fn main() -> Result<(), OwaError> {
    let owa = Owa4x::new();
    owa.init()?;
    println!("Initializing GPS");
    owa.gps.initialize();
    let sats = owa.gps.get_satellites();
    println!("Satellites: {:?}", sats);
    loop {
        if let Some(pos) = owa.gps.get_position() {
            println!("GPS position: {:?}", pos);
        }
        thread::sleep(time::Duration::from_millis(1000));
    }
}
