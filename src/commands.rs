use crate::device::DeviceInfo;
use crate::types::COMMAND_ENDPOINT;
use crate::types::INTERRUPT_ENDPOINT;
use crate::types::TIMEOUT;
use rusb::{Context, DeviceHandle};

pub fn process_command(
    device_info: &mut DeviceInfo,
    command: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Device found: {:?}", device_info.device);

    match device_info
        .handle
        .kernel_driver_active(device_info.interface_number)
    {
        Ok(active) => {
            if active {
                device_info
                    .handle
                    .detach_kernel_driver(device_info.interface_number)?;
                println!("Kernel driver detached successfully");

                device_info
                    .handle
                    .claim_interface(device_info.interface_number)?;

                send_command(&device_info.handle, command)?;
                println!("Command sent successfully");

                device_info
                    .handle
                    .release_interface(device_info.interface_number)?;
                device_info
                    .handle
                    .attach_kernel_driver(device_info.interface_number)?;
            } else {
                eprintln!("Kernel driver is not active.");
            }
        }
        Err(e) => eprintln!("Error checking kernel driver: {}", e),
    }

    Ok(())
}

fn send_command(
    handle: &DeviceHandle<Context>,
    command: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 64];
    let size = handle.write_interrupt(COMMAND_ENDPOINT, command, TIMEOUT)?;
    println!("Wrote {} bytes", size);

    let size = handle.read_interrupt(INTERRUPT_ENDPOINT, &mut buffer, TIMEOUT)?;
    println!("Read {} bytes", size);

    Ok(())
}
