# ULX-Reverse

ULX-Reverse is a simple, cross-platform CLI application that reverse-engineers and enhances the functionality of Finalmouse's Xpanel software. 

***This project is not affiliated with Finalmouse.***

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.55+-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue.svg)](https://github.com/johnklucinec/ulx_reverse)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Custom DPI Settings](#custom-dpi-settings)
- [Settings Share Codes](#settings-share-codes)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Cross-Platform Compatibility**: Works seamlessly on macOS and Linux (Dealing with Windows driver issues).
- **Compact Size**: The entire application compiles to just 300KB on Windows.
- **Custom DPI Settings**: Set your mouse DPI to any value, not just presets.
- **Settings Share Codes**: Easily share and apply mouse configurations using unique codes.
- **Full Functionality**: Replicates all features of the original software, except firmware updates.

## Installation
> [!NOTE]  
> You only have to do these steps if you plan on compiling the code yourself.

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

2. Clone the repository:
   git clone https://github.com/johnklucinec/ulx_reverse.git

3. Navigate to the project directory:
   cd ulx_reverse

4. Build the project:
   cargo build --release

5. The compiled binary will be available in the `target/release` directory.

## Usage
> [!IMPORTANT] 
> According to [libusb](https://github.com/libusb/libusb/wiki/Windows#driver-installation) wiki under How to use libusb on Windows|Known Restrictions:
> 
> *HID keyboards and mice cannot be accessed using the native HID driver as Windows reserves exclusive access to them.*
>
> The Windows executable will remain available, but it will be non-functional until an alternative solution is implemented.

To run ULX-Reverse, use the following command:

`./ulx_reverse`

> If you are on Linux, make sure to add these [Finalmouse ULX udev rules](https://github.com/teamfinalmouse/xpanel-linux-permissions/tree/main).
> On macOS, there are no udev rules, so you have to run the app with sudo privileges. 

## Custom DPI Settings
ULX-Reverse allows you to set custom DPI values beyond the presets offered by the official software:

<div align="center">
    <img src="https://github.com/user-attachments/assets/ea98fe70-c51c-487c-936c-cc9ffe0a6798" width="50%" alt="Custom DPI Settings">
</div>

## Settings Share Codes

Share your mouse configuration easily with others:

1. Generate a share code:
<div align="center">
    <img src="https://github.com/user-attachments/assets/490112cd-e72d-4bb1-84b0-083218989e01" width="25%" alt="Generate Custom Share Code">
</div>

3. Apply a shared configuration:
<div align="center">
    <img src="https://github.com/user-attachments/assets/a98564c2-a324-4e68-9829-1c440522f6eb" width="25%" alt="Apply Custom Share Code">
</div>

> [!NOTE]  
> The default settings for generating a share code are 4000 Hz, 1600 DPI, motion sync off, battery level indication on, and 1 mm LOD. Adjust settings to personal preference, then select option 2 to generate the code. You can also change the DPI on the fly by modifying the last four digits of the share code.

## Contributing

We welcome contributions to ULX-Reverse! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with a clear commit message.
4. Push your changes to your fork.
5. Submit a pull request to the main repository.

Please ensure your code adheres to the project's coding standards and includes appropriate tests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---


