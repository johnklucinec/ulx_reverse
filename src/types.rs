pub const VID: u16 = 0x361d;
pub const PID: u16 = 0x0100;
pub const COMMAND_ENDPOINT: u8 = 0x01;
pub const INTERRUPT_ENDPOINT: u8 = 0x81;
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(1000);

pub enum DpiOptions {
    Dpi400 = 400,
    Dpi800 = 800,
    Dpi1600 = 1600,
    Dpi3200 = 3200,
    Dpi6400 = 6400,
}

#[derive(PartialEq)]
pub enum PollingOptions {
    Poll500 = 500,
    Poll1000 = 1000,
    Poll2000 = 2000,
    Poll4000 = 4000,
}

pub enum LodOptions {
    Lod1 = 1,
    Lod2 = 2,
}

pub enum DongleLedOptions {
    LedOff = 0,
    LedWhite = 1,
    LedBattery = 2,
}

pub struct CurrentSettings {
    pub dpi: DpiOptions,
    pub polling_rate: PollingOptions,
    pub motion_sync: bool,
    pub lod: LodOptions,
    pub dongle_led: DongleLedOptions,
}
