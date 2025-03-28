use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    let owa = Owa4x::new();
    owa.init()?;

    let err = OwaError::FmsAlreadyInitialized;
    println!("error display: {}", err);

    let s = owa.get_serial_number()?;
    println!("Serial Number: {}", s);

    let v_in = owa.power.get_power_voltage()?;
    println!("Power supply voltage: {:?}", v_in);

    let v_batt = owa.power.get_battery_voltage()?;
    println!("Battery voltage: {:?}", v_batt);

    let charge = owa.power.get_battery_state()?;
    println!("Battery charge state: {:?}", charge);

    let pwr_source = owa.power.get_power_source()?;
    println!("Power source: {:?}", pwr_source);
    Ok(())
}
