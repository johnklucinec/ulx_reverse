use rusb::Context;
use ulx_reverse::device;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;

    // Initialize DeviceInfo
    let mut device_info = device::initialize_device_info(&context)?;

    // Set Motion Sync
    device_info.set_motion_sync(false)?;

    Ok(())
}
