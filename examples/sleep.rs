use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    println!("Starting i/o subsystem");
    let owa = Owa4x::new();
    owa.init()?;

    owa.take_a_nap();

    Ok(())
}
