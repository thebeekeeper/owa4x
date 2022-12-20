use std::os::raw::{c_int, c_uchar};

#[cfg(target_arch = "arm")]
use owa4x_sys as owa;
#[cfg(all(target_arch = "aarch64", not(target_os = "macos")))]
use owa5x_sys as owa;
#[cfg(any(target_arch = "x86_64", all(target_arch = "aarch64", target_os = "macos")))]
use crate::sys_stub as owa;

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

    pub fn set_analog_range(&self, pin: AnalogPin, high_range: bool) {
        unsafe {
            owa::DIGIO_Set_ADC_RANGE(pin as c_uchar, high_range as c_uchar);
        }
    }

    pub fn read_analog(&self, pin: AnalogPin) -> u32 {
        let mut result: c_int = 0;
        unsafe {
            owa::ANAGIO_GetAnalogIn(pin as c_int, &mut result);
        }
        result as u32
    }

    /// Reads a value from the specified analog input pin and scales it to a voltage
    pub fn read_volts(&self, pin: AnalogPin) -> f32 {
        let mut result: c_int = 0;
        unsafe {
            // since we can't query the current range, we'll only do this read in the extended range
            // of 0..30.72v
            owa::DIGIO_Set_ADC_RANGE(pin as c_uchar, 1);
            owa::ANAGIO_GetAnalogIn(pin as c_int, &mut result);
        }
        let scale = 0.007_501_831_501_831;
        (result as f32) * scale
    }

    pub fn set_digital(&self, pin: DigitalPin, on: bool) -> u32 {
        unsafe {
            let result = owa::DIGIO_Set_DOUT(pin as c_uchar, on as c_uchar);
            result as u32
        }
    }

    pub fn enable_uart(&self) -> u32 {
        unsafe {
            owa::DIGIO_Enable_Uart5(1) as u32
        }
    }
}
