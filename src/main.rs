use rusb::{
    Context, Device, DeviceHandle, EndpointDescriptors, Error, Interface, InterfaceDescriptors,
    UsbContext,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;
    const VID: u16 = 0x361d; // Finalmouse
    const PID: u16 = 0x0100; // UltralightX dongle

    const COMMAND_ENDPOINT: u8 = 0x01;
    const INTERRUPT_ENDPOINT: u8 = 0x81;

    // Find the device
    let device = context.devices()?.iter().find(|device| {
        let device_desc = device.device_descriptor().unwrap();
        device_desc.vendor_id() == VID && device_desc.product_id() == PID
    });

    match device {
        Some(device) => {
            println!("Device found: {:?}", device);

            let mut handle = device
                .open()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + 'static>)?;
            println!("Device opened");

            let config_descriptor = device.active_config_descriptor();

            for interface in config_descriptor {
                for descriptor in interface.interfaces() {
                    let mut endpoints = descriptor.descriptors();
                    let control_interface_opt =
                        endpoints.find(|endpoint| endpoint.sub_class_code() == 1);

                    if let Some(control_interface) = control_interface_opt {
                        let _command_endpoint = control_interface
                            .endpoint_descriptors()
                            .find(|ep| ep.address() == COMMAND_ENDPOINT);

                        let _command_endpoint = control_interface
                            .endpoint_descriptors()
                            .find(|ep| ep.address() == INTERRUPT_ENDPOINT);
                    } else {
                        println!("Control interface not found");
                    }
                }
            }
        }
        None => {
            println!("Device not found!");
        }
    }

    Ok(())
}
