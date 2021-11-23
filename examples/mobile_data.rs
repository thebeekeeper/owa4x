use owa4x::inet::{Inet, InetConfig};
use owa4x::{Owa4x, OwaError};

fn main() -> Result<(), OwaError> {
    env_logger::init();
    let owa = Owa4x::new();
    owa.init()?;
    if let Ok(gsm) = owa4x::gprs::Gprs::initialize() {
        let inet = Inet::new();
        let cfg = InetConfig {
            username: "".to_string(),
            password: "".to_string(),
            dns_1: String::from("8.8.8.8"),
            dns_2: String::from("8.8.4.4"),
            ap_name: "test".to_string(),
        };
        inet.initialize(cfg).expect("Failed to init inet");

        let strength = gsm.get_signal_strength();
        println!("Signal strength: {:?}", strength);

        let imei = gsm.get_imei();
        println!("IMEI: {:?}", imei);
    }

    let mut line = String::new();
    let input = std::io::stdin().read_line(&mut line).expect("Failed to read line");

    println!("{}", input);
    Ok(())
}
