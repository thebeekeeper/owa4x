use crate::owa_error::OwaError;
use std::ffi::c_void;

#[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_os = "macos")
))]
use crate::sys_stub as owa;
#[cfg(target_arch = "arm")]
use owa4x_sys as owa;
#[cfg(all(target_arch = "aarch64", not(target_os = "macos")))]
use owa5x_sys as owa;

#[derive(Debug, serde::Serialize, Default)]
pub enum NetworkRegistration {
    #[default]
    NoNetwork,
    Registered,
    RegisteredButRoaming,
    SosOnly,
}

pub struct Gprs {
    pub signal_strength: u8,
}

impl Gprs {
    pub fn initialize() -> Result<Self, OwaError> {
        let mut is_active = 0;
        unsafe {
            owa::GSM_IsActive(&mut is_active);
        }
        debug!("GSM active: {:?}", is_active);
        let mut gsm_config = owa::TGSM_MODULE_CONFIGURATION::default();
        let gsm_ptr: *mut c_void = &mut gsm_config as *mut _ as *mut c_void;
        unsafe {
            let err = owa::GSM_Initialize(gsm_ptr) as u32;
            debug!("GSM Initialize returned: {:?}", err);
            if err != owa::NO_ERROR {
                let _e = owa::GSM_Finalize();
                return Err(OwaError::from_or_unknown(err));
            }
            debug!("Starting GSM");
            let err = owa::GSM_Start() as u32;
            debug!("GSM_Start returned: {:?}", err);
            if err != owa::NO_ERROR {
                let e = owa::GSM_Finalize();
                debug!("GSM_Finalize: {:?}", e);
                return Err(OwaError::from_or_unknown(err));
            }
        }
        debug!("Returning gprs");
        Ok(Gprs { signal_strength: 0 })
    }

    pub fn get_is_active(&self) -> Result<bool, OwaError> {
        let mut s: u8 = 255;
        let res = unsafe { owa::GSM_GetSignalStrength(&mut s) as u32 };
        if res != owa::NO_ERROR {
            return Err(OwaError::from_or_unknown(res));
        }
        Ok(s != 0)
    }

    pub fn get_signal_strength(&self) -> Result<u8, OwaError> {
        let mut s: u8 = 255;
        let res = unsafe { owa::GSM_GetSignalStrength(&mut s) as u32 };
        if res != owa::NO_ERROR {
            return Err(OwaError::from_or_unknown(res));
        }
        Ok(s)
    }

    pub fn get_signal_quality(&self) -> Result<u8, OwaError> {
        let mut s: u8 = 255;
        let res = unsafe { owa::GSM_GetSignalQuality(&mut s) as u32 };
        if res != owa::NO_ERROR {
            return Err(OwaError::from_or_unknown(res));
        }
        Ok(s)
    }

    pub fn get_network_status(&self) -> Result<NetworkRegistration, OwaError> {
        let mut s: u8 = 255;
        let res = unsafe { owa::GSM_GetSignalQuality(&mut s) as u32 };
        if res != owa::NO_ERROR {
            return Err(OwaError::from_or_unknown(res));
        }
        match s {
            0 => Ok(NetworkRegistration::NoNetwork),
            1 => Ok(NetworkRegistration::Registered),
            2 => Ok(NetworkRegistration::RegisteredButRoaming),
            3 => Ok(NetworkRegistration::SosOnly),
            _ => Err(OwaError::UnknownError),
        }
    }

    pub fn get_imei(&self) -> Result<String, OwaError> {
        let mut buffer = vec![0u8; 16];
        let res = unsafe { owa::GSM_GetIMEI(buffer.as_mut_ptr(), 15) as u32 };
        if res != owa::NO_ERROR {
            return Err(OwaError::from_or_unknown(res));
        }
        let read_length = buffer.iter().position(|&b| b == 0);
        if read_length.is_none() {
            return Err(OwaError::ParseError);
        }
        buffer.truncate(read_length.unwrap());
        let s = std::ffi::CString::new(buffer)
            .unwrap()
            .into_string()
            .unwrap();

        Ok(s)
    }
}

