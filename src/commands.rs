use crate::device::DeviceInfo;
use crate::types::COMMAND_ENDPOINT;
#[cfg(debug_assertions)]
use crate::types::INTERRUPT_ENDPOINT;
use crate::types::TIMEOUT;
use rusb::{Context, DeviceHandle};

pub fn process_command(
    device_info: &mut DeviceInfo,
    command: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
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

                device_info
                    .handle
                    .claim_interface(device_info.interface_number)?;

                let send_result = send_command(&device_info.handle, command);

                // Always release interface and reattach kernel driver
                let release_result = device_info
                    .handle
                    .release_interface(device_info.interface_number);
                let reattach_result = device_info
                    .handle
                    .attach_kernel_driver(device_info.interface_number);

                // This will return early if there's an error
                send_result?;

                // Check release_interface and reattach_kernel_driver results
                release_result?;
                reattach_result?;

                #[cfg(debug_assertions)]
                eprintln!("Command sent successfully");
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
    // Send the command that was passed in.
    handle.write_interrupt(COMMAND_ENDPOINT, command, TIMEOUT)?;

    #[cfg(debug_assertions)] // Only run this block if the program is in debug mode.
    {
        let mut first_buffer = [0u8; 64];
        let mut second_buffer = [0u8; 64];

        // Read the first response.
        handle.read_interrupt(INTERRUPT_ENDPOINT, &mut first_buffer, TIMEOUT)?;
        println!("{:?}", first_buffer);

        // Read the second response.
        handle.read_interrupt(INTERRUPT_ENDPOINT, &mut second_buffer, TIMEOUT)?;
        println!("{:?}", second_buffer);
    }

    Ok(())
}
