extern crate owa4x_sys;
#[macro_use]
extern crate log;

use std::error::Error;
use std::fmt;

pub mod gprs;
pub mod gps;
pub mod inet;
pub mod io;
pub mod leds;

use crate::leds::Leds;
use crate::gps::Gps;
use crate::io::Io;

use owa4x_sys as owa;

#[derive(Debug, Clone)]
pub struct OwaError {
    pub error_code: i32
}

impl Error for OwaError {
}

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
            io: Io { },
        }
    }

    pub fn init(&self) -> Result<(), OwaError> {
        start_rtu()?;
        start_io()?;
        Ok(())
    }

pub fn start_wifi(&self) -> bool {
    unsafe {
        let r = owa::DIGIO_Enable_Wifi(1) as u32;
        r == owa::NO_ERROR
    }
}

pub fn stop_wifi(&self) -> bool {
    unsafe {
        let r = owa::DIGIO_Enable_Wifi(0) as u32;
        r == owa::NO_ERROR
    }
}

pub fn start_can(&self) -> bool {
    unsafe {
        let r = owa::DIGIO_Enable_Can(1) as u32;
        r == owa::NO_ERROR
    }
}

pub fn stop_can(&self) -> bool {
    unsafe {
        let r = owa::DIGIO_Enable_Can(0) as u32;
        r == owa::NO_ERROR
    }
}
}

// the ok result for this should be the RTU struct
pub fn start_rtu() -> Result<u8, OwaError> {
    unsafe {
        let rtu = owa::RTUControl_Initialize(std::ptr::null_mut());
        if rtu != (owa::NO_ERROR as i32) {
            //return Err(format!("Error initializing RTU: {}", rtu));
            return Err(OwaError { error_code: rtu });
        }
        let rtu_start = owa::RTUControl_Start();
        if rtu_start != (owa::NO_ERROR as i32) {
            //return Err(format!("Error calling RTUControl_Start: {}", rtu_start));
            //return Err(rtu_start);
            return Err(OwaError { error_code: rtu_start });
        }
    }
    Ok(0)
}

pub fn start_io() -> Result<u8, OwaError> {
    unsafe {
        let io_init = owa::IO_Initialize();
        if io_init != (owa::NO_ERROR as i32) {
            return Err(OwaError { error_code: io_init });
            //return Err(format!("Error calling IO_Initialize: {}", io_init));
        }

        let io_start = owa::IO_Start();
        if io_start != (owa::NO_ERROR as i32) {
            owa::IO_Finalize();
            return Err(OwaError { error_code: io_start });
            //return Err(format!("Error calling IO_Start: {}", io_start));
        }
    }
    Ok(0)
}

