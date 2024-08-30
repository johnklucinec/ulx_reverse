use crate::utils::extract_bits;
use strum_macros::FromRepr;

pub const VID: u16 = 0x361d;
pub const PID: u16 = 0x0100;
pub const COMMAND_ENDPOINT: u8 = 0x01;
pub const INTERRUPT_ENDPOINT: u8 = 0x81;
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(1000);

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum PollingOptions {
    Poll500 = 1 << 0,
    Poll1000 = 1 << 1,
    Poll2000 = 1 << 2,
    Poll4000 = 1 << 3,
    // Poll8000 = 1 << 4, // Not supported by non beta firmware.
}

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum DpiOptions {
    Dpi400 = 1 << 5,
    Dpi800 = 1 << 6,
    Dpi1600 = 1 << 7,
    Dpi3200 = 1 << 8,
    Dpi6400 = 1 << 9,
}

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum DongleLedOptions {
    LedOff = 1 << 10,
    LedWhite = 1 << 11,
    LedBattery = 1 << 12,
}

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum MotionSyncOptions {
    SyncOff = 1 << 13,
    SyncOn = 1 << 14,
}

#[derive(FromRepr, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum LodOptions {
    Lod1 = 1 << 15,
    Lod2 = 1 << 16,
}

pub struct CurrentSettings {
    pub polling_rate: PollingOptions,
    pub dpi: DpiOptions,
    pub dongle_led: DongleLedOptions,
    pub lod: LodOptions,
    pub motion_sync: MotionSyncOptions,
}

impl CurrentSettings {
    pub fn to_u32(&self) -> u32 {
        self.polling_rate as u32
            | self.dpi as u32
            | self.dongle_led as u32
            | self.motion_sync as u32
            | self.lod as u32
    }

    pub fn from_u32(code: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let polling_rate =
            PollingOptions::from_repr(extract_bits(code, 0, 4)).ok_or("Invalid polling rate")?;

        let dpi = DpiOptions::from_repr(extract_bits(code, 5, 5)).ok_or("Invalid DPI")?;

        let dongle_led =
            DongleLedOptions::from_repr(extract_bits(code, 10, 3)).ok_or("Invalid dongle LED")?;

        let motion_sync =
            MotionSyncOptions::from_repr(extract_bits(code, 13, 2)).ok_or("Invalid motion sync")?;

        let lod = LodOptions::from_repr(extract_bits(code, 15, 2)).ok_or("Invalid LOD")?;

        Ok(Self {
            polling_rate,
            dpi,
            dongle_led,
            motion_sync,
            lod,
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
            self.polling_rate, self.dpi, self.dongle_led, self.motion_sync, self.lod
        )
    }
}
