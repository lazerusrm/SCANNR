# Building RustScan GUI for Windows

## Quick Build

To build a standalone Windows executable:

```powershell
cargo build --release --bin rustscan-gui
```

The executable will be at: `target\release\rustscan-gui.exe`

**That's it!** The executable is completely standalone - just double-click to run.

## Adding an Icon (Optional)

The executable works fine without a custom icon, but you can add one:

1. **Quick method (if you have ImageMagick):**
   ```powershell
   .\scripts\create_icon.ps1
   ```

2. **Manual method:**
   - Create or obtain a `.ico` file (recommended sizes: 16x16, 32x32, 48x48, 256x256)
   - Place it at: `assets\rustscan.ico`
   - Rebuild: `cargo build --release --bin rustscan-gui`

3. **Online converter:**
   - Use https://convertio.co/png-ico/ to convert `pictures\rustscan.png` to ICO format
   - Save as `assets\rustscan.ico`

The icon will be automatically embedded in the executable on the next build.

## Features

- ✅ Single standalone executable (no dependencies needed)
- ✅ No console window (Windows subsystem)
- ✅ Embedded icon (if provided)
- ✅ Auto-launches GUI on double-click
- ✅ Fully functional port scanner

## Distribution

The `rustscan-gui.exe` file is completely standalone - you can:
- Copy it to any Windows machine
- Run it by double-clicking
- No installation required
- No additional files needed

## Troubleshooting

**Icon not showing?**
- Make sure `assets\rustscan.ico` exists before building
- Icon must be a valid `.ico` file with multiple sizes
- Rebuild after adding the icon

**Console window appears?**
- Make sure you're building with `--release` flag
- The `#![windows_subsystem = "windows"]` attribute should prevent console

**Build errors?**
- Ensure you have the Windows SDK installed (usually comes with Visual Studio)
- Run `rustup target add x86_64-pc-windows-msvc` if needed

