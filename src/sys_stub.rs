#![allow(non_snake_case)]

pub const NO_ERROR: u32 = 0;
pub const RTU_WKUP_PWRFAIL: u32 = 2;
pub const RTU_WKUP_RTC: u32 = 64;
pub const INET_BEARER_ENHANCED_GPRS: u32 = 2;
pub const COM1: u32 = 0;
pub const COM4: u32 = 3;
pub const IGNPAR: u32 = 4;
pub const CS8: u32 = 48;
pub const B115200: u32 = 4098;
pub type speed_t = ::std::os::raw::c_uint;

pub fn GSM_DefinePDPContext(
    pConfiguration: *mut GPRS_ENHANCED_CONFIGURATION,
) -> ::std::os::raw::c_int {
    0
}

pub struct THW_TIME_DATE {
    pub sec: ::std::os::raw::c_uchar,
    pub min: ::std::os::raw::c_uchar,
    pub hour: ::std::os::raw::c_uchar,
    pub day: ::std::os::raw::c_uchar,
    pub month: ::std::os::raw::c_uchar,
    pub year: ::std::os::raw::c_ushort,
}

pub fn RTUSetWakeUpTime(CurrentTime: THW_TIME_DATE) -> ::std::os::raw::c_int {
    0
}
pub fn GSM_IsActive(wActive: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    0
}
pub fn GetSerialNumber(wSerialNumber: *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {
    0
}
pub fn RTUEnterStop(
    wMainWakeup: ::std::os::raw::c_ulong,
    wExpWakeup: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int {
    0
}
pub fn RTUSetIncrementalWakeUpTime(secs: ::std::os::raw::c_int) -> ::std::os::raw::c_int{
    0
}
pub fn IO_Finalize() -> ::std::os::raw::c_int { 0 }
pub fn IO_Start() -> ::std::os::raw::c_int { 0 }
pub fn IO_Initialize() -> ::std::os::raw::c_int { 0 }

pub fn RTUControl_Start() -> ::std::os::raw::c_int {0}
pub fn RTUControl_Initialize(
    wConfiguration: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {0}

pub fn DIGIO_Enable_Can(wValue: ::std::os::raw::c_char) -> ::std::os::raw::c_int {0}
pub fn DIGIO_Enable_Bluetooth(wValue: ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
pub fn DIGIO_Enable_Wifi(wValue: ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
pub fn GSM_GetIMEI(
    wImei: *mut ::std::os::raw::c_uchar,
    wSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int{0}
pub fn GSM_GetSignalQuality(
    wSignalQuality: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {0}
pub fn GSM_GetSignalStrength(
    wSignalStrength: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {0}
pub fn GSM_Start() -> ::std::os::raw::c_int {0}
pub fn GSM_Initialize(wConfiguration: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int {0}
    pub fn RTUGetBatteryState(BattState: *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
    pub fn RTUGetAD_V_IN(ad_v_in: *mut f32) -> ::std::os::raw::c_int {0}
    pub fn RTUGetAD_VBAT_MAIN(ad_vbat_main: *mut f32) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Get_PWR_FAIL(wValue: *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Set_PPS_GPS(wValue: ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Set_LED_SW0(wValue: ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Set_LED_SW1(wValue: ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Set_LED_SW2(wValue: ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Enable_Uart5(wValue: ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Set_DOUT(
        wOutput: ::std::os::raw::c_uchar,
        wValue: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {0}
    pub fn ANAGIO_GetAnalogIn(
        anag_nbr: ::std::os::raw::c_int,
        value: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Set_ADC_RANGE(
        wRange: ::std::os::raw::c_uchar,
        wValue: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {0}
    pub fn DIGIO_Get_DIN(
        wInput: ::std::os::raw::c_uchar,
        wValue: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {0}
    pub fn iNet_Start() -> ::std::os::raw::c_int {0}
#[derive(Debug, Default, Copy, Clone)]
pub struct _INET_Events {
    pub evType: ::std::os::raw::c_uchar,
    pub evHandled: ::std::os::raw::c_int,
}
pub type INET_Events = _INET_Events;
#[derive(Debug, Copy, Clone)]
pub struct _TINET_MODULE_CONFIGURATION {
    #[doc = "< Internet Bearer"]
    pub wBearer: ::std::os::raw::c_uchar,
    pub wBearerParameters: *mut ::std::os::raw::c_void,
    pub inet_action: ::std::option::Option<unsafe extern "C" fn(arg1: *mut INET_Events)>,
}
impl Default for _TINET_MODULE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type TINET_MODULE_CONFIGURATION = _TINET_MODULE_CONFIGURATION;
    pub fn iNet_Initialize(wConfiguration: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int {0}

#[derive(Copy, Clone)]
pub struct _GPRS_ENHANCED_CONFIGURATION {
    #[doc = "< User defined in the GPRS/INET network"]
    pub gprsUser: [::std::os::raw::c_uchar; 256usize],
    #[doc = "< Password defined in the GPRS/INET network"]
    pub gprsPass: [::std::os::raw::c_uchar; 256usize],
    #[doc = "< DNS1 defined in the GPRS network, if exists"]
    pub gprsDNS1: [::std::os::raw::c_uchar; 40usize],
    #[doc = "< DNS2 defined in the GPRS network, if exists"]
    pub gprsDNS2: [::std::os::raw::c_uchar; 40usize],
    #[doc = "< Access Point Name for iNet in the GPRS network"]
    pub gprsAPN: [::std::os::raw::c_uchar; 64usize],
}
pub type GPRS_ENHANCED_CONFIGURATION = _GPRS_ENHANCED_CONFIGURATION;
impl Default for _GPRS_ENHANCED_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}



pub fn GPS_GetSV_inView(pData: *mut tGSV_Data) -> ::std::os::raw::c_int {0}

#[derive(Copy, Clone)]
pub struct tGSV_Data {
    pub SV_InView: ::std::os::raw::c_uchar,
    pub SV: [tSV_Data; 64usize],
}
impl Default for tGSV_Data {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[derive(Debug, Default, Copy, Clone)]
pub struct tPOSITION_DATA {
    #[doc = "< Valid Fix (according to configuration)"]
    pub PosValid: ::std::os::raw::c_uchar,
    #[doc = "< Data not updated"]
    pub OldValue: ::std::os::raw::c_uchar,
    #[doc = "< Latitude"]
    pub Latitude: TGPS_COORD,
    #[doc = "< Longitude"]
    pub Longitude: TGPS_COORD,
    #[doc = "< Altitude in meters"]
    pub Altitude: f64,
    #[doc = "< Navigation status"]
    pub NavStatus: [::std::os::raw::c_char; 3usize],
    #[doc = "< Horizontal Accuracy"]
    pub HorizAccu: f64,
    #[doc = "< Vertical Accuracy"]
    pub VertiAccu: f64,
    #[doc = "< Speed over ground"]
    pub Speed: f64,
    #[doc = "< Course over ground"]
    pub Course: f64,
    #[doc = "< Horizontal dilution of precision"]
    pub HDOP: f64,
    #[doc = "< Vertical dilution of precision"]
    pub VDOP: f64,
    #[doc = "< Time dilution of precision"]
    pub TDOP: f64,
    #[doc = "< Number of satellites used in the navigation solution"]
    pub numSvs: ::std::os::raw::c_uchar,
    #[doc = "< Latitude     negative degrees means South"]
    pub LatDecimal: f64,
    #[doc = "< Longitude    negative degrees means West"]
    pub LonDecimal: f64,
}
    pub fn GPS_IsActive(wActive: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int {0}
    pub fn GPS_Start() -> ::std::os::raw::c_int {0}
#[derive(Debug, Default, Copy, Clone)]
pub struct TGPS_COORD {
    pub Degrees: ::std::os::raw::c_ushort,
    pub Minutes: ::std::os::raw::c_uchar,
    pub Seconds: f64,
    pub Dir: ::std::os::raw::c_char,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct tSV_Data {
    #[doc = "< Satellite id"]
    pub SV_Id: ::std::os::raw::c_uchar,
    #[doc = "< SV elevation in degrees"]
    pub SV_Elevation: ::std::os::raw::c_uchar,
    #[doc = "< SV azimuth in degrees"]
    pub SV_Azimuth: ::std::os::raw::c_short,
    #[doc = "< C/No ratio, range 0 to 99, null when no tracking"]
    pub SV_SNR: ::std::os::raw::c_char,
}
    pub fn GPS_GetAllPositionData(pCurCoords: *mut tPOSITION_DATA) -> ::std::os::raw::c_int {0}
    pub fn GPS_Initialize(wConfiguration: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int {0}
#[derive(Debug, Default, Copy, Clone)]
pub struct TGPS_MODULE_CONFIGURATION {
    pub DeviceReceiverName: [::std::os::raw::c_uchar; 20usize],
    pub ParamBaud: speed_t,
    pub ParamParity: ::std::os::raw::c_int,
    pub ParamLength: ::std::os::raw::c_uchar,
    pub ProtocolName: [::std::os::raw::c_uchar; 10usize],
    pub GPSPort: ::std::os::raw::c_uchar,
}
#[derive(Debug, Default, Copy, Clone)]
pub struct _TGSM_MODULE_CONFIGURATION {
    pub wCode: [::std::os::raw::c_uchar; 9usize],
    pub wMECode: [::std::os::raw::c_uchar; 9usize],
    pub gsm_action: Option<u8>,
    //pub gsm_action: ::std::option::Option<unsafe extern "C" fn(arg1: *mut gsmEvents_s)>,
}
pub type TGSM_MODULE_CONFIGURATION = _TGSM_MODULE_CONFIGURATION;


pub fn GSM_GetVersion(wVersion: *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {
    0
}
pub fn iNet_GetVersion(wVersion: *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {
    0
}
pub fn GPS_GetVersion(wVersion: *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {
    0
}
pub fn IO_GetVersion(wVersion: *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {
    0
}

pub fn RTUControl_GetVersion(wVersion: *mut ::std::os::raw::c_uchar) -> ::std::os::raw::c_int {
    0
}

