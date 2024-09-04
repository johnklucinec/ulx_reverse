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
    pub fn set_motion_sync(
        &mut self,
        motion_sync: MotionSyncOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut command: Vec<u8> = vec![0u8; 64];

        command[0] = 0x04;
        command[1] = 0x03;
        command[2] = 0x92;
        command[3] = 0x01;

        if motion_sync == MotionSyncOptions::SyncOn {
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

    pub fn set_dpi(&mut self, dpi: DpiOption) -> Result<(), Box<dyn std::error::Error>> {
        let mut command: Vec<u8> = vec![0u8; 64];

        command[0] = 0x04;
        command[1] = 0x04;
        command[2] = 0x90;
        command[3] = 0x02;

        // Set the DPI bytes directly from the DpiOption
        command[4] = dpi.low_byte();
        println!("First bit: {}", command[4]);
        command[5] = dpi.high_byte();
        println!("Second bit: {}", command[5]);

        // Attempt to process the command
        if let Err(e) = process_command(self, &command) {
            eprintln!("Failed to process command: {}", e);
            return Err(e);
        }

        // Update the current settings if the command was successful
        self.current_settings.dpi = dpi;

        Ok(())
    }

    pub fn set_polling_rate(
        &mut self,
        polling_rate: PollingOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut command: Vec<u8> = vec![0u8; 64];

        command[0] = 0x04;
        command[1] = 0x04;
        command[2] = 0x91;
        command[3] = 0x02;

        match polling_rate {
            PollingOptions::Poll500 => {
                command[4] = 0xf4;
                command[5] = 0x01;
            }
            PollingOptions::Poll1000 => {
                command[4] = 0xe8;
                command[5] = 0x03;
            }
            PollingOptions::Poll2000 => {
                command[4] = 0xd0;
                command[5] = 0x07;
            }
            PollingOptions::Poll4000 => {
                command[4] = 0xa0;
                command[5] = 0x0f;
            }
        }

        // Attempt to process the command
        if let Err(e) = process_command(self, &command) {
            eprintln!("Failed to process command: {}", e);
            return Err(e);
        }

        // Update the current settings if the command was successful
        self.current_settings.polling_rate = polling_rate;

        Ok(())
    }

    pub fn set_lod(&mut self, lod: LodOptions) -> Result<(), Box<dyn std::error::Error>> {
        let mut command: Vec<u8> = vec![0u8; 64];

        command[0] = 0x04;
        command[1] = 0x03;
        command[2] = 0x95;
        command[3] = 0x01;

        match lod {
            LodOptions::Lod1 => {
                command[4] = 0x01;
            }
            LodOptions::Lod2 => {
                command[4] = 0x02;
            }
        }

        // Attempt to process the command
        if let Err(e) = process_command(self, &command) {
            eprintln!("Failed to process command: {}", e);
            return Err(e);
        }

        // Update the current settings if the command was successful
        self.current_settings.lod = lod;

        Ok(())
    }

    pub fn set_dongle_led(
        &mut self,
        led_option: DongleLedOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut command: Vec<u8> = vec![0u8; 64];

        command[0] = 0x04;
        command[1] = 0x03;
        command[2] = 0x94;
        command[3] = 0x01;

        match led_option {
            DongleLedOptions::LedOff => {
                command[4] = 0x00;
            }
            DongleLedOptions::LedBattery => {
                command[4] = 0x01;
            }
            DongleLedOptions::LedWhite => {
                command[4] = 0x02;
            }
        }

        // Attempt to process the command
        if let Err(e) = process_command(self, &command) {
            eprintln!("Failed to process command: {}", e);
            return Err(e);
        }

        // Update the current settings if the command was successful
        self.current_settings.dongle_led = led_option;

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
        polling_rate: PollingOptions::Poll4000,
        motion_sync: MotionSyncOptions::SyncOff,
        lod: LodOptions::Lod1,
        dongle_led: DongleLedOptions::LedBattery,
        dpi: DpiOption::new(1600).unwrap(), // Won't panic.
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
