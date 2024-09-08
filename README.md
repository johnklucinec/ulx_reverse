# ULX-Reverse

ULX-Reverse is a simple, cross-platform CLI application that reverse-engineers and enhances the functionality of Finalmouse's Xpanel mouse software. This project aims to provide users with greater control and customization options for their ULX mouse.

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

- **Cross-Platform Compatibility**: Works seamlessly on Windows, macOS, and Linux.
- **Compact Size**: The entire application compiles to just 300KB on Windows.
- **Custom DPI Settings**: Set your mouse DPI to any value, not just presets.
- **Settings Share Codes**: Easily share and apply mouse configurations using unique codes.
- **Full Functionality**: Replicates all features of the original software, except firmware updates.

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

2. Clone the repository:
   git clone https://github.com/johnklucinec/ulx_reverse.git

3. Navigate to the project directory:
   cd ulx_reverse

4. Build the project:
   cargo build --release

5. The compiled binary will be available in the `target/release` directory.

> [!NOTE]  
> If you are on Linux, make sure to add these [Finalmouse ULX udev rules](https://github.com/teamfinalmouse/xpanel-linux-permissions/tree/main)


## Usage

To run ULX-Reverse, use the following command:
`./ulx_reverse`


## Custom DPI Settings
ULX-Reverse allows you to set custom DPI values beyond the presets offered by the official software:

Replace `[VALUE]` with your desired DPI setting.

## Settings Share Codes

Share your mouse configuration easily with others:

1. Generate a share code:
2. Apply a shared configuration:


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

<p align="center">
Made by John Klucinec
</p>




