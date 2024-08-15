use rusb::Context;
use ulx_reverse::device;
use ulx_reverse::types::*;
use ulx_reverse::utils::*;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let context = Context::new()?;

//     // Initialize DeviceInfo
//     let mut device_info = device::initialize_device_info(&context)?;

//     device_info.set_dpi(DpiOptions::Dpi1600)?;
//     device_info.set_polling_rate(PollingOptions::Poll4000)?;
//     device_info.set_motion_sync(false)?;
//     device_info.set_lod(LodOptions::Lod1)?;
//     device_info.set_dongle_led(DongleLedOptions::LedBattery)?;

//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = CurrentSettings {
        dpi: DpiOptions::Dpi1600,
        polling_rate: PollingOptions::Poll4000,
        motion_sync: MotionSyncOptions::SyncOff,
        lod: LodOptions::Lod1,
        dongle_led: DongleLedOptions::LedBattery,
    };

    let encoded_settings = settings.to_u32();
    println!("Encoded Settings: {:b}", encoded_settings);

    let hex_settings = binary_to_hex(encoded_settings);
    println!("Hex Settings: {:}", hex_settings);
    Ok(())
}
