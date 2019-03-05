use owa4x_sys as owa;
use std::ffi::c_void;

#[derive(Debug, Copy, Clone)]
pub struct Inet {
}

impl Inet {
    pub fn new() -> Self {
        Inet { }
    }

    pub fn initialize(&self) -> bool {
        let mut inet_config = owa::TINET_MODULE_CONFIGURATION::default();
        let mut gprs = owa::GPRS_ENHANCED_CONFIGURATION::default();
        let user_str = b"test user";
        let mut array = [0u8; 256];
        for (x, y) in user_str.iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsUser = array;

        let user_str = b"grps-password";
        let mut array = [0u8; 256];
        for (x, y) in user_str.iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsPass = array;

        let user_str = b"gprs dns 1";
        let mut array = [0u8; 40];
        for (x, y) in user_str.iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsDNS1 = array;

        let user_str = b"gprs dns 1";
        let mut array = [0u8; 40];
        for (x, y) in user_str.iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsDNS2 = array;

        let user_str = b"ap name";
        let mut array = [0u8; 64];
        for (x, y) in user_str.iter().zip(array.iter_mut()) {
            *y = *x;
        }
        gprs.gprsAPN = array;

        inet_config.wBearer = owa::INET_BEARER_ENHANCED_GPRS as u8;
        let gprs_ptr: *mut c_void = &mut gprs as *mut _ as *mut c_void;
        inet_config.wBearerParameters = gprs_ptr;

        return true;
    }
}
