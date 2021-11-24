
use owa4x_sys as owa;
use crate::OwaError;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

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

#[derive(Debug, Copy, Clone)]
pub struct Power {
}

impl Power {
    /// Immediately enters sleep mode.  Instructs the system to allow wakeup from the RTC.
    pub fn enter_sleep(&self) -> Result<(), OwaError> {
        unsafe {
            let r = owa::RTUEnterStop(owa::RTU_WKUP_RTC, 0) as u32;
            if r != owa::NO_ERROR {
                return Err(OwaError::try_from(r).unwrap_or(OwaError::UnknownError));
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
            return Err(OwaError::try_from(res).unwrap_or(OwaError::UnknownError));
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
            return Err(OwaError::try_from(res).unwrap_or(OwaError::UnknownError));
        }
        Ok(v)
    }

    pub fn get_battery_state(&self) -> Result<BatteryChargeState, OwaError> {
        let mut s: u8 = 0;
        let res: u32;
        unsafe {
            res = owa::RTUGetBatteryState(&mut s) as u32;
        }
        if res != owa::NO_ERROR {
            return Err(OwaError::try_from(res).unwrap_or(OwaError::UnknownError));
        }
        let r = BatteryChargeState::try_from(s);
        match r {
            Ok(e) => Ok(e),
            Err(_) => Err(OwaError::ParseError),
        }
    }

    pub fn get_wakeup_reason(&self) -> Result<WakeupReason, OwaError> {
        let mut reason: u32 = 0;
        let res: u32;
        unsafe {
            res = owa::RTUGetWakeUpReason(&mut reason) as u32;
        }
        if res != owa::NO_ERROR {
            return Err(OwaError::try_from(res).unwrap_or(OwaError::UnknownError));
        }
        let r = WakeupReason::try_from(reason);
        match r {
            Ok(e) => Ok(e),
            Err(_) => Err(OwaError::ParseError),
        }
    }
}
