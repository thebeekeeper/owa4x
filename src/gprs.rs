use owa4x_sys as owa;
use std::ffi::c_void;

pub struct Gprs { }

impl Gprs {
    pub fn initialize() -> Result<(), &'static str> {
        let mut gsm_config = owa::TGSM_MODULE_CONFIGURATION::default();
        let gsm_ptr: *mut c_void = &mut gsm_config as *mut _ as *mut c_void;
        unsafe {
            if owa::GSM_Initialize(gsm_ptr) != (owa::NO_ERROR as i32) {
                return Err("Failed to initialize GSM");
            }
            if owa::GSM_Start() != (owa::NO_ERROR as i32) {
                return Err("Failed to start GSM");
            }
        }
        Ok(())
    }
}
