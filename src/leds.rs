#[cfg(target_arch = "arm")]
use owa4x_sys as owa;
#[cfg(all(target_arch = "aarch64", not(target_os = "macos")))]
use owa5x_sys as owa;
#[cfg(any(target_arch = "x86_64", all(target_arch = "aarch64", target_os = "macos")))]
use crate::sys_stub as owa;

#[derive(Debug, Copy, Clone)]
pub enum PanelLed {
    Yellow,
    Green,
    Red,
    Orange,
}

#[derive(Debug, Copy, Clone)]
pub struct Leds {}

impl Leds {
    pub fn new() -> Self {
        Leds {}
    }

    pub fn set_state(&self, led: PanelLed, status: bool) {
        unsafe {
            trace!("Settting led to: {}", status);
            let val = match status {
                true => 1,
                false => 0,
            };
            match led {
                PanelLed::Yellow => owa::DIGIO_Set_LED_SW0(val),
                PanelLed::Green => owa::DIGIO_Set_LED_SW1(val),
                PanelLed::Red => owa::DIGIO_Set_LED_SW2(val),
                PanelLed::Orange => owa::DIGIO_Set_PPS_GPS(val),
            };
        }
    }
}
