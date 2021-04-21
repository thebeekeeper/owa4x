use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    let owa = Owa4x::new();
    println!("Initializing hardware");
    owa.init()?;
    println!("Initializing radio");
    let gprs = owa4x::gprs::Gprs::initialize();
    if let Ok(gsm) = gprs {
        println!("checking signal strength");
        loop {
            gsm.get_signal_strength(); 
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    } else {
        println!("Failed to start radio");
    }
        
    Ok(())
}
