extern crate owa4x_sys;
#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;

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
}
