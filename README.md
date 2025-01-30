# zix - an `ls` alternative
## Overview
`zix` is a simple and fast alternative to the traditional Unix `ls` command, designed specifically for Windows. It lists directory contents with a focus on speed and simplicity, aiming to improve the user experience in Windows environments.

![zix gif](./assets/zix.gif)

## Features
- **Customizable output formats**
- **Cross-platform**: Primarily for Windows, but adaptable for others

## Installation


### Using Cargo (Rust's package manager)
You can easily install `zix` using Cargo by running the following command:

```bash
cargo install zix
```
### Linux (using curl)
You can quickly install zix using curl with our automated installation script:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/zix-rs/zix/refs/heads/main/scripts/install.sh | sh
```

### Windows (using powershell)
If you prefer using PowerShell, you can clone the repository and run an installation script to install zix:

Open PowerShell and run the following command to clone the repository and install zix:
powershell

```bash
powershell -c "irm https://raw.githubusercontent.com/zix-rs/zix/refs/heads/main/scripts/install.ps1|iex"
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
META OPTIONS:

    --help, -?
        print help
    --version, -v
        show version of six

DISPLAY OPTIONS:

    --list -l
        detailed list format
    --tree, -t
        recurse into directories as a tree

FILTERING AND SORTING OPTIONS:

    --all, -a
        show hidden and 'dot' files

LONG VIEW OPTIONS:

    --headers, -h
        add a header row to each column

## Contributing
Feel free to [contribute](./CONTRIBUTING.md) by submitting issues or pull requests.


## License
This project is licensed under the [MIT License](./LICENSE).
