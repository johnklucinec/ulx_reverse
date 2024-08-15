pub const VID: u16 = 0x361d;
pub const PID: u16 = 0x0100;
pub const COMMAND_ENDPOINT: u8 = 0x01;
pub const INTERRUPT_ENDPOINT: u8 = 0x81;
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(1000);

#[derive(PartialEq, Copy, Clone)]
pub enum PollingOptions {
    Poll500 = 1 << 0,
    Poll1000 = 1 << 1,
    Poll2000 = 1 << 2,
    Poll4000 = 1 << 3,
    // Poll8000 = 1 << 4, // Not supported by non beta firmware.
}

#[derive(PartialEq, Copy, Clone)]
pub enum DpiOptions {
    Dpi400 = 1 << 5,
    Dpi800 = 1 << 6,
    Dpi1600 = 1 << 7,
    Dpi3200 = 1 << 8,
    Dpi6400 = 1 << 9,
}

#[derive(PartialEq, Copy, Clone)]
pub enum LodOptions {
    Lod1 = 1 << 10,
    Lod2 = 1 << 11,
}

#[derive(PartialEq, Copy, Clone)]
pub enum DongleLedOptions {
    LedOff = 1 << 12,
    LedWhite = 1 << 13,
    LedBattery = 1 << 14,
}

#[derive(PartialEq, Copy, Clone)]
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
