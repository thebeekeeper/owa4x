#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;

pub mod gprs;
pub mod gps;
pub mod inet;
pub mod io;
pub mod leds;
pub mod power;
pub mod owa_error;

use crate::gps::Gps;
use crate::io::Io;
use crate::leds::Leds;
use crate::power::Power;
pub use crate::owa_error::OwaError;

#[cfg(target_arch = "arm")]
use owa4x_sys as owa;
#[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "aarch64", not(target_os = "macos"))
))]
use owa5x_sys as owa;

// use a stub if we're not building for supported hardware
#[cfg(any(target_arch = "x86_64", all(target_arch = "aarch64", target_os = "macos")))]
mod sys_stub;
#[cfg(any(target_arch = "x86_64", all(target_arch = "aarch64", target_os = "macos")))]
use sys_stub as owa;

#[derive(Debug, Copy, Clone)]
pub struct Owa4x {
    pub led: Leds,
    pub gps: Gps,
    pub io: Io,
    pub power: Power,
}

impl Owa4x {
    pub fn new() -> Self {
        Owa4x {
            led: Leds::new(),
            gps: Gps::new(),
            io: Io {},
            power: Power {},
        }
    }

    pub fn init(&self) -> Result<(), OwaError> {
        self.start_rtu()?;
        self.start_io()?;
        Ok(())
    }

    pub fn start_wifi(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Wifi(1) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(error_code));
            }
        }
        Ok(())
    }

    pub fn stop_wifi(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Wifi(0) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(error_code));
            }
        }
        Ok(())
    }

    pub fn start_bluetooth(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Bluetooth(1) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(error_code));
            }
        }
        Ok(())
    }

    pub fn stop_bluetooth(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Bluetooth(0) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(error_code));
            }
        }
        Ok(())
    }

    pub fn start_can(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Can(1) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(error_code));
            }
        }
        Ok(())
    }

    pub fn stop_can(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Can(0) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(error_code));
            }
        }
        Ok(())
    }
    // the ok result for this should be the RTU struct
    pub fn start_rtu(&self) -> Result<(), OwaError> {
        unsafe {
            let rtu = owa::RTUControl_Initialize(std::ptr::null_mut()) as u32;
            if rtu != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(rtu));
            }
            let rtu_start = owa::RTUControl_Start() as u32;
            if rtu_start != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(rtu_start));
            }
        }
        Ok(())
    }

    pub fn start_io(&self) -> Result<(), OwaError> {
        unsafe {
            let io_init = owa::IO_Initialize() as u32;
            if io_init != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(io_init));
            }

            let io_start = owa::IO_Start() as u32;
            if io_start != owa::NO_ERROR {
                owa::IO_Finalize();
                return Err(OwaError::from_or_unknown(io_start));
            }
        }
        Ok(())
    }

    /// Configures the clock subsystem to wakeup after the given number of seconds.  Immediately
    /// puts the system to sleep.
    ///
    /// # Arguments
    ///
    /// * `seconds` - Number of seconds to sleep for.  Hardware documentation is light, but
    /// theoretically could sleep for max(i32) seconds which is about 25k days
    ///
    pub fn take_a_nap(&self, seconds: i32) -> Result<(), OwaError>  {
        unsafe {
            let r = owa::RTUSetIncrementalWakeUpTime(seconds) as u32;
            if r != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(r));
            }
            let r = owa::RTUEnterStop((owa::RTU_WKUP_PWRFAIL | owa::RTU_WKUP_RTC).into(), 0) as u32;
            if r != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(r));
            }
        }
        Ok(())
    }

    // todo: consider taking a dependency on chrono to shorten this signature
    
    /// Schedules a date and time in the future for the system to wake up.  Does not put the system
    /// to sleep.  See `enter_sleep` to put the system to sleep.
    pub fn schedule_nap(&self, year: u16, month: u8, day: u8, hour: u8, min: u8, sec: u8) -> Result<(), OwaError> {
        let td = owa::THW_TIME_DATE {
            year,
            month,
            day,
            hour,
            min,
            sec,
        };
        unsafe {
            let r = owa::RTUSetWakeUpTime(td) as u32;
            if r != owa::NO_ERROR {
                return Err(OwaError::from_or_unknown(r));
            }
        }
        Ok(())
    }


    pub fn get_serial_number(&self) -> Result<String, OwaError> {
        let mut buffer = vec![0u8; 6];
        let res: u32;
        unsafe {
            res = owa::GetSerialNumber(buffer.as_mut_ptr()) as u32;
        }
        if res != owa::NO_ERROR {
            return Err(OwaError::from_or_unknown(res));
        }
        match std::ffi::CString::new(buffer).unwrap().into_string() {
            Ok(s) => Ok(s),
            Err(_) => Err(OwaError::ParseError),
        }
    }
}
