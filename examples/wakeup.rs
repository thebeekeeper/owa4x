
use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    let owa = Owa4x::new();
    owa.init()?;

    let wakeup_reason = owa.power.get_wakeup_reason();
    println!("Wakeup reason: {:?}", wakeup_reason);
    Ok(())
}
