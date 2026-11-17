# waycalc
Simple calculator widget for Waybar written in Rust using GTK4 library and `gtk4-layer-shell`.
## Requirements
Rust and Cargo
- GTK4
- `gtk4-layer-shell`
- A Wayland compositor with layer-shell support

Install dependencies on Arch Linux:
```bash
sudo pacman -S --needed base-devel git rust gtk4 gtk4-layer-shell
```
## Install
### Build from source
```bash
git clone https://github.com/rvrs/waycalc.git
cd waycalc
cargo build --release
install -Dm755 target/release/waycalc ~/.local/bin/waycalc
```
Make sure that `~/.local/bin` is in your `PATH`.
### Prebuilt binary
Grab prebuilt binary from releases page. Extract it and drop waycalc executable somewhere on your `PATH`. 
### AUR (Soon)
## Usage
Write your problem, click **Enter** and get your result. Supported operators: 
- **Addition** `+`, 
- **Substraction** `-`,`
- **Multiplication** `*`,
- **Division** `/`

Support for more advanced operators (like `log` or `sqrt`) is planned for future releases.

## Waybar intergation
```json
"custom/waycalc": {
    "format": "waycalc",
    "tooltip": "waycalc",
    "on-click": "pkill -x waycalc || waycalc"
  }
```
Then reference it in one of the `modules-` block:
```json
"modules-right": ["tray", "custom/waycalc", ...]
```

Styling is not supported in current release, but it's planned for future.
## License
This project is licensed under the [MIT License](LICENSE.md).
