use owa4x::inet::{Inet, InetConfig};
use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    let owa = Owa4x::new();
    owa.init()?;
    if let Ok(()) = owa4x::gprs::Gprs::initialize() {
        let inet = Inet::new();
        let cfg = InetConfig {
            username: "".to_string(),
            password: "".to_string(),
            dns_1: String::from("8.8.8.8"),
            dns_2: String::from("8.8.4.4"),
            ap_name: "hologram".to_string(),
        };
        inet.initialize(cfg).expect("Failed to init inet");
    }
    Ok(())
}
