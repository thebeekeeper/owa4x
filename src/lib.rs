extern crate owa4x_sys;
#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

pub mod gprs;
pub mod gps;
pub mod inet;
pub mod io;
pub mod leds;

use crate::gps::Gps;
use crate::io::Io;
use crate::leds::Leds;

use owa4x_sys as owa;

#[derive(Debug, Clone)]
pub struct OwaError {
    pub error_code: u32,
}

impl Error for OwaError {}

impl fmt::Display for OwaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //let s = format!("OWA error: {}", self.error_code);
        write!(f, "OWA error")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Owa4x {
    pub led: Leds,
    pub gps: Gps,
    pub io: Io,
}

impl Owa4x {
    pub fn new() -> Self {
        Owa4x {
            led: Leds::new(),
            gps: Gps::new(),
            io: Io {},
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
                return Err(OwaError { error_code });
            }
        }
        Ok(())
    }

    pub fn stop_wifi(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Wifi(0) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError { error_code });
            }
        }
        Ok(())
    }

    pub fn start_bluetooth(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Bluetooth(1) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError { error_code });
            }
        }
        Ok(())
    }

    pub fn stop_bluetooth(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Bluetooth(0) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError { error_code });
            }
        }
        Ok(())
    }

    pub fn start_can(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Can(1) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError { error_code });
            }
        }
        Ok(())
    }

    pub fn stop_can(&self) -> Result<(), OwaError> {
        unsafe {
            let error_code = owa::DIGIO_Enable_Can(0) as u32;
            if error_code != owa::NO_ERROR {
                return Err(OwaError { error_code });
            }
        }
        Ok(())
    }
    // the ok result for this should be the RTU struct
    pub fn start_rtu(&self) -> Result<(), OwaError> {
        unsafe {
            let rtu = owa::RTUControl_Initialize(std::ptr::null_mut()) as u32;
            if rtu != owa::NO_ERROR {
                return Err(OwaError { error_code: rtu });
            }
            let rtu_start = owa::RTUControl_Start() as u32;
            if rtu_start != owa::NO_ERROR {
                return Err(OwaError {
                    error_code: rtu_start,
                });
            }
        }
        Ok(())
    }

    pub fn start_io(&self) -> Result<(), OwaError> {
        unsafe {
            let io_init = owa::IO_Initialize() as u32;
            if io_init != owa::NO_ERROR {
                return Err(OwaError {
                    error_code: io_init,
                });
            }

            let io_start = owa::IO_Start() as u32;
            if io_start != owa::NO_ERROR {
                owa::IO_Finalize();
                return Err(OwaError {
                    error_code: io_start,
                });
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
                return Err(OwaError {
                    error_code: r,
                });
            }
            let r = owa::RTUEnterStop(owa::RTU_WKUP_RTC, 0) as u32;
            if r != owa::NO_ERROR {
                return Err(OwaError {
                    error_code: r,
                });
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
                return Err(OwaError { error_code: r });
            }
        }
        Ok(())
    }

    /// Immediately enters sleep mode.  Instructs the system to allow wakeup from the RTC.
    pub fn enter_sleep(&self) -> Result<(), OwaError> {
        unsafe {
            let r = owa::RTUEnterStop(owa::RTU_WKUP_RTC, 0) as u32;
            if r != owa::NO_ERROR {
                return Err(OwaError {
                    error_code: r,
                });
            }
        }
        Ok(())
    }

    pub fn get_battery_voltage(&self) -> Result<f32, OwaError> {
        let mut v: f32 = 0.0;
        let res: u32;
        unsafe {
            res = owa::RTUGetAD_VBAT_MAIN(&mut v) as u32;
        }
        if res != owa::NO_ERROR {
            return Err(OwaError {
                error_code: res,
            });
        }
        Ok(v)
    }


    pub fn get_power_voltage(&self) -> Result<f32, OwaError> {
        let mut v: f32 = 0.0;
        let res: u32;
        unsafe {
            res = owa::RTUGetAD_V_IN(&mut v) as u32;
        }
        if res != owa::NO_ERROR {
            return Err(OwaError {
                error_code: res,
            });
        }
        Ok(v)
    }

    pub fn get_serial_number(&self) -> Result<String, OwaError> {
        let mut buffer = vec![0u8; 6];
        let res: u32;
        unsafe {
            res = owa::GetSerialNumber(buffer.as_mut_ptr()) as u32;
        }
        if res != owa::NO_ERROR {
            return Err(OwaError {
                error_code: res,
            });
        }
        let s = std::ffi::CString::new(buffer).unwrap().into_string().unwrap();
        Ok(s)
    }

    pub fn get_battery_state(&self) -> Result<BatteryChargeState, OwaError> {
        let mut s: u8 = 0;
        let res: u32;
        unsafe {
            res = owa::RTUGetBatteryState(&mut s) as u32;
        }
        if res != owa::NO_ERROR {
            return Err(OwaError {
                error_code: res,
            });
        }
        let r = BatteryChargeState::try_from(s);
        match r {
            Ok(e) => Ok(e),
            Err(_) => Err(OwaError { error_code: owa::ERROR_IN_PARAMETERS })
        }
    }

    pub fn get_wakeup_reason(&self) -> Result<WakeupReason, OwaError> {
        let mut reason: u32 = 0;
        let res: u32;
        unsafe {
            res = owa::RTUGetWakeUpReason(&mut reason) as u32;
        }
        if res != owa::NO_ERROR {
            return Err(OwaError {
                error_code: res,
            });
        }
        let r = WakeupReason::try_from(reason);
        match r {
            Ok(e) => Ok(e),
            Err(_) => Err(OwaError { error_code: owa::ERROR_IN_PARAMETERS })
        }
    }

}
    #[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u8)]
    pub enum BatteryChargeState {
        Precharge = 0,
        ChargeDone = 1,
        FastCharging = 2,
        ChargeSuspended = 3,
    }

    #[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
    #[repr(u32)]
    pub enum WakeupReason {
        // this isn't documented, but it seems that when i turn power on i get 0 for the reason
        PowerSwitch = 0,
        Moving = 1,
        PowerFail = 2,
        Console = 4,
        Gsm = 8,
        Can1 = 16,
        Rtc = 64,
        DigitalIn0 = 128,
        DigitalIn1 = 256,
        DigitalIn2 = 512,
        DigitalIn3 = 1024,
        DigitalIn4 = 2048,
        DigitalIn5 = 4096,
        DigitalIn6 = 8192,
        DigitalIn7 = 16384,
        DigitalIn8 = 32768,
        DigitalIn9 = 65536,
    }
