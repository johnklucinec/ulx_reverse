use crate::utils::extract_bits;
use strum_macros::FromRepr;

pub const VID: u16 = 0x361d; // Vendor ID for Finalmouse
pub const PID: u16 = 0x0100; // Product ID for ULX Dongle
pub const COMMAND_ENDPOINT: u8 = 0x01;
#[cfg(debug_assertions)]
pub const INTERRUPT_ENDPOINT: u8 = 0x81;
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(1000);

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u16)]
pub enum PollingOptions {
    Poll500 = 0,
    Poll1000 = 1,
    Poll2000 = 2,
    Poll4000 = 3,
}

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u16)]
pub enum DongleLedOptions {
    LedOff = 0,
    LedWhite = 1,
    LedBattery = 2,
}

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u16)]
pub enum MotionSyncOptions {
    SyncOff = 0,
    SyncOn = 1,
}

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u16)]
pub enum LodOptions {
    Lod1 = 0,
    Lod2 = 1,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct DpiOption(u16);

impl DpiOption {
    pub fn new(value: u16) -> Result<Self, &'static str> {
        if Self::is_valid(value) {
            Ok(DpiOption(value))
        } else {
            Err("Invalid DPI value")
        }
    }

    pub fn is_valid(value: u16) -> bool {
        (value >= 400 && value <= 6400) && (value % 100 == 0)
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn high_byte(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn low_byte(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}

impl TryFrom<u16> for DpiOption {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

pub struct CurrentSettings {
    pub polling_rate: PollingOptions,
    pub dongle_led: DongleLedOptions,
    pub lod: LodOptions,
    pub motion_sync: MotionSyncOptions,
    pub dpi: DpiOption,
}

impl CurrentSettings {
    pub fn to_code(&self) -> String {
        let other_settings = (self.polling_rate as u16)
            | ((self.dongle_led as u16) << 2)
            | ((self.motion_sync as u16) << 4)
            | ((self.lod as u16) << 5);

        format!("{:x}{:04}", other_settings, self.dpi.value())
    }

    pub fn from_u32(code: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let other_settings = (code >> 16) as u16;

        // Convert the code to a string and get the last 4 digits
        let code_str = format!("{:08X}", code);
        let dpi_value = code_str[4..].parse::<u16>()?;

        let polling_rate = extract_bits(other_settings, 0, 2);
        let dongle_led = extract_bits(other_settings, 2, 2);
        let motion_sync = extract_bits(other_settings, 4, 1);
        let lod = extract_bits(other_settings, 5, 1);
        let dpi_option = DpiOption::new(dpi_value);

        Ok(Self {
            polling_rate: PollingOptions::from_repr(polling_rate).ok_or("Invalid polling rate")?,
            dongle_led: DongleLedOptions::from_repr(dongle_led)
                .ok_or("Invalid dongle LED option")?,
            motion_sync: MotionSyncOptions::from_repr(motion_sync)
                .ok_or("Invalid motion sync option")?,
            lod: LodOptions::from_repr(lod).ok_or("Invalid LOD option")?,
            dpi: dpi_option?,
        })
    }
}

impl std::fmt::Display for CurrentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\n\
            Polling Rate: {:?}\n\
            DPI: {:?}\n\
            Dongle LED: {:?}\n\
            Motion Sync: {:?}\n\
            LOD: {:?}\n",
            self.polling_rate,
            self.dpi.value(),
            self.dongle_led,
            self.motion_sync,
            self.lod
        )
    }
}
