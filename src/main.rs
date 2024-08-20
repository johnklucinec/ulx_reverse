use rusb::Context;
use ulx_reverse::device;
use ulx_reverse::types::*;
use ulx_reverse::utils::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let settings = CurrentSettings {
    //     dpi: DpiOptions::Dpi1600,
    //     polling_rate: PollingOptions::Poll4000,
    //     motion_sync: MotionSyncOptions::SyncOff,
    //     lod: LodOptions::Lod1,
    //     dongle_led: DongleLedOptions::LedBattery,
    // };

    // This number is the result of running settings.to_u32()
    let settings: u32 = 83080;

    let mut temp_settings = CurrentSettings {
        dpi: DpiOptions::Dpi400,
        polling_rate: PollingOptions::Poll500,
        motion_sync: MotionSyncOptions::SyncOn,
        lod: LodOptions::Lod2,
        dongle_led: DongleLedOptions::LedOff,
    };

    println!("Before validation: {}", temp_settings);

    let validate_polling: Result<PollingOptions, _> = extract_bits(settings, 0, 4).try_into();
    temp_settings.polling_rate = validate_polling.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let validate_dpi: Result<DpiOptions, _> = extract_bits(settings, 5, 5).try_into();
    temp_settings.dpi = validate_dpi.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let validate_lod: Result<LodOptions, _> = extract_bits(settings, 10, 2).try_into();
    temp_settings.lod = validate_lod.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let validate_led: Result<DongleLedOptions, _> = extract_bits(settings, 12, 3).try_into();
    temp_settings.dongle_led = validate_led.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let validate_sync: Result<MotionSyncOptions, _> = extract_bits(settings, 15, 2).try_into();
    temp_settings.motion_sync = validate_sync.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    println!("Settings after validating and updating: {}", temp_settings);

    Ok(())
}
