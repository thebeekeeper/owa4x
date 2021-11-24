
use owa4x::io::AnalogPin;

use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    println!("Starting i/o subsystem");
    let owa = Owa4x::new();
    owa.init()?;
    loop {
        let analog = owa.io.read_analog(AnalogPin::Ain3);
        println!("Analog input 3 value: {}", analog);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
