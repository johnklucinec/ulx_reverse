//use rusb::Context;
//use ulx_reverse::device;
use ulx_reverse::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Valid settings code: 83080
    // Invalid settings code: 83081
    let code: u32 = 83080;

    let settings = CurrentSettings::from_u32(code)?;

    println!("Settings from code: {}", settings);

    Ok(())
}
