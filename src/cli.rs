use crate::{
    device::{self, DeviceInfo},
    types::*,
};
use rusb::Context;
use std::io::{self, Write};

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::new()?;
    let mut device_info = device::initialize_device_info(&context)?;

    println!("ULX CLI Tool");

    loop {
        println!("\nPlease choose an option:");
        println!("1. Import a settings code");
        println!("2. Export a settings code");
        println!("3. Change Polling Rate");
        println!("4. Change DPI");
        println!("5. Change Dongle LED");
        println!("6. Change Lift Off Distance");
        println!("7. Change Motion Sync");
        println!("8. Exit");

        let choice = get_user_input("Enter your choice (1-8): ");

        let result = match choice.as_str() {
            "1" => import_settings(&mut device_info),
            "2" => export_settings(&mut device_info),
            "3" => change_polling_rate(&mut device_info),
            "4" => change_dpi(&mut device_info),
            "5" => change_dongle_led(&mut device_info),
            "6" => change_lod(&mut device_info),
            "7" => change_motion_sync(&mut device_info),
            "8" => break,
            _ => {
                println!("Invalid choice. Please try again.");
                continue;
            }
        };

        if let Err(e) = result {
            println!("Failed to process command: {}", e);
        }
    }
    Ok(())
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn import_settings(device_info: &mut DeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    let settings = get_user_input("Enter the settings code: ");
    let code = u32::from_str_radix(&settings, 16)?;
    device_info.current_settings = CurrentSettings::from_u32(code)?;

    device_info.set_polling_rate(device_info.current_settings.polling_rate)?;
    device_info.set_dpi(device_info.current_settings.dpi)?;
    device_info.set_lod(device_info.current_settings.lod)?;
    device_info.set_dongle_led(device_info.current_settings.dongle_led)?;
    device_info.set_motion_sync(device_info.current_settings.motion_sync)?;

    println!("\nUpdated mouse settings: {}", device_info.current_settings);

    Ok(())
}

fn export_settings(device_info: &mut DeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    let settings = device_info.current_settings.to_code();

    println!("\nCurrent Mouse Settings: {}", device_info.current_settings);
    println!("Share Code: {}", settings);

    Ok(())
}

fn change_polling_rate(device_info: &mut DeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    let polling_rate =
        get_user_input("Choose a polling rate (1: 500hz, 2: 1000hz, 3: 2000hz, 4: 4000hz): ");
    let polling_rate = match polling_rate.as_str() {
        "1" => PollingOptions::Poll500,
        "2" => PollingOptions::Poll1000,
        "3" => PollingOptions::Poll2000,
        "4" => PollingOptions::Poll4000,
        _ => {
            println!("Invalid option. Please try again.");
            return Ok(());
        }
    };

    device_info.set_polling_rate(polling_rate)?;
    println!("Polling rate updated.",);

    Ok(())
}

fn change_dpi(device_info: &mut DeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    let dpi_input = get_user_input("Choose a DPI (400-6400, multiple of 100): ");
    let dpi_value = match dpi_input.parse::<u16>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return Ok(());
        }
    };

    match DpiOption::new(dpi_value) {
        Ok(dpi_option) => {
            device_info.set_dpi(dpi_option)?;
            println!("DPI updated to {}.", dpi_value);
        }
        Err(err) => {
            println!("Invalid DPI value: {}. Please try again.", err);
        }
    }

    Ok(())
}

fn change_dongle_led(device_info: &mut DeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    let dongle_led = get_user_input(
        "Choose a dongle LED Option (1: Battery Level Indication, 2: Solid White, 3: Off): ",
    );
    let dongle_led = match dongle_led.as_str() {
        "1" => DongleLedOptions::LedBattery,
        "2" => DongleLedOptions::LedWhite,
        "3" => DongleLedOptions::LedOff,
        _ => {
            println!("Invalid option. Please try again.");
            return Ok(());
        }
    };

    device_info.set_dongle_led(dongle_led)?;
    println!("Dongle LED updated.");

    Ok(())
}

fn change_lod(device_info: &mut DeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    let lod = get_user_input("Choose a lift off distance (1: 1mm, 2: 2mm): ");
    let lod = match lod.as_str() {
        "1" => LodOptions::Lod1,
        "2" => LodOptions::Lod2,
        _ => {
            println!("Invalid option. Please try again.");
            return Ok(());
        }
    };

    device_info.set_lod(lod)?;
    println!("Lift off distance updated.");

    Ok(())
}

fn change_motion_sync(device_info: &mut DeviceInfo) -> Result<(), Box<dyn std::error::Error>> {
    let motion_sync = get_user_input("Choose a motion sync option (1: Off, 2: On): ");
    let motion_sync = match motion_sync.as_str() {
        "1" => MotionSyncOptions::SyncOff,
        "2" => MotionSyncOptions::SyncOn,
        _ => {
            println!("Invalid option. Please try again.");
            return Ok(());
        }
    };

    device_info.set_motion_sync(motion_sync)?;
    println!("Motion sync updated.");

    Ok(())
}
