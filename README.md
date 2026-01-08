<div align="center" markdown="1">

# SCANNR
### The Modern GUI for RustScan

**Fast, smart, visual.**

![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-Purple)
![License](https://img.shields.io/badge/License-GPL--3.0-blue)

</div>

# 🤔 What is SCANNR?

**SCANNR** is a high-performance, modern GUI and enhanced CLI suite built on top of the legendary [RustScan](https://github.com/RustScan/RustScan) engine. While maintaining the incredible speed of the original project, SCANNR adds a powerful graphical interface, real-time network visualization, and advanced discovery logic.

**Note:** SCANNR is an independent GUI project based on RustScan. We are deep admirers of the original work by the RustScan team and aim to provide a visual-first experience for their world-class scanning engine.

# ✨ Key Features

- 🚀 **Extreme Speed:** Scans thousands of ports in seconds using the optimized RustScan engine.
- 🎨 **Modern GUI:** A clean, responsive interface with light/dark theme support.
- 🌊 **Cascade Scanning:**
    - **Phase 0:** Instant local neighbor discovery via ARP cache.
    - **Phase 0.5:** High-speed Active TCP Probe to force discovery of non-cached hosts.
    - **Phase 1:** Quick probing of common services.
    - **Phase 2:** Full deep-scan of all requested ports.
- 📊 **Real-time Progress:** A granular, multi-phase progress bar that shows exactly what the scanner is doing in the background.
- 🌐 **Network Topology:** Interactive visualization of your network structure, including router/server identification and traceroute mapping.
- 🏷️ **Vendor Identification:** Automatic MAC OUI lookup to identify device manufacturers (Apple, Cisco, Raspberry Pi, etc.).
- 🛠️ **Binary Suite:**
    - `scannr-gui`: The full graphical experience.
    - `scannr-cli`: The enhanced command-line version.

# 🚀 Installation

### Windows
1. Download the latest `scannr-gui.exe` or `scannr-cli.exe` from our [releases page](https://github.com/lazerusrm/SCANNR/releases).
2. Run and enjoy.

### Building from Source
Ensure you have the Rust toolchain installed, then:
```bash
git clone https://github.com/lazerusrm/SCANNR
cd SCANNR
cargo build --release
```
Binaries will be available in `target/release/`.

# 🤸 Usage

### GUI Version
Launch `scannr-gui.exe`. Enter your target CIDR (e.g., `192.168.1.0/24`) and hit **Scan Network**. Results will appear in real-time as they are discovered.

### CLI Version
```bash
./scannr-cli -a 192.168.1.1/24 -t 500 -b 3000
```

# ⚖️ License & Attribution

SCANNR is licensed under the **GNU GPL-3.0**.

This project is a fork/extension of [RustScan](https://github.com/RustScan/RustScan). We owe a huge debt of gratitude to the original authors and contributors of RustScan for building the lightning-fast engine that powers this GUI. 

The original RustScan code remains the property of its respective owners and is used here in accordance with the GPL-3.0 license.

---
<div align="center">
  Built with ❤️ for the security community.
</div>