use rusb::Context;
use ulx_reverse::device;
use ulx_reverse::types::DpiOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;

    // Initialize DeviceInfo
    let mut device_info = device::initialize_device_info(&context)?;

    device_info.set_dpi(DpiOptions::Dpi1600)?;

    Ok(())
}
