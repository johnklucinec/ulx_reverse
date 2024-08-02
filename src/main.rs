use rusb::{Context, Device, DeviceHandle, Error, UsbContext};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;
    //const VID: u16 = 0x361d; // Finalmouse
    //const PID: u16 = 0x0100; // UltralightX dongle

    const VID: u16 = 0x3151; // Monsgeek
    const PID: u16 = 0x4015; // Keyboard?

    // Find the device
    let device = context.devices()?.iter().find(|device| {
        let device_desc = device.device_descriptor().unwrap();
        device_desc.vendor_id() == VID && device_desc.product_id() == PID
    });

    match device {
        Some(device) => {
            println!("Device found: {:?}", device);

            let mut handle = match device.open() {
                Ok(handle) => {
                    println!("Device opened");
                    handle
                }
                Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error + 'static>),
            };

            // Claim the first interface
            let interface_number = 0;
            if let Err(e) = handle.claim_interface(interface_number) {
                eprintln!("Failed to claim interface {}: {}", interface_number, e);
                return Err(Box::new(e) as Box<dyn std::error::Error + 'static>);
            }

            // Release the interface
            if let Err(e) = handle.release_interface(interface_number) {
                eprintln!("Failed to release interface {}: {}", interface_number, e);
                return Err(Box::new(e) as Box<dyn std::error::Error + 'static>);
            }
        }
        None => {
            println!("Device not found!");
        }
    }

    Ok(())
}
