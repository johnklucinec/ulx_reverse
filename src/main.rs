use rusb::{Context, Error, UsbContext};

fn main() -> Result<(), Error> {
    let context = Context::new()?;
    const VID: u16 = 0x361d; // Finalmouse
    const PID: u16 = 0x0100; // UltralightX dongle

    let device = context.devices()?.iter().find(|device| {
        let device_desc = device.device_descriptor().unwrap();
        device_desc.vendor_id() == VID && device_desc.product_id() == PID
    });

    println!("Device found: {:?}", device);

    Ok(())
}
