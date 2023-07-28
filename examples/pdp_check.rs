use owa4x::{get_library_version, OwaLibrary, OwaError};
use semver::VersionReq;
use std::process::ExitCode;

// this is intended to be run as a systemd service file ExecCondition=
// exits with status code = 0 when we're on the 1.3.0 firmware or above
// exits with status code = 1 when we're on an older unit

fn main() -> ExitCode {
    env_logger::init();
    let gsm_version = get_library_version(OwaLibrary::GSM).unwrap();
    println!("ver: {:?}", gsm_version);
    let req = VersionReq::parse(">= 1.0.13").unwrap();
    let init_pdp = req.matches(&gsm_version);
    println!("init_pdp: {:?}", init_pdp);

    if init_pdp {
        return ExitCode::from(0);
    }
    return ExitCode::from(1);
}
