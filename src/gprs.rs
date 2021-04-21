use owa4x_sys as owa;
use std::ffi::c_void;

pub struct Gprs {}

impl Gprs {
    pub fn initialize() -> Result<Self, &'static str> {
        let mut is_active = 0;
        unsafe {
            owa::GSM_IsActive(&mut is_active);
        }
        if is_active == 0 {
            println!("GSM inactive, starting");
        let mut gsm_config = owa::TGSM_MODULE_CONFIGURATION::default();
        let gsm_ptr: *mut c_void = &mut gsm_config as *mut _ as *mut c_void;
        unsafe {
            let err = owa::GSM_Initialize(gsm_ptr);
            if err != (owa::NO_ERROR as i32) {
                error!("GSM init error: {}", err);
                return Err("Failed to initialize GSM");
            }
            let err = owa::GSM_Start();
            if err != (owa::NO_ERROR as i32) {
                error!("GSM start error: {}", err);
                return Err("Failed to start GSM");
            }
        }
        }
        Ok(Gprs { })
    }

    pub fn get_signal_strength(&self) {
        let mut s: u8 = 255;
        unsafe {
            let res: i32 = owa::GSM_GetSignalStrength(&mut s);
            //println!("Result: {}", res);
            println!("signal strength: {}", s);
            let res: i32 = owa::GSM_GetSignalQuality(&mut s);
            println!("signal quality: {}", s);
        }
    }
}
