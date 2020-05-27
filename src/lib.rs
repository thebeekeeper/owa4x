extern crate owa4x_sys;
#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;

pub mod gps;
pub mod leds;
pub mod inet;
pub mod io;
pub mod gprs;

use owa4x_sys as owa;

// the ok result for this should be the RTU struct
pub fn start_rtu() -> Result<u8, String> {
    unsafe {
        let rtu = owa::RTUControl_Initialize(std::ptr::null_mut());
        if rtu != (owa::NO_ERROR as i32) {
            return Err(format!("Error initializing RTU: {}", rtu));
        }
        let rtu_start = owa::RTUControl_Start();
        if rtu_start != (owa::NO_ERROR as i32) {
            return Err(format!("Error calling RTUControl_Start: {}", rtu_start));
        }
    }
    Ok(0)
}

pub fn start_io() -> Result<u8, String> {
    unsafe {
        let io_init = owa::IO_Initialize();
        if io_init != (owa::NO_ERROR as i32) {
            return Err(format!("Error calling IO_Initialize: {}", io_init));
        }

        let io_start = owa::IO_Start(); 
        if io_start != (owa::NO_ERROR as i32) {
            owa::IO_Finalize();
            return Err(format!("Error calling IO_Start: {}", io_start));
        }
    }
    Ok(0)
}


pub fn start_wifi() -> bool {
    unsafe {
        let r = owa::DIGIO_Enable_Wifi(1) as u32;
        r == owa::NO_ERROR
    }
}

pub fn stop_wifi() -> bool {
    unsafe {
        let r = owa::DIGIO_Enable_Wifi(0) as u32;
        r == owa::NO_ERROR
    }
}

pub fn start_can() -> bool {
    unsafe {
        let r = owa::DIGIO_Enable_Can(1) as u32;
        r == owa::NO_ERROR
    }
}

pub fn stop_can() -> bool {
    unsafe {
        let r = owa::DIGIO_Enable_Can(0) as u32;
        r == owa::NO_ERROR
    }
}
