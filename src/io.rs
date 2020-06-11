use owa4x_sys as owa;
use std::os::raw::{c_int, c_uchar};

#[derive(Debug, Copy, Clone)]
pub enum DigitalPin {
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
    Pin5 = 5,
    Pin6 = 6,
    Pin7 = 7,
    Pin8 = 8,
    Pin9 = 9,
}

#[derive(Debug, Copy, Clone)]
pub enum AnalogPin {
    Ain0 = 0,
    Ain1 = 1,
    Ain2 = 2,
    Ain3 = 3,
}

#[derive(Debug, Copy, Clone)]
pub struct Io {}

impl Io {
    pub fn read_digital(&self, pin: DigitalPin) -> bool {
        let mut result: c_uchar = 0xF;
        unsafe {
            owa::DIGIO_Get_DIN(pin as c_uchar, &mut result);
        }
        result == 1
    }

    pub fn read_analog(&self, pin: AnalogPin) -> u32 {
        let mut result: c_int = 0;
        unsafe {
            owa::ANAGIO_GetAnalogIn(pin as c_int, &mut result);
        }
        result as u32
    }
}
