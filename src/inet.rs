use std::ffi::c_void;

#[cfg(target_arch = "arm")]
use owa4x_sys as owa;
#[cfg(all(target_arch = "aarch64", not(target_os = "macos")))]
use owa5x_sys as owa;
#[cfg(any(target_arch = "x86_64", all(target_arch = "aarch64", target_os = "macos")))]
use crate::sys_stub as owa;

#[derive(Debug, Copy, Clone)]
pub struct Inet {}

#[derive(Debug)]
pub struct InetConfig {
    pub username: String,
    pub password: String,
    pub dns_1: String,
    pub dns_2: String,
    pub ap_name: String,
}

#[derive(FromPrimitive, Debug)]
pub enum InetError {
    AlreadyRunning = 600,
    NotInitialized = 601,
    NotStarted = 602,
    InterfaceNotReady = 603,
    IpNotAvailable = 604,
    GsmOnVoice = 605,
    GsmOnCall = 606,
}

impl Inet {
    pub fn new() -> Self {
        Inet {}
    }

    // wrapping the new call so we can maintain the method signature and avoid a breaking change
    // need to no call the PDP context function so that older units don't crash
    pub fn initialize(&self, config: InetConfig) -> Result<(), InetError> {
        let mut inet_config = owa::TINET_MODULE_CONFIGURATION::default();
        let mut gprs = owa::GPRS_ENHANCED_CONFIGURATION::default();

        let mut array = [0u8; 256];
        for (x, y) in config.username.as_bytes().iter().zip(array.iter_mut()) {
            *y = *x;
        }
        //a.copy_from_slice(&config.username.as_bytes());
        gprs.gprsUser = array;

        let mut array = [0u8; 256];
        for (x, y) in config.password.as_bytes().iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsPass = array;

        let mut array = [0u8; 40];
        for (x, y) in config.dns_1.as_bytes().iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsDNS1 = array;

        let mut array = [0u8; 40];
        for (x, y) in config.dns_2.as_bytes().iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsDNS2 = array;

        let mut array = [0u8; 64];
        for (x, y) in config.ap_name.as_bytes().iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsAPN = array;

        inet_config.wBearer = owa::INET_BEARER_ENHANCED_GPRS as u8;
        inet_config.inet_action = Some(inet_event_handler);
        let gprs_ptr: *mut c_void = &mut gprs as *mut _ as *mut c_void;
        inet_config.wBearerParameters = gprs_ptr;
        let net_ptr: *mut c_void = &mut inet_config as *mut _ as *mut c_void;

        trace!("Calling inet init");
        unsafe {
            let mut ver = [0u8; 40];
            let r = owa::GSM_GetVersion(ver.as_mut_ptr()) as u32;
            if r != owa::NO_ERROR {
                let version_str = std::str::from_utf8(&ver).unwrap();
                trace!("GSM library version: {}", version_str);
            }
            else {
                warn!("{}", r);
            }

            call_pdp_context_if_available(&mut gprs);

            // units with older firmware from the vendor don't have this function defined which
            // causes a crash if we call it
            /*#[cfg(feature = "pdp")] {
                trace!("Setting PDP context");
                let r = owa::GSM_DefinePDPContext(&mut gprs) as u32;
                if r != owa::NO_ERROR {
                    trace!("GSM_DefinePDPContext error: {}", r);
                    let e: InetError = num::FromPrimitive::from_u32(r).unwrap();
                    return Err(e);
                }
            }*/

            let r = owa::iNet_Initialize(net_ptr) as u32;
            if r != owa::NO_ERROR {
                trace!("inet init: {}", r);
                let e: InetError = num::FromPrimitive::from_u32(r).unwrap();
                return Err(e);
            }
            let r = owa::iNet_Start() as u32;
            if r != owa::NO_ERROR {
                trace!("inet start: {}", r);
            }
        }

        Ok(())
    }

}

pub fn call_pdp_context_if_available(cfg: &mut owa::GPRS_ENHANCED_CONFIGURATION) {
    println!("Calling init pdp context");
    unsafe {
        let lib = libloading::Library::new("/lib/libGSM_Module.so").expect("Failed to find inet shared library");

        let f: Result<libloading::Symbol<unsafe extern fn(*mut crate::owa::GPRS_ENHANCED_CONFIGURATION) -> u16>, libloading::Error> = lib.get(b"GSM_DefinePDPContext");
        if let Ok(func) = f {
            println!("loaded function");
            func(cfg);
        }
        else {
            println!("function doesnt exist");
        }
    }
    println!("done");
}

#[no_mangle]
pub extern "C" fn inet_event_handler(p_to_event: *mut owa::INET_Events) {
    trace!("callback");
    unsafe {
        trace!("Event type: {}", (*p_to_event).evType);
    }
}
