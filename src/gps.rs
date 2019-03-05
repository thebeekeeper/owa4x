use owa4x_sys as owa;
use std::ffi::c_void;

#[derive(Debug, Copy, Clone)]
pub struct Gps {
}

#[derive(Debug, Copy, Clone)]
pub struct GpsPosition {
    #[doc = "< Altitude in meters"]
    pub altitude: f64,
    //#[doc = "< Navigation status"]
    //pub NavStatus: [::std::os::raw::c_char; 3usize],
    #[doc = "< Horizontal Accuracy"]
    pub horizontal_accuracy: f64,
    #[doc = "< Vertical Accuracy"]
    pub vertical_accuracy: f64,
    #[doc = "< Speed over ground"]
    pub speed: f64,
    #[doc = "< Course over ground"]
    pub course: f64,
    #[doc = "< Horizontal dilution of precision"]
    pub hdop: f64,
    #[doc = "< Vertical dilution of precision"]
    pub vdop: f64,
    #[doc = "< Time dilution of precision"]
    pub tdop: f64,
    #[doc = "< Number of satellites used in the navigation solution"]
    pub satellite_count: u8,
    #[doc = "< Latitude     negative degrees means South"]
    pub latitude: f64,
    #[doc = "< Longitude    negative degrees means East"]
    pub longitude: f64,
}

impl Gps {
    pub fn new() -> Self {
        Gps { }
    }

    /// Starts and initializes the GPS receiver
    /// using the default configuration parameters
    pub fn initialize(&self) -> bool {
        // there's gotta be a better way to do this
        let input = b"GPS_UBLOX";
        let mut array = [0u8; 20];
        for (x, y) in input.iter().zip(array.iter_mut()) {
            *y = *x;
        }

        let s = b"NMEA";
        let mut nmea = [0u8; 10];
        for (x, y) in s.iter().zip(nmea.iter_mut()) {
            *y = *x;
        }

        // it's unclear to me if there are any other values that
        // could reasonably go here
        let mut config = owa::TGPS_MODULE_CONFIGURATION {
            DeviceReceiverName: array,
            ParamBaud: owa::B115200,
            ParamLength: owa::CS8 as u8,
            ParamParity: owa::IGNPAR as i32,
            ProtocolName: nmea,
            GPSPort: owa::COM1 as u8,
        };

        unsafe {
            let config_ptr: *mut c_void = &mut config as *mut _ as *mut c_void;
            trace!("Calling init");
            let gps_init = owa::GPS_Initialize(config_ptr);
            if gps_init != (owa::NO_ERROR as i32) {
                trace!("Error configuring gps: {}", gps_init);
                return false;
            }

            trace!("Calling start");
            let gps_start = owa::GPS_Start();
            if gps_start != (owa::NO_ERROR as i32) {
                trace!("Error starting gps: {}", gps_start);
                return false;
            }

            let mut is_active: std::os::raw::c_int = 0;
            owa::GPS_IsActive(&mut is_active);
            trace!("is_active: {}", is_active);
        }
        return true;
    }

    pub fn get_position(&self) -> Option<GpsPosition> {
        trace!("Getting position");
        let mut l: owa::tPOSITION_DATA = Default::default();
        let mut get_pos = 0xff;
        unsafe {
            get_pos = owa::GPS_GetAllPositionData(&mut l);
        }
        match get_pos as u32 {
            owa::NO_ERROR => {
                Some(GpsPosition{ altitude: l.Altitude, horizontal_accuracy: l.HorizAccu,
                    vertical_accuracy: l.VertiAccu, speed: l.Speed, course: l.Course,
                    hdop: l.HDOP, vdop: l.VDOP, tdop: l.TDOP, satellite_count: l.numSvs,
                    latitude: l.LatDecimal, longitude: l.LonDecimal })
            }
            e => {
                warn!("Error getting GPS psotion data: {}", e);
                None
            }
        }
    }
}
