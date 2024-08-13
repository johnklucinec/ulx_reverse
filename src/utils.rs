/* Constants for bit flags */
const MOUSE_POLLING_500: u32 = 1 << 0;
const MOUSE_POLLING_1000: u32 = 1 << 1;
const MOUSE_POLLING_2000: u32 = 1 << 2;
const MOUSE_POLLING_4000: u32 = 1 << 3;
const MOUSE_POLLING_8000: u32 = 1 << 4;
const DPI_400: u32 = 1 << 5;
const DPI_800: u32 = 1 << 6;
const DPI_1600: u32 = 1 << 7;
const DPI_3200: u32 = 1 << 8;
const DPI_6400: u32 = 1 << 9;
const LOD_1MM: u32 = 1 << 10;
const LOD_2MM: u32 = 1 << 11;
const LIGHT_OFF: u32 = 1 << 12;
const LIGHT_ON: u32 = 1 << 13;
const LIGHT_WHITE: u32 = 1 << 14;
const MOTION_SYNC_OFF: u32 = 1 << 15;
const MOTION_SYNC_ON: u32 = 1 << 16;

/*  */
pub fn parse_settings(input: u32) -> MouseSettings {
    MouseSettings {
        polling_rate: match input & 0b11111 {
            0b10000 => PollingOptions::Poll8000,
            0b01000 => PollingOptions::Poll4000,
            0b00100 => PollingOptions::Poll2000,
            0b00010 => PollingOptions::Poll1000,
            _ => PollingOptions::Poll500,
        },
        dpi: match (input >> 5) & 0b11111 {
            0b10000 => DpiOptions::Dpi6400,
            0b01000 => DpiOptions::Dpi3200,
            0b00100 => DpiOptions::Dpi1600,
            0b00010 => DpiOptions::Dpi800,
            _ => DpiOptions::Dpi400,
        },
        lod: if input & LOD_2MM != 0 { LodOptions::Lod2 } else { LodOptions::Lod1 },
        light: match (input >> 12) & 0b111 {
            0b100 => DongleLedOptions::LedWhite,
            0b010 => DongleLedOptions::LedBattery,
            _ => DongleLedOptions::LedOff,
        },
        motion_sync: input & MOTION_SYNC_ON != 0,
    }
}

pub fn binary_to_hex(binary: u32) -> String {
    format!("{:X}", binary)
}

pub fn hex_to_binary(hex: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(hex, 16)
}
