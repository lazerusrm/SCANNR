# RustScan GUI - Windows Standalone Binary

## ✅ Complete Setup

Your RustScan GUI is now configured to build as a **single, standalone Windows executable** that:
- ✅ Launches with a double-click (no CLI needed)
- ✅ Has no console window (Windows subsystem)
- ✅ Can have a custom icon (optional)
- ✅ Contains everything needed (no external dependencies)
- ✅ Works on any Windows machine

## Quick Start

### Build the Executable

```powershell
cargo build --release --bin rustscan-gui
```

**Output:** `target\release\rustscan-gui.exe`

### Run It

Just **double-click** `rustscan-gui.exe` - that's it!

## Features

- **Auto-detects your network subnet** on startup
- **One-click scanning** - just click "Run Scan"
- **Modern UI** with light/dark/auto themes
- **Detailed results** showing:
  - IP addresses
  - Open ports
  - MAC addresses (via ARP lookup)
  - Hostnames (via DNS reverse lookup)
- **Real-time progress** during scanning
- **Fully async** and fast

## Adding a Custom Icon

1. **Option 1 - Use the helper script:**
   ```powershell
   .\scripts\create_icon.ps1
   ```

2. **Option 2 - Manual:**
   - Convert `pictures\rustscan.png` to ICO format
   - Save as `assets\rustscan.ico`
   - Rebuild: `cargo build --release --bin rustscan-gui`

3. **Option 3 - Online:**
   - Go to https://convertio.co/png-ico/
   - Upload `pictures\rustscan.png`
   - Download and save as `assets\rustscan.ico`
   - Rebuild

## Distribution

The `rustscan-gui.exe` file is **completely standalone**:

- ✅ No installation required
- ✅ No additional files needed
- ✅ No dependencies to install
- ✅ Works on Windows 10/11
- ✅ Can be copied to any Windows machine

Just copy the `.exe` file and run it!

## Technical Details

- **Subsystem:** Windows (no console window)
- **Icon embedding:** Automatic if `assets\rustscan.ico` exists
- **Static linking:** All dependencies included
- **Size:** ~10-15 MB (includes all dependencies)
- **Runtime:** Tokio async runtime embedded

## Troubleshooting

**Icon not showing?**
- Icon is optional - app works fine without it
- Make sure `assets\rustscan.ico` exists before building
- Rebuild after adding the icon

**Console window appears?**
- Make sure you're using `--release` flag
- The Windows subsystem attribute is set in the code

**Build errors?**
- Ensure Windows SDK is installed (comes with Visual Studio)
- Run: `rustup target add x86_64-pc-windows-msvc`

## File Structure

```
RustScan/
├── assets/
│   ├── rustscan.ico      # (optional) Windows icon
│   └── README.md         # Icon instructions
├── scripts/
│   └── create_icon.ps1  # Icon creation helper
├── src/
│   ├── gui_main.rs       # GUI entry point
│   └── gui.rs            # GUI implementation
├── build.rs              # Build script (icon embedding)
├── Cargo.toml            # Project configuration
└── BUILD_WINDOWS.md      # Build instructions
```

## Next Steps

1. Build: `cargo build --release --bin rustscan-gui`
2. Test: Double-click `target\release\rustscan-gui.exe`
3. (Optional) Add icon: Follow instructions above
4. Distribute: Copy the `.exe` file anywhere!

Enjoy your modern, standalone RustScan GUI! 🚀

