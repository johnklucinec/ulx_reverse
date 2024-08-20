use std::{convert::TryFrom, fmt};

pub const VID: u16 = 0x361d;
pub const PID: u16 = 0x0100;
pub const COMMAND_ENDPOINT: u8 = 0x01;
pub const INTERRUPT_ENDPOINT: u8 = 0x81;
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(1000);

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum PollingOptions {
    Poll500 = 1 << 0,
    Poll1000 = 1 << 1,
    Poll2000 = 1 << 2,
    Poll4000 = 1 << 3,
    // Poll8000 = 1 << 4, // Not supported by non beta firmware.
}

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum DpiOptions {
    Dpi400 = 1 << 5,
    Dpi800 = 1 << 6,
    Dpi1600 = 1 << 7,
    Dpi3200 = 1 << 8,
    Dpi6400 = 1 << 9,
}

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum LodOptions {
    Lod1 = 1 << 10,
    Lod2 = 1 << 11,
}

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum DongleLedOptions {
    LedOff = 1 << 12,
    LedWhite = 1 << 13,
    LedBattery = 1 << 14,
}

#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum MotionSyncOptions {
    SyncOn = 1 << 15,
    SyncOff = 1 << 16,
}

pub struct CurrentSettings {
    pub dpi: DpiOptions,
    pub polling_rate: PollingOptions,
    pub motion_sync: MotionSyncOptions,
    pub lod: LodOptions,
    pub dongle_led: DongleLedOptions,
}

impl CurrentSettings {
    pub fn to_u32(&self) -> u32 {
        let mut result: u32 = 0;

        result |= self.dpi as u32;
        result |= self.polling_rate as u32;
        result |= self.motion_sync as u32;
        result |= self.lod as u32;
        result |= self.dongle_led as u32;

        result
    }
}

impl fmt::Display for CurrentSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\
            DPI: {:?}\n\
            Polling Rate: {:?}\n\
            Motion Sync: {:?}\n\
            LOD: {:?}\n\
            Dongle LED: {:?}
            \n",
            self.dpi, self.polling_rate, self.motion_sync, self.lod, self.dongle_led
        )
    }
}

impl TryFrom<u32> for PollingOptions {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == PollingOptions::Poll500 as u32 => Ok(PollingOptions::Poll500),
            x if x == PollingOptions::Poll1000 as u32 => Ok(PollingOptions::Poll1000),
            x if x == PollingOptions::Poll2000 as u32 => Ok(PollingOptions::Poll2000),
            x if x == PollingOptions::Poll4000 as u32 => Ok(PollingOptions::Poll4000),
            // x if x == PollingOptions::Poll8000 as u32 => Ok(PollingOptions::Poll8000),
            _ => Err("Invalid polling rate"),
        }
    }
}

impl TryFrom<u32> for DpiOptions {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == DpiOptions::Dpi400 as u32 => Ok(DpiOptions::Dpi400),
            x if x == DpiOptions::Dpi800 as u32 => Ok(DpiOptions::Dpi800),
            x if x == DpiOptions::Dpi1600 as u32 => Ok(DpiOptions::Dpi1600),
            x if x == DpiOptions::Dpi3200 as u32 => Ok(DpiOptions::Dpi3200),
            x if x == DpiOptions::Dpi6400 as u32 => Ok(DpiOptions::Dpi6400),
            _ => Err("Invalid DPI"),
        }
    }
}

impl TryFrom<u32> for LodOptions {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == LodOptions::Lod1 as u32 => Ok(LodOptions::Lod1),
            x if x == LodOptions::Lod2 as u32 => Ok(LodOptions::Lod2),
            _ => Err("Invalid LOD"),
        }
    }
}

impl TryFrom<u32> for DongleLedOptions {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == DongleLedOptions::LedOff as u32 => Ok(DongleLedOptions::LedOff),
            x if x == DongleLedOptions::LedWhite as u32 => Ok(DongleLedOptions::LedWhite),
            x if x == DongleLedOptions::LedBattery as u32 => Ok(DongleLedOptions::LedBattery),
            _ => Err("Invalid dongle LED option"),
        }
    }
}

impl TryFrom<u32> for MotionSyncOptions {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == MotionSyncOptions::SyncOn as u32 => Ok(MotionSyncOptions::SyncOn),
            x if x == MotionSyncOptions::SyncOff as u32 => Ok(MotionSyncOptions::SyncOff),
            _ => Err("Invalid motion sync option"),
        }
    }
}
