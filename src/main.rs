use rusb::Context;
use ulx_reverse::device;
use ulx_reverse::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;

    // Initialize DeviceInfo
    let mut device_info = device::initialize_device_info(&context)?;

    device_info.set_dpi(DpiOptions::Dpi1600)?;
    device_info.set_polling_rate(PollingOptions::Poll4000)?;
    device_info.set_motion_sync(false)?;
    device_info.set_lod(LodOptions::Lod1)?;
    device_info.set_dongle_led(DongleLedOptions::LedBattery)?;

    Ok(())
}
