use std::ffi::c_void;

#[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_os = "macos")
))]
use crate::sys_stub as owa;
use crate::OwaError;
#[cfg(target_arch = "arm")]
use owa4x_sys as owa;
#[cfg(all(target_arch = "aarch64", not(target_os = "macos")))]
use owa5x_sys as owa;

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

impl Inet {
    pub fn new() -> Self {
        Inet {}
    }

    // wrapping the new call so we can maintain the method signature and avoid a breaking change
    // need to no call the PDP context function so that older units don't crash
    pub fn initialize(&self, config: InetConfig) -> Result<(), OwaError> {
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
            } else {
                warn!("{}", r);
            }

            call_pdp_context_if_available(&mut gprs);

        }
        let r = unsafe { owa::iNet_Initialize(net_ptr) as u32 };
        if r != owa::NO_ERROR {
            trace!("inet init: {}", r);
            return Err(OwaError::from_or_unknown(r));
        }

        let r = unsafe { owa::iNet_Start() as u32 };
        if r != owa::NO_ERROR {
            trace!("inet start: {}", r);
            return Err(OwaError::from_or_unknown(r));
        }

        Ok(())
    }

    pub fn is_active(&self) -> Result<bool, OwaError> {
        let mut is_active = 0;
        let e = unsafe { owa::iNet_IsActive(&mut is_active) as u32 };
        if e != owa::NO_ERROR {
            Err(OwaError::from_or_unknown(e))
        } else {
            let b = is_active == 1;
            Ok(b)
        }
    }

    pub fn get_ip_address(&self) -> Result<String, OwaError> {
        let mut s = vec![0u8; 16];
        let e = unsafe { owa::iNet_GetIPAddress(s.as_mut_ptr()) as u32 };
        if e != owa::NO_ERROR {
            Err(OwaError::from_or_unknown(e))
        } else {
            let ip_str = std::ffi::CString::new(s).unwrap().into_string().unwrap();
            Ok(ip_str)
        }
    }
}

pub fn call_pdp_context_if_available(cfg: &mut owa::GPRS_ENHANCED_CONFIGURATION) {
    debug!("Calling init pdp context");
    unsafe {
        let lib = libloading::Library::new("/lib/libGSM_Module.so")
            .expect("Failed to find inet shared library");

        let f: Result<
            libloading::Symbol<
                unsafe extern "C" fn(*mut crate::owa::GPRS_ENHANCED_CONFIGURATION) -> u16,
            >,
            libloading::Error,
        > = lib.get(b"GSM_DefinePDPContext");
        if let Ok(func) = f {
            debug!("loaded function");
            func(cfg);
        } else {
            debug!("function doesnt exist");
        }
    }
    debug!("done");
}

#[no_mangle]
pub extern "C" fn inet_event_handler(p_to_event: *mut owa::INET_Events) {
    trace!("callback");
    unsafe {
        trace!("Event type: {}", (*p_to_event).evType);
    }
}
