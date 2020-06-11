use owa4x_sys as owa;
use std::ffi::c_void;

#[derive(Debug, Copy, Clone)]
pub struct Inet {
}

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
        Inet { }
    }

    pub fn initialize(&self, config: InetConfig) -> bool {

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
            let r = owa::iNet_Initialize(net_ptr);
            trace!("inet init: {}", r);
            let r = owa::iNet_Start();
            trace!("inet start: {}", r);
        }


        return true;
    }

}
#[no_mangle]
    pub extern "C" fn inet_event_handler(_p_to_event: *mut owa::INET_Events) {
        trace!("inet_event_handler callback");
    }
