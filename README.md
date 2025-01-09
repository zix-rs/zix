# zix - an `ls` alternative
## Overview
`zix` is a simple and fast alternative to the traditional Unix `ls` command, designed specifically for Windows. It lists directory contents with a focus on speed and simplicity, aiming to improve the user experience in Windows environments.

![zix gif](./web/public/zix.gif)

## Features
- **Fast directory listing**
- **Customizable output formats**
- **Cross-platform**: Primarily for Windows, but adaptable for others
- **Lightweight and efficient**

## Installation

### Using the Installer Executable (Recommended)
The easiest way to install zix is by downloading the installer executable from the GitHub Releases page.
1. Download the latest .exe file for your system.
2. Run the installer; it will automatically download zix, install the command, and add it to your system's PATH for easy access.
[Click here for install zix](https://github.com/zix-rs/zix/releases/download/v0.0.5/zix-installer.exe)

### Using Cargo (Rust's package manager)
You can easily install `zix` using Cargo by running the following command:

```bash
cargo install zix
```

### Using PowerShell
If you prefer using PowerShell, you can clone the repository and run an installation script to install zix:

Open PowerShell and run the following command to clone the repository and install zix:
powershell

```bash
git clone https://github.com/zix-rs/zix.git
cd zix
.\installer.ps1
```

## Usage
Basic usage:
```bash
zx [options] [directory]
```

Example:
```bash
zx -l
```

## Options
- `-l`: Detailed list format
- `-a`: Include hidden files

## Contributing
Feel free to [contribute](./CONTRIBUTING.md) by submitting issues or pull requests.


## License
This project is licensed under the [MIT License](./LICENSE).
