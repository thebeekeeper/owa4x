use owa4x::io::DigitalPin;
use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    println!("Starting i/o subsystem");
    let owa = Owa4x::new();
    owa.init()?;

    owa.io.set_digital(DigitalPin::Pin9, false);
    std::thread::sleep(std::time::Duration::from_millis(10));
    owa.io.set_digital(DigitalPin::Pin9, true);

    Ok(())
}
