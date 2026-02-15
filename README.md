# Hardware Info Dashboard (Rust/Tauri)

A modern, cross-platform hardware information dashboard built with **Rust**, **Tauri v2**, and **React**.

![Hardware Info Dashboard](public/vite.svg)

## Features

- **CPU**: Brand, speed, cores, cache, real-time load & frequency
- **Memory**: Total, used, available, swap, detailed SPD slot info (DDR type, speed, manufacturer)
- **Graphics**: GPU model, vendor, VRAM (where available)
- **Storage**: Disk layout, partitions, usage
- **Network**: Interfaces, MAC addresses
- **Sensors**: CPU temperatures
- **System**: OS version, kernel, uptime, BIOS/UEFI info

## Tech Stack

- **Frontend**: React, TypeScript, Vite, CSS Modules
- **Backend**: Rust (Tauri), `sysinfo` crate, `wmi` crate (Windows)
- **Platform**: Linux (GTK/WebKit), Windows (WebView2)

## Prerequisites

### Linux
- `libwebkit2gtk-4.0-dev` or `libwebkit2gtk-4.1-dev`
- `build-essential`
- `curl`, `wget`, `file`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`
- **Runtime Dependencies**:
  - `lspci` (part of `pciutils`) for GPU info
  - `dmidecode` (usually pre-installed, may require sudo for full details) for Motherboard/RAM slot info

### Windows
- Microsoft Visual Studio C++ Build Tools

## Development

1. Install dependencies:
   ```bash
   npm install
   ```

2. Run in development mode:
   ```bash
   npm run tauri dev
   ```

## Build

Build a standalone binary (AppImage/deb on Linux, .exe/MSI on Windows):
```bash
npm run tauri build
```
The output will be in `src-tauri/target/release/bundle/`.

> **Note**: To build the Windows version (`.exe`/`.msi`), you must run this command on a Windows machine. Tauri does not support cross-compilation from Linux out of the box.

## Permissions & Privileges

Some hardware data (specifically **SPD** memory details and **Mainboard** serial numbers) requires root privileges on Linux.

- The app uses `pkexec` to prompt for your password once at startup to read this data.
- If you cancel the prompt, the app will continue to run, but those specific fields will be empty.
- To avoid the prompt entirely, you can run the application as root:
  ```bash
  sudo ./hardware-info-rs
  ```

## License

MIT
