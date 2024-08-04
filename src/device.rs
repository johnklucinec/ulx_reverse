use crate::commands::process_command;
use crate::types::*;
use rusb::{Context, Device, DeviceHandle, Error, UsbContext};

pub struct DeviceInfo {
    pub device: Device<Context>,
    pub handle: DeviceHandle<Context>,
    pub interface_number: u8,
    pub current_settings: CurrentSettings,
}

impl DeviceInfo {
    pub fn set_motion_sync(&mut self, motion_sync: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut command: Vec<u8> = vec![0u8; 64];

        command[0] = 0x04;
        command[1] = 0x03;
        command[2] = 0x92;
        command[3] = 0x01;

        if motion_sync {
            command[4] = 0x01; // This byte turns on Motion Sync
        }

        // Attempt to process the command
        if let Err(e) = process_command(self, &command) {
            eprintln!("Failed to process command: {}", e);
            return Err(e);
        }

        // Update the current settings if the command was successful
        self.current_settings.motion_sync = motion_sync;

        Ok(())
    }
}

pub fn initialize_device_info(context: &Context) -> Result<DeviceInfo, Box<dyn std::error::Error>> {
    let device = find_device(context)?.ok_or("Device not found")?;
    let handle = device.open()?;
    let config = device.active_config_descriptor()?;
    let interface_number = config
        .interfaces()
        .next()
        .ok_or("No interfaces found")?
        .number();

    // Initialize with default settings (TODO: Read from device)
    let current_settings = CurrentSettings {
        dpi: DpiOptions::Dpi1600,
        polling_rate: PollingOptions::Poll4000,
        motion_sync: false,
        lod: LodOptions::Lod1,
        dongle_led: DongleLedOptions::LedBattery,
    };

    Ok(DeviceInfo {
        device,
        handle,
        interface_number,
        current_settings,
    })
}

fn find_device(context: &Context) -> Result<Option<Device<Context>>, Error> {
    let device = context.devices()?.iter().find(|device| {
        let device_desc = device.device_descriptor().unwrap();
        device_desc.vendor_id() == VID && device_desc.product_id() == PID
    });
    Ok(device)
}
