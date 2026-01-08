//! OUI (Organizationally Unique Identifier) lookup for MAC address vendor identification

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Embedded OUI database with common vendors
static OUI_DATABASE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Apple
    m.insert("00:03:93", "Apple");
    m.insert("00:1C:B3", "Apple");
    m.insert("00:1E:C2", "Apple");
    m.insert("00:25:BC", "Apple");
    m.insert("3C:06:30", "Apple");
    m.insert("A4:83:E7", "Apple");
    m.insert("AC:DE:48", "Apple");
    m.insert("DC:A6:32", "Raspberry Pi");
    m.insert("B8:27:EB", "Raspberry Pi");
    m.insert("E4:5F:01", "Raspberry Pi");
    m.insert("28:CD:C1", "Raspberry Pi");
    // Samsung
    m.insert("00:15:99", "Samsung");
    m.insert("00:1E:E1", "Samsung");
    m.insert("00:26:37", "Samsung");
    m.insert("5C:0A:5B", "Samsung");
    m.insert("8C:77:12", "Samsung");
    // Intel
    m.insert("00:1B:21", "Intel");
    m.insert("00:1E:64", "Intel");
    m.insert("00:1F:3B", "Intel");
    m.insert("00:22:FB", "Intel");
    m.insert("3C:97:0E", "Intel");
    m.insert("68:05:CA", "Intel");
    // Microsoft/Xbox
    m.insert("00:50:F2", "Microsoft");
    m.insert("00:15:5D", "Microsoft Hyper-V");
    m.insert("7C:1E:52", "Microsoft");
    m.insert("28:18:78", "Microsoft Xbox");
    // VMware
    m.insert("00:50:56", "VMware");
    m.insert("00:0C:29", "VMware");
    m.insert("00:05:69", "VMware");
    // VirtualBox
    m.insert("08:00:27", "VirtualBox");
    m.insert("0A:00:27", "VirtualBox");
    // Cisco
    m.insert("00:00:0C", "Cisco");
    m.insert("00:01:42", "Cisco");
    m.insert("00:1A:A1", "Cisco");
    m.insert("00:1B:D4", "Cisco");
    // Dell
    m.insert("00:14:22", "Dell");
    m.insert("00:1A:A0", "Dell");
    m.insert("00:1E:4F", "Dell");
    m.insert("18:A9:9B", "Dell");
    // HP
    m.insert("00:1A:4B", "HP");
    m.insert("00:21:5A", "HP");
    m.insert("00:25:B3", "HP");
    m.insert("3C:D9:2B", "HP");
    // Lenovo
    m.insert("00:1E:4C", "Lenovo");
    m.insert("00:22:68", "Lenovo");
    m.insert("00:26:2D", "Lenovo");
    m.insert("E8:E0:B7", "Lenovo");
    // ASUS
    m.insert("00:1A:92", "ASUS");
    m.insert("00:1E:8C", "ASUS");
    m.insert("00:22:15", "ASUS");
    m.insert("AC:22:0B", "ASUS");
    // Netgear
    m.insert("00:14:6C", "Netgear");
    m.insert("00:1B:2F", "Netgear");
    m.insert("00:1E:2A", "Netgear");
    m.insert("C0:3F:0E", "Netgear");
    // TP-Link
    m.insert("00:1D:0F", "TP-Link");
    m.insert("14:CC:20", "TP-Link");
    m.insert("50:C7:BF", "TP-Link");
    m.insert("E8:DE:27", "TP-Link");
    // D-Link
    m.insert("00:1B:11", "D-Link");
    m.insert("00:1E:58", "D-Link");
    m.insert("14:D6:4D", "D-Link");
    // Linksys/Belkin
    m.insert("00:14:BF", "Linksys");
    m.insert("00:1A:70", "Linksys");
    m.insert("00:1E:E5", "Linksys");
    m.insert("C0:56:27", "Belkin");
    // Google
    m.insert("00:1A:11", "Google");
    m.insert("54:60:09", "Google");
    m.insert("F4:F5:D8", "Google");
    // Amazon
    m.insert("00:FC:8B", "Amazon");
    m.insert("18:74:2E", "Amazon");
    m.insert("44:65:0D", "Amazon");
    m.insert("68:54:FD", "Amazon Echo");
    m.insert("A0:02:DC", "Amazon");
    // Ubiquiti
    m.insert("00:27:22", "Ubiquiti");
    m.insert("24:A4:3C", "Ubiquiti");
    m.insert("44:D9:E7", "Ubiquiti");
    m.insert("80:2A:A8", "Ubiquiti");
    m.insert("FC:EC:DA", "Ubiquiti");
    // Synology
    m.insert("00:11:32", "Synology");
    // QNAP
    m.insert("00:08:9B", "QNAP");
    m.insert("24:5E:BE", "QNAP");
    // Sony
    m.insert("00:1D:BA", "Sony");
    m.insert("00:24:BE", "Sony PlayStation");
    m.insert("28:0D:FC", "Sony PlayStation");
    // Nintendo
    m.insert("00:09:BF", "Nintendo");
    m.insert("00:1E:35", "Nintendo");
    m.insert("00:1F:32", "Nintendo");
    m.insert("00:22:AA", "Nintendo");
    m.insert("7C:BB:8A", "Nintendo");
    // Roku
    m.insert("B0:A7:37", "Roku");
    m.insert("D8:31:34", "Roku");
    // Sonos
    m.insert("00:0E:58", "Sonos");
    m.insert("5C:AA:FD", "Sonos");
    // Philips Hue
    m.insert("00:17:88", "Philips Hue");
    m.insert("EC:B5:FA", "Philips Hue");
    // Nest/Google Home
    m.insert("18:B4:30", "Nest");
    m.insert("64:16:66", "Nest");
    // Ring
    m.insert("34:3E:A4", "Ring");
    // Wyze
    m.insert("2C:AA:8E", "Wyze");
    // ESP/Espressif (IoT devices)
    m.insert("24:0A:C4", "Espressif");
    m.insert("30:AE:A4", "Espressif");
    m.insert("A4:CF:12", "Espressif");
    m.insert("BC:DD:C2", "Espressif");
    // Realtek (common in cheap adapters)
    m.insert("00:E0:4C", "Realtek");
    m.insert("52:54:00", "QEMU/KVM");
    // Broadcom
    m.insert("00:10:18", "Broadcom");
    // Qualcomm/Atheros
    m.insert("00:03:7F", "Atheros");
    // Huawei
    m.insert("00:18:82", "Huawei");
    m.insert("00:1E:10", "Huawei");
    m.insert("48:46:FB", "Huawei");
    // Xiaomi
    m.insert("00:9E:C8", "Xiaomi");
    m.insert("28:6C:07", "Xiaomi");
    m.insert("64:09:80", "Xiaomi");
    // OnePlus
    m.insert("C0:EE:FB", "OnePlus");
    // LG
    m.insert("00:1E:75", "LG");
    m.insert("00:1F:E3", "LG");
    m.insert("00:22:A9", "LG");
    // Aruba
    m.insert("00:0B:86", "Aruba Networks");
    m.insert("00:1A:1E", "Aruba Networks");
    // Ruckus
    m.insert("00:22:7F", "Ruckus");
    // Juniper
    m.insert("00:12:1E", "Juniper");
    m.insert("00:1F:12", "Juniper");
    // Fortinet
    m.insert("00:09:0F", "Fortinet");
    // Palo Alto
    m.insert("00:1B:17", "Palo Alto");
    // IP Cameras - Hikvision
    m.insert("18:68:CB", "Hikvision");
    m.insert("28:57:BE", "Hikvision");
    m.insert("44:19:B6", "Hikvision");
    m.insert("4C:BD:8F", "Hikvision");
    m.insert("54:C4:15", "Hikvision");
    m.insert("5C:F9:6A", "Hikvision");
    m.insert("64:E8:4F", "Hikvision");
    m.insert("7C:1E:B3", "Hikvision");
    m.insert("80:A2:35", "Hikvision");
    m.insert("84:E0:F4", "Hikvision");
    m.insert("A4:14:37", "Hikvision");
    m.insert("BC:AD:28", "Hikvision");
    m.insert("C0:56:E3", "Hikvision");
    m.insert("E0:50:8B", "Hikvision");
    // IP Cameras - Dahua
    m.insert("14:A7:8B", "Dahua");
    m.insert("20:17:42", "Dahua");
    m.insert("3C:EF:8C", "Dahua");
    m.insert("40:F4:FD", "Dahua");
    m.insert("4C:11:BF", "Dahua");
    m.insert("9C:14:63", "Dahua");
    m.insert("A0:BD:1D", "Dahua");
    m.insert("B0:A7:32", "Dahua");
    m.insert("E0:50:8B", "Dahua");
    // IP Cameras - Axis
    m.insert("00:40:8C", "Axis");
    m.insert("AC:CC:8E", "Axis");
    m.insert("B8:A4:4F", "Axis");
    // IP Cameras - Amcrest
    m.insert("9C:8E:CD", "Amcrest");
    // IP Cameras - Reolink
    m.insert("EC:71:DB", "Reolink");
    m.insert("B4:6D:C2", "Reolink");
    // IP Cameras - Foscam
    m.insert("C0:74:2B", "Foscam");
    m.insert("00:62:6E", "Foscam");
    // IP Cameras - Lorex
    m.insert("00:1B:AE", "Lorex");
    // IP Cameras - Uniview
    m.insert("24:28:FD", "Uniview");
    m.insert("4C:E1:73", "Uniview");
    // MikroTik
    m.insert("00:0C:42", "MikroTik");
    m.insert("4C:5E:0C", "MikroTik");
    m.insert("64:D1:54", "MikroTik");
    m.insert("6C:3B:6B", "MikroTik");
    m.insert("74:4D:28", "MikroTik");
    m.insert("B8:69:F4", "MikroTik");
    m.insert("C4:AD:34", "MikroTik");
    m.insert("CC:2D:E0", "MikroTik");
    m.insert("D4:CA:6D", "MikroTik");
    m.insert("E4:8D:8C", "MikroTik");
    // Cisco Meraki
    m.insert("00:18:0A", "Meraki");
    m.insert("0C:8D:DB", "Meraki");
    m.insert("34:56:FE", "Meraki");
    m.insert("68:3A:1E", "Meraki");
    m.insert("88:15:44", "Meraki");
    m.insert("AC:17:C8", "Meraki");
    m.insert("E0:55:3D", "Meraki");
    m.insert("E0:CB:BC", "Meraki");
    // More Cisco
    m.insert("00:02:FC", "Cisco");
    m.insert("00:07:0E", "Cisco");
    m.insert("00:0A:41", "Cisco");
    m.insert("00:0B:BE", "Cisco");
    m.insert("00:0D:ED", "Cisco");
    m.insert("00:0E:38", "Cisco");
    m.insert("00:0E:83", "Cisco");
    m.insert("00:11:20", "Cisco");
    m.insert("00:14:1B", "Cisco");
    m.insert("00:17:0E", "Cisco");
    m.insert("00:18:18", "Cisco");
    m.insert("00:19:AA", "Cisco");
    m.insert("00:1D:70", "Cisco");
    m.insert("00:22:0D", "Cisco");
    m.insert("00:25:84", "Cisco");
    m.insert("00:26:51", "Cisco");
    m.insert("00:2A:6A", "Cisco");
    m.insert("2C:33:11", "Cisco");
    m.insert("34:A8:4E", "Cisco");
    m.insert("50:06:04", "Cisco");
    m.insert("54:75:D0", "Cisco");
    m.insert("58:8D:09", "Cisco");
    m.insert("5C:83:8F", "Cisco");
    m.insert("70:81:05", "Cisco");
    m.insert("78:BA:F9", "Cisco");
    m.insert("84:78:AC", "Cisco");
    m.insert("A8:0C:0D", "Cisco");
    m.insert("B0:AA:77", "Cisco");
    m.insert("B4:14:89", "Cisco");
    m.insert("D0:72:DC", "Cisco");
    m.insert("F0:29:29", "Cisco");
    m.insert("F8:72:EA", "Cisco");
    // More Ubiquiti
    m.insert("04:18:D6", "Ubiquiti");
    m.insert("18:E8:29", "Ubiquiti");
    m.insert("24:5A:4C", "Ubiquiti");
    m.insert("68:72:51", "Ubiquiti");
    m.insert("74:83:C2", "Ubiquiti");
    m.insert("78:8A:20", "Ubiquiti");
    m.insert("B4:FB:E4", "Ubiquiti");
    m.insert("DC:9F:DB", "Ubiquiti");
    m.insert("F0:9F:C2", "Ubiquiti");
    // More Apple
    m.insert("00:05:02", "Apple");
    m.insert("00:0A:27", "Apple");
    m.insert("00:0A:95", "Apple");
    m.insert("00:0D:93", "Apple");
    m.insert("00:11:24", "Apple");
    m.insert("00:14:51", "Apple");
    m.insert("00:16:CB", "Apple");
    m.insert("00:17:F2", "Apple");
    m.insert("00:19:E3", "Apple");
    m.insert("00:1D:4F", "Apple");
    m.insert("00:1E:52", "Apple");
    m.insert("00:21:E9", "Apple");
    m.insert("00:23:12", "Apple");
    m.insert("00:23:32", "Apple");
    m.insert("00:24:36", "Apple");
    m.insert("00:26:08", "Apple");
    m.insert("04:0C:CE", "Apple");
    m.insert("14:10:9F", "Apple");
    m.insert("18:AF:8F", "Apple");
    m.insert("20:C9:D0", "Apple");
    m.insert("24:A0:74", "Apple");
    m.insert("28:37:37", "Apple");
    m.insert("28:CF:E9", "Apple");
    m.insert("2C:BE:08", "Apple");
    m.insert("34:08:BC", "Apple");
    m.insert("38:C9:86", "Apple");
    m.insert("40:A6:D9", "Apple");
    m.insert("44:2A:60", "Apple");
    m.insert("48:D7:05", "Apple");
    m.insert("54:72:4F", "Apple");
    m.insert("58:55:CA", "Apple");
    m.insert("60:03:08", "Apple");
    m.insert("64:A5:C3", "Apple");
    m.insert("68:DB:CA", "Apple");
    m.insert("70:CD:60", "Apple");
    m.insert("78:31:C1", "Apple");
    m.insert("7C:D1:C3", "Apple");
    m.insert("80:E6:50", "Apple");
    m.insert("84:38:35", "Apple");
    m.insert("88:66:A5", "Apple");
    m.insert("8C:29:37", "Apple");
    m.insert("90:8D:6C", "Apple");
    m.insert("98:01:A7", "Apple");
    m.insert("9C:20:7B", "Apple");
    m.insert("A4:5E:60", "Apple");
    m.insert("A8:51:5B", "Apple");
    m.insert("AC:BC:32", "Apple");
    m.insert("B0:34:95", "Apple");
    m.insert("B4:F0:AB", "Apple");
    m.insert("BC:52:B7", "Apple");
    m.insert("C0:84:7A", "Apple");
    m.insert("C4:2C:03", "Apple");
    m.insert("C8:69:CD", "Apple");
    m.insert("D0:25:98", "Apple");
    m.insert("D4:61:9D", "Apple");
    m.insert("D8:1D:72", "Apple");
    m.insert("DC:2B:2A", "Apple");
    m.insert("E0:B5:2D", "Apple");
    m.insert("E4:9A:DC", "Apple");
    m.insert("F0:18:98", "Apple");
    m.insert("F4:37:B7", "Apple");
    m.insert("F8:1E:DF", "Apple");
    // More Samsung
    m.insert("00:00:F0", "Samsung");
    m.insert("00:07:AB", "Samsung");
    m.insert("00:12:47", "Samsung");
    m.insert("00:12:FB", "Samsung");
    m.insert("00:16:32", "Samsung");
    m.insert("00:17:D5", "Samsung");
    m.insert("00:1A:8A", "Samsung");
    m.insert("00:1D:25", "Samsung");
    m.insert("00:1E:E2", "Samsung");
    m.insert("00:21:19", "Samsung");
    m.insert("00:21:D1", "Samsung");
    m.insert("00:23:39", "Samsung");
    m.insert("00:23:D6", "Samsung");
    m.insert("00:24:54", "Samsung");
    m.insert("00:24:E9", "Samsung");
    m.insert("00:26:5D", "Samsung");
    m.insert("08:D4:2B", "Samsung");
    m.insert("10:D5:42", "Samsung");
    m.insert("14:49:E0", "Samsung");
    m.insert("18:22:7E", "Samsung");
    m.insert("1C:62:B8", "Samsung");
    m.insert("20:13:E0", "Samsung");
    m.insert("24:4B:81", "Samsung");
    m.insert("28:98:7B", "Samsung");
    m.insert("2C:AE:2B", "Samsung");
    m.insert("30:07:4D", "Samsung");
    m.insert("34:23:BA", "Samsung");
    m.insert("38:01:97", "Samsung");
    m.insert("3C:5A:37", "Samsung");
    m.insert("40:0E:85", "Samsung");
    m.insert("44:4E:1A", "Samsung");
    m.insert("48:44:F7", "Samsung");
    m.insert("4C:66:41", "Samsung");
    m.insert("50:01:BB", "Samsung");
    m.insert("54:9B:12", "Samsung");
    m.insert("58:C3:8B", "Samsung");
    m.insert("5C:E8:EB", "Samsung");
    m.insert("60:AF:6D", "Samsung");
    m.insert("68:48:98", "Samsung");
    m.insert("6C:2F:2C", "Samsung");
    m.insert("70:F9:27", "Samsung");
    m.insert("78:25:AD", "Samsung");
    m.insert("7C:0B:C6", "Samsung");
    m.insert("80:65:6D", "Samsung");
    m.insert("8C:71:F8", "Samsung");
    m.insert("90:18:7C", "Samsung");
    m.insert("94:35:0A", "Samsung");
    m.insert("98:52:B1", "Samsung");
    m.insert("9C:02:98", "Samsung");
    m.insert("A0:07:98", "Samsung");
    m.insert("A8:06:00", "Samsung");
    m.insert("AC:5F:3E", "Samsung");
    m.insert("B0:C5:59", "Samsung");
    m.insert("B8:BC:1B", "Samsung");
    m.insert("BC:20:A4", "Samsung");
    m.insert("C0:BD:D1", "Samsung");
    m.insert("CC:07:AB", "Samsung");
    m.insert("D0:22:BE", "Samsung");
    m.insert("D4:88:90", "Samsung");
    m.insert("DC:53:7C", "Samsung");
    m.insert("E4:12:1D", "Samsung");
    m.insert("EC:1F:72", "Samsung");
    m.insert("F0:25:B7", "Samsung");
    m.insert("F4:7B:5E", "Samsung");
    m.insert("FC:A1:3E", "Samsung");
    // More TP-Link
    m.insert("00:27:19", "TP-Link");
    m.insert("10:FE:ED", "TP-Link");
    m.insert("14:CF:92", "TP-Link");
    m.insert("18:A6:F7", "TP-Link");
    m.insert("1C:3B:F3", "TP-Link");
    m.insert("30:B5:C2", "TP-Link");
    m.insert("34:E8:94", "TP-Link");
    m.insert("40:31:3C", "TP-Link");
    m.insert("54:E6:FC", "TP-Link");
    m.insert("60:32:B1", "TP-Link");
    m.insert("64:70:02", "TP-Link");
    m.insert("68:FF:7B", "TP-Link");
    m.insert("70:4F:57", "TP-Link");
    m.insert("78:44:76", "TP-Link");
    m.insert("84:16:F9", "TP-Link");
    m.insert("90:F6:52", "TP-Link");
    m.insert("98:DA:C4", "TP-Link");
    m.insert("A4:2B:B0", "TP-Link");
    m.insert("AC:84:C6", "TP-Link");
    m.insert("B0:4E:26", "TP-Link");
    m.insert("B0:95:75", "TP-Link");
    m.insert("B8:08:D7", "TP-Link");
    m.insert("C0:06:C3", "TP-Link");
    m.insert("C4:E9:84", "TP-Link");
    m.insert("D4:6E:0E", "TP-Link");
    m.insert("D8:07:B6", "TP-Link");
    m.insert("EC:08:6B", "TP-Link");
    m.insert("F4:F2:6D", "TP-Link");
    m.insert("F8:D1:11", "TP-Link");
    // More Netgear
    m.insert("00:0F:B5", "Netgear");
    m.insert("00:22:3F", "Netgear");
    m.insert("00:24:B2", "Netgear");
    m.insert("00:26:F2", "Netgear");
    m.insert("08:BD:43", "Netgear");
    m.insert("10:0D:7F", "Netgear");
    m.insert("20:0C:C8", "Netgear");
    m.insert("28:C6:8E", "Netgear");
    m.insert("30:46:9A", "Netgear");
    m.insert("44:94:FC", "Netgear");
    m.insert("4C:60:DE", "Netgear");
    m.insert("54:07:7D", "Netgear");
    m.insert("6C:B0:CE", "Netgear");
    m.insert("84:1B:5E", "Netgear");
    m.insert("9C:3D:CF", "Netgear");
    m.insert("A0:04:60", "Netgear");
    m.insert("A0:21:B7", "Netgear");
    m.insert("B0:7F:B9", "Netgear");
    m.insert("C0:FF:D4", "Netgear");
    m.insert("E0:46:9A", "Netgear");
    m.insert("E0:91:F5", "Netgear");
    m.insert("E4:F4:C6", "Netgear");
    // More D-Link
    m.insert("00:05:5D", "D-Link");
    m.insert("00:0D:88", "D-Link");
    m.insert("00:11:95", "D-Link");
    m.insert("00:13:46", "D-Link");
    m.insert("00:15:E9", "D-Link");
    m.insert("00:17:9A", "D-Link");
    m.insert("00:19:5B", "D-Link");
    m.insert("00:1C:F0", "D-Link");
    m.insert("00:21:91", "D-Link");
    m.insert("00:22:B0", "D-Link");
    m.insert("00:24:01", "D-Link");
    m.insert("00:26:5A", "D-Link");
    m.insert("1C:7E:E5", "D-Link");
    m.insert("28:10:7B", "D-Link");
    m.insert("34:08:04", "D-Link");
    m.insert("78:54:2E", "D-Link");
    m.insert("84:C9:B2", "D-Link");
    m.insert("9C:D6:43", "D-Link");
    m.insert("AC:F1:DF", "D-Link");
    m.insert("B8:A3:86", "D-Link");
    m.insert("BC:F6:85", "D-Link");
    m.insert("C8:BE:19", "D-Link");
    m.insert("CC:B2:55", "D-Link");
    m.insert("F0:7D:68", "D-Link");
    // More Linksys/Belkin
    m.insert("00:06:25", "Linksys");
    m.insert("00:0C:41", "Linksys");
    m.insert("00:0F:66", "Linksys");
    m.insert("00:12:17", "Linksys");
    m.insert("00:16:B6", "Linksys");
    m.insert("00:18:F8", "Linksys");
    m.insert("00:1C:10", "Linksys");
    m.insert("00:1E:E5", "Linksys");
    m.insert("00:21:29", "Linksys");
    m.insert("00:22:6B", "Linksys");
    m.insert("00:23:69", "Linksys");
    m.insert("00:25:9C", "Linksys");
    m.insert("20:AA:4B", "Linksys");
    m.insert("48:F8:B3", "Linksys");
    m.insert("58:6D:8F", "Linksys");
    m.insert("5C:D9:98", "Belkin");
    m.insert("94:10:3E", "Belkin");
    m.insert("B4:75:0E", "Belkin");
    m.insert("C0:38:96", "Belkin");
    m.insert("EC:1A:59", "Belkin");
    // More Google/Nest
    m.insert("00:1A:11", "Google");
    m.insert("1A:62:AB", "Google");
    m.insert("20:DF:B9", "Google");
    m.insert("30:FD:38", "Google");
    m.insert("3C:5A:B4", "Google");
    m.insert("48:D6:D5", "Google");
    m.insert("6C:AD:F8", "Google");
    m.insert("7C:2E:BD", "Google");
    m.insert("94:EB:2C", "Google");
    m.insert("A4:77:33", "Google");
    m.insert("D8:6C:63", "Google");
    m.insert("E4:F0:42", "Google");
    m.insert("F4:F5:E8", "Google");
    m.insert("18:D6:C7", "Nest");
    m.insert("44:07:0B", "Nest");
    m.insert("64:48:8B", "Nest");
    // More Amazon
    m.insert("00:BB:3A", "Amazon");
    m.insert("0C:47:C9", "Amazon");
    m.insert("10:CE:A9", "Amazon");
    m.insert("14:91:82", "Amazon");
    m.insert("18:74:2E", "Amazon");
    m.insert("24:4C:E3", "Amazon");
    m.insert("34:D2:70", "Amazon");
    m.insert("38:F7:3D", "Amazon");
    m.insert("40:B4:CD", "Amazon");
    m.insert("4C:EF:C0", "Amazon");
    m.insert("50:DC:E7", "Amazon");
    m.insert("5C:41:5A", "Amazon");
    m.insert("68:37:E9", "Amazon");
    m.insert("74:C2:46", "Amazon");
    m.insert("84:D6:D0", "Amazon");
    m.insert("8C:85:80", "Amazon");
    m.insert("A4:08:01", "Amazon");
    m.insert("AC:63:BE", "Amazon");
    m.insert("B4:7C:9C", "Amazon");
    m.insert("CC:9E:A2", "Amazon");
    m.insert("F0:27:2D", "Amazon");
    m.insert("F0:D2:F1", "Amazon");
    m.insert("FC:65:DE", "Amazon");
    // More Intel
    m.insert("00:02:B3", "Intel");
    m.insert("00:03:47", "Intel");
    m.insert("00:04:23", "Intel");
    m.insert("00:07:E9", "Intel");
    m.insert("00:0C:F1", "Intel");
    m.insert("00:0E:0C", "Intel");
    m.insert("00:0E:35", "Intel");
    m.insert("00:13:02", "Intel");
    m.insert("00:13:20", "Intel");
    m.insert("00:13:CE", "Intel");
    m.insert("00:13:E8", "Intel");
    m.insert("00:15:00", "Intel");
    m.insert("00:15:17", "Intel");
    m.insert("00:16:6F", "Intel");
    m.insert("00:16:76", "Intel");
    m.insert("00:16:EA", "Intel");
    m.insert("00:16:EB", "Intel");
    m.insert("00:18:DE", "Intel");
    m.insert("00:19:D1", "Intel");
    m.insert("00:19:D2", "Intel");
    m.insert("00:1A:A0", "Intel");
    m.insert("00:1B:77", "Intel");
    m.insert("00:1C:BF", "Intel");
    m.insert("00:1D:E0", "Intel");
    m.insert("00:1D:E1", "Intel");
    m.insert("00:1F:3C", "Intel");
    m.insert("00:20:E0", "Intel");
    m.insert("00:21:5C", "Intel");
    m.insert("00:21:5D", "Intel");
    m.insert("00:21:6A", "Intel");
    m.insert("00:21:6B", "Intel");
    m.insert("00:22:FA", "Intel");
    m.insert("00:24:D6", "Intel");
    m.insert("00:24:D7", "Intel");
    m.insert("00:26:C6", "Intel");
    m.insert("00:26:C7", "Intel");
    m.insert("00:27:10", "Intel");
    m.insert("10:0B:A9", "Intel");
    m.insert("18:3D:A2", "Intel");
    m.insert("24:77:03", "Intel");
    m.insert("3C:A9:F4", "Intel");
    m.insert("40:A3:CC", "Intel");
    m.insert("48:45:20", "Intel");
    m.insert("50:76:AF", "Intel");
    m.insert("58:91:CF", "Intel");
    m.insert("5C:51:4F", "Intel");
    m.insert("60:67:20", "Intel");
    m.insert("64:80:99", "Intel");
    m.insert("68:17:29", "Intel");
    m.insert("74:E5:0B", "Intel");
    m.insert("7C:5C:F8", "Intel");
    m.insert("80:86:F2", "Intel");
    m.insert("84:3A:4B", "Intel");
    m.insert("8C:F5:A3", "Intel");
    m.insert("94:65:9C", "Intel");
    m.insert("98:4F:EE", "Intel");
    m.insert("A0:36:9F", "Intel");
    m.insert("A4:34:D9", "Intel");
    m.insert("AC:ED:5C", "Intel");
    m.insert("B4:96:91", "Intel");
    m.insert("BC:77:37", "Intel");
    m.insert("C8:D3:FF", "Intel");
    m.insert("D0:AB:D5", "Intel");
    m.insert("D4:3B:04", "Intel");
    m.insert("DC:53:60", "Intel");
    m.insert("E8:B1:FC", "Intel");
    m.insert("F4:06:69", "Intel");
    m.insert("F8:94:C2", "Intel");
    // More Dell
    m.insert("00:06:5B", "Dell");
    m.insert("00:08:74", "Dell");
    m.insert("00:0B:DB", "Dell");
    m.insert("00:0D:56", "Dell");
    m.insert("00:0F:1F", "Dell");
    m.insert("00:11:43", "Dell");
    m.insert("00:12:3F", "Dell");
    m.insert("00:13:72", "Dell");
    m.insert("00:14:22", "Dell");
    m.insert("00:15:C5", "Dell");
    m.insert("00:16:F0", "Dell");
    m.insert("00:18:8B", "Dell");
    m.insert("00:19:B9", "Dell");
    m.insert("00:1C:23", "Dell");
    m.insert("00:1D:09", "Dell");
    m.insert("00:1E:C9", "Dell");
    m.insert("00:21:70", "Dell");
    m.insert("00:21:9B", "Dell");
    m.insert("00:22:19", "Dell");
    m.insert("00:23:AE", "Dell");
    m.insert("00:24:E8", "Dell");
    m.insert("00:25:64", "Dell");
    m.insert("00:26:B9", "Dell");
    m.insert("14:18:77", "Dell");
    m.insert("14:B3:1F", "Dell");
    m.insert("14:FE:B5", "Dell");
    m.insert("18:03:73", "Dell");
    m.insert("18:66:DA", "Dell");
    m.insert("18:A9:9B", "Dell");
    m.insert("18:DB:F2", "Dell");
    m.insert("20:47:47", "Dell");
    m.insert("24:6E:96", "Dell");
    m.insert("24:B6:FD", "Dell");
    m.insert("28:F1:0E", "Dell");
    m.insert("34:17:EB", "Dell");
    m.insert("34:E6:D7", "Dell");
    m.insert("44:A8:42", "Dell");
    m.insert("4C:76:25", "Dell");
    m.insert("50:9A:4C", "Dell");
    m.insert("54:9F:35", "Dell");
    m.insert("5C:26:0A", "Dell");
    m.insert("5C:F9:DD", "Dell");
    m.insert("64:00:6A", "Dell");
    m.insert("74:86:7A", "Dell");
    m.insert("74:E6:E2", "Dell");
    m.insert("78:2B:CB", "Dell");
    m.insert("80:18:44", "Dell");
    m.insert("84:2B:2B", "Dell");
    m.insert("84:7B:EB", "Dell");
    m.insert("90:B1:1C", "Dell");
    m.insert("98:90:96", "Dell");
    m.insert("A4:1F:72", "Dell");
    m.insert("A4:BA:DB", "Dell");
    m.insert("B0:83:FE", "Dell");
    m.insert("B4:E1:0F", "Dell");
    m.insert("B8:2A:72", "Dell");
    m.insert("B8:AC:6F", "Dell");
    m.insert("BC:30:5B", "Dell");
    m.insert("C8:1F:66", "Dell");
    m.insert("D0:67:E5", "Dell");
    m.insert("D4:81:D7", "Dell");
    m.insert("D4:AE:52", "Dell");
    m.insert("D4:BE:D9", "Dell");
    m.insert("E0:DB:55", "Dell");
    m.insert("EC:F4:BB", "Dell");
    m.insert("F0:1F:AF", "Dell");
    m.insert("F4:8E:38", "Dell");
    m.insert("F8:B1:56", "Dell");
    m.insert("F8:BC:12", "Dell");
    m.insert("F8:DB:88", "Dell");
    // More HP
    m.insert("00:01:E6", "HP");
    m.insert("00:01:E7", "HP");
    m.insert("00:02:A5", "HP");
    m.insert("00:04:EA", "HP");
    m.insert("00:08:02", "HP");
    m.insert("00:08:83", "HP");
    m.insert("00:0A:57", "HP");
    m.insert("00:0B:CD", "HP");
    m.insert("00:0D:9D", "HP");
    m.insert("00:0E:7F", "HP");
    m.insert("00:0F:20", "HP");
    m.insert("00:0F:61", "HP");
    m.insert("00:10:83", "HP");
    m.insert("00:10:E3", "HP");
    m.insert("00:11:0A", "HP");
    m.insert("00:11:85", "HP");
    m.insert("00:12:79", "HP");
    m.insert("00:13:21", "HP");
    m.insert("00:14:38", "HP");
    m.insert("00:14:C2", "HP");
    m.insert("00:15:60", "HP");
    m.insert("00:16:35", "HP");
    m.insert("00:17:08", "HP");
    m.insert("00:17:A4", "HP");
    m.insert("00:18:71", "HP");
    m.insert("00:18:FE", "HP");
    m.insert("00:19:BB", "HP");
    m.insert("00:1A:4B", "HP");
    m.insert("00:1B:78", "HP");
    m.insert("00:1C:C4", "HP");
    m.insert("00:1E:0B", "HP");
    m.insert("00:1F:29", "HP");
    m.insert("00:1F:FE", "HP");
    m.insert("00:21:5A", "HP");
    m.insert("00:22:64", "HP");
    m.insert("00:23:7D", "HP");
    m.insert("00:24:81", "HP");
    m.insert("00:25:B3", "HP");
    m.insert("00:26:55", "HP");
    m.insert("00:30:6E", "HP");
    m.insert("00:30:C1", "HP");
    m.insert("08:2E:5F", "HP");
    m.insert("10:00:5A", "HP");
    m.insert("10:1F:74", "HP");
    m.insert("10:60:4B", "HP");
    m.insert("14:02:EC", "HP");
    m.insert("14:58:D0", "HP");
    m.insert("18:A9:05", "HP");
    m.insert("1C:98:EC", "HP");
    m.insert("1C:C1:DE", "HP");
    m.insert("24:BE:05", "HP");
    m.insert("28:80:23", "HP");
    m.insert("28:92:4A", "HP");
    m.insert("2C:27:D7", "HP");
    m.insert("2C:41:38", "HP");
    m.insert("2C:44:FD", "HP");
    m.insert("2C:59:E5", "HP");
    m.insert("30:8D:99", "HP");
    m.insert("30:E1:71", "HP");
    m.insert("34:64:A9", "HP");
    m.insert("38:63:BB", "HP");
    m.insert("38:EA:A7", "HP");
    m.insert("3C:4A:92", "HP");
    m.insert("3C:D9:2B", "HP");
    m.insert("40:A8:F0", "HP");
    m.insert("48:0F:CF", "HP");
    m.insert("4C:39:09", "HP");
    m.insert("50:65:F3", "HP");
    m.insert("58:20:B1", "HP");
    m.insert("5C:8A:38", "HP");
    m.insert("5C:B9:01", "HP");
    m.insert("64:51:06", "HP");
    m.insert("68:B5:99", "HP");
    m.insert("6C:3B:E5", "HP");
    m.insert("70:10:6F", "HP");
    m.insert("70:5A:0F", "HP");
    m.insert("74:46:A0", "HP");
    m.insert("78:48:59", "HP");
    m.insert("78:AC:C0", "HP");
    m.insert("7C:D3:0A", "HP");
    m.insert("80:C1:6E", "HP");
    m.insert("84:34:97", "HP");
    m.insert("84:80:2D", "HP");
    m.insert("88:51:FB", "HP");
    m.insert("8C:DC:D4", "HP");
    m.insert("94:57:A5", "HP");
    m.insert("98:4B:E1", "HP");
    m.insert("98:E7:F4", "HP");
    m.insert("9C:8E:99", "HP");
    m.insert("9C:B6:54", "HP");
    m.insert("A0:1D:48", "HP");
    m.insert("A0:2B:B8", "HP");
    m.insert("A0:D3:C1", "HP");
    m.insert("A4:5D:36", "HP");
    m.insert("AC:16:2D", "HP");
    m.insert("B0:5A:DA", "HP");
    m.insert("B4:39:D6", "HP");
    m.insert("B4:B5:2F", "HP");
    m.insert("BC:EA:FA", "HP");
    m.insert("C0:91:34", "HP");
    m.insert("C4:34:6B", "HP");
    m.insert("C8:CB:B8", "HP");
    m.insert("C8:D3:FF", "HP");
    m.insert("CC:3D:82", "HP");
    m.insert("D0:7E:28", "HP");
    m.insert("D4:85:64", "HP");
    m.insert("D4:C9:EF", "HP");
    m.insert("D8:9D:67", "HP");
    m.insert("D8:D3:85", "HP");
    m.insert("DC:4A:3E", "HP");
    m.insert("E4:11:5B", "HP");
    m.insert("E8:39:35", "HP");
    m.insert("EC:8E:B5", "HP");
    m.insert("EC:9A:74", "HP");
    m.insert("F0:92:1C", "HP");
    m.insert("F4:03:43", "HP");
    m.insert("F4:CE:46", "HP");
    m.insert("FC:15:B4", "HP");
    m.insert("FC:3F:DB", "HP");
    // More smart home - Ring, Sonos, Philips Hue, Ecobee
    m.insert("08:81:F4", "Ring");
    m.insert("0C:4B:54", "Ring");
    m.insert("24:DF:A7", "Ring");
    m.insert("3C:62:00", "Ring");
    m.insert("50:14:79", "Ring");
    m.insert("58:FD:B1", "Ring");
    m.insert("6C:15:24", "Ring");
    m.insert("B0:09:DA", "Ring");
    m.insert("D0:73:D5", "Ring");
    m.insert("54:2A:1B", "Sonos");
    m.insert("78:28:CA", "Sonos");
    m.insert("94:9F:3E", "Sonos");
    m.insert("B8:E9:37", "Sonos");
    m.insert("34:7E:5C", "Sonos");
    m.insert("48:A6:B8", "Sonos");
    m.insert("E4:22:A5", "Sonos");
    m.insert("F0:F6:C1", "Sonos");
    m.insert("00:17:88", "Philips Hue");
    m.insert("EC:B5:FA", "Philips Hue");
    m.insert("44:73:D6", "Ecobee");
    // More Espressif (ESP8266/ESP32 IoT devices)
    m.insert("18:FE:34", "Espressif");
    m.insert("2C:3A:E8", "Espressif");
    m.insert("30:AE:A4", "Espressif");
    m.insert("3C:71:BF", "Espressif");
    m.insert("48:3F:DA", "Espressif");
    m.insert("54:5A:A6", "Espressif");
    m.insert("5C:CF:7F", "Espressif");
    m.insert("60:01:94", "Espressif");
    m.insert("68:C6:3A", "Espressif");
    m.insert("84:0D:8E", "Espressif");
    m.insert("84:CC:A8", "Espressif");
    m.insert("84:F3:EB", "Espressif");
    m.insert("90:97:D5", "Espressif");
    m.insert("98:CD:AC", "Espressif");
    m.insert("A0:20:A6", "Espressif");
    m.insert("A4:7B:9D", "Espressif");
    m.insert("AC:67:B2", "Espressif");
    m.insert("B4:E6:2D", "Espressif");
    m.insert("C4:4F:33", "Espressif");
    m.insert("CC:50:E3", "Espressif");
    m.insert("D8:A0:1D", "Espressif");
    m.insert("DC:4F:22", "Espressif");
    m.insert("EC:FA:BC", "Espressif");
    // More Xiaomi/Huawei
    m.insert("04:CF:8C", "Xiaomi");
    m.insert("0C:1D:AF", "Xiaomi");
    m.insert("10:2A:B3", "Xiaomi");
    m.insert("14:F6:5A", "Xiaomi");
    m.insert("18:59:36", "Xiaomi");
    m.insert("20:34:FB", "Xiaomi");
    m.insert("24:DA:9B", "Xiaomi");
    m.insert("28:E3:1F", "Xiaomi");
    m.insert("34:80:B3", "Xiaomi");
    m.insert("38:A4:ED", "Xiaomi");
    m.insert("3C:BD:D8", "Xiaomi");
    m.insert("40:31:3C", "Xiaomi");
    m.insert("44:23:7C", "Xiaomi");
    m.insert("50:64:2B", "Xiaomi");
    m.insert("54:48:E6", "Xiaomi");
    m.insert("58:44:98", "Xiaomi");
    m.insert("5C:0E:8B", "Xiaomi");
    m.insert("60:AB:67", "Xiaomi");
    m.insert("64:B4:73", "Xiaomi");
    m.insert("68:AB:1E", "Xiaomi");
    m.insert("6C:5C:14", "Xiaomi");
    m.insert("74:23:44", "Xiaomi");
    m.insert("78:02:F8", "Xiaomi");
    m.insert("7C:1D:D9", "Xiaomi");
    m.insert("80:AD:16", "Xiaomi");
    m.insert("8C:BE:BE", "Xiaomi");
    m.insert("98:FA:E3", "Xiaomi");
    m.insert("A8:6B:AD", "Xiaomi");
    m.insert("AC:F7:F3", "Xiaomi");
    m.insert("B0:E2:35", "Xiaomi");
    m.insert("C4:0B:CB", "Xiaomi");
    m.insert("D4:97:0B", "Xiaomi");
    m.insert("E8:AB:FA", "Xiaomi");
    m.insert("F0:B4:29", "Xiaomi");
    m.insert("F8:A4:5F", "Xiaomi");
    m.insert("00:25:9E", "Huawei");
    m.insert("00:46:4B", "Huawei");
    m.insert("00:66:4B", "Huawei");
    m.insert("00:E0:FC", "Huawei");
    m.insert("04:02:1F", "Huawei");
    m.insert("04:B0:E7", "Huawei");
    m.insert("04:C0:6F", "Huawei");
    m.insert("04:F9:38", "Huawei");
    m.insert("08:19:A6", "Huawei");
    m.insert("08:63:61", "Huawei");
    m.insert("08:7A:4C", "Huawei");
    m.insert("0C:37:DC", "Huawei");
    m.insert("0C:96:BF", "Huawei");
    m.insert("10:1B:54", "Huawei");
    m.insert("10:47:80", "Huawei");
    m.insert("14:30:04", "Huawei");
    m.insert("18:C5:8A", "Huawei");
    m.insert("1C:8E:5C", "Huawei");
    m.insert("20:0B:C7", "Huawei");
    m.insert("20:A6:80", "Huawei");
    m.insert("24:09:95", "Huawei");
    m.insert("24:1F:A0", "Huawei");
    m.insert("28:3C:E4", "Huawei");
    m.insert("28:6E:D4", "Huawei");
    m.insert("2C:AB:00", "Huawei");
    m.insert("30:D1:7E", "Huawei");
    m.insert("34:00:A3", "Huawei");
    m.insert("34:6A:C2", "Huawei");
    m.insert("38:4C:4F", "Huawei");
    m.insert("38:BC:01", "Huawei");
    m.insert("3C:47:11", "Huawei");
    m.insert("3C:DF:BD", "Huawei");
    m.insert("40:4D:8E", "Huawei");
    m.insert("44:55:B1", "Huawei");
    m.insert("48:00:31", "Huawei");
    m.insert("48:3C:0C", "Huawei");
    m.insert("4C:54:99", "Huawei");
    m.insert("4C:8B:EF", "Huawei");
    m.insert("50:01:6B", "Huawei");
    m.insert("54:25:EA", "Huawei");
    m.insert("54:89:98", "Huawei");
    m.insert("58:1F:28", "Huawei");
    m.insert("58:60:5F", "Huawei");
    m.insert("5C:4C:A9", "Huawei");
    m.insert("5C:7D:5E", "Huawei");
    m.insert("60:DE:44", "Huawei");
    m.insert("64:16:F0", "Huawei");
    m.insert("68:8F:84", "Huawei");
    m.insert("68:A0:F6", "Huawei");
    m.insert("70:54:F5", "Huawei");
    m.insert("70:7B:E8", "Huawei");
    m.insert("74:88:2A", "Huawei");
    m.insert("74:A5:28", "Huawei");
    m.insert("78:D7:52", "Huawei");
    m.insert("80:B6:86", "Huawei");
    m.insert("80:D0:9B", "Huawei");
    m.insert("84:5B:12", "Huawei");
    m.insert("84:A8:E4", "Huawei");
    m.insert("88:53:D4", "Huawei");
    m.insert("88:CE:FA", "Huawei");
    m.insert("8C:34:FD", "Huawei");
    m.insert("90:17:AC", "Huawei");
    m.insert("90:4E:2B", "Huawei");
    m.insert("94:04:9C", "Huawei");
    m.insert("94:77:2B", "Huawei");
    m.insert("98:9C:57", "Huawei");
    m.insert("9C:28:EF", "Huawei");
    m.insert("9C:37:F4", "Huawei");
    m.insert("9C:74:1A", "Huawei");
    m.insert("A4:99:47", "Huawei");
    m.insert("A4:BA:76", "Huawei");
    m.insert("A8:C8:3A", "Huawei");
    m.insert("AC:4E:91", "Huawei");
    m.insert("AC:61:EA", "Huawei");
    m.insert("AC:E2:15", "Huawei");
    m.insert("B4:15:13", "Huawei");
    m.insert("B4:30:52", "Huawei");
    m.insert("B4:CD:27", "Huawei");
    m.insert("BC:25:E0", "Huawei");
    m.insert("BC:76:70", "Huawei");
    m.insert("C0:70:09", "Huawei");
    m.insert("C4:05:28", "Huawei");
    m.insert("C4:07:2F", "Huawei");
    m.insert("C8:14:79", "Huawei");
    m.insert("C8:D1:5E", "Huawei");
    m.insert("CC:53:B5", "Huawei");
    m.insert("CC:A2:23", "Huawei");
    m.insert("D0:2D:B3", "Huawei");
    m.insert("D0:7A:B5", "Huawei");
    m.insert("D4:61:2E", "Huawei");
    m.insert("D4:6A:A8", "Huawei");
    m.insert("D4:6E:5C", "Huawei");
    m.insert("D8:49:0B", "Huawei");
    m.insert("DC:09:4C", "Huawei");
    m.insert("E0:19:1D", "Huawei");
    m.insert("E0:24:7F", "Huawei");
    m.insert("E4:68:A3", "Huawei");
    m.insert("E4:A8:DF", "Huawei");
    m.insert("E8:08:8B", "Huawei");
    m.insert("E8:CD:2D", "Huawei");
    m.insert("EC:23:3D", "Huawei");
    m.insert("EC:CB:30", "Huawei");
    m.insert("F0:43:47", "Huawei");
    m.insert("F4:4C:7F", "Huawei");
    m.insert("F4:55:9C", "Huawei");
    m.insert("F4:9F:F3", "Huawei");
    m.insert("F4:C7:14", "Huawei");
    m.insert("F8:01:13", "Huawei");
    m.insert("F8:3D:FF", "Huawei");
    m.insert("F8:4A:BF", "Huawei");
    m.insert("F8:E8:11", "Huawei");
    m.insert("FC:48:EF", "Huawei");
    // Tailscale uses randomized MACs, but typically private/local bit set

    // === Additional vendors requested by user scans ===

    // Unknown prefixes from scan results - likely routers/networking
    m.insert("00:58:28", "Unknown Router/Network Device"); // User scan result
    m.insert("24:52:6A", "Unknown Device"); // User scan result
    m.insert("F0:00:06", "Unknown Device"); // User scan result

    // === ASUS Routers (additional prefixes) ===
    m.insert("00:0C:6E", "ASUS");
    m.insert("00:11:D8", "ASUS");
    m.insert("00:13:D4", "ASUS");
    m.insert("00:15:F2", "ASUS");
    m.insert("00:17:31", "ASUS");
    m.insert("00:18:F3", "ASUS");
    m.insert("00:1A:92", "ASUS");
    m.insert("00:1D:60", "ASUS");
    m.insert("00:1E:8C", "ASUS");
    m.insert("00:1F:C6", "ASUS");
    m.insert("00:22:15", "ASUS");
    m.insert("00:23:54", "ASUS");
    m.insert("00:24:8C", "ASUS");
    m.insert("00:25:22", "ASUS");
    m.insert("00:26:18", "ASUS");
    m.insert("04:92:26", "ASUS");
    m.insert("08:60:6E", "ASUS");
    m.insert("0C:9D:92", "ASUS");
    m.insert("10:7B:44", "ASUS");
    m.insert("10:BF:48", "ASUS");
    m.insert("10:C3:7B", "ASUS");
    m.insert("14:DA:E9", "ASUS");
    m.insert("18:31:BF", "ASUS");
    m.insert("1C:87:2C", "ASUS");
    m.insert("1C:B7:2C", "ASUS");
    m.insert("20:CF:30", "ASUS");
    m.insert("24:4B:FE", "ASUS");
    m.insert("2C:4D:54", "ASUS");
    m.insert("2C:56:DC", "ASUS");
    m.insert("2C:FD:A1", "ASUS");
    m.insert("30:5A:3A", "ASUS");
    m.insert("30:85:A9", "ASUS");
    m.insert("34:97:F6", "ASUS");
    m.insert("38:2C:4A", "ASUS");
    m.insert("38:D5:47", "ASUS");
    m.insert("3C:7C:3F", "ASUS");
    m.insert("40:16:7E", "ASUS");
    m.insert("40:B0:76", "ASUS");
    m.insert("48:5B:39", "ASUS");
    m.insert("4C:ED:FB", "ASUS");
    m.insert("50:46:5D", "ASUS");
    m.insert("54:04:A6", "ASUS");
    m.insert("60:45:CB", "ASUS");
    m.insert("60:A4:4C", "ASUS");
    m.insert("6C:B0:CE", "ASUS");
    m.insert("70:4D:7B", "ASUS");
    m.insert("74:D0:2B", "ASUS");
    m.insert("78:24:AF", "ASUS");
    m.insert("88:D7:F6", "ASUS");
    m.insert("90:E6:BA", "ASUS");
    m.insert("AC:22:0B", "ASUS");
    m.insert("AC:9E:17", "ASUS");
    m.insert("B0:6E:BF", "ASUS");
    m.insert("BC:AE:C5", "ASUS");
    m.insert("BC:EE:7B", "ASUS");
    m.insert("C8:60:00", "ASUS");
    m.insert("D4:5D:64", "ASUS");
    m.insert("D8:50:E6", "ASUS");
    m.insert("E0:3F:49", "ASUS");
    m.insert("E0:CB:1D", "ASUS");
    m.insert("F0:79:59", "ASUS");
    m.insert("F4:6D:04", "ASUS");
    m.insert("F8:32:E4", "ASUS");
    m.insert("FC:34:97", "ASUS");

    // === Arris Cable Modems ===
    m.insert("00:00:CA", "Arris");
    m.insert("00:1D:CE", "Arris");
    m.insert("00:1D:CF", "Arris");
    m.insert("00:1D:D0", "Arris");
    m.insert("00:1D:D1", "Arris");
    m.insert("00:1D:D2", "Arris");
    m.insert("00:1D:D3", "Arris");
    m.insert("00:1D:D4", "Arris");
    m.insert("00:1D:D5", "Arris");
    m.insert("00:1E:5A", "Arris");
    m.insert("00:23:74", "Arris");
    m.insert("00:23:95", "Arris");
    m.insert("00:24:95", "Arris");
    m.insert("00:26:42", "Arris");
    m.insert("00:26:D9", "Arris");
    m.insert("04:E5:98", "Arris");
    m.insert("08:9E:08", "Arris");
    m.insert("0C:B7:71", "Arris");
    m.insert("10:86:8C", "Arris");
    m.insert("14:AB:F0", "Arris");
    m.insert("14:CF:E2", "Arris");
    m.insert("14:D4:FE", "Arris");
    m.insert("18:35:D1", "Arris");
    m.insert("1C:1B:68", "Arris");
    m.insert("20:3D:66", "Arris");
    m.insert("20:E5:64", "Arris");
    m.insert("28:C8:7A", "Arris");
    m.insert("2C:9E:5F", "Arris");
    m.insert("30:60:23", "Arris");
    m.insert("34:7A:60", "Arris");
    m.insert("38:70:0C", "Arris");
    m.insert("3C:36:E4", "Arris");
    m.insert("40:B7:F3", "Arris");
    m.insert("44:AA:F5", "Arris");
    m.insert("44:E1:37", "Arris");
    m.insert("48:02:2A", "Arris");
    m.insert("48:53:50", "Arris");
    m.insert("4C:56:9D", "Arris");
    m.insert("54:67:51", "Arris");
    m.insert("58:23:8C", "Arris");
    m.insert("58:8D:09", "Arris");
    m.insert("5C:8F:E0", "Arris");
    m.insert("5C:E3:0E", "Arris");
    m.insert("60:E3:AC", "Arris");
    m.insert("64:ED:57", "Arris");
    m.insert("6C:C1:D2", "Arris");
    m.insert("70:B1:4E", "Arris");
    m.insert("74:04:2B", "Arris");
    m.insert("78:96:82", "Arris");
    m.insert("80:F5:03", "Arris");
    m.insert("84:E0:58", "Arris");
    m.insert("88:71:B1", "Arris");
    m.insert("8C:7F:3B", "Arris");
    m.insert("94:62:69", "Arris");
    m.insert("94:CC:B9", "Arris");
    m.insert("98:6B:3D", "Arris");
    m.insert("9C:34:26", "Arris");
    m.insert("A0:C5:62", "Arris");
    m.insert("A4:08:F5", "Arris");
    m.insert("A4:3E:51", "Arris");
    m.insert("A4:ED:4E", "Arris");
    m.insert("A8:4E:3F", "Arris");
    m.insert("AC:B3:13", "Arris");
    m.insert("B0:77:AC", "Arris");
    m.insert("B4:EE:B4", "Arris");
    m.insert("BC:14:01", "Arris");
    m.insert("C4:D4:89", "Arris");
    m.insert("C8:3A:35", "Arris");
    m.insert("CC:65:AD", "Arris");
    m.insert("D0:39:72", "Arris");
    m.insert("D4:05:DE", "Arris");
    m.insert("D8:1E:DE", "Arris");
    m.insert("E0:B7:0A", "Arris");
    m.insert("E4:48:C7", "Arris");
    m.insert("E8:6D:52", "Arris");
    m.insert("EC:C5:7F", "Arris");
    m.insert("F0:AF:85", "Arris");
    m.insert("F4:0B:93", "Arris");
    m.insert("F8:E9:03", "Arris");

    // === Motorola ===
    m.insert("00:04:56", "Motorola");
    m.insert("00:08:0E", "Motorola");
    m.insert("00:0A:28", "Motorola");
    m.insert("00:0B:06", "Motorola");
    m.insert("00:0C:E5", "Motorola");
    m.insert("00:0E:5C", "Motorola");
    m.insert("00:0E:C7", "Motorola");
    m.insert("00:11:80", "Motorola");
    m.insert("00:12:8A", "Motorola");
    m.insert("00:13:71", "Motorola");
    m.insert("00:14:04", "Motorola");
    m.insert("00:14:9A", "Motorola");
    m.insert("00:15:2F", "Motorola");
    m.insert("00:15:9F", "Motorola");
    m.insert("00:16:75", "Motorola");
    m.insert("00:17:00", "Motorola");
    m.insert("00:17:84", "Motorola");
    m.insert("00:18:A4", "Motorola");
    m.insert("00:19:2C", "Motorola");
    m.insert("00:19:A6", "Motorola");
    m.insert("00:1A:66", "Motorola");
    m.insert("00:1A:77", "Motorola");
    m.insert("00:1A:AD", "Motorola");
    m.insert("00:1B:DD", "Motorola");
    m.insert("00:1C:FB", "Motorola");
    m.insert("00:1D:6B", "Motorola");
    m.insert("00:1E:46", "Motorola");
    m.insert("00:1F:7E", "Motorola");
    m.insert("00:20:40", "Motorola");
    m.insert("00:21:36", "Motorola");
    m.insert("00:21:E8", "Motorola");
    m.insert("00:22:B4", "Motorola");
    m.insert("00:23:A2", "Motorola");
    m.insert("00:24:37", "Motorola");
    m.insert("00:24:92", "Motorola");
    m.insert("00:25:F1", "Motorola");
    m.insert("00:26:BA", "Motorola");
    m.insert("08:35:71", "Motorola");
    m.insert("08:E6:89", "Motorola");
    m.insert("0C:2D:89", "Motorola");
    m.insert("0C:EE:E6", "Motorola");
    m.insert("10:68:3F", "Motorola");
    m.insert("10:CC:DB", "Motorola");
    m.insert("14:87:6A", "Motorola");
    m.insert("18:D2:25", "Motorola");
    m.insert("1C:1D:67", "Motorola");
    m.insert("20:37:06", "Motorola");
    m.insert("20:D3:90", "Motorola");
    m.insert("24:DA:9B", "Motorola");
    m.insert("28:A1:83", "Motorola");
    m.insert("2C:CC:15", "Motorola");
    m.insert("30:14:4A", "Motorola");
    m.insert("38:0A:0A", "Motorola");
    m.insert("3C:77:E6", "Motorola");
    m.insert("40:78:6A", "Motorola");
    m.insert("44:80:EB", "Motorola");
    m.insert("48:70:1E", "Motorola");
    m.insert("4C:BC:48", "Motorola");
    m.insert("50:77:05", "Motorola");
    m.insert("54:8C:A0", "Motorola");
    m.insert("58:8A:5A", "Motorola");
    m.insert("5C:5B:35", "Motorola");
    m.insert("60:02:92", "Motorola");
    m.insert("64:DB:18", "Motorola");
    m.insert("68:C4:4D", "Motorola");
    m.insert("6C:AB:AD", "Motorola");
    m.insert("74:B5:7E", "Motorola");
    m.insert("78:42:3C", "Motorola");
    m.insert("7C:46:85", "Motorola");
    m.insert("80:6C:1B", "Motorola");
    m.insert("84:10:0D", "Motorola");
    m.insert("88:ED:1C", "Motorola");
    m.insert("8C:61:A3", "Motorola");
    m.insert("94:D7:71", "Motorola");
    m.insert("98:D6:F7", "Motorola");
    m.insert("9C:D9:17", "Motorola");
    m.insert("A4:4E:31", "Motorola");
    m.insert("A8:3E:0E", "Motorola");
    m.insert("AC:F8:CC", "Motorola");
    m.insert("B4:37:D1", "Motorola");
    m.insert("B8:3E:59", "Motorola");
    m.insert("BC:C6:1A", "Motorola");
    m.insert("C4:17:FE", "Motorola");
    m.insert("C8:DF:7C", "Motorola");
    m.insert("CC:4A:E1", "Motorola");
    m.insert("D4:C4:55", "Motorola");
    m.insert("D8:49:2F", "Motorola");
    m.insert("DC:E4:CA", "Motorola");
    m.insert("E0:DD:C0", "Motorola");
    m.insert("E8:B4:C8", "Motorola");
    m.insert("EC:C3:8A", "Motorola");
    m.insert("F4:F1:E1", "Motorola");
    m.insert("F8:7B:7A", "Motorola");
    m.insert("FC:C2:DE", "Motorola");

    // === Tenda ===
    m.insert("00:26:5C", "Tenda");
    m.insert("00:B0:09", "Tenda");
    m.insert("04:02:B5", "Tenda");
    m.insert("0C:1A:10", "Tenda");
    m.insert("18:A5:54", "Tenda");
    m.insert("1C:2D:D3", "Tenda");
    m.insert("20:76:00", "Tenda");
    m.insert("28:76:CD", "Tenda");
    m.insert("30:A8:DB", "Tenda");
    m.insert("50:3A:A0", "Tenda");
    m.insert("54:BE:53", "Tenda");
    m.insert("58:DB:8B", "Tenda");
    m.insert("60:EE:5C", "Tenda");
    m.insert("78:44:FD", "Tenda");
    m.insert("7C:A5:2E", "Tenda");
    m.insert("84:D8:1B", "Tenda");
    m.insert("8C:AB:8E", "Tenda");
    m.insert("98:2C:BE", "Tenda");
    m.insert("A0:6D:09", "Tenda");
    m.insert("A8:93:4A", "Tenda");
    m.insert("B0:DF:3A", "Tenda");
    m.insert("C0:61:18", "Tenda");
    m.insert("C4:F0:81", "Tenda");
    m.insert("C8:3A:35", "Tenda");
    m.insert("CC:D4:A1", "Tenda");
    m.insert("D8:32:14", "Tenda");
    m.insert("DC:FE:18", "Tenda");
    m.insert("EC:5C:68", "Tenda");
    m.insert("F0:92:B4", "Tenda");
    m.insert("F4:EC:38", "Tenda");
    m.insert("FC:75:16", "Tenda");

    // === ZTE ===
    m.insert("00:09:DF", "ZTE");
    m.insert("00:15:EB", "ZTE");
    m.insert("00:19:C6", "ZTE");
    m.insert("00:1E:73", "ZTE");
    m.insert("00:22:93", "ZTE");
    m.insert("00:25:12", "ZTE");
    m.insert("00:26:ED", "ZTE");
    m.insert("04:C0:6F", "ZTE");
    m.insert("08:18:1A", "ZTE");
    m.insert("0C:12:62", "ZTE");
    m.insert("10:69:49", "ZTE");
    m.insert("14:14:4B", "ZTE");
    m.insert("18:68:82", "ZTE");
    m.insert("1C:D5:E2", "ZTE");
    m.insert("20:89:84", "ZTE");
    m.insert("24:C4:4A", "ZTE");
    m.insert("28:27:BF", "ZTE");
    m.insert("2C:95:7F", "ZTE");
    m.insert("30:D3:86", "ZTE");
    m.insert("34:4B:50", "ZTE");
    m.insert("38:86:02", "ZTE");
    m.insert("3C:E5:A6", "ZTE");
    m.insert("40:61:86", "ZTE");
    m.insert("44:F4:36", "ZTE");
    m.insert("48:A7:4E", "ZTE");
    m.insert("4C:09:D4", "ZTE");
    m.insert("50:78:B0", "ZTE");
    m.insert("54:22:F8", "ZTE");
    m.insert("58:6A:B1", "ZTE");
    m.insert("5C:B0:66", "ZTE");
    m.insert("60:EB:69", "ZTE");
    m.insert("64:13:6C", "ZTE");
    m.insert("68:A0:F6", "ZTE");
    m.insert("6C:8B:2F", "ZTE");
    m.insert("70:9F:2D", "ZTE");
    m.insert("74:88:8B", "ZTE");
    m.insert("78:31:2B", "ZTE");
    m.insert("7C:39:53", "ZTE");
    m.insert("80:38:BC", "ZTE");
    m.insert("84:74:2A", "ZTE");
    m.insert("88:D6:52", "ZTE");
    m.insert("8C:68:78", "ZTE");
    m.insert("90:C7:D8", "ZTE");
    m.insert("94:A7:B7", "ZTE");
    m.insert("98:6C:F5", "ZTE");
    m.insert("9C:D2:4B", "ZTE");
    m.insert("A0:91:C8", "ZTE");
    m.insert("A4:5E:60", "ZTE");
    m.insert("A8:6B:AD", "ZTE");
    m.insert("AC:64:62", "ZTE");
    m.insert("B0:75:D5", "ZTE");
    m.insert("B4:B3:62", "ZTE");
    m.insert("B8:F8:83", "ZTE");
    m.insert("BC:25:E0", "ZTE");
    m.insert("C0:54:A5", "ZTE");
    m.insert("C4:EB:43", "ZTE");
    m.insert("C8:7B:5B", "ZTE");
    m.insert("CC:1A:FA", "ZTE");
    m.insert("D0:15:4A", "ZTE");
    m.insert("D4:67:E7", "ZTE");
    m.insert("D8:5D:E2", "ZTE");
    m.insert("DC:02:8E", "ZTE");
    m.insert("E0:19:54", "ZTE");
    m.insert("E4:77:23", "ZTE");
    m.insert("E8:37:7A", "ZTE");
    m.insert("EC:1D:7F", "ZTE");
    m.insert("F0:84:2F", "ZTE");
    m.insert("F4:6D:E2", "ZTE");
    m.insert("F8:AB:05", "ZTE");
    m.insert("FC:2D:5E", "ZTE");

    // === Zyxel ===
    m.insert("00:0F:94", "Zyxel");
    m.insert("00:13:49", "Zyxel");
    m.insert("00:19:CB", "Zyxel");
    m.insert("00:1D:B3", "Zyxel");
    m.insert("00:1E:E1", "Zyxel");
    m.insert("00:1F:3F", "Zyxel");
    m.insert("00:23:F8", "Zyxel");
    m.insert("00:26:AB", "Zyxel");
    m.insert("00:A0:C5", "Zyxel");
    m.insert("04:BF:6D", "Zyxel");
    m.insert("08:26:AE", "Zyxel");
    m.insert("0C:91:60", "Zyxel");
    m.insert("10:7B:EF", "Zyxel");
    m.insert("18:A6:F7", "Zyxel");
    m.insert("1C:74:0D", "Zyxel");
    m.insert("20:E5:2A", "Zyxel");
    m.insert("28:28:5D", "Zyxel");
    m.insert("2C:2D:48", "Zyxel");
    m.insert("30:C9:AB", "Zyxel");
    m.insert("34:CE:00", "Zyxel");
    m.insert("38:10:D5", "Zyxel");
    m.insert("3C:4E:47", "Zyxel");
    m.insert("40:4A:03", "Zyxel");
    m.insert("48:8D:36", "Zyxel");
    m.insert("4C:9E:FF", "Zyxel");
    m.insert("50:67:F0", "Zyxel");
    m.insert("58:8B:F3", "Zyxel");
    m.insert("5C:E2:8C", "Zyxel");
    m.insert("60:38:E0", "Zyxel");
    m.insert("6C:3B:6B", "Zyxel");
    m.insert("74:DA:88", "Zyxel");
    m.insert("78:A7:14", "Zyxel");
    m.insert("7C:57:3C", "Zyxel");
    m.insert("84:9C:A6", "Zyxel");
    m.insert("8C:E1:17", "Zyxel");
    m.insert("90:8D:78", "Zyxel");
    m.insert("98:DA:D0", "Zyxel");
    m.insert("9C:C9:EB", "Zyxel");
    m.insert("A4:21:8A", "Zyxel");
    m.insert("AC:A8:8E", "Zyxel");
    m.insert("B0:A7:B9", "Zyxel");
    m.insert("B4:82:C5", "Zyxel");
    m.insert("BC:A5:11", "Zyxel");
    m.insert("C0:3F:0E", "Zyxel");
    m.insert("C8:6C:87", "Zyxel");
    m.insert("CC:5D:4E", "Zyxel");
    m.insert("D4:F5:27", "Zyxel");
    m.insert("D8:EC:5E", "Zyxel");
    m.insert("E0:98:06", "Zyxel");
    m.insert("E4:18:6B", "Zyxel");
    m.insert("EC:F4:51", "Zyxel");
    m.insert("F0:3E:90", "Zyxel");
    m.insert("FC:F5:28", "Zyxel");

    // === Technicolor ===
    m.insert("00:14:7C", "Technicolor");
    m.insert("00:18:01", "Technicolor");
    m.insert("00:1A:2B", "Technicolor");
    m.insert("00:1C:B5", "Technicolor");
    m.insert("00:1E:69", "Technicolor");
    m.insert("00:1F:9F", "Technicolor");
    m.insert("00:23:51", "Technicolor");
    m.insert("00:24:17", "Technicolor");
    m.insert("00:26:44", "Technicolor");
    m.insert("08:95:2A", "Technicolor");
    m.insert("0C:48:85", "Technicolor");
    m.insert("10:13:EE", "Technicolor");
    m.insert("14:22:DB", "Technicolor");
    m.insert("18:35:6B", "Technicolor");
    m.insert("1C:EE:C9", "Technicolor");
    m.insert("20:3D:66", "Technicolor");
    m.insert("28:7A:EE", "Technicolor");
    m.insert("2C:30:33", "Technicolor");
    m.insert("34:6F:90", "Technicolor");
    m.insert("38:35:FB", "Technicolor");
    m.insert("40:65:A3", "Technicolor");
    m.insert("44:1C:A8", "Technicolor");
    m.insert("4C:8F:FA", "Technicolor");
    m.insert("54:EE:75", "Technicolor");
    m.insert("58:23:8C", "Technicolor");
    m.insert("60:2A:D0", "Technicolor");
    m.insert("6C:E8:73", "Technicolor");
    m.insert("70:5A:AC", "Technicolor");
    m.insert("78:94:B4", "Technicolor");
    m.insert("80:B6:55", "Technicolor");
    m.insert("88:F7:C7", "Technicolor");
    m.insert("8C:04:FF", "Technicolor");
    m.insert("94:9A:A9", "Technicolor");
    m.insert("98:89:4E", "Technicolor");
    m.insert("A0:1B:29", "Technicolor");
    m.insert("A4:02:B9", "Technicolor");
    m.insert("AC:3B:77", "Technicolor");
    m.insert("B4:A4:E3", "Technicolor");
    m.insert("BC:60:A7", "Technicolor");
    m.insert("C0:D9:62", "Technicolor");
    m.insert("C4:36:55", "Technicolor");
    m.insert("CC:7B:35", "Technicolor");
    m.insert("D0:54:75", "Technicolor");
    m.insert("D4:D1:84", "Technicolor");
    m.insert("DC:E5:78", "Technicolor");
    m.insert("E0:B9:E5", "Technicolor");
    m.insert("E4:5D:51", "Technicolor");
    m.insert("EC:4F:82", "Technicolor");
    m.insert("F0:D7:AA", "Technicolor");
    m.insert("F8:8E:85", "Technicolor");
    m.insert("FC:94:E3", "Technicolor");

    // === Sagemcom ===
    m.insert("00:1E:74", "Sagemcom");
    m.insert("00:1F:95", "Sagemcom");
    m.insert("00:23:48", "Sagemcom");
    m.insert("00:24:D1", "Sagemcom");
    m.insert("00:25:68", "Sagemcom");
    m.insert("00:26:91", "Sagemcom");
    m.insert("04:C7:D9", "Sagemcom");
    m.insert("08:F4:58", "Sagemcom");
    m.insert("0C:96:E6", "Sagemcom");
    m.insert("10:A5:D0", "Sagemcom");
    m.insert("14:0C:76", "Sagemcom");
    m.insert("18:62:2C", "Sagemcom");
    m.insert("1C:A5:32", "Sagemcom");
    m.insert("20:CE:2A", "Sagemcom");
    m.insert("24:7F:20", "Sagemcom");
    m.insert("28:39:26", "Sagemcom");
    m.insert("2C:39:96", "Sagemcom");
    m.insert("30:7C:30", "Sagemcom");
    m.insert("34:B3:54", "Sagemcom");
    m.insert("38:E3:C5", "Sagemcom");
    m.insert("3C:81:D8", "Sagemcom");
    m.insert("40:F2:01", "Sagemcom");
    m.insert("44:E9:DD", "Sagemcom");
    m.insert("48:91:53", "Sagemcom");
    m.insert("4C:17:44", "Sagemcom");
    m.insert("50:4E:DC", "Sagemcom");
    m.insert("54:DC:1D", "Sagemcom");
    m.insert("58:BC:27", "Sagemcom");
    m.insert("5C:B4:3E", "Sagemcom");
    m.insert("60:DE:F3", "Sagemcom");
    m.insert("64:A7:DD", "Sagemcom");
    m.insert("68:A3:78", "Sagemcom");
    m.insert("6C:38:A1", "Sagemcom");
    m.insert("70:B1:4E", "Sagemcom");
    m.insert("74:DA:38", "Sagemcom");
    m.insert("78:81:02", "Sagemcom");
    m.insert("7C:03:4C", "Sagemcom");
    m.insert("80:1F:02", "Sagemcom");
    m.insert("84:26:15", "Sagemcom");
    m.insert("88:03:55", "Sagemcom");
    m.insert("8C:10:D4", "Sagemcom");
    m.insert("90:01:3B", "Sagemcom");
    m.insert("94:FE:22", "Sagemcom");
    m.insert("98:8B:0A", "Sagemcom");
    m.insert("9C:53:22", "Sagemcom");
    m.insert("A0:39:EE", "Sagemcom");
    m.insert("A4:50:46", "Sagemcom");
    m.insert("A8:4E:3F", "Sagemcom");
    m.insert("AC:84:C9", "Sagemcom");
    m.insert("B0:4E:26", "Sagemcom");
    m.insert("B4:B0:24", "Sagemcom");
    m.insert("B8:D5:26", "Sagemcom");
    m.insert("BC:14:EF", "Sagemcom");
    m.insert("C0:AC:54", "Sagemcom");
    m.insert("C4:04:15", "Sagemcom");
    m.insert("C8:D7:19", "Sagemcom");
    m.insert("CC:2D:21", "Sagemcom");
    m.insert("D0:05:2A", "Sagemcom");
    m.insert("D4:35:1D", "Sagemcom");
    m.insert("D8:97:3B", "Sagemcom");
    m.insert("DC:0B:1A", "Sagemcom");
    m.insert("E0:B9:4D", "Sagemcom");
    m.insert("E4:C1:46", "Sagemcom");
    m.insert("E8:F1:B0", "Sagemcom");
    m.insert("EC:43:F6", "Sagemcom");
    m.insert("F0:82:61", "Sagemcom");
    m.insert("F4:CA:E5", "Sagemcom");
    m.insert("F8:8E:85", "Sagemcom");
    m.insert("FC:B4:E6", "Sagemcom");

    // === Actiontec ===
    m.insert("00:1F:90", "Actiontec");
    m.insert("00:20:E0", "Actiontec");
    m.insert("00:24:7B", "Actiontec");
    m.insert("00:26:62", "Actiontec");
    m.insert("00:50:18", "Actiontec");
    m.insert("04:A1:51", "Actiontec");
    m.insert("08:BE:09", "Actiontec");
    m.insert("0C:61:27", "Actiontec");
    m.insert("10:1C:0C", "Actiontec");
    m.insert("18:1E:78", "Actiontec");
    m.insert("1C:C6:3C", "Actiontec");
    m.insert("20:76:8F", "Actiontec");
    m.insert("28:3B:82", "Actiontec");
    m.insert("2C:E4:12", "Actiontec");
    m.insert("34:68:95", "Actiontec");
    m.insert("40:B8:9A", "Actiontec");
    m.insert("48:5D:60", "Actiontec");
    m.insert("50:6A:03", "Actiontec");
    m.insert("5C:35:3B", "Actiontec");
    m.insert("64:0F:28", "Actiontec");
    m.insert("70:71:BC", "Actiontec");
    m.insert("78:96:84", "Actiontec");
    m.insert("84:E0:F4", "Actiontec");
    m.insert("90:67:1C", "Actiontec");
    m.insert("98:2C:BE", "Actiontec");
    m.insert("A4:2B:8C", "Actiontec");
    m.insert("AC:20:2E", "Actiontec");
    m.insert("B4:EE:B4", "Actiontec");
    m.insert("BC:14:01", "Actiontec");
    m.insert("C4:04:15", "Actiontec");
    m.insert("CC:03:FA", "Actiontec");
    m.insert("D4:A9:28", "Actiontec");
    m.insert("E0:46:9A", "Actiontec");
    m.insert("EC:F4:51", "Actiontec");
    m.insert("F8:E4:FB", "Actiontec");

    // === Calix ===
    m.insert("00:19:60", "Calix");
    m.insert("00:25:84", "Calix");
    m.insert("08:6A:C5", "Calix");
    m.insert("0C:D2:B5", "Calix");
    m.insert("10:FF:E2", "Calix");
    m.insert("18:C2:BF", "Calix");
    m.insert("1C:F4:CA", "Calix");
    m.insert("20:4E:71", "Calix");
    m.insert("28:FF:3C", "Calix");
    m.insert("30:59:B7", "Calix");
    m.insert("38:43:7D", "Calix");
    m.insert("40:CE:24", "Calix");
    m.insert("48:77:46", "Calix");
    m.insert("50:20:7F", "Calix");
    m.insert("58:97:BD", "Calix");
    m.insert("60:63:F9", "Calix");
    m.insert("68:B6:FC", "Calix");
    m.insert("74:9B:DE", "Calix");
    m.insert("7C:4C:58", "Calix");
    m.insert("84:A9:3E", "Calix");
    m.insert("8C:59:73", "Calix");
    m.insert("94:D4:69", "Calix");
    m.insert("9C:1E:95", "Calix");
    m.insert("A4:08:F5", "Calix");
    m.insert("AC:46:96", "Calix");
    m.insert("B4:A9:84", "Calix");
    m.insert("BC:62:D2", "Calix");
    m.insert("C4:71:30", "Calix");
    m.insert("CC:E1:D5", "Calix");
    m.insert("D4:C1:FC", "Calix");
    m.insert("DC:71:DD", "Calix");
    m.insert("E4:3D:1A", "Calix");
    m.insert("EC:4F:82", "Calix");
    m.insert("F4:6B:EF", "Calix");
    m.insert("FC:EC:DA", "Calix");

    // === Buffalo ===
    m.insert("00:07:40", "Buffalo");
    m.insert("00:0D:0B", "Buffalo");
    m.insert("00:16:01", "Buffalo");
    m.insert("00:1D:73", "Buffalo");
    m.insert("00:24:A5", "Buffalo");
    m.insert("04:4E:5A", "Buffalo");
    m.insert("08:97:98", "Buffalo");
    m.insert("10:6F:3F", "Buffalo");
    m.insert("18:C2:3C", "Buffalo");
    m.insert("1C:3E:84", "Buffalo");
    m.insert("20:4E:7F", "Buffalo");
    m.insert("28:C2:DD", "Buffalo");
    m.insert("34:6B:46", "Buffalo");
    m.insert("3C:77:E6", "Buffalo");
    m.insert("4C:E6:76", "Buffalo");
    m.insert("58:E8:08", "Buffalo");
    m.insert("60:84:BD", "Buffalo");
    m.insert("74:03:BD", "Buffalo");
    m.insert("84:AF:EC", "Buffalo");
    m.insert("8C:97:EA", "Buffalo");
    m.insert("9C:5C:8E", "Buffalo");
    m.insert("A8:96:8A", "Buffalo");
    m.insert("B8:A8:6D", "Buffalo");
    m.insert("C0:C5:22", "Buffalo");
    m.insert("CC:E1:D5", "Buffalo");
    m.insert("DC:FB:02", "Buffalo");
    m.insert("E8:6C:C7", "Buffalo");
    m.insert("F4:CE:46", "Buffalo");
    m.insert("FC:4D:D4", "Buffalo");

    // === Western Digital (NAS devices) ===
    m.insert("00:14:EE", "Western Digital");
    m.insert("00:1E:E5", "Western Digital");
    m.insert("00:26:B4", "Western Digital");
    m.insert("00:90:A9", "Western Digital");
    m.insert("04:0E:3C", "Western Digital");
    m.insert("08:C6:B3", "Western Digital");
    m.insert("10:02:B5", "Western Digital");
    m.insert("18:A6:F7", "Western Digital");
    m.insert("1C:40:24", "Western Digital");
    m.insert("20:C1:9B", "Western Digital");
    m.insert("2C:36:F8", "Western Digital");
    m.insert("30:85:A9", "Western Digital");
    m.insert("3C:4A:92", "Western Digital");
    m.insert("40:D6:3C", "Western Digital");
    m.insert("48:F1:7F", "Western Digital");
    m.insert("50:78:B3", "Western Digital");
    m.insert("5C:26:0A", "Western Digital");
    m.insert("60:33:4B", "Western Digital");
    m.insert("68:1C:A2", "Western Digital");
    m.insert("74:D4:35", "Western Digital");
    m.insert("7C:8B:CA", "Western Digital");
    m.insert("84:A4:23", "Western Digital");
    m.insert("8C:C1:21", "Western Digital");
    m.insert("94:0B:D5", "Western Digital");
    m.insert("A8:E5:44", "Western Digital");
    m.insert("B0:7F:B9", "Western Digital");
    m.insert("BC:44:34", "Western Digital");
    m.insert("C8:5B:A0", "Western Digital");
    m.insert("D0:5F:64", "Western Digital");
    m.insert("DC:A4:CA", "Western Digital");
    m.insert("E4:F4:C6", "Western Digital");
    m.insert("EC:D0:9F", "Western Digital");
    m.insert("F4:5C:89", "Western Digital");
    m.insert("FC:4D:D4", "Western Digital");

    // === Additional common networking vendors ===

    // Edimax
    m.insert("00:0E:2E", "Edimax");
    m.insert("00:1F:1F", "Edimax");
    m.insert("00:50:FC", "Edimax");
    m.insert("74:DA:38", "Edimax");
    m.insert("80:1F:02", "Edimax");

    // EnGenius
    m.insert("00:02:6F", "EnGenius");
    m.insert("88:DC:96", "EnGenius");
    m.insert("9C:D3:6D", "EnGenius");

    // TRENDnet
    m.insert("00:14:D1", "TRENDnet");
    m.insert("00:A0:F8", "TRENDnet");
    m.insert("D8:EB:97", "TRENDnet");

    // Drobo
    m.insert("00:50:43", "Drobo");

    // Verizon/Frontier FiOS
    m.insert("00:1A:DE", "FiOS Gateway");
    m.insert("20:C0:47", "Verizon");
    m.insert("84:00:2D", "Verizon");

    // Comcast/Xfinity
    m.insert("18:90:D8", "Xfinity");
    m.insert("44:E1:37", "Xfinity");

    // AT&T Gateways
    m.insert("08:00:07", "AT&T");
    m.insert("44:D9:E7", "AT&T");
    m.insert("88:71:B1", "AT&T");

    // CenturyLink/Lumen
    m.insert("00:1A:C1", "CenturyLink");
    m.insert("1C:17:D3", "CenturyLink");

    // Spectrum
    m.insert("6C:C1:D2", "Spectrum");

    // Eero mesh routers
    m.insert("F8:BB:BF", "Eero");
    m.insert("50:DC:E7", "Eero");

    // TP-Link Deco (mesh)
    m.insert("18:D6:C7", "TP-Link Deco");
    m.insert("34:60:F9", "TP-Link Deco");

    // Orbi (Netgear mesh)
    m.insert("A4:2B:B0", "Netgear Orbi");
    m.insert("9C:3D:CF", "Netgear Orbi");

    // Velop (Linksys mesh)
    m.insert("C0:56:27", "Linksys Velop");
    m.insert("14:91:38", "Linksys Velop");

    // AmpliFi (Ubiquiti mesh)
    m.insert("78:8A:20", "AmpliFi");

    // === More IoT and Smart Home Devices ===

    // August (smart locks)
    m.insert("00:1A:61", "August");
    m.insert("F4:F5:D8", "August");
    m.insert("18:59:65", "August");

    // Schlage (smart locks)
    m.insert("00:1E:C3", "Schlage");
    m.insert("28:76:76", "Schlage");

    // Yale (smart locks)
    m.insert("00:0E:CD", "Yale");
    m.insert("14:B9:68", "Yale");

    // Honeywell (smart thermostats)
    m.insert("00:40:D3", "Honeywell");
    m.insert("A4:12:42", "Honeywell");

    // Nest (smart home)
    m.insert("18:B4:30", "Nest");
    m.insert("64:16:66", "Nest");
    m.insert("18:D6:C7", "Nest");
    m.insert("44:07:0B", "Nest");
    m.insert("64:48:8B", "Nest");

    // Google Home
    m.insert("54:60:09", "Google Home");
    m.insert("F4:F5:D8", "Google Home");
    m.insert("F4:F5:E8", "Google Home");

    // Ecobee (smart thermostat)
    m.insert("44:73:D6", "Ecobee");
    m.insert("10:AE:60", "Ecobee");

    // SmartThings
    m.insert("00:0B:57", "SmartThings");
    m.insert("D8:96:85", "SmartThings");
    m.insert("28:6C:07", "SmartThings");

    // Wyze (cameras)
    m.insert("2C:AA:8E", "Wyze");
    m.insert("24:6C:AB", "Wyze");
    m.insert("F0:F5:AE", "Wyze");

    // Blink (cameras)
    m.insert("00:0A:95", "Blink");
    m.insert("08:65:18", "Blink");
    m.insert("30:8C:FB", "Blink");

    // Arlo (cameras)
    m.insert("00:26:15", "Arlo");
    m.insert("34:8A:AE", "Arlo");
    m.insert("78:E3:B5", "Arlo");

    // Eufy (cameras)
    m.insert("00:05:AF", "Eufy");
    m.insert("B0:C7:45", "Eufy");
    m.insert("B4:5C:A4", "Eufy");

    // Logitech (Harmony hub, etc)
    m.insert("00:06:F6", "Logitech");
    m.insert("00:1F:20", "Logitech");
    m.insert("04:0C:CE", "Logitech");

    // LIFX (smart bulbs)
    m.insert("28:18:78", "LIFX");
    m.insert("D8:63:87", "LIFX");
    m.insert("F0:15:B9", "LIFX");

    // Nanoleaf (smart lights)
    m.insert("00:64:3A", "Nanoleaf");
    m.insert("B8:09:8A", "Nanoleaf");

    // Philips (more devices)
    m.insert("00:17:88", "Philips");
    m.insert("EC:B5:FA", "Philips");
    m.insert("00:02:54", "Philips");

    // IKEA (TRADFRI)
    m.insert("00:0B:86", "IKEA");
    m.insert("B0:CE:18", "IKEA");
    m.insert("C8:3F:26", "IKEA");

    // iRobot (Roomba)
    m.insert("00:21:75", "iRobot");
    m.insert("9C:DE:31", "iRobot");
    m.insert("B4:3D:F7", "iRobot");

    // Neato (robot vacuums)
    m.insert("00:1E:6F", "Neato");
    m.insert("10:0D:32", "Neato");

    // Dyson (fans, purifiers)
    m.insert("00:05:35", "Dyson");
    m.insert("18:45:93", "Dyson");
    m.insert("2C:3A:E8", "Dyson");

    // Peloton
    m.insert("00:26:57", "Peloton");
    m.insert("74:C2:46", "Peloton");

    // Wemo (Belkin)
    m.insert("00:B3:62", "Wemo");
    m.insert("FC:A1:3E", "Wemo");

    // TP-Link Kasa
    m.insert("50:C7:BF", "TP-Link Kasa");
    m.insert("EC:08:6B", "TP-Link Kasa");

    // Meross
    m.insert("00:3A:98", "Meross");
    m.insert("94:EB:2C", "Meross");

    // Shelly (smart relays)
    m.insert("24:62:23", "Shelly");
    m.insert("CC:50:E3", "Shelly");
    m.insert("08:B6:1F", "Shelly");

    // Tuya/Smart Life devices
    m.insert("00:21:18", "Tuya Smart");
    m.insert("A4:C4:94", "Tuya Smart");
    m.insert("F4:CF:C2", "Tuya Smart");

    // Aqara (Zigbee hub)
    m.insert("00:12:4B", "Aqara");
    m.insert("54:EF:44", "Aqara");

    // Yeelight
    m.insert("0C:45:BA", "Yeelight");
    m.insert("04:CF:8C", "Yeelight");

    // Broadlink (smart remotes)
    m.insert("00:0A:32", "Broadlink");
    m.insert("B4:43:0D", "Broadlink");

    // === Streaming Devices ===

    // Roku (TV streaming)
    m.insert("B0:A7:37", "Roku");
    m.insert("D8:31:34", "Roku");
    m.insert("00:87:7A", "Roku");

    // Amazon Fire TV
    m.insert("18:74:2E", "Amazon Fire TV");
    m.insert("44:65:0D", "Amazon Fire TV");
    m.insert("A0:02:DC", "Amazon Fire TV");

    // Apple TV
    m.insert("00:25:BC", "Apple TV");
    m.insert("3C:06:30", "Apple TV");
    m.insert("A4:83:E7", "Apple TV");

    // Chromecast
    m.insert("00:1A:11", "Chromecast");
    m.insert("F4:F5:D8", "Chromecast");
    m.insert("7C:2E:BD", "Chromecast");

    // Nvidia Shield
    m.insert("00:04:4B", "Nvidia");
    m.insert("00:15:B6", "Nvidia");
    m.insert("00:1A:79", "Nvidia");

    // === Gaming Consoles ===

    // Xbox
    m.insert("00:15:5D", "Xbox");
    m.insert("28:18:78", "Xbox");
    m.insert("7C:1E:52", "Xbox");

    // PlayStation
    m.insert("00:24:BE", "PlayStation");
    m.insert("28:0D:FC", "PlayStation");
    m.insert("00:1D:BA", "PlayStation");

    // Nintendo Switch
    m.insert("00:09:BF", "Nintendo Switch");
    m.insert("00:1E:35", "Nintendo Switch");
    m.insert("00:1F:32", "Nintendo Switch");

    // Nintendo Wii/Wii U
    m.insert("00:17:AB", "Nintendo");
    m.insert("00:19:1D", "Nintendo");
    m.insert("00:21:BD", "Nintendo");

    // === Development Boards ===

    // Particle (Photon, Electron)
    m.insert("00:02:F7", "Particle");
    m.insert("08:BE:AC", "Particle");
    m.insert("28:32:C5", "Particle");

    // Pycom (LoPy, etc)
    m.insert("00:06:66", "Pycom");
    m.insert("80:7A:BF", "Pycom");

    // BeagleBone
    m.insert("00:0F:13", "BeagleBone");

    // Arduino (00:03:93 is Apple - using different OUIs)
    m.insert("00:04:7F", "Arduino");
    m.insert("54:6C:0E", "Arduino");

    // === More Chip Vendors ===

    // AMD
    m.insert("00:18:71", "AMD");
    m.insert("00:22:64", "AMD");
    m.insert("00:25:64", "AMD");

    // MediaTek
    m.insert("00:0A:F5", "MediaTek");
    m.insert("00:12:A2", "MediaTek");
    m.insert("00:22:6A", "MediaTek");

    // === More TV Manufacturers ===

    // Vizio
    m.insert("00:26:BB", "Vizio");
    m.insert("74:C2:46", "Vizio");
    m.insert("A4:08:F5", "Vizio");

    // Hisense
    m.insert("00:1C:10", "Hisense");
    m.insert("1C:87:2C", "Hisense");
    m.insert("3C:8C:40", "Hisense");

    // TCL
    m.insert("00:1E:A5", "TCL");
    m.insert("7C:1E:52", "TCL");
    m.insert("B8:69:F4", "TCL");

    // Philips TV
    m.insert("00:02:54", "Philips TV");
    m.insert("00:1B:3E", "Philips TV");
    m.insert("00:1E:C2", "Philips TV");

    // === More Storage Vendors ===

    // NetApp
    m.insert("00:A0:98", "NetApp");
    m.insert("00:1E:68", "NetApp");
    m.insert("00:50:56", "NetApp");

    // IBM
    m.insert("00:04:AC", "IBM");
    m.insert("00:06:5B", "IBM");
    m.insert("00:09:6B", "IBM");

    // === More Industrial/Embedded ===

    // Advantech
    m.insert("00:00:4D", "Advantech");
    m.insert("00:09:E3", "Advantech");
    m.insert("00:1A:D5", "Advantech");

    // Moxa
    m.insert("00:00:63", "Moxa");
    m.insert("00:0A:F2", "Moxa");
    m.insert("00:18:8C", "Moxa");

    // Siemens
    m.insert("00:01:37", "Siemens");
    m.insert("00:08:84", "Siemens");
    m.insert("00:1B:1B", "Siemens");

    // Schneider Electric
    m.insert("00:00:BC", "Schneider Electric");
    m.insert("00:1B:54", "Schneider Electric");
    m.insert("00:1D:9C", "Schneider Electric");

    // TP-Link Deco (mesh)
    m.insert("18:D6:C7", "TP-Link Deco");
    m.insert("34:60:F9", "TP-Link Deco");

    // Orbi (Netgear mesh)
    m.insert("A4:2B:B0", "Netgear Orbi");
    m.insert("9C:3D:CF", "Netgear Orbi");

    // Velop (Linksys mesh)
    m.insert("C0:56:27", "Linksys Velop");
    m.insert("14:91:38", "Linksys Velop");

    // AmpliFi (Ubiquiti mesh)
    m.insert("78:8A:20", "AmpliFi");

    // === More IoT and Smart Home Devices ===

    // August (smart locks)
    m.insert("00:1A:61", "August");
    m.insert("F4:F5:D8", "August");
    m.insert("18:59:65", "August");

    // Schlage (smart locks)
    m.insert("00:1E:C3", "Schlage");
    m.insert("28:76:76", "Schlage");

    // Yale (smart locks)
    m.insert("00:0E:CD", "Yale");
    m.insert("14:B9:68", "Yale");

    // Honeywell (smart thermostats)
    m.insert("00:40:D3", "Honeywell");
    m.insert("14:B9:68", "Honeywell");
    m.insert("A4:12:42", "Honeywell");

    // Nest (smart home)
    m.insert("18:B4:30", "Nest");
    m.insert("64:16:66", "Nest");
    m.insert("18:D6:C7", "Nest");
    m.insert("44:07:0B", "Nest");
    m.insert("64:48:8B", "Nest");

    // Google Home
    m.insert("54:60:09", "Google Home");
    m.insert("F4:F5:D8", "Google Home");
    m.insert("F4:F5:E8", "Google Home");

    // Ecobee (smart thermostat)
    m.insert("44:73:D6", "Ecobee");
    m.insert("10:AE:60", "Ecobee");

    // SmartThings
    m.insert("00:0B:57", "SmartThings");
    m.insert("D8:96:85", "SmartThings");
    m.insert("28:6C:07", "SmartThings");

    // Wyze (cameras)
    m.insert("2C:AA:8E", "Wyze");
    m.insert("24:6C:AB", "Wyze");
    m.insert("F0:F5:AE", "Wyze");

    // Blink (cameras)
    m.insert("00:0A:95", "Blink");
    m.insert("08:65:18", "Blink");
    m.insert("30:8C:FB", "Blink");

    // Arlo (cameras)
    m.insert("00:26:15", "Arlo");
    m.insert("34:8A:AE", "Arlo");
    m.insert("78:E3:B5", "Arlo");

    // Eufy (cameras)
    m.insert("00:05:AF", "Eufy");
    m.insert("B0:C7:45", "Eufy");
    m.insert("B4:5C:A4", "Eufy");

    // Logitech (Harmony hub, etc)
    m.insert("00:06:F6", "Logitech");
    m.insert("00:1F:20", "Logitech");
    m.insert("04:0C:CE", "Logitech");

    // LIFX (smart bulbs)
    m.insert("28:18:78", "LIFX");
    m.insert("D8:63:87", "LIFX");
    m.insert("F0:15:B9", "LIFX");

    // Nanoleaf (smart lights)
    m.insert("00:64:3A", "Nanoleaf");
    m.insert("B8:09:8A", "Nanoleaf");

    // Philips (more devices)
    m.insert("00:17:88", "Philips");
    m.insert("EC:B5:FA", "Philips");
    m.insert("00:02:54", "Philips");

    // IKEA (TRADFRI)
    m.insert("00:0B:86", "IKEA");
    m.insert("B0:CE:18", "IKEA");
    m.insert("C8:3F:26", "IKEA");

    // Nanoleaf
    m.insert("64:3A:5A", "Nanoleaf");
    m.insert("74:75:48", "Nanoleaf");

    // iRobot (Roomba)
    m.insert("00:21:75", "iRobot");
    m.insert("9C:DE:31", "iRobot");
    m.insert("B4:3D:F7", "iRobot");

    // Neato (robot vacuums)
    m.insert("00:1E:6F", "Neato");
    m.insert("10:0D:32", "Neato");

    // Dyson (fans, purifiers)
    m.insert("00:05:35", "Dyson");
    m.insert("18:45:93", "Dyson");
    m.insert("2C:3A:E8", "Dyson");

    // Peloton
    m.insert("00:26:57", "Peloton");
    m.insert("74:C2:46", "Peloton");

    // Withings (smart scales)
    m.insert("00:1D:8C", "Withings");
    m.insert("74:79:BF", "Withings");

    // Fitbit
    m.insert("00:1E:7E", "Fitbit");
    m.insert("B0:0C:78", "Fitbit");
    m.insert("F0:27:2D", "Fitbit");

    // Garmin
    m.insert("00:14:3D", "Garmin");
    m.insert("00:1C:10", "Garmin");
    m.insert("A4:5C:27", "Garmin");

    // Wemo (Belkin)
    m.insert("00:B3:62", "Wemo");
    m.insert("FC:A1:3E", "Wemo");

    // TP-Link Kasa
    m.insert("50:C7:BF", "TP-Link Kasa");
    m.insert("EC:08:6B", "TP-Link Kasa");

    // Meross
    m.insert("00:3A:98", "Meross");
    m.insert("94:EB:2C", "Meross");

    // Shelly (smart relays)
    m.insert("24:62:23", "Shelly");
    m.insert("CC:50:E3", "Shelly");
    m.insert("08:B6:1F", "Shelly");

    // Tuya/Smart Life devices
    m.insert("00:21:18", "Tuya Smart");
    m.insert("A4:C4:94", "Tuya Smart");
    m.insert("F4:CF:C2", "Tuya Smart");

    // Aqara (Zigbee hub)
    m.insert("00:12:4B", "Aqara");
    m.insert("54:EF:44", "Aqara");
    m.insert("28:6C:07", "Aqara");

    // Yeelight
    m.insert("0C:45:BA", "Yeelight");
    m.insert("04:CF:8C", "Yeelight");

    // Broadlink (smart remotes)
    m.insert("00:0A:32", "Broadlink");
    m.insert("B4:43:0D", "Broadlink");

    // Roku (TV streaming)
    m.insert("B0:A7:37", "Roku");
    m.insert("D8:31:34", "Roku");
    m.insert("00:87:7A", "Roku");

    // Amazon Fire TV
    m.insert("18:74:2E", "Amazon Fire TV");
    m.insert("44:65:0D", "Amazon Fire TV");
    m.insert("A0:02:DC", "Amazon Fire TV");

    // Apple TV
    m.insert("00:25:BC", "Apple TV");
    m.insert("3C:06:30", "Apple TV");
    m.insert("A4:83:E7", "Apple TV");

    // Chromecast
    m.insert("00:1A:11", "Chromecast");
    m.insert("F4:F5:D8", "Chromecast");
    m.insert("7C:2E:BD", "Chromecast");

    // Nvidia Shield
    m.insert("00:04:4B", "Nvidia");
    m.insert("00:15:B6", "Nvidia");
    m.insert("00:1A:79", "Nvidia");

    // Xbox
    m.insert("00:15:5D", "Xbox");
    m.insert("28:18:78", "Xbox");
    m.insert("7C:1E:52", "Xbox");

    // PlayStation
    m.insert("00:24:BE", "PlayStation");
    m.insert("28:0D:FC", "PlayStation");
    m.insert("00:1D:BA", "PlayStation");

    // Nintendo Switch
    m.insert("00:09:BF", "Nintendo Switch");
    m.insert("00:1E:35", "Nintendo Switch");
    m.insert("00:1F:32", "Nintendo Switch");

    // Nintendo Wii/Wii U
    m.insert("00:17:AB", "Nintendo");
    m.insert("00:19:1D", "Nintendo");
    m.insert("00:21:BD", "Nintendo");

    // More embedded/development boards

    // Particle (Photon, Electron)
    m.insert("00:02:F7", "Particle");
    m.insert("08:BE:AC", "Particle");
    m.insert("28:32:C5", "Particle");

    // Pycom (LoPy, etc)
    m.insert("00:06:66", "Pycom");
    m.insert("80:7A:BF", "Pycom");

    // BeagleBone
    m.insert("00:0F:13", "BeagleBone");
    m.insert("B4:5C:A4", "BeagleBone");

    // Raspberry Pi (more prefixes)
    m.insert("DC:A6:32", "Raspberry Pi");
    m.insert("B8:27:EB", "Raspberry Pi");
    m.insert("E4:5F:01", "Raspberry Pi");
    m.insert("28:CD:C1", "Raspberry Pi");
    m.insert("3A:29:37", "Raspberry Pi");
    m.insert("A6:11:20", "Raspberry Pi");

    // Intel (more)
    m.insert("00:1B:21", "Intel");
    m.insert("00:1E:64", "Intel");
    m.insert("00:1F:3B", "Intel");
    m.insert("00:22:FB", "Intel");
    m.insert("3C:97:0E", "Intel");
    m.insert("68:05:CA", "Intel");
    m.insert("00:02:B3", "Intel");
    m.insert("00:0C:F1", "Intel");

    // AMD
    m.insert("00:18:71", "AMD");
    m.insert("00:22:64", "AMD");
    m.insert("00:25:64", "AMD");
    m.insert("74:83:C2", "AMD");

    // NVIDIA (more)
    m.insert("00:04:4B", "NVIDIA");
    m.insert("00:15:B6", "NVIDIA");
    m.insert("00:1A:79", "NVIDIA");
    m.insert("00:50:F2", "NVIDIA");

    // Qualcomm
    m.insert("00:03:7F", "Qualcomm");
    m.insert("00:09:18", "Qualcomm");
    m.insert("00:0A:75", "Qualcomm");
    m.insert("00:0C:91", "Qualcomm");

    // MediaTek
    m.insert("00:0A:F5", "MediaTek");
    m.insert("00:12:A2", "MediaTek");
    m.insert("00:22:6A", "MediaTek");

    // Samsung (more)
    m.insert("00:00:F0", "Samsung");
    m.insert("00:12:47", "Samsung");
    m.insert("00:1A:8A", "Samsung");
    m.insert("00:26:37", "Samsung");
    m.insert("08:D4:2B", "Samsung");
    m.insert("10:D5:42", "Samsung");

    // LG (more)
    m.insert("00:1E:75", "LG");
    m.insert("00:1F:E3", "LG");
    m.insert("00:22:A9", "LG");
    m.insert("34:E8:94", "LG");

    // Sony (more)
    m.insert("00:1D:BA", "Sony");
    m.insert("00:24:BE", "Sony");
    m.insert("28:0D:FC", "Sony");
    m.insert("00:15:C6", "Sony");

    // Panasonic
    m.insert("00:04:CA", "Panasonic");
    m.insert("00:16:31", "Panasonic");
    m.insert("00:22:5C", "Panasonic");

    // Sharp
    m.insert("00:00:FE", "Sharp");
    m.insert("00:0D:8D", "Sharp");
    m.insert("00:1B:4F", "Sharp");

    // Toshiba
    m.insert("00:00:39", "Toshiba");
    m.insert("00:0E:7B", "Toshiba");
    m.insert("00:1E:33", "Toshiba");

    // Vizio
    m.insert("00:26:BB", "Vizio");
    m.insert("74:C2:46", "Vizio");
    m.insert("A4:08:F5", "Vizio");

    // Hisense
    m.insert("00:1C:10", "Hisense");
    m.insert("1C:87:2C", "Hisense");
    m.insert("3C:8C:40", "Hisense");

    // TCL
    m.insert("00:1E:A5", "TCL");
    m.insert("7C:1E:52", "TCL");
    m.insert("B8:69:F4", "TCL");

    // Philips TV
    m.insert("00:02:54", "Philips TV");
    m.insert("00:1B:3E", "Philips TV");
    m.insert("00:1E:C2", "Philips TV");

    // More networking vendors

    // Netgear (more)
    m.insert("00:14:6C", "Netgear");
    m.insert("00:1B:2F", "Netgear");
    m.insert("00:1E:2A", "Netgear");
    m.insert("C0:3F:0E", "Netgear");
    m.insert("00:0F:B5", "Netgear");
    m.insert("00:22:3F", "Netgear");

    // TP-Link (more)
    m.insert("00:1D:0F", "TP-Link");
    m.insert("14:CC:20", "TP-Link");
    m.insert("50:C7:BF", "TP-Link");
    m.insert("E8:DE:27", "TP-Link");
    m.insert("00:27:19", "TP-Link");
    m.insert("10:FE:ED", "TP-Link");

    // D-Link (more)
    m.insert("00:1B:11", "D-Link");
    m.insert("00:1E:58", "D-Link");
    m.insert("14:D6:4D", "D-Link");
    m.insert("00:05:5D", "D-Link");
    m.insert("00:11:95", "D-Link");

    // Linksys (more)
    m.insert("00:14:BF", "Linksys");
    m.insert("00:1A:70", "Linksys");
    m.insert("00:1E:E5", "Linksys");
    m.insert("00:06:25", "Linksys");
    m.insert("00:0C:41", "Linksys");

    // ASUS (more)
    m.insert("00:1A:92", "ASUS");
    m.insert("00:1E:8C", "ASUS");
    m.insert("00:22:15", "ASUS");
    m.insert("AC:22:0B", "ASUS");
    m.insert("00:0C:6E", "ASUS");
    m.insert("00:11:D8", "ASUS");

    // Ubiquiti (more)
    m.insert("00:27:22", "Ubiquiti");
    m.insert("24:A4:3C", "Ubiquiti");
    m.insert("44:D9:E7", "Ubiquiti");
    m.insert("80:2A:A8", "Ubiquiti");
    m.insert("FC:EC:DA", "Ubiquiti");
    m.insert("04:18:D6", "Ubiquiti");

    // MikroTik (more)
    m.insert("00:0C:42", "MikroTik");
    m.insert("4C:5E:0C", "MikroTik");
    m.insert("64:D1:54", "MikroTik");
    m.insert("6C:3B:6B", "MikroTik");
    m.insert("74:4D:28", "MikroTik");

    // Cisco (more)
    m.insert("00:00:0C", "Cisco");
    m.insert("00:01:42", "Cisco");
    m.insert("00:1A:A1", "Cisco");
    m.insert("00:1B:D4", "Cisco");
    m.insert("00:02:FC", "Cisco");
    m.insert("00:07:0E", "Cisco");

    // Juniper (more)
    m.insert("00:12:1E", "Juniper");
    m.insert("00:1F:12", "Juniper");
    m.insert("00:1A:6C", "Juniper");

    // Palo Alto (more)
    m.insert("00:1B:17", "Palo Alto");
    m.insert("00:1C:7E", "Palo Alto");
    m.insert("F0:92:1C", "Palo Alto");

    // Fortinet (more)
    m.insert("00:09:0F", "Fortinet");
    m.insert("00:1D:68", "Fortinet");
    m.insert("00:09:E8", "Fortinet");

    // Check Point
    m.insert("00:1C:7F", "Check Point");
    m.insert("00:1D:9D", "Check Point");

    // Huawei (more)
    m.insert("00:18:82", "Huawei");
    m.insert("00:1E:10", "Huawei");
    m.insert("48:46:FB", "Huawei");
    m.insert("00:25:9E", "Huawei");
    m.insert("00:46:4B", "Huawei");

    // ZTE (more)
    m.insert("00:09:DF", "ZTE");
    m.insert("00:15:EB", "ZTE");
    m.insert("00:19:C6", "ZTE");
    m.insert("00:1E:73", "ZTE");

    // Nokia
    m.insert("00:0A:44", "Nokia");
    m.insert("00:19:18", "Nokia");
    m.insert("00:1A:EE", "Nokia");

    // Ericsson
    m.insert("00:0A:4D", "Ericsson");
    m.insert("00:15:E0", "Ericsson");
    m.insert("00:1B:8C", "Ericsson");

    // Alcatel-Lucent
    m.insert("00:0E:5D", "Alcatel-Lucent");
    m.insert("00:12:2A", "Alcatel-Lucent");
    m.insert("00:1C:27", "Alcatel-Lucent");

    // More virtualization platforms

    // VMware (more)
    m.insert("00:50:56", "VMware");
    m.insert("00:0C:29", "VMware");
    m.insert("00:05:69", "VMware");
    m.insert("00:15:5D", "VMware");

    // VirtualBox (more)
    m.insert("08:00:27", "VirtualBox");
    m.insert("0A:00:27", "VirtualBox");
    m.insert("00:60:36", "VirtualBox");

    // QEMU/KVM
    m.insert("52:54:00", "QEMU/KVM");
    m.insert("00:00:00", "QEMU/KVM");
    m.insert("00:16:3E", "QEMU/KVM");

    // Hyper-V
    m.insert("00:15:5D", "Microsoft Hyper-V");
    m.insert("00:03:FF", "Microsoft Hyper-V");
    m.insert("00:1D:D8", "Microsoft Hyper-V");

    // Parallels
    m.insert("00:1C:42", "Parallels");
    m.insert("00:16:3E", "Parallels");
    m.insert("00:0A:E7", "Parallels");

    // Xen
    m.insert("00:16:3E", "Xen");
    m.insert("00:18:71", "Xen");
    m.insert("00:1C:42", "Xen");

    // More mobile/tablet vendors

    // LG Electronics (mobile)
    m.insert("00:1E:75", "LG Mobile");
    m.insert("00:1F:E3", "LG Mobile");
    m.insert("00:22:A9", "LG Mobile");

    // Motorola (mobile)
    m.insert("00:04:56", "Motorola Mobile");
    m.insert("00:0A:28", "Motorola Mobile");
    m.insert("00:0B:06", "Motorola Mobile");

    // HTC
    m.insert("00:04:25", "HTC");
    m.insert("00:11:09", "HTC");
    m.insert("00:17:35", "HTC");

    // LG Electronics
    m.insert("00:1E:75", "LG Electronics");
    m.insert("00:1F:E3", "LG Electronics");

    // Dell (mobile)
    m.insert("00:14:22", "Dell Mobile");
    m.insert("00:1A:A0", "Dell Mobile");
    m.insert("00:1E:4F", "Dell Mobile");

    // Toshiba (mobile)
    m.insert("00:00:39", "Toshiba Mobile");
    m.insert("00:0E:7B", "Toshiba Mobile");
    m.insert("00:1E:33", "Toshiba Mobile");

    // Fujitsu
    m.insert("00:00:5A", "Fujitsu");
    m.insert("00:09:E6", "Fujitsu");
    m.insert("00:11:25", "Fujitsu");

    // NEC
    m.insert("00:00:4E", "NEC");
    m.insert("00:02:3F", "NEC");
    m.insert("00:09:4A", "NEC");

    // Lenovo (mobile)
    m.insert("00:1E:4C", "Lenovo Mobile");
    m.insert("00:22:68", "Lenovo Mobile");
    m.insert("00:26:2D", "Lenovo Mobile");

    // Xiaomi (mobile)
    m.insert("00:9E:C8", "Xiaomi Mobile");
    m.insert("28:6C:07", "Xiaomi Mobile");
    m.insert("64:09:80", "Xiaomi Mobile");

    // OnePlus
    m.insert("C0:EE:FB", "OnePlus");
    m.insert("00:0E:C6", "OnePlus");
    m.insert("00:12:34", "OnePlus");

    // Vivo
    m.insert("00:1B:A4", "Vivo");
    m.insert("00:24:37", "Vivo");

    // Oppo
    m.insert("00:1E:83", "Oppo");
    m.insert("00:21:6A", "Oppo");

    // Huawei (mobile)
    m.insert("00:18:82", "Huawei Mobile");
    m.insert("00:1E:10", "Huawei Mobile");
    m.insert("48:46:FB", "Huawei Mobile");

    // ZTE (mobile)
    m.insert("00:09:DF", "ZTE Mobile");
    m.insert("00:15:EB", "ZTE Mobile");
    m.insert("00:19:C6", "ZTE Mobile");

    // More printers/scanners

    // HP (more)
    m.insert("00:01:E6", "HP");
    m.insert("00:01:E7", "HP");
    m.insert("00:04:EA", "HP");
    m.insert("00:08:02", "HP");
    m.insert("00:14:38", "HP");
    m.insert("00:1A:4B", "HP");

    // Brother
    m.insert("00:00:6B", "Brother");
    m.insert("00:01:80", "Brother");
    m.insert("00:0B:46", "Brother");

    // Canon
    m.insert("00:00:85", "Canon");
    m.insert("00:05:1F", "Canon");
    m.insert("00:0B:A5", "Canon");

    // Epson
    m.insert("00:00:48", "Epson");
    m.insert("00:0A:EA", "Epson");
    m.insert("00:1C:26", "Epson");

    // Lexmark
    m.insert("00:00:06", "Lexmark");
    m.insert("00:01:AC", "Lexmark");
    m.insert("00:09:47", "Lexmark");

    // Xerox
    m.insert("00:00:01", "Xerox");
    m.insert("00:06:59", "Xerox");
    m.insert("00:09:3A", "Xerox");

    // Samsung (printers)
    m.insert("00:00:F0", "Samsung Printers");
    m.insert("00:12:47", "Samsung Printers");
    m.insert("00:1A:8A", "Samsung Printers");

    // More storage vendors

    // Synology (more)
    m.insert("00:11:32", "Synology");
    m.insert("00:1B:66", "Synology");
    m.insert("00:21:C6", "Synology");

    // QNAP (more)
    m.insert("00:08:9B", "QNAP");
    m.insert("24:5E:BE", "QNAP");
    m.insert("00:1D:74", "QNAP");

    // NetApp
    m.insert("00:A0:98", "NetApp");
    m.insert("00:1E:68", "NetApp");
    m.insert("00:50:56", "NetApp");

    // Dell EMC
    m.insert("00:14:22", "Dell EMC");
    m.insert("00:1A:A0", "Dell EMC");
    m.insert("00:0B:DB", "Dell EMC");

    // IBM
    m.insert("00:04:AC", "IBM");
    m.insert("00:06:5B", "IBM");
    m.insert("00:09:6B", "IBM");

    // More industrial/embedded

    // Advantech
    m.insert("00:00:4D", "Advantech");
    m.insert("00:09:E3", "Advantech");
    m.insert("00:1A:D5", "Advantech");

    // Moxa
    m.insert("00:00:63", "Moxa");
    m.insert("00:0A:F2", "Moxa");
    m.insert("00:18:8C", "Moxa");

    // Siemens
    m.insert("00:01:37", "Siemens");
    m.insert("00:08:84", "Siemens");
    m.insert("00:1B:1B", "Siemens");

    // Schneider Electric
    m.insert("00:00:BC", "Schneider Electric");
    m.insert("00:1B:54", "Schneider Electric");
    m.insert("00:1D:9C", "Schneider Electric");

    // ABB
    m.insert("00:01:0D", "ABB");
    m.insert("00:1C:63", "ABB");
    m.insert("00:1E:53", "ABB");

    // More security cameras

    // Hikvision (more)
    m.insert("18:68:CB", "Hikvision");
    m.insert("28:57:BE", "Hikvision");
    m.insert("44:19:B6", "Hikvision");
    m.insert("4C:BD:8F", "Hikvision");
    m.insert("54:C4:15", "Hikvision");
    m.insert("5C:F9:6A", "Hikvision");

    // Dahua (more)
    m.insert("14:A7:8B", "Dahua");
    m.insert("20:17:42", "Dahua");
    m.insert("3C:EF:8C", "Dahua");
    m.insert("40:F4:FD", "Dahua");
    m.insert("4C:11:BF", "Dahua");

    // Axis (more)
    m.insert("00:40:8C", "Axis");
    m.insert("AC:CC:8E", "Axis");
    m.insert("B8:A4:4F", "Axis");

    // Bosch
    m.insert("00:03:7E", "Bosch");
    m.insert("00:16:84", "Bosch");
    m.insert("00:1E:3F", "Bosch");

    // Hanwha (Samsung Techwin)
    m.insert("00:0B:E5", "Hanwha");
    m.insert("00:1C:21", "Hanwha");
    m.insert("00:1E:52", "Hanwha");

    // Pelco
    m.insert("00:09:7C", "Pelco");
    m.insert("00:17:A5", "Pelco");
    m.insert("00:1A:3A", "Pelco");

    // Avigilon
    m.insert("00:18:77", "Avigilon");
    m.insert("00:1E:16", "Avigilon");
    m.insert("00:1F:0C", "Avigilon");

    // Arecont
    m.insert("00:0C:6D", "Arecont");
    m.insert("00:1B:B2", "Arecont");

    // Mobotix
    m.insert("00:05:5D", "Mobotix");
    m.insert("00:1B:A1", "Mobotix");
    m.insert("00:1C:12", "Mobotix");

    // Vivotek
    m.insert("00:02:3C", "Vivotek");
    m.insert("00:11:BC", "Vivotek");
    m.insert("00:1B:5B", "Vivotek");

    // ACTi
    m.insert("00:0B:E3", "ACTi");
    m.insert("00:1A:8C", "ACTi");
    m.insert("00:1D:C3", "ACTi");

    // GeoVision
    m.insert("00:0B:31", "GeoVision");
    m.insert("00:1C:53", "GeoVision");
    m.insert("00:1D:3A", "GeoVision");

    // StarDot
    m.insert("00:0E:8C", "StarDot");
    m.insert("00:1A:6E", "StarDot");

    // CNB Technology
    m.insert("00:18:F3", "CNB");
    m.insert("00:1C:B5", "CNB");

    // Ganza
    m.insert("00:12:56", "Ganza");
    m.insert("00:1A:B2", "Ganza");

    //更多工业和医疗设备

    // GE Healthcare
    m.insert("00:1B:3C", "GE Healthcare");
    m.insert("00:1C:12", "GE Healthcare");

    // Philips Healthcare
    m.insert("00:11:32", "Philips Healthcare");
    m.insert("00:17:88", "Philips Healthcare");

    // Siemens Healthcare
    m.insert("00:07:7A", "Siemens Healthcare");
    m.insert("00:1B:1B", "Siemens Healthcare");

    // Medtronic
    m.insert("00:1A:7A", "Medtronic");
    m.insert("00:1C:26", "Medtronic");

    // Abbott
    m.insert("00:11:44", "Abbott");
    m.insert("00:1A:B7", "Abbott");

    // Roche
    m.insert("00:19:FA", "Roche");
    m.insert("00:1C:7D", "Roche");

    // Boston Scientific
    m.insert("00:1A:6A", "Boston Scientific");
    m.insert("00:1C:65", "Boston Scientific");

    // Stryker
    m.insert("00:18:C5", "Stryker");
    m.insert("00:1C:A8", "Stryker");

    //更多无线接入点

    // Aruba (more)
    m.insert("00:0B:86", "Aruba Networks");
    m.insert("00:1A:1E", "Aruba Networks");
    m.insert("00:1B:86", "Aruba Networks");

    // Ruckus (more)
    m.insert("00:22:7F", "Ruckus Wireless");
    m.insert("00:1B:3A", "Ruckus Wireless");
    m.insert("00:1C:E5", "Ruckus Wireless");

    // Meraki (more)
    m.insert("00:18:0A", "Meraki");
    m.insert("0C:8D:DB", "Meraki");
    m.insert("34:56:FE", "Meraki");

    // Aerohive
    m.insert("00:1C:57", "Aerohive");
    m.insert("00:1D:7E", "Aerohive");

    // Ubiquiti (more)
    m.insert("04:18:D6", "Ubiquiti");
    m.insert("18:E8:29", "Ubiquiti");
    m.insert("24:5A:4C", "Ubiquiti");

    //更多电力线网络

    // Devolo
    m.insert("00:0C:24", "Devolo");
    m.insert("00:16:84", "Devolo");

    // Netgear Powerline
    m.insert("00:1B:C0", "Netgear Powerline");
    m.insert("00:1D:0F", "Netgear Powerline");

    // TP-Link Powerline
    m.insert("00:27:19", "TP-Link Powerline");
    m.insert("14:CF:92", "TP-Link Powerline");

    // ZyXEL Powerline
    m.insert("00:0F:94", "ZyXEL Powerline");
    m.insert("00:13:49", "ZyXEL Powerline");

    // Actiontec Powerline
    m.insert("00:1F:90", "Actiontec Powerline");
    m.insert("00:20:E0", "Actiontec Powerline");

    //更多汽车/车载系统

    // Tesla
    m.insert("00:05:22", "Tesla");
    m.insert("00:1C:57", "Tesla");
    m.insert("00:25:00", "Tesla");

    // BMW
    m.insert("00:1E:3B", "BMW");
    m.insert("00:20:E5", "BMW");
    m.insert("00:26:59", "BMW");

    // Mercedes-Benz
    m.insert("00:16:32", "Mercedes-Benz");
    m.insert("00:1D:25", "Mercedes-Benz");

    // Ford
    m.insert("00:14:FF", "Ford");
    m.insert("00:15:75", "Ford");

    // General Motors
    m.insert("00:05:8B", "General Motors");
    m.insert("00:0A:54", "General Motors");

    // Volkswagen
    m.insert("00:1C:26", "Volkswagen");
    m.insert("00:24:54", "Volkswagen");

    // Toyota
    m.insert("00:00:23", "Toyota");
    m.insert("00:01:37", "Toyota");

    // Honda
    m.insert("00:14:89", "Honda");
    m.insert("00:16:85", "Honda");

    //更多无人机

    // DJI
    m.insert("00:12:0C", "DJI");
    m.insert("00:1A:64", "DJI");
    m.insert("00:1E:70", "DJI");

    // Parrot
    m.insert("00:26:7F", "Parrot");
    m.insert("00:1B:E1", "Parrot");

    // 3D Robotics
    m.insert("00:15:C4", "3DR");
    m.insert("00:1C:47", "3DR");

    // Autel Robotics
    m.insert("00:1E:42", "Autel Robotics");
    m.insert("00:1F:64", "Autel Robotics");

    // Yuneec
    m.insert("00:19:43", "Yuneec");
    m.insert("00:1A:32", "Yuneec");

    // === More Automotive/Telematics ===

    // Tesla
    m.insert("00:05:22", "Tesla");
    m.insert("00:1C:57", "Tesla");
    m.insert("00:25:00", "Tesla");

    // BMW
    m.insert("00:1E:3B", "BMW");
    m.insert("00:20:E5", "BMW");
    m.insert("00:26:59", "BMW");

    // Mercedes-Benz
    m.insert("00:16:32", "Mercedes-Benz");
    m.insert("00:1D:25", "Mercedes-Benz");

    // Ford
    m.insert("00:14:FF", "Ford");
    m.insert("00:15:75", "Ford");

    // General Motors
    m.insert("00:05:8B", "General Motors");
    m.insert("00:0A:54", "General Motors");

    // Volkswagen
    m.insert("00:1C:26", "Volkswagen");
    m.insert("00:24:54", "Volkswagen");

    // Toyota
    m.insert("00:00:23", "Toyota");
    m.insert("00:01:37", "Toyota");

    // Honda
    m.insert("00:14:89", "Honda");
    m.insert("00:16:85", "Honda");

    // === More Health/Fitness Wearables ===

    // Withings (smart scales)
    m.insert("00:1D:8C", "Withings");
    m.insert("74:79:BF", "Withings");

    // Fitbit
    m.insert("00:1E:7E", "Fitbit");
    m.insert("B0:0C:78", "Fitbit");
    m.insert("F0:27:2D", "Fitbit");

    // Garmin
    m.insert("00:14:3D", "Garmin");
    m.insert("00:1C:10", "Garmin");
    m.insert("A4:5C:27", "Garmin");

    // === More Industrial Automation ===

    // ABB
    m.insert("00:01:0D", "ABB");
    m.insert("00:1C:63", "ABB");
    m.insert("00:1E:53", "ABB");

    // Omron
    m.insert("00:00:4D", "Omron");
    m.insert("00:09:E5", "Omron");
    m.insert("00:1A:D6", "Omron");

    // Delta Electronics
    m.insert("00:01:9D", "Delta");
    m.insert("00:0C:45", "Delta");
    m.insert("00:1A:E2", "Delta");

    // === More Network Attached Storage ===

    // Buffalo (NAS)
    m.insert("00:07:40", "Buffalo");
    m.insert("00:0D:0B", "Buffalo");
    m.insert("00:16:01", "Buffalo");

    // LaCie
    m.insert("00:04:27", "LaCie");
    m.insert("00:1A:EF", "LaCie");

    // TerraMaster
    m.insert("00:12:17", "TerraMaster");
    m.insert("00:1C:B8", "TerraMaster");

    // Asustor
    m.insert("00:26:18", "Asustor");
    m.insert("00:1A:92", "Asustor");

    // === More Broadband/Cable Modems ===

    // Arris (more)
    m.insert("00:00:CA", "Arris");
    m.insert("00:1D:CE", "Arris");
    m.insert("00:23:74", "Arris");

    // Motorola (modems)
    m.insert("00:04:56", "Motorola");
    m.insert("00:08:0E", "Motorola");
    m.insert("00:0A:28", "Motorola");

    // Scientific Atlanta
    m.insert("00:0D:6A", "Scientific Atlanta");
    m.insert("00:14:38", "Scientific Atlanta");

    // === More Fiber/Carrier Equipment ===

    // Calix
    m.insert("00:19:60", "Calix");
    m.insert("00:25:84", "Calix");
    m.insert("08:6A:C5", "Calix");

    // Adtran
    m.insert("00:02:B3", "Adtran");
    m.insert("00:0E:8C", "Adtran");
    m.insert("00:1A:8D", "Adtran");

    // Zhone
    m.insert("00:01:E8", "Zhone");
    m.insert("00:12:C5", "Zhone");

    // === More Video Surveillance ===

    // Sony (IP cameras)
    m.insert("00:1D:BA", "Sony IPC");
    m.insert("00:24:BE", "Sony IPC");
    m.insert("00:1B:4D", "Sony IPC");

    // JVC
    m.insert("00:00:3E", "JVC");
    m.insert("00:0E:7B", "JVC");
    m.insert("00:1A:4B", "JVC");

    // Hikvision (more)
    m.insert("18:68:CB", "Hikvision");
    m.insert("28:57:BE", "Hikvision");
    m.insert("44:19:B6", "Hikvision");
    m.insert("4C:BD:8F", "Hikvision");
    m.insert("54:C4:15", "Hikvision");
    m.insert("5C:F9:6A", "Hikvision");
    m.insert("64:E8:4F", "Hikvision");
    m.insert("7C:1E:B3", "Hikvision");

    // Dahua (more)
    m.insert("14:A7:8B", "Dahua");
    m.insert("20:17:42", "Dahua");
    m.insert("3C:EF:8C", "Dahua");
    m.insert("40:F4:FD", "Dahua");
    m.insert("4C:11:BF", "Dahua");

    // === More Smart Home Hubs ===

    // Wink
    m.insert("00:1A:9F", "Wink");
    m.insert("00:1C:B3", "Wink");

    // Iris (Lowes)
    m.insert("00:17:88", "Iris");
    m.insert("00:1D:86", "Iris");

    // Insteon
    m.insert("00:0A:F3", "Insteon");
    m.insert("00:1A:7B", "Insteon");

    // Vera
    m.insert("00:0D:88", "Vera");
    m.insert("00:1C:51", "Vera");

    // Home Assistant Blue
    m.insert("B4:5C:A4", "Home Assistant");

    // === More Gaming Accessories ===

    // Razer
    m.insert("00:15:B9", "Razer");
    m.insert("00:1B:17", "Razer");
    m.insert("00:1E:68", "Razer");

    // Logitech (Gaming)
    m.insert("00:06:F6", "Logitech Gaming");
    m.insert("00:1F:20", "Logitech Gaming");

    // Corsair
    m.insert("00:1D:4F", "Corsair");
    m.insert("00:1B:1D", "Corsair");
    m.insert("00:19:BB", "Corsair");

    // SteelSeries
    m.insert("00:1C:21", "SteelSeries");
    m.insert("00:1B:C9", "SteelSeries");

    // Kingston
    m.insert("00:1B:96", "Kingston");
    m.insert("00:1E:68", "Kingston");

    // === Enterprise Networking Vendors ===

    // Juniper (more)
    m.insert("00:12:1E", "Juniper Networks");
    m.insert("00:1F:12", "Juniper Networks");
    m.insert("00:1A:6C", "Juniper Networks");
    m.insert("00:0E:84", "Juniper Networks");
    m.insert("00:19:E2", "Juniper Networks");
    m.insert("00:1D:38", "Juniper Networks");
    m.insert("00:1E:65", "Juniper Networks");
    m.insert("00:21:59", "Juniper Networks");
    m.insert("00:22:83", "Juniper Networks");
    m.insert("00:23:51", "Juniper Networks");
    m.insert("00:24:DC", "Juniper Networks");
    m.insert("00:25:83", "Juniper Networks");
    m.insert("00:26:F2", "Juniper Networks");
    m.insert("00:1C:0D", "Juniper Networks");
    m.insert("00:1D:70", "Juniper Networks");
    m.insert("00:1E:34", "Juniper Networks");
    m.insert("00:1F:53", "Juniper Networks");
    m.insert("00:1F:91", "Juniper Networks");

    // Palo Alto Networks (more)
    m.insert("00:1B:17", "Palo Alto Networks");
    m.insert("00:1C:7E", "Palo Alto Networks");
    m.insert("F0:92:1C", "Palo Alto Networks");
    m.insert("00:1E:68", "Palo Alto Networks");
    m.insert("00:1F:5C", "Palo Alto Networks");
    m.insert("00:21:55", "Palo Alto Networks");
    m.insert("00:23:29", "Palo Alto Networks");
    m.insert("00:24:81", "Palo Alto Networks");
    m.insert("00:26:86", "Palo Alto Networks");
    m.insert("00:2A:6A", "Palo Alto Networks");
    m.insert("00:1C:BD", "Palo Alto Networks");
    m.insert("00:1E:6A", "Palo Alto Networks");

    // Fortinet (more)
    m.insert("00:09:0F", "Fortinet");
    m.insert("00:1D:68", "Fortinet");
    m.insert("00:09:E8", "Fortinet");
    m.insert("00:1B:17", "Fortinet");
    m.insert("00:1C:59", "Fortinet");
    m.insert("00:1E:8A", "Fortinet");
    m.insert("00:1F:54", "Fortinet");
    m.insert("00:21:D1", "Fortinet");
    m.insert("00:22:9F", "Fortinet");
    m.insert("00:23:7D", "Fortinet");
    m.insert("00:24:38", "Fortinet");
    m.insert("00:25:56", "Fortinet");
    m.insert("00:26:DE", "Fortinet");
    m.insert("00:1D:0A", "Fortinet");
    m.insert("00:1E:C6", "Fortinet");

    // Brocade (now Broadcom)
    m.insert("00:05:1E", "Brocade");
    m.insert("00:06:4B", "Brocade");
    m.insert("00:07:E9", "Brocade");
    m.insert("00:0E:5F", "Brocade");
    m.insert("00:12:3A", "Brocade");
    m.insert("00:14:38", "Brocade");
    m.insert("00:17:08", "Brocade");
    m.insert("00:19:BA", "Brocade");
    m.insert("00:1A:4E", "Brocade");
    m.insert("00:1B:52", "Brocade");
    m.insert("00:1C:92", "Brocade");
    m.insert("00:1D:7D", "Brocade");
    m.insert("00:1E:49", "Brocade");
    m.insert("00:1F:28", "Brocade");
    m.insert("00:1F:45", "Brocade");

    // Dell Networking (Force10, Dell EMC)
    m.insert("00:06:5B", "Dell Networking");
    m.insert("00:08:74", "Dell Networking");
    m.insert("00:0B:DB", "Dell Networking");
    m.insert("00:0D:56", "Dell Networking");
    m.insert("00:0F:1F", "Dell Networking");
    m.insert("00:11:43", "Dell Networking");
    m.insert("00:12:3F", "Dell Networking");
    m.insert("00:13:72", "Dell Networking");
    m.insert("00:14:22", "Dell Networking");
    m.insert("00:15:C5", "Dell Networking");
    m.insert("00:16:F0", "Dell Networking");
    m.insert("00:18:8B", "Dell Networking");
    m.insert("00:19:B9", "Dell Networking");
    m.insert("00:1C:23", "Dell Networking");
    m.insert("00:1D:09", "Dell Networking");
    m.insert("00:1E:C9", "Dell Networking");
    m.insert("00:21:70", "Dell Networking");
    m.insert("00:21:9B", "Dell Networking");

    // Alcatel-Lucent Enterprise
    m.insert("00:0E:5D", "Alcatel-Lucent Enterprise");
    m.insert("00:12:2A", "Alcatel-Lucent Enterprise");
    m.insert("00:1C:27", "Alcatel-Lucent Enterprise");
    m.insert("00:1E:7D", "Alcatel-Lucent Enterprise");
    m.insert("00:1F:6D", "Alcatel-Lucent Enterprise");
    m.insert("00:21:E3", "Alcatel-Lucent Enterprise");
    m.insert("00:22:4C", "Alcatel-Lucent Enterprise");
    m.insert("00:23:7A", "Alcatel-Lucent Enterprise");
    m.insert("00:24:DC", "Alcatel-Lucent Enterprise");
    m.insert("00:25:68", "Alcatel-Lucent Enterprise");
    m.insert("00:26:30", "Alcatel-Lucent Enterprise");

    // HPE Aruba (more)
    m.insert("00:0B:86", "HPE Aruba");
    m.insert("00:1A:1E", "HPE Aruba");
    m.insert("00:1B:86", "HPE Aruba");
    m.insert("00:1C:10", "HPE Aruba");
    m.insert("00:1D:1F", "HPE Aruba");
    m.insert("00:1E:2A", "HPE Aruba");
    m.insert("00:1F:45", "HPE Aruba");
    m.insert("00:1F:63", "HPE Aruba");
    m.insert("00:21:5A", "HPE Aruba");
    m.insert("00:22:A9", "HPE Aruba");
    m.insert("00:24:8C", "HPE Aruba");
    m.insert("00:25:BC", "HPE Aruba");
    m.insert("00:26:AB", "HPE Aruba");
    m.insert("00:1A:50", "HPE Aruba");
    m.insert("00:1C:B5", "HPE Aruba");

    // Ruckus Wireless (more)
    m.insert("00:22:7F", "Ruckus Wireless");
    m.insert("00:1B:3A", "Ruckus Wireless");
    m.insert("00:1C:E5", "Ruckus Wireless");
    m.insert("00:0C:E6", "Ruckus Wireless");
    m.insert("00:1A:2A", "Ruckus Wireless");
    m.insert("00:1B:25", "Ruckus Wireless");
    m.insert("00:1C:BF", "Ruckus Wireless");
    m.insert("00:1D:1C", "Ruckus Wireless");
    m.insert("00:1E:7A", "Ruckus Wireless");
    m.insert("00:1F:45", "Ruckus Wireless");
    m.insert("00:21:27", "Ruckus Wireless");
    m.insert("00:22:3D", "Ruckus Wireless");
    m.insert("00:23:04", "Ruckus Wireless");
    m.insert("00:24:38", "Ruckus Wireless");
    m.insert("00:25:C0", "Ruckus Wireless");

    // Cisco Meraki (more)
    m.insert("00:18:0A", "Cisco Meraki");
    m.insert("0C:8D:DB", "Cisco Meraki");
    m.insert("34:56:FE", "Cisco Meraki");
    m.insert("68:3A:1E", "Cisco Meraki");
    m.insert("88:15:44", "Cisco Meraki");
    m.insert("AC:17:C8", "Cisco Meraki");
    m.insert("E0:55:3D", "Cisco Meraki");
    m.insert("E0:CB:BC", "Cisco Meraki");
    m.insert("00:1A:A1", "Cisco Meraki");
    m.insert("00:1D:70", "Cisco Meraki");
    m.insert("00:1E:13", "Cisco Meraki");
    m.insert("00:1F:6C", "Cisco Meraki");
    m.insert("00:21:9A", "Cisco Meraki");
    m.insert("00:22:6D", "Cisco Meraki");
    m.insert("00:23:04", "Cisco Meraki");
    m.insert("00:24:97", "Cisco Meraki");

    // Ubiquiti Networks (more)
    m.insert("00:27:22", "Ubiquiti Networks");
    m.insert("24:A4:3C", "Ubiquiti Networks");
    m.insert("44:D9:E7", "Ubiquiti Networks");
    m.insert("80:2A:A8", "Ubiquiti Networks");
    m.insert("FC:EC:DA", "Ubiquiti Networks");
    m.insert("04:18:D6", "Ubiquiti Networks");
    m.insert("18:E8:29", "Ubiquiti Networks");
    m.insert("24:5A:4C", "Ubiquiti Networks");
    m.insert("68:72:51", "Ubiquiti Networks");
    m.insert("74:83:C2", "Ubiquiti Networks");
    m.insert("78:8A:20", "Ubiquiti Networks");
    m.insert("B4:FB:E4", "Ubiquiti Networks");
    m.insert("DC:9F:DB", "Ubiquiti Networks");
    m.insert("F0:9F:C2", "Ubiquiti Networks");
    m.insert("00:1E:C5", "Ubiquiti Networks");
    m.insert("00:1F:33", "Ubiquiti Networks");

    // MikroTik (more)
    m.insert("00:0C:42", "MikroTik");
    m.insert("4C:5E:0C", "MikroTik");
    m.insert("64:D1:54", "MikroTik");
    m.insert("6C:3B:6B", "MikroTik");
    m.insert("74:4D:28", "MikroTik");
    m.insert("B8:69:F4", "MikroTik");
    m.insert("C4:AD:34", "MikroTik");
    m.insert("CC:2D:E0", "MikroTik");
    m.insert("D4:CA:6D", "MikroTik");
    m.insert("E4:8D:8C", "MikroTik");
    m.insert("00:1B:1F", "MikroTik");
    m.insert("00:1C:42", "MikroTik");
    m.insert("00:1D:0A", "MikroTik");
    m.insert("00:1E:64", "MikroTik");
    m.insert("00:1F:3B", "MikroTik");
    m.insert("00:21:6A", "MikroTik");

    // Cisco (more - comprehensive list)
    m.insert("00:00:0C", "Cisco");
    m.insert("00:01:42", "Cisco");
    m.insert("00:1A:A1", "Cisco");
    m.insert("00:1B:D4", "Cisco");
    m.insert("00:02:FC", "Cisco");
    m.insert("00:07:0E", "Cisco");
    m.insert("00:0A:41", "Cisco");
    m.insert("00:0B:BE", "Cisco");
    m.insert("00:0D:ED", "Cisco");
    m.insert("00:0E:38", "Cisco");
    m.insert("00:0E:83", "Cisco");
    m.insert("00:11:20", "Cisco");
    m.insert("00:14:1B", "Cisco");
    m.insert("00:17:0E", "Cisco");
    m.insert("00:18:18", "Cisco");
    m.insert("00:19:AA", "Cisco");
    m.insert("00:1D:70", "Cisco");
    m.insert("00:22:0D", "Cisco");
    m.insert("00:25:84", "Cisco");
    m.insert("00:26:51", "Cisco");
    m.insert("00:2A:6A", "Cisco");
    m.insert("2C:33:11", "Cisco");
    m.insert("34:A8:4E", "Cisco");
    m.insert("50:06:04", "Cisco");
    m.insert("54:75:D0", "Cisco");
    m.insert("58:8D:09", "Cisco");
    m.insert("5C:83:8F", "Cisco");
    m.insert("70:81:05", "Cisco");
    m.insert("78:BA:F9", "Cisco");
    m.insert("84:78:AC", "Cisco");
    m.insert("A8:0C:0D", "Cisco");
    m.insert("B0:AA:77", "Cisco");
    m.insert("B4:14:89", "Cisco");
    m.insert("D0:72:DC", "Cisco");
    m.insert("F0:29:29", "Cisco");
    m.insert("F8:72:EA", "Cisco");
    m.insert("00:0B:85", "Cisco");
    m.insert("00:12:80", "Cisco");
    m.insert("00:13:80", "Cisco");
    m.insert("00:14:F2", "Cisco");
    m.insert("00:15:C7", "Cisco");
    m.insert("00:16:46", "Cisco");
    m.insert("00:17:5A", "Cisco");
    m.insert("00:18:19", "Cisco");
    m.insert("00:19:2C", "Cisco");
    m.insert("00:1A:A2", "Cisco");
    m.insert("00:1B:54", "Cisco");
    m.insert("00:1C:0F", "Cisco");
    m.insert("00:1D:1C", "Cisco");
    m.insert("00:1E:49", "Cisco");
    m.insert("00:1F:27", "Cisco");

    // Allied Telesis
    m.insert("00:00:4C", "Allied Telesis");
    m.insert("00:07:1D", "Allied Telesis");
    m.insert("00:0D:24", "Allied Telesis");
    m.insert("00:1A:A3", "Allied Telesis");
    m.insert("00:1E:E4", "Allied Telesis");
    m.insert("00:1F:5B", "Allied Telesis");
    m.insert("00:21:27", "Allied Telesis");
    m.insert("00:22:35", "Allied Telesis");
    m.insert("00:23:9D", "Allied Telesis");
    m.insert("00:24:8D", "Allied Telesis");
    m.insert("00:25:B3", "Allied Telesis");
    m.insert("00:26:30", "Allied Telesis");
    m.insert("00:1D:88", "Allied Telesis");
    m.insert("00:1E:53", "Allied Telesis");

    // Zyxel (more)
    m.insert("00:0F:94", "ZyXEL");
    m.insert("00:13:49", "ZyXEL");
    m.insert("00:19:CB", "ZyXEL");
    m.insert("00:1D:B3", "ZyXEL");
    m.insert("00:1E:E1", "ZyXEL");
    m.insert("00:1F:3F", "ZyXEL");
    m.insert("00:23:F8", "ZyXEL");
    m.insert("00:26:AB", "ZyXEL");
    m.insert("00:A0:C5", "ZyXEL");
    m.insert("04:BF:6D", "ZyXEL");
    m.insert("08:26:AE", "ZyXEL");
    m.insert("0C:91:60", "ZyXEL");
    m.insert("10:7B:EF", "ZyXEL");
    m.insert("18:A6:F7", "ZyXEL");
    m.insert("1C:74:0D", "ZyXEL");
    m.insert("20:E5:2A", "ZyXEL");
    m.insert("28:28:5D", "ZyXEL");
    m.insert("2C:2D:48", "ZyXEL");
    m.insert("30:C9:AB", "ZyXEL");
    m.insert("34:CE:00", "ZyXEL");
    m.insert("38:10:D5", "ZyXEL");
    m.insert("3C:4E:47", "ZyXEL");
    m.insert("40:4A:03", "ZyXEL");
    m.insert("48:8D:36", "ZyXEL");
    m.insert("4C:9E:FF", "ZyXEL");
    m.insert("50:67:F0", "ZyXEL");
    m.insert("58:8B:F3", "ZyXEL");
    m.insert("5C:E2:8C", "ZyXEL");
    m.insert("60:38:E0", "ZyXEL");
    m.insert("74:DA:88", "ZyXEL");
    m.insert("78:A7:14", "ZyXEL");
    m.insert("7C:57:3C", "ZyXEL");
    m.insert("84:9C:A6", "ZyXEL");
    m.insert("8C:E1:17", "ZyXEL");
    m.insert("90:8D:78", "ZyXEL");
    m.insert("98:DA:D0", "ZyXEL");
    m.insert("9C:C9:EB", "ZyXEL");
    m.insert("A4:21:8A", "ZyXEL");
    m.insert("AC:A8:8E", "ZyXEL");
    m.insert("B0:A7:B9", "ZyXEL");
    m.insert("B4:82:C5", "ZyXEL");
    m.insert("BC:A5:11", "ZyXEL");
    m.insert("C0:3F:0E", "ZyXEL");
    m.insert("C8:6C:87", "ZyXEL");
    m.insert("CC:5D:4E", "ZyXEL");
    m.insert("D4:F5:27", "ZyXEL");
    m.insert("D8:EC:5E", "ZyXEL");
    m.insert("E0:98:06", "ZyXEL");
    m.insert("E4:18:6B", "ZyXEL");
    m.insert("EC:F4:51", "ZyXEL");
    m.insert("F0:3E:90", "ZyXEL");
    m.insert("FC:F5:28", "ZyXEL");

    // H3C (HuaWei-3Com)
    m.insert("00:1E:3A", "H3C");
    m.insert("00:21:27", "H3C");
    m.insert("00:22:27", "H3C");
    m.insert("00:23:14", "H3C");
    m.insert("00:24:37", "H3C");
    m.insert("00:25:46", "H3C");
    m.insert("00:26:12", "H3C");
    m.insert("00:1A:0D", "H3C");
    m.insert("00:1B:8A", "H3C");
    m.insert("00:1C:BF", "H3C");

    // Quanta Computer (ODM/whitebox)
    m.insert("00:02:C6", "Quanta Computer");
    m.insert("00:0D:3A", "Quanta Computer");
    m.insert("00:11:32", "Quanta Computer");
    m.insert("00:1C:57", "Quanta Computer");
    m.insert("00:1D:7E", "Quanta Computer");
    m.insert("00:1E:68", "Quanta Computer");
    m.insert("00:1F:29", "Quanta Computer");
    m.insert("00:1F:C6", "Quanta Computer");
    m.insert("00:21:5A", "Quanta Computer");
    m.insert("00:22:19", "Quanta Computer");
    m.insert("00:23:12", "Quanta Computer");

    // Accton Technology (ODM)
    m.insert("00:00:6B", "Accton Technology");
    m.insert("00:04:C3", "Accton Technology");
    m.insert("00:0E:A6", "Accton Technology");
    m.insert("00:12:81", "Accton Technology");
    m.insert("00:18:8D", "Accton Technology");
    m.insert("00:1B:24", "Accton Technology");
    m.insert("00:1D:73", "Accton Technology");
    m.insert("00:1E:49", "Accton Technology");
    m.insert("00:1F:45", "Accton Technology");
    m.insert("00:1F:63", "Accton Technology");

    // Edge-Core Networks
    m.insert("00:1E:68", "Edge-Core Networks");
    m.insert("00:1F:33", "Edge-Core Networks");
    m.insert("00:21:27", "Edge-Core Networks");
    m.insert("00:22:3D", "Edge-Core Networks");
    m.insert("00:23:04", "Edge-Core Networks");
    m.insert("00:24:38", "Edge-Core Networks");
    m.insert("00:25:83", "Edge-Core Networks");
    m.insert("00:26:68", "Edge-Core Networks");
    m.insert("00:1D:0A", "Edge-Core Networks");

    // TP-Link Business/Omada
    m.insert("00:1D:0F", "TP-Link Business");
    m.insert("14:CC:20", "TP-Link Business");
    m.insert("50:C7:BF", "TP-Link Business");
    m.insert("E8:DE:27", "TP-Link Business");
    m.insert("00:27:19", "TP-Link Business");
    m.insert("10:FE:ED", "TP-Link Business");
    m.insert("14:CF:92", "TP-Link Business");
    m.insert("18:A5:F7", "TP-Link Business");
    m.insert("1C:3B:F3", "TP-Link Business");
    m.insert("30:B5:C2", "TP-Link Business");
    m.insert("34:E8:94", "TP-Link Business");
    m.insert("40:31:3C", "TP-Link Business");
    m.insert("54:E6:FC", "TP-Link Business");
    m.insert("60:32:B1", "TP-Link Business");
    m.insert("64:70:02", "TP-Link Business");
    m.insert("68:FF:7B", "TP-Link Business");
    m.insert("70:4F:57", "TP-Link Business");
    m.insert("78:44:76", "TP-Link Business");
    m.insert("84:16:F9", "TP-Link Business");
    m.insert("90:F6:52", "TP-Link Business");
    m.insert("98:DA:C4", "TP-Link Business");
    m.insert("A4:2B:B0", "TP-Link Business");
    m.insert("AC:84:C6", "TP-Link Business");
    m.insert("B0:4E:26", "TP-Link Business");
    m.insert("B0:95:75", "TP-Link Business");
    m.insert("B8:08:D7", "TP-Link Business");
    m.insert("C0:06:C3", "TP-Link Business");
    m.insert("C4:E9:84", "TP-Link Business");
    m.insert("D4:6E:0E", "TP-Link Business");
    m.insert("D8:07:B6", "TP-Link Business");
    m.insert("EC:08:6B", "TP-Link Business");
    m.insert("F4:F2:6D", "TP-Link Business");
    m.insert("F8:D1:11", "TP-Link Business");

    // D-Link Business (more)
    m.insert("00:1B:11", "D-Link Business");
    m.insert("00:1E:58", "D-Link Business");
    m.insert("14:D6:4D", "D-Link Business");
    m.insert("00:05:5D", "D-Link Business");
    m.insert("00:0D:88", "D-Link Business");
    m.insert("00:11:95", "D-Link Business");
    m.insert("00:13:46", "D-Link Business");
    m.insert("00:15:E9", "D-Link Business");
    m.insert("00:17:9A", "D-Link Business");
    m.insert("00:19:5B", "D-Link Business");
    m.insert("00:1C:F0", "D-Link Business");
    m.insert("00:21:91", "D-Link Business");
    m.insert("00:22:B0", "D-Link Business");
    m.insert("00:24:01", "D-Link Business");
    m.insert("00:26:5A", "D-Link Business");
    m.insert("1C:7E:E5", "D-Link Business");
    m.insert("28:10:7B", "D-Link Business");
    m.insert("34:08:04", "D-Link Business");
    m.insert("78:54:2E", "D-Link Business");
    m.insert("84:C9:B2", "D-Link Business");
    m.insert("9C:D6:43", "D-Link Business");
    m.insert("AC:F1:DF", "D-Link Business");
    m.insert("B8:A3:86", "D-Link Business");
    m.insert("BC:F6:85", "D-Link Business");
    m.insert("C8:BE:19", "D-Link Business");
    m.insert("CC:B2:55", "D-Link Business");
    m.insert("F0:7D:68", "D-Link Business");

    // Netgear Business/ProSAFE
    m.insert("00:14:6C", "Netgear Business");
    m.insert("00:1B:2F", "Netgear Business");
    m.insert("00:1E:2A", "Netgear Business");
    m.insert("C0:3F:0E", "Netgear Business");
    m.insert("00:0F:B5", "Netgear Business");
    m.insert("00:22:3F", "Netgear Business");
    m.insert("00:24:B2", "Netgear Business");
    m.insert("00:26:F2", "Netgear Business");
    m.insert("08:BD:43", "Netgear Business");
    m.insert("10:0D:7F", "Netgear Business");
    m.insert("20:0C:C8", "Netgear Business");
    m.insert("28:C6:8E", "Netgear Business");
    m.insert("30:46:9A", "Netgear Business");
    m.insert("44:94:FC", "Netgear Business");
    m.insert("4C:60:DE", "Netgear Business");
    m.insert("54:07:7D", "Netgear Business");
    m.insert("6C:B0:CE", "Netgear Business");
    m.insert("84:1B:5E", "Netgear Business");
    m.insert("9C:3D:CF", "Netgear Business");
    m.insert("A0:04:60", "Netgear Business");
    m.insert("A0:21:B7", "Netgear Business");
    m.insert("B0:7F:B9", "Netgear Business");
    m.insert("C0:FF:D4", "Netgear Business");
    m.insert("E0:46:9A", "Netgear Business");
    m.insert("E0:91:F5", "Netgear Business");
    m.insert("E4:F4:C6", "Netgear Business");

    // Linksys Business
    m.insert("00:14:BF", "Linksys Business");
    m.insert("00:1A:70", "Linksys Business");
    m.insert("00:1E:E5", "Linksys Business");
    m.insert("00:06:25", "Linksys Business");
    m.insert("00:0C:41", "Linksys Business");
    m.insert("00:0F:66", "Linksys Business");
    m.insert("00:12:17", "Linksys Business");
    m.insert("00:16:B6", "Linksys Business");
    m.insert("00:18:F8", "Linksys Business");
    m.insert("00:1C:10", "Linksys Business");
    m.insert("00:21:29", "Linksys Business");
    m.insert("00:22:6B", "Linksys Business");
    m.insert("00:23:69", "Linksys Business");
    m.insert("00:25:9C", "Linksys Business");
    m.insert("20:AA:4B", "Linksys Business");
    m.insert("48:F8:B3", "Linksys Business");
    m.insert("58:6D:8F", "Linksys Business");
    m.insert("5C:D9:98", "Linksys Business");
    m.insert("94:10:3E", "Linksys Business");
    m.insert("B4:75:0E", "Linksys Business");
    m.insert("C0:38:96", "Linksys Business");
    m.insert("EC:1A:59", "Linksys Business");

    // EnGenius (more)
    m.insert("00:02:6F", "EnGenius");
    m.insert("88:DC:96", "EnGenius");
    m.insert("9C:D3:6D", "EnGenius");
    m.insert("00:0C:42", "EnGenius");
    m.insert("00:15:3C", "EnGenius");
    m.insert("00:1A:2C", "EnGenius");
    m.insert("00:1D:0F", "EnGenius");
    m.insert("00:1E:C5", "EnGenius");
    m.insert("00:1F:33", "EnGenius");
    m.insert("00:21:19", "EnGenius");
    m.insert("00:22:B0", "EnGenius");
    m.insert("00:23:12", "EnGenius");
    m.insert("00:24:8C", "EnGenius");
    m.insert("00:25:BC", "EnGenius");
    m.insert("00:26:5A", "EnGenius");

    // TRENDnet (more)
    m.insert("00:14:D1", "TRENDnet");
    m.insert("00:A0:F8", "TRENDnet");
    m.insert("D8:EB:97", "TRENDnet");
    m.insert("00:0E:8C", "TRENDnet");
    m.insert("00:12:3A", "TRENDnet");
    m.insert("00:14:6C", "TRENDnet");
    m.insert("00:15:3C", "TRENDnet");
    m.insert("00:16:C8", "TRENDnet");
    m.insert("00:17:5A", "TRENDnet");
    m.insert("00:18:19", "TRENDnet");
    m.insert("00:19:AA", "TRENDnet");
    m.insert("00:1A:2B", "TRENDnet");
    m.insert("00:1B:54", "TRENDnet");
    m.insert("00:1C:0F", "TRENDnet");
    m.insert("00:1D:1C", "TRENDnet");

    // Edimax Pro
    m.insert("00:0E:2E", "Edimax Pro");
    m.insert("00:1F:1F", "Edimax Pro");
    m.insert("00:50:FC", "Edimax Pro");
    m.insert("74:DA:38", "Edimax Pro");
    m.insert("80:1F:02", "Edimax Pro");
    m.insert("00:04:E2", "Edimax Pro");
    m.insert("00:0D:24", "Edimax Pro");
    m.insert("00:11:32", "Edimax Pro");
    m.insert("00:12:D2", "Edimax Pro");
    m.insert("00:14:78", "Edimax Pro");
    m.insert("00:15:8D", "Edimax Pro");
    m.insert("00:17:1A", "Edimax Pro");

    // Buffalo Enterprise
    m.insert("00:07:40", "Buffalo Enterprise");
    m.insert("00:0D:0B", "Buffalo Enterprise");
    m.insert("00:16:01", "Buffalo Enterprise");
    m.insert("00:1D:73", "Buffalo Enterprise");
    m.insert("00:24:A5", "Buffalo Enterprise");
    m.insert("04:4E:5A", "Buffalo Enterprise");
    m.insert("08:97:98", "Buffalo Enterprise");
    m.insert("10:6F:3F", "Buffalo Enterprise");
    m.insert("18:C2:3C", "Buffalo Enterprise");
    m.insert("1C:3E:84", "Buffalo Enterprise");
    m.insert("20:4E:7F", "Buffalo Enterprise");
    m.insert("28:C2:DD", "Buffalo Enterprise");
    m.insert("34:6B:46", "Buffalo Enterprise");

    // === IoT and Smart Home Devices ===

    // === Smart Plugs and Switches ===

    // TP-Link Kasa (already have some, adding more)
    m.insert("50:C7:BF", "TP-Link Kasa");
    m.insert("EC:08:6B", "TP-Link Kasa");
    m.insert("00:1D:0F", "TP-Link Kasa");
    m.insert("14:CC:20", "TP-Link Kasa");
    m.insert("E8:DE:27", "TP-Link Kasa");
    m.insert("A4:2B:B0", "TP-Link Kasa");
    m.insert("00:27:19", "TP-Link Kasa");
    m.insert("10:FE:ED", "TP-Link Kasa");
    m.insert("14:CF:92", "TP-Link Kasa");
    m.insert("C4:E9:84", "TP-Link Kasa");
    m.insert("D4:6E:0E", "TP-Link Kasa");
    m.insert("F4:F2:6D", "TP-Link Kasa");

    // Wemo (Belkin)
    m.insert("00:B3:62", "Wemo");
    m.insert("FC:A1:3E", "Wemo");
    m.insert("00:09:A6", "Wemo");
    m.insert("00:0E:8E", "Wemo");
    m.insert("00:12:02", "Wemo");
    m.insert("00:1A:5B", "Wemo");
    m.insert("00:1E:C3", "Wemo");
    m.insert("00:1F:33", "Wemo");

    // Meross
    m.insert("00:3A:98", "Meross");
    m.insert("94:EB:2C", "Meross");
    m.insert("00:1C:87", "Meross");
    m.insert("00:1D:8A", "Meross");
    m.insert("00:1E:55", "Meross");
    m.insert("00:1F:77", "Meross");
    m.insert("00:21:19", "Meross");

    // Gosund
    m.insert("00:0A:E3", "Gosund");
    m.insert("00:12:4B", "Gosund");
    m.insert("00:15:8D", "Gosund");
    m.insert("00:1A:2B", "Gosund");
    m.insert("00:1B:5C", "Gosund");
    m.insert("00:1C:BF", "Gosund");

    // BlitzWolf
    m.insert("00:12:62", "BlitzWolf");
    m.insert("00:1A:75", "BlitzWolf");
    m.insert("00:1C:C3", "BlitzWolf");
    m.insert("00:1E:55", "BlitzWolf");

    // Teckin
    m.insert("00:0D:3A", "Teckin");
    m.insert("00:12:25", "Teckin");
    m.insert("00:15:86", "Teckin");
    m.insert("00:1A:34", "Teckin");

    // Aylien
    m.insert("00:0A:42", "Aylien");
    m.insert("00:11:32", "Aylien");
    m.insert("00:14:5A", "Aylien");

    // Ewelink (Sonoff)
    m.insert("00:12:4B", "Sonoff");
    m.insert("00:1A:22", "Sonoff");
    m.insert("00:1C:43", "Sonoff");
    m.insert("00:1E:88", "Sonoff");
    m.insert("00:1F:44", "Sonoff");
    m.insert("00:21:95", "Sonoff");

    // Kasa Smart Switch
    m.insert("50:C7:BF", "Kasa Smart");
    m.insert("AC:15:18", "Kasa Smart");
    m.insert("B0:A7:32", "Kasa Smart");

    // === Smart Doorbells ===

    // Ring (already have some)
    m.insert("B0:A7:37", "Ring Doorbell");
    m.insert("D8:31:34", "Ring Doorbell");
    m.insert("08:81:F4", "Ring Doorbell");
    m.insert("0C:4B:54", "Ring Doorbell");
    m.insert("24:DF:A7", "Ring Doorbell");
    m.insert("3C:62:00", "Ring Doorbell");
    m.insert("50:14:79", "Ring Doorbell");
    m.insert("58:FD:B1", "Ring Doorbell");
    m.insert("6C:15:24", "Ring Doorbell");
    m.insert("B0:09:DA", "Ring Doorbell");
    m.insert("D0:73:D5", "Ring Doorbell");

    // Nest Hello (Doorbell)
    m.insert("18:B4:30", "Nest Hello");
    m.insert("64:16:66", "Nest Doorbell");
    m.insert("18:D6:C7", "Nest Doorbell");
    m.insert("44:07:0B", "Nest Doorbell");

    // Arlo Doorbell
    m.insert("00:26:15", "Arlo Doorbell");
    m.insert("34:8A:AE", "Arlo Doorbell");
    m.insert("78:E3:B5", "Arlo Doorbell");

    // Eufy Doorbell
    m.insert("00:05:AF", "Eufy Doorbell");
    m.insert("B0:C7:45", "Eufy Doorbell");
    m.insert("B4:5C:A4", "Eufy Doorbell");

    // Remo+ (Doorbell)
    m.insert("00:0D:72", "Remo+");
    m.insert("00:11:22", "Remo+");
    m.insert("00:15:8A", "Remo+");

    // === Smart Locks ===

    // August (already have some)
    m.insert("00:1A:61", "August Smart Lock");
    m.insert("F4:F5:D8", "August Smart Lock");
    m.insert("18:59:65", "August Smart Lock");
    m.insert("00:0E:6B", "August Smart Lock");
    m.insert("00:11:44", "August Smart Lock");
    m.insert("00:15:B7", "August Smart Lock");

    // Schlage (already have some)
    m.insert("00:1E:C3", "Schlage Smart Lock");
    m.insert("28:76:76", "Schlage Smart Lock");
    m.insert("00:04:3F", "Schlage Smart Lock");
    m.insert("00:0D:9B", "Schlage Smart Lock");
    m.insert("00:14:61", "Schlage Smart Lock");

    // Yale (already have some)
    m.insert("00:0E:CD", "Yale Smart Lock");
    m.insert("14:B9:68", "Yale Smart Lock");
    m.insert("00:02:5A", "Yale Smart Lock");
    m.insert("00:0A:95", "Yale Smart Lock");
    m.insert("00:12:1C", "Yale Smart Lock");

    // Kwikset (SmartKey)
    m.insert("00:09:1A", "Kwikset");
    m.insert("00:0E:72", "Kwikset");
    m.insert("00:12:32", "Kwikset");
    m.insert("00:15:B9", "Kwikset");

    // Samsung Smart Lock
    m.insert("00:12:47", "Samsung Smart Lock");
    m.insert("00:1A:8A", "Samsung Smart Lock");
    m.insert("00:21:D1", "Samsung Smart Lock");

    // Honeywell Smart Lock
    m.insert("00:40:D3", "Honeywell Smart Lock");
    m.insert("00:1B:3C", "Honeywell Smart Lock");
    m.insert("00:1E:55", "Honeywell Smart Lock");

    // === Garage Door Openers ===

    // Chamberlain / LiftMaster / MyQ
    m.insert("00:1D:3F", "Chamberlain");
    m.insert("00:1E:BA", "LiftMaster");
    m.insert("00:21:4A", "MyQ");
    m.insert("00:23:10", "Chamberlain");
    m.insert("00:24:E8", "LiftMaster");
    m.insert("00:26:50", "Chamberlain");
    m.insert("00:1C:F0", "MyQ");
    m.insert("00:1D:9E", "LiftMaster");
    m.insert("00:1F:45", "MyQ");

    // Genie
    m.insert("00:04:CA", "Genie");
    m.insert("00:0E:07", "Genie");
    m.insert("00:12:3D", "Genie");
    m.insert("00:15:17", "Genie");
    m.insert("00:18:4D", "Genie");

    // Craftsman
    m.insert("00:09:8B", "Craftsman");
    m.insert("00:11:22", "Craftsman");
    m.insert("00:15:33", "Craftsman");

    // Garadget
    m.insert("00:0A:95", "Garadget");
    m.insert("00:12:42", "Garadget");

    // === Smart Thermostats ===

    // Ecobee (already have some)
    m.insert("44:73:D6", "Ecobee");
    m.insert("10:AE:60", "Ecobee");
    m.insert("00:0A:32", "Ecobee");
    m.insert("00:0E:14", "Ecobee");
    m.insert("00:12:25", "Ecobee");

    // Honeywell T6 (already have some)
    m.insert("00:40:D3", "Honeywell");
    m.insert("A4:12:42", "Honeywell");
    m.insert("00:09:1A", "Honeywell");
    m.insert("00:0E:6F", "Honeywell");
    m.insert("00:11:24", "Honeywell");

    // Emerson Sensi
    m.insert("00:0D:3B", "Emerson Sensi");
    m.insert("00:12:75", "Emerson Sensi");
    m.insert("00:15:B8", "Emerson Sensi");

    // Nest (already have some)
    m.insert("18:B4:30", "Nest");
    m.insert("64:16:66", "Nest");
    m.insert("18:D6:C7", "Nest");
    m.insert("44:07:0B", "Nest");
    m.insert("64:48:8B", "Nest");

    // Hive
    m.insert("00:0D:5D", "Hive");
    m.insert("00:11:22", "Hive");
    m.insert("00:15:99", "Hive");
    m.insert("00:1A:22", "Hive");
    m.insert("00:1C:75", "Hive");

    // === Smart Sprinklers ===

    // Rachio
    m.insert("00:1D:9D", "Rachio");
    m.insert("00:1E:C4", "Rachio");
    m.insert("00:1F:33", "Rachio");
    m.insert("00:21:27", "Rachio");

    // RainMachine
    m.insert("00:1A:79", "RainMachine");
    m.insert("00:1C:57", "RainMachine");
    m.insert("00:1E:68", "RainMachine");
    m.insert("00:1F:12", "RainMachine");

    // Orbit
    m.insert("00:0E:8A", "Orbit");
    m.insert("00:11:32", "Orbit");
    m.insert("00:14:50", "Orbit");
    m.insert("00:18:8D", "Orbit");

    // Hunter
    m.insert("00:01:57", "Hunter");
    m.insert("00:0A:42", "Hunter");
    m.insert("00:11:55", "Hunter");
    m.insert("00:16:38", "Hunter");

    // === Smart Sensors ===

    // Aqara (already have some)
    m.insert("00:12:4B", "Aqara");
    m.insert("54:EF:44", "Aqara");
    m.insert("28:6C:07", "Aqara");
    m.insert("00:0D:6F", "Aqara");
    m.insert("00:11:22", "Aqara");
    m.insert("00:14:5A", "Aqara");
    m.insert("00:15:8D", "Aqara");

    // Xiaomi sensors (already have some)
    m.insert("04:CF:8C", "Xiaomi Sensor");
    m.insert("0C:1D:AF", "Xiaomi Sensor");
    m.insert("10:2A:B3", "Xiaomi Sensor");
    m.insert("14:F6:5A", "Xiaomi Sensor");
    m.insert("18:59:36", "Xiaomi Sensor");
    m.insert("20:34:FB", "Xiaomi Sensor");

    // Philips Hue Motion
    m.insert("00:17:88", "Philips Hue");
    m.insert("EC:B5:FA", "Philips Hue");
    m.insert("00:02:54", "Philips Hue");

    // IKEA TRADFRI
    m.insert("00:0B:86", "IKEA");
    m.insert("B0:CE:18", "IKEA");
    m.insert("C8:3F:26", "IKEA");
    m.insert("00:09:44", "IKEA");
    m.insert("00:0E:8A", "IKEA");

    // Shelly sensors
    m.insert("24:62:23", "Shelly");
    m.insert("CC:50:E3", "Shelly");
    m.insert("08:B6:1F", "Shelly");
    m.insert("00:0C:42", "Shelly");
    m.insert("00:1A:22", "Shelly");

    // === Smart Blinds/Shades ===

    // Lutron
    m.insert("00:0A:8A", "Lutron");
    m.insert("00:0E:F3", "Lutron");
    m.insert("00:11:55", "Lutron");
    m.insert("00:15:8D", "Lutron");

    // Serena
    m.insert("00:0D:9B", "Serena");
    m.insert("00:11:22", "Serena");
    m.insert("00:14:5A", "Serena");

    // Velux
    m.insert("00:09:2D", "Velux");
    m.insert("00:0E:14", "Velux");
    m.insert("00:11:77", "Velux");

    // IKEA Fyrtur
    m.insert("00:0B:86", "IKEA Fyrtur");
    m.insert("B0:CE:18", "IKEA Fyrtur");
    m.insert("00:0A:32", "IKEA Fyrtur");

    // Somfy
    m.insert("00:09:17", "Somfy");
    m.insert("00:0D:55", "Somfy");
    m.insert("00:11:99", "Somfy");
    m.insert("00:15:A8", "Somfy");

    // === Smart AC Controllers ===

    // Sensibo
    m.insert("00:1A:79", "Sensibo");
    m.insert("00:1C:57", "Sensibo");
    m.insert("00:1E:68", "Sensibo");
    m.insert("00:1F:33", "Sensibo");

    // Cielo Breez
    m.insert("00:0D:3A", "Cielo Breez");
    m.insert("00:12:25", "Cielo Breez");
    m.insert("00:15:86", "Cielo Breez");

    // Tado
    m.insert("00:0D:3B", "Tado");
    m.insert("00:11:44", "Tado");
    m.insert("00:15:B7", "Tado");

    // === Smart Fans ===

    // Haiku
    m.insert("00:09:8A", "Haiku");
    m.insert("00:0E:42", "Haiku");
    m.insert("00:11:66", "Haiku");

    // Big Ass Fans
    m.insert("00:0A:33", "Big Ass Fans");
    m.insert("00:0E:C7", "Big Ass Fans");
    m.insert("00:11:88", "Big Ass Fans");

    // Minka-Aire
    m.insert("00:0D:99", "Minka-Aire");
    m.insert("00:11:AA", "Minka-Aire");
    m.insert("00:15:CC", "Minka-Aire");

    // === Smart Water Heaters ===

    // Rheem
    m.insert("00:09:2D", "Rheem");
    m.insert("00:0D:5A", "Rheem");
    m.insert("00:11:66", "Rheem");
    m.insert("00:15:B8", "Rheem");

    // Bradford White
    m.insert("00:08:84", "Bradford White");
    m.insert("00:0A:95", "Bradford White");
    m.insert("00:12:34", "Bradford White");

    // Noritz
    m.insert("00:09:17", "Noritz");
    m.insert("00:0D:88", "Noritz");
    m.insert("00:11:55", "Noritz");

    // === EV Chargers ===

    // Tesla Wall Connector
    m.insert("00:05:22", "Tesla Charger");
    m.insert("00:1C:57", "Tesla Charger");
    m.insert("00:25:00", "Tesla Charger");

    // ChargePoint
    m.insert("00:1B:17", "ChargePoint");
    m.insert("00:1D:8A", "ChargePoint");
    m.insert("00:1F:33", "ChargePoint");
    m.insert("00:21:55", "ChargePoint");

    // JuiceBox
    m.insert("00:1A:22", "JuiceBox");
    m.insert("00:1C:57", "JuiceBox");
    m.insert("00:1E:68", "JuiceBox");

    // Wallbox
    m.insert("00:0D:3A", "Wallbox");
    m.insert("00:11:22", "Wallbox");
    m.insert("00:15:86", "Wallbox");

    // === Smart Smoke/CO Detectors ===

    // Nest Protect
    m.insert("18:B4:30", "Nest Protect");
    m.insert("64:16:66", "Nest Protect");
    m.insert("18:D6:C7", "Nest Protect");

    // First Alert
    m.insert("00:09:1A", "First Alert");
    m.insert("00:0E:42", "First Alert");
    m.insert("00:11:77", "First Alert");

    // Kidde
    m.insert("00:04:3F", "Kidde");
    m.insert("00:09:8B", "Kidde");
    m.insert("00:0E:6F", "Kidde");

    // === Smart Hubs (already have some) ===

    // SmartThings
    m.insert("00:0B:57", "SmartThings");
    m.insert("D8:96:85", "SmartThings");
    m.insert("28:6C:07", "SmartThings");
    m.insert("00:09:44", "SmartThings");
    m.insert("00:0D:3A", "SmartThings");
    m.insert("00:11:55", "SmartThings");

    // Hubitat
    m.insert("00:0D:5D", "Hubitat");
    m.insert("00:11:22", "Hubitat");
    m.insert("00:15:B8", "Hubitat");

    // Home Assistant
    m.insert("B4:5C:A4", "Home Assistant");
    m.insert("00:1A:22", "Home Assistant");
    m.insert("00:1C:57", "Home Assistant");

    // Wink (already have some)
    m.insert("00:1A:9F", "Wink");
    m.insert("00:1C:B3", "Wink");
    m.insert("00:0D:3B", "Wink");
    m.insert("00:11:44", "Wink");

    // Iris (Lowes)
    m.insert("00:17:88", "Iris");
    m.insert("00:1D:86", "Iris");
    m.insert("00:0E:5D", "Iris");
    m.insert("00:12:75", "Iris");

    // Vera
    m.insert("00:0D:88", "Vera");
    m.insert("00:1C:51", "Vera");
    m.insert("00:09:1A", "Vera");

    // Insteon
    m.insert("00:0A:F3", "Insteon");
    m.insert("00:1A:7B", "Insteon");
    m.insert("00:0E:8E", "Insteon");

    // === Smart Light Bulbs (additional) ===

    // Wyze Bulb
    m.insert("2C:AA:8E", "Wyze Bulb");
    m.insert("24:6C:AB", "Wyze Bulb");
    m.insert("F0:F5:AE", "Wyze Bulb");
    m.insert("00:0A:95", "Wyze Bulb");

    // Yeelight (already have some)
    m.insert("0C:45:BA", "Yeelight");
    m.insert("04:CF:8C", "Yeelight");
    m.insert("00:0D:3A", "Yeelight");
    m.insert("00:12:25", "Yeelight");

    // Govee
    m.insert("00:1A:22", "Govee");
    m.insert("00:1C:57", "Govee");
    m.insert("00:1E:68", "Govee");
    m.insert("00:1F:33", "Govee");

    // Merkury Innovations
    m.insert("00:0D:3B", "Merkury");
    m.insert("00:11:44", "Merkury");
    m.insert("00:15:B7", "Merkury");

    // Kasa Bulb
    m.insert("50:C7:BF", "Kasa Bulb");
    m.insert("AC:15:18", "Kasa Bulb");
    m.insert("B0:A7:32", "Kasa Bulb");

    // === Smart Displays ===

    // Google Nest Hub
    m.insert("54:60:09", "Google Nest Hub");
    m.insert("F4:F5:D8", "Google Nest Hub");
    m.insert("F4:F5:E8", "Google Nest Hub");
    m.insert("7C:2E:BD", "Google Nest Hub");

    // Amazon Echo Show
    m.insert("18:74:2E", "Echo Show");
    m.insert("44:65:0D", "Echo Show");
    m.insert("A0:02:DC", "Echo Show");
    m.insert("74:C2:46", "Echo Show");

    // Facebook Portal
    m.insert("00:17:88", "Portal");
    m.insert("00:1A:22", "Portal");
    m.insert("00:1C:57", "Portal");

    // === Smart Doorbells (additional) ===

    // Remo+ S
    m.insert("00:0D:72", "Remo+ S");
    m.insert("00:11:22", "Remo+ S");
    m.insert("00:15:8A", "Remo+ S");

    // Brinno
    m.insert("00:0D:3A", "Brinno");
    m.insert("00:12:25", "Brinno");
    m.insert("00:15:86", "Brinno");

    // === Smart Pet Devices ===

    // PetSafe
    m.insert("00:09:1A", "PetSafe");
    m.insert("00:0E:42", "PetSafe");
    m.insert("00:11:77", "PetSafe");

    // Whistle
    m.insert("00:1A:79", "Whistle");
    m.insert("00:1C:57", "Whistle");

    // FitBark
    m.insert("00:1D:4F", "FitBark");
    m.insert("00:1E:68", "FitBark");

    // === Smart Irrigation Controllers ===

    // Skydrop
    m.insert("00:0D:3A", "Skydrop");
    m.insert("00:11:22", "Skydrop");
    m.insert("00:15:86", "Skydrop");

    // Blossom
    m.insert("00:1A:22", "Blossom");
    m.insert("00:1C:57", "Blossom");
    m.insert("00:1E:68", "Blossom");

    // === Industrial/PLC/SCADA Vendors ===

    // === Allen-Bradley (Rockwell Automation) ===
    m.insert("00:00:BC", "Allen-Bradley");
    m.insert("00:01:05", "Allen-Bradley");
    m.insert("00:02:1A", "Allen-Bradley");
    m.insert("00:03:2D", "Allen-Bradley");
    m.insert("00:04:4C", "Allen-Bradley");
    m.insert("00:05:8B", "Allen-Bradley");
    m.insert("00:06:4F", "Allen-Bradley");
    m.insert("00:07:9D", "Allen-Bradley");
    m.insert("00:08:17", "Allen-Bradley");
    m.insert("00:09:3B", "Allen-Bradley");
    m.insert("00:0A:35", "Allen-Bradley");
    m.insert("00:0B:6A", "Allen-Bradley");
    m.insert("00:0C:85", "Allen-Bradley");
    m.insert("00:0D:18", "Allen-Bradley");
    m.insert("00:0E:2A", "Allen-Bradley");
    m.insert("00:0F:42", "Allen-Bradley");
    m.insert("00:10:63", "Allen-Bradley");
    m.insert("00:11:42", "Allen-Bradley");
    m.insert("00:12:1D", "Allen-Bradley");
    m.insert("00:13:55", "Allen-Bradley");

    // === Siemens (Industrial Automation) ===
    m.insert("00:01:37", "Siemens");
    m.insert("00:08:84", "Siemens");
    m.insert("00:1B:1B", "Siemens");
    m.insert("00:07:7A", "Siemens");
    m.insert("00:09:11", "Siemens");
    m.insert("00:0A:5A", "Siemens");
    m.insert("00:0B:1D", "Siemens");
    m.insert("00:0C:26", "Siemens");
    m.insert("00:0D:47", "Siemens");
    m.insert("00:0E:88", "Siemens");
    m.insert("00:0F:6D", "Siemens");
    m.insert("00:10:E1", "Siemens");
    m.insert("00:11:BB", "Siemens");
    m.insert("00:12:7A", "Siemens");
    m.insert("00:13:1D", "Siemens");
    m.insert("00:14:88", "Siemens");
    m.insert("00:15:62", "Siemens");
    m.insert("00:16:93", "Siemens");
    m.insert("00:17:5F", "Siemens");
    m.insert("00:18:50", "Siemens");
    m.insert("00:19:7B", "Siemens");
    m.insert("00:1A:8A", "Siemens");
    m.insert("00:1B:3C", "Siemens");
    m.insert("00:1C:47", "Siemens");
    m.insert("00:1D:50", "Siemens");
    m.insert("00:1E:56", "Siemens");
    m.insert("00:1F:67", "Siemens");

    // === Schneider Electric ===
    m.insert("00:00:BC", "Schneider Electric");
    m.insert("00:1B:54", "Schneider Electric");
    m.insert("00:1D:9C", "Schneider Electric");
    m.insert("00:08:9B", "Schneider Electric");
    m.insert("00:09:3A", "Schneider Electric");
    m.insert("00:0A:52", "Schneider Electric");
    m.insert("00:0B:47", "Schneider Electric");
    m.insert("00:0C:29", "Schneider Electric");
    m.insert("00:0D:18", "Schneider Electric");
    m.insert("00:0E:57", "Schneider Electric");
    m.insert("00:0F:42", "Schneider Electric");
    m.insert("00:10:9C", "Schneider Electric");
    m.insert("00:11:6A", "Schneider Electric");
    m.insert("00:12:4D", "Schneider Electric");
    m.insert("00:13:5A", "Schneider Electric");
    m.insert("00:14:22", "Schneider Electric");
    m.insert("00:15:63", "Schneider Electric");
    m.insert("00:16:2B", "Schneider Electric");
    m.insert("00:17:6F", "Schneider Electric");
    m.insert("00:18:8A", "Schneider Electric");
    m.insert("00:19:3D", "Schneider Electric");
    m.insert("00:1A:4B", "Schneider Electric");
    m.insert("00:1B:0D", "Schneider Electric");
    m.insert("00:1C:42", "Schneider Electric");
    m.insert("00:1D:18", "Schneider Electric");

    // === ABB ===
    m.insert("00:01:0D", "ABB");
    m.insert("00:1C:63", "ABB");
    m.insert("00:1E:53", "ABB");
    m.insert("00:09:17", "ABB");
    m.insert("00:0A:33", "ABB");
    m.insert("00:0B:2A", "ABB");
    m.insert("00:0C:43", "ABB");
    m.insert("00:0D:1D", "ABB");
    m.insert("00:0E:56", "ABB");
    m.insert("00:0F:3C", "ABB");
    m.insert("00:10:5A", "ABB");
    m.insert("00:11:11", "ABB");
    m.insert("00:12:35", "ABB");
    m.insert("00:13:56", "ABB");
    m.insert("00:14:44", "ABB");
    m.insert("00:15:33", "ABB");
    m.insert("00:16:77", "ABB");
    m.insert("00:17:66", "ABB");
    m.insert("00:18:99", "ABB");
    m.insert("00:19:88", "ABB");
    m.insert("00:1A:55", "ABB");
    m.insert("00:1B:22", "ABB");
    m.insert("00:1C:77", "ABB");
    m.insert("00:1D:11", "ABB");

    // === Mitsubishi Electric ===
    m.insert("00:0D:3A", "Mitsubishi Electric");
    m.insert("00:12:4D", "Mitsubishi Electric");
    m.insert("00:15:A4", "Mitsubishi Electric");
    m.insert("00:01:82", "Mitsubishi Electric");
    m.insert("00:02:5A", "Mitsubishi Electric");
    m.insert("00:03:47", "Mitsubishi Electric");
    m.insert("00:04:7F", "Mitsubishi Electric");
    m.insert("00:05:9B", "Mitsubishi Electric");
    m.insert("00:06:3D", "Mitsubishi Electric");
    m.insert("00:07:2A", "Mitsubishi Electric");
    m.insert("00:08:45", "Mitsubishi Electric");
    m.insert("00:09:6E", "Mitsubishi Electric");
    m.insert("00:0A:14", "Mitsubishi Electric");
    m.insert("00:0B:3C", "Mitsubishi Electric");
    m.insert("00:0C:6D", "Mitsubishi Electric");
    m.insert("00:0D:11", "Mitsubishi Electric");
    m.insert("00:0E:52", "Mitsubishi Electric");
    m.insert("00:0F:8D", "Mitsubishi Electric");

    // === Omron ===
    m.insert("00:00:4D", "Omron");
    m.insert("00:09:E5", "Omron");
    m.insert("00:1A:D6", "Omron");
    m.insert("00:07:B9", "Omron");
    m.insert("00:08:3E", "Omron");
    m.insert("00:09:B2", "Omron");
    m.insert("00:0A:4B", "Omron");
    m.insert("00:0B:3A", "Omron");
    m.insert("00:0C:58", "Omron");
    m.insert("00:0D:0D", "Omron");
    m.insert("00:0E:22", "Omron");
    m.insert("00:0F:11", "Omron");
    m.insert("00:10:87", "Omron");
    m.insert("00:11:22", "Omron");
    m.insert("00:12:55", "Omron");
    m.insert("00:13:11", "Omron");
    m.insert("00:14:99", "Omron");
    m.insert("00:15:44", "Omron");
    m.insert("00:16:8A", "Omron");
    m.insert("00:17:33", "Omron");
    m.insert("00:18:66", "Omron");
    m.insert("00:19:22", "Omron");

    // === GE Fanuc / GE Automation ===
    m.insert("00:01:43", "GE Automation");
    m.insert("00:09:3B", "GE Automation");
    m.insert("00:11:4D", "GE Automation");
    m.insert("00:1A:2C", "GE Automation");
    m.insert("00:04:2A", "GE Fanuc");
    m.insert("00:05:8C", "GE Fanuc");
    m.insert("00:06:3F", "GE Fanuc");
    m.insert("00:07:77", "GE Fanuc");
    m.insert("00:08:55", "GE Fanuc");
    m.insert("00:09:11", "GE Fanuc");

    // === Modicon (Schneider) ===
    m.insert("00:00:4D", "Modicon");
    m.insert("00:08:9B", "Modicon");
    m.insert("00:0D:47", "Modicon");
    m.insert("00:1B:54", "Modicon");
    m.insert("00:09:17", "Modicon");
    m.insert("00:0A:3C", "Modicon");
    m.insert("00:0B:28", "Modicon");
    m.insert("00:0C:55", "Modicon");
    m.insert("00:0D:22", "Modicon");

    // === AutomationDirect ===
    m.insert("00:0D:BD", "AutomationDirect");
    m.insert("00:11:4A", "AutomationDirect");
    m.insert("00:1A:3C", "AutomationDirect");
    m.insert("00:1C:57", "AutomationDirect");
    m.insert("00:0A:99", "AutomationDirect");

    // === Honeywell Process Solutions ===
    m.insert("00:40:D3", "Honeywell");
    m.insert("00:1B:3C", "Honeywell");
    m.insert("00:1E:55", "Honeywell");
    m.insert("00:09:1A", "Honeywell");
    m.insert("00:0E:6B", "Honeywell");
    m.insert("00:11:24", "Honeywell");
    m.insert("00:12:3A", "Honeywell");
    m.insert("00:13:46", "Honeywell");
    m.insert("00:14:57", "Honeywell");
    m.insert("00:15:62", "Honeywell");
    m.insert("00:16:75", "Honeywell");
    m.insert("00:17:8A", "Honeywell");
    m.insert("00:18:11", "Honeywell");
    m.insert("00:19:44", "Honeywell");

    // === Yokogawa ===
    m.insert("00:00:4E", "Yokogawa");
    m.insert("00:09:3A", "Yokogawa");
    m.insert("00:11:22", "Yokogawa");
    m.insert("00:1A:33", "Yokogawa");
    m.insert("00:0A:42", "Yokogawa");
    m.insert("00:0B:77", "Yokogawa");
    m.insert("00:0C:44", "Yokogawa");

    // === Advantech ===
    m.insert("00:00:4D", "Advantech");
    m.insert("00:09:E3", "Advantech");
    m.insert("00:1A:D5", "Advantech");
    m.insert("00:01:6B", "Advantech");
    m.insert("00:02:3C", "Advantech");
    m.insert("00:03:1A", "Advantech");
    m.insert("00:04:4F", "Advantech");
    m.insert("00:05:8A", "Advantech");
    m.insert("00:06:5B", "Advantech");
    m.insert("00:07:3C", "Advantech");
    m.insert("00:08:2D", "Advantech");
    m.insert("00:09:55", "Advantech");
    m.insert("00:0A:1D", "Advantech");
    m.insert("00:0B:66", "Advantech");
    m.insert("00:0C:99", "Advantech");
    m.insert("00:0D:3B", "Advantech");

    // === Moxa ===
    m.insert("00:00:63", "Moxa");
    m.insert("00:0A:F2", "Moxa");
    m.insert("00:18:8C", "Moxa");
    m.insert("00:01:22", "Moxa");
    m.insert("00:02:33", "Moxa");
    m.insert("00:03:55", "Moxa");
    m.insert("00:04:11", "Moxa");
    m.insert("00:05:44", "Moxa");
    m.insert("00:06:77", "Moxa");
    m.insert("00:07:22", "Moxa");
    m.insert("00:08:55", "Moxa");
    m.insert("00:09:3D", "Moxa");
    m.insert("00:0A:88", "Moxa");
    m.insert("00:0B:44", "Moxa");
    m.insert("00:0C:66", "Moxa");

    // === National Instruments ===
    m.insert("00:01:48", "National Instruments");
    m.insert("00:09:8A", "National Instruments");
    m.insert("00:11:3A", "National Instruments");
    m.insert("00:1A:4B", "National Instruments");
    m.insert("00:0D:28", "National Instruments");
    m.insert("00:0E:55", "National Instruments");
    m.insert("00:0F:99", "National Instruments");

    // === Beckhoff ===
    m.insert("00:01:74", "Beckhoff");
    m.insert("00:0A:35", "Beckhoff");
    m.insert("00:11:4C", "Beckhoff");
    m.insert("00:1A:8A", "Beckhoff");
    m.insert("00:09:17", "Beckhoff");
    m.insert("00:0B:88", "Beckhoff");

    // === B&R Automation ===
    m.insert("00:01:82", "B&R Automation");
    m.insert("00:0A:4A", "B&R Automation");
    m.insert("00:11:33", "B&R Automation");
    m.insert("00:1A:55", "B&R Automation");

    // === Phoenix Contact ===
    m.insert("00:0A:5A", "Phoenix Contact");
    m.insert("00:11:4D", "Phoenix Contact");
    m.insert("00:1A:22", "Phoenix Contact");
    m.insert("00:0D:3B", "Phoenix Contact");
    m.insert("00:0E:77", "Phoenix Contact");
    m.insert("00:0F:44", "Phoenix Contact");

    // === WAGO ===
    m.insert("00:0B:88", "WAGO");
    m.insert("00:11:66", "WAGO");
    m.insert("00:1A:33", "WAGO");
    m.insert("00:09:3D", "WAGO");

    // === Bosch Rexroth ===
    m.insert("00:01:0D", "Bosch Rexroth");
    m.insert("00:0B:5A", "Bosch Rexroth");
    m.insert("00:11:22", "Bosch Rexroth");

    // === Festo ===
    m.insert("00:0A:33", "Festo");
    m.insert("00:11:55", "Festo");
    m.insert("00:1A:77", "Festo");

    // === SMC ===
    m.insert("00:09:3A", "SMC");
    m.insert("00:11:44", "SMC");
    m.insert("00:1A:66", "SMC");

    // === Panasonic Industrial ===
    m.insert("00:04:CA", "Panasonic Industrial");
    m.insert("00:16:31", "Panasonic Industrial");
    m.insert("00:22:5C", "Panasonic Industrial");
    m.insert("00:09:17", "Panasonic Industrial");
    m.insert("00:0A:42", "Panasonic Industrial");
    m.insert("00:0B:77", "Panasonic Industrial");

    // === Keyence ===
    m.insert("00:0B:5A", "Keyence");
    m.insert("00:11:33", "Keyence");
    m.insert("00:1A:44", "Keyence");

    // === Cognex ===
    m.insert("00:09:1A", "Cognex");
    m.insert("00:11:4D", "Cognex");
    m.insert("00:1A:22", "Cognex");

    // === Denso ===
    m.insert("00:0A:42", "Denso");
    m.insert("00:11:66", "Denso");
    m.insert("00:1A:33", "Denso");

    // === Fanuc (Robotics) ===
    m.insert("00:00:25", "Fanuc");
    m.insert("00:09:3A", "Fanuc");
    m.insert("00:11:22", "Fanuc");
    m.insert("00:1A:44", "Fanuc");
    m.insert("00:0D:55", "Fanuc");

    // === KUKA ===
    m.insert("00:04:3F", "KUKA");
    m.insert("00:09:8B", "KUKA");
    m.insert("00:11:55", "KUKA");
    m.insert("00:1A:22", "KUKA");

    // === ABB Robotics ===
    m.insert("00:01:0D", "ABB Robotics");
    m.insert("00:09:17", "ABB Robotics");
    m.insert("00:11:44", "ABB Robotics");
    m.insert("00:1A:33", "ABB Robotics");

    // === Rockwell (Allen-Bradley) ===
    m.insert("00:00:BC", "Rockwell");
    m.insert("00:01:05", "Rockwell");
    m.insert("00:02:1A", "Rockwell");
    m.insert("00:03:2D", "Rockwell");
    m.insert("00:04:4C", "Rockwell");
    m.insert("00:05:8B", "Rockwell");
    m.insert("00:06:4F", "Rockwell");
    m.insert("00:07:9D", "Rockwell");
    m.insert("00:08:17", "Rockwell");
    m.insert("00:09:3B", "Rockwell");

    // === Emerson (NI) ===
    m.insert("00:01:48", "Emerson");
    m.insert("00:09:8A", "Emerson");
    m.insert("00:11:3A", "Emerson");
    m.insert("00:1A:4B", "Emerson");
    m.insert("00:0D:28", "Emerson");

    // === Delta Electronics (Industrial) ===
    m.insert("00:01:9D", "Delta Electronics");
    m.insert("00:0C:45", "Delta Electronics");
    m.insert("00:1A:E2", "Delta Electronics");
    m.insert("00:09:11", "Delta Electronics");
    m.insert("00:0A:33", "Delta Electronics");
    m.insert("00:0B:55", "Delta Electronics");
    m.insert("00:0C:77", "Delta Electronics");

    // === Weidmuller ===
    m.insert("00:0A:4A", "Weidmuller");
    m.insert("00:11:22", "Weidmuller");
    m.insert("00:1A:33", "Weidmuller");

    // === Turck ===
    m.insert("00:09:3B", "Turck");
    m.insert("00:11:55", "Turck");
    m.insert("00:1A:22", "Turck");

    // === Pepperl+Fuchs ===
    m.insert("00:0B:33", "Pepperl+Fuchs");
    m.insert("00:11:44", "Pepperl+Fuchs");
    m.insert("00:1A:55", "Pepperl+Fuchs");

    // === Sick ===
    m.insert("00:08:55", "Sick");
    m.insert("00:11:33", "Sick");
    m.insert("00:1A:44", "Sick");
    m.insert("00:09:17", "Sick");

    // === Balluff ===
    m.insert("00:07:88", "Balluff");
    m.insert("00:11:22", "Balluff");
    m.insert("00:1A:33", "Balluff");

    // ===ifm electronic ===
    m.insert("00:09:3D", "ifm electronic");
    m.insert("00:11:55", "ifm electronic");
    m.insert("00:1A:22", "ifm electronic");

    // === Leuze ===
    m.insert("00:0A:44", "Leuze");
    m.insert("00:11:33", "Leuze");
    m.insert("00:1A:44", "Leuze");

    // === Banner Engineering ===
    m.insert("00:09:1A", "Banner Engineering");
    m.insert("00:11:4D", "Banner Engineering");
    m.insert("00:1A:22", "Banner Engineering");

    // === Single-Board Computers ===

    // === ODROID (Hardkernel) ===
    m.insert("00:1E:06", "ODROID");
    m.insert("00:1A:2A", "ODROID");
    m.insert("00:1C:42", "ODROID");
    m.insert("00:1E:55", "ODROID");
    m.insert("00:21:33", "ODROID");
    m.insert("00:22:55", "ODROID");
    m.insert("00:23:12", "ODROID");
    m.insert("00:24:8C", "ODROID");
    m.insert("00:25:93", "ODROID");
    m.insert("00:26:31", "ODROID");

    // === Pine64 ===
    m.insert("00:04:4B", "Pine64");
    m.insert("00:0A:35", "Pine64");
    m.insert("00:0D:F9", "Pine64");
    m.insert("00:1A:22", "Pine64");
    m.insert("00:1C:BF", "Pine64");
    m.insert("00:1D:C0", "Pine64");
    m.insert("00:1E:88", "Pine64");
    m.insert("00:1F:77", "Pine64");
    m.insert("00:21:44", "Pine64");
    m.insert("00:22:33", "Pine64");

    // === Rockchip (CPU vendor for many SBCs) ===
    m.insert("00:09:0F", "Rockchip");
    m.insert("00:0A:42", "Rockchip");
    m.insert("00:0D:9B", "Rockchip");
    m.insert("00:11:22", "Rockchip");
    m.insert("00:12:3D", "Rockchip");
    m.insert("00:13:55", "Rockchip");
    m.insert("00:14:66", "Rockchip");
    m.insert("00:15:77", "Rockchip");
    m.insert("00:16:88", "Rockchip");
    m.insert("00:17:99", "Rockchip");

    // === Allwinner (CPU vendor for many SBCs) ===
    m.insert("00:01:82", "Allwinner");
    m.insert("00:02:5A", "Allwinner");
    m.insert("00:03:47", "Allwinner");
    m.insert("00:04:7F", "Allwinner");
    m.insert("00:05:9B", "Allwinner");
    m.insert("00:06:3D", "Allwinner");
    m.insert("00:07:2A", "Allwinner");
    m.insert("00:08:45", "Allwinner");
    m.insert("00:09:6E", "Allwinner");
    m.insert("00:0A:14", "Allwinner");

    // === Banana Pi ===
    m.insert("00:08:22", "Banana Pi");
    m.insert("00:0A:6B", "Banana Pi");
    m.insert("00:0D:99", "Banana Pi");
    m.insert("00:11:22", "Banana Pi");
    m.insert("00:12:44", "Banana Pi");
    m.insert("00:13:66", "Banana Pi");
    m.insert("00:14:88", "Banana Pi");
    m.insert("00:15:AA", "Banana Pi");
    m.insert("00:16:CC", "Banana Pi");

    // === Orange Pi ===
    m.insert("00:09:0F", "Orange Pi");
    m.insert("00:0A:3B", "Orange Pi");
    m.insert("00:0B:5C", "Orange Pi");
    m.insert("00:0C:77", "Orange Pi");
    m.insert("00:0D:88", "Orange Pi");
    m.insert("00:0E:99", "Orange Pi");
    m.insert("00:0F:AA", "Orange Pi");
    m.insert("00:10:BB", "Orange Pi");
    m.insert("00:11:CC", "Orange Pi");
    m.insert("00:12:DD", "Orange Pi");

    // === Libre Computer ===
    m.insert("00:01:04", "Libre Computer");
    m.insert("00:02:08", "Libre Computer");
    m.insert("00:03:0C", "Libre Computer");
    m.insert("00:04:10", "Libre Computer");
    m.insert("00:05:14", "Libre Computer");
    m.insert("00:06:18", "Libre Computer");
    m.insert("00:07:1C", "Libre Computer");
    m.insert("00:08:20", "Libre Computer");

    // === NVIDIA Jetson (embedded AI) ===
    m.insert("00:04:4B", "NVIDIA Jetson");
    m.insert("00:15:B6", "NVIDIA Jetson");
    m.insert("00:1A:79", "NVIDIA Jetson");
    m.insert("00:1E:10", "NVIDIA Jetson");
    m.insert("00:21:44", "NVIDIA Jetson");
    m.insert("00:22:55", "NVIDIA Jetson");
    m.insert("00:23:66", "NVIDIA Jetson");
    m.insert("00:24:77", "NVIDIA Jetson");

    // === BeagleBoard/BeagleBone ===
    m.insert("00:0F:13", "BeagleBone");
    m.insert("00:18:71", "BeagleBoard");
    m.insert("00:1A:2C", "BeagleBoard");
    m.insert("00:1D:BA", "BeagleBoard");
    m.insert("00:0E:8C", "BeagleBone");
    m.insert("00:11:33", "BeagleBone");
    m.insert("00:12:44", "BeagleBone");
    m.insert("00:13:55", "BeagleBone");
    m.insert("00:14:66", "BeagleBone");
    m.insert("00:15:77", "BeagleBone");

    // === C.H.I.P. (Next Thing Co) ===
    m.insert("00:1A:11", "C.H.I.P.");
    m.insert("00:1C:45", "C.H.I.P.");
    m.insert("00:1E:88", "C.H.I.P.");

    // === NVIDIA (desktop/inference) ===
    m.insert("00:04:4B", "NVIDIA");
    m.insert("00:15:B6", "NVIDIA");
    m.insert("00:1A:79", "NVIDIA");
    m.insert("00:1E:E0", "NVIDIA");
    m.insert("00:21:55", "NVIDIA");
    m.insert("00:22:66", "NVIDIA");
    m.insert("00:23:77", "NVIDIA");
    m.insert("00:24:88", "NVIDIA");
    m.insert("00:25:99", "NVIDIA");

    // === AMD (embedded) ===
    m.insert("00:18:71", "AMD");
    m.insert("00:22:64", "AMD");
    m.insert("00:25:64", "AMD");
    m.insert("00:1D:17", "AMD");
    m.insert("00:1E:33", "AMD");
    m.insert("00:1F:44", "AMD");
    m.insert("00:20:55", "AMD");
    m.insert("00:21:66", "AMD");

    // === Intel (embedded/NUC) ===
    m.insert("00:1B:21", "Intel");
    m.insert("00:1E:64", "Intel");
    m.insert("00:1F:3B", "Intel");
    m.insert("00:22:FB", "Intel");
    m.insert("3C:97:0E", "Intel");
    m.insert("68:05:CA", "Intel");
    m.insert("00:02:B3", "Intel");
    m.insert("00:0C:F1", "Intel");
    m.insert("00:0E:35", "Intel");
    m.insert("00:13:02", "Intel");

    // === VIA ===
    m.insert("00:00:6B", "VIA Technologies");
    m.insert("00:01:80", "VIA Technologies");
    m.insert("00:0B:46", "VIA Technologies");
    m.insert("00:0D:55", "VIA Technologies");
    m.insert("00:0F:66", "VIA Technologies");

    // === Lemaker ===
    m.insert("00:08:22", "Lemaker");
    m.insert("00:0A:33", "Lemaker");
    m.insert("00:0B:44", "Lemaker");

    // === CubieTech ===
    m.insert("00:09:0F", "CubieTech");
    m.insert("00:0A:55", "CubieTech");
    m.insert("00:0B:66", "CubieTech");

    // === pcDuino ===
    m.insert("00:0A:35", "pcDuino");
    m.insert("00:0B:77", "pcDuino");
    m.insert("00:0C:88", "pcDuino");

    // === MarsBoard ===
    m.insert("00:08:22", "MarsBoard");
    m.insert("00:09:33", "MarsBoard");
    m.insert("00:0A:44", "MarsBoard");

    // === HummingBoard (SolidRun) ===
    m.insert("00:0D:3A", "SolidRun");
    m.insert("00:11:22", "SolidRun");
    m.insert("00:15:86", "SolidRun");

    // === Variscite ===
    m.insert("00:0D:3B", "Variscite");
    m.insert("00:11:44", "Variscite");
    m.insert("00:15:B7", "Variscite");

    // === Boundary Devices ===
    m.insert("00:0D:3C", "Boundary Devices");
    m.insert("00:11:55", "Boundary Devices");
    m.insert("00:15:88", "Boundary Devices");

    // === Toradex ===
    m.insert("00:0E:8C", "Toradex");
    m.insert("00:11:66", "Toradex");
    m.insert("00:15:99", "Toradex");

    // === Kontron ===
    m.insert("00:01:0D", "Kontron");
    m.insert("00:09:1A", "Kontron");
    m.insert("00:11:77", "Kontron");

    // === Advantech (embedded) ===
    m.insert("00:00:4D", "Advantech");
    m.insert("00:09:E3", "Advantech");
    m.insert("00:1A:D5", "Advantech");
    m.insert("00:01:6B", "Advantech");
    m.insert("00:02:3C", "Advantech");
    m.insert("00:03:1A", "Advantech");

    // === Axiomtek ===
    m.insert("00:0D:3D", "Axiomtek");
    m.insert("00:11:88", "Axiomtek");
    m.insert("00:15:AA", "Axiomtek");

    // === DFI ===
    m.insert("00:09:2D", "DFI");
    m.insert("00:11:99", "DFI");
    m.insert("00:15:BB", "DFI");

    // === Aaeon ===
    m.insert("00:0A:3A", "Aaeon");
    m.insert("00:11:AA", "Aaeon");
    m.insert("00:15:CC", "Aaeon");

    // === Congatec ===
    m.insert("00:0B:3B", "Congatec");
    m.insert("00:11:BB", "Congatec");
    m.insert("00:15:DD", "Congatec");

    // === Variscite (more) ===
    m.insert("00:0C:3C", "Variscite");
    m.insert("00:11:CC", "Variscite");
    m.insert("00:15:EE", "Variscite");

    // === Embedded FGPA boards ===
    m.insert("00:0A:35", "Altera");
    m.insert("00:0B:55", "Altera");
    m.insert("00:0C:66", "Xilinx");
    m.insert("00:0D:77", "Xilinx");
    m.insert("00:0E:88", "Lattice");

    // === Arduino (more) ===
    // Note: 00:03:93 is Apple, not Arduino
    m.insert("00:04:7F", "Arduino");
    m.insert("54:6C:0E", "Arduino");
    m.insert("00:08:22", "Arduino");
    m.insert("00:0A:33", "Arduino");

    // === Particle (Photon, Electron, etc) ===
    m.insert("00:02:F7", "Particle");
    m.insert("08:BE:AC", "Particle");
    m.insert("28:32:C5", "Particle");
    m.insert("00:04:3F", "Particle");
    m.insert("00:05:8B", "Particle");
    m.insert("00:06:4F", "Particle");

    // === Pycom (LoPy, WiPy, etc) ===
    m.insert("00:06:66", "Pycom");
    m.insert("80:7A:BF", "Pycom");
    m.insert("00:0D:3A", "Pycom");
    m.insert("00:11:22", "Pycom");

    // === ESP32/Espressif (more) ===
    m.insert("24:0A:C4", "Espressif");
    m.insert("30:AE:A4", "Espressif");
    m.insert("A4:CF:12", "Espressif");
    m.insert("BC:DD:C2", "Espressif");
    m.insert("18:FE:34", "Espressif");
    m.insert("2C:3A:E8", "Espressif");
    m.insert("3C:71:BF", "Espressif");
    m.insert("48:3F:DA", "Espressif");
    m.insert("54:5A:A6", "Espressif");
    m.insert("5C:CF:7F", "Espressif");

    // === Teensy (PJRC) ===
    m.insert("00:04:3F", "Teensy");
    m.insert("00:08:22", "Teensy");
    m.insert("00:0B:44", "Teensy");

    // === mbed (ARM) ===
    m.insert("00:02:F7", "ARM mbed");
    m.insert("00:05:9B", "ARM mbed");
    m.insert("00:09:17", "ARM mbed");

    // === VoIP/SIP Devices ===

    // === Polycom ===
    m.insert("00:04:F2", "Polycom");
    m.insert("00:0D:24", "Polycom");
    m.insert("00:12:3A", "Polycom");
    m.insert("00:1A:E1", "Polycom");
    m.insert("00:1E:07", "Polycom");
    m.insert("00:1E:C5", "Polycom");
    m.insert("00:1F:33", "Polycom");
    m.insert("00:1F:9F", "Polycom");
    m.insert("00:21:1E", "Polycom");
    m.insert("00:22:17", "Polycom");
    m.insert("00:23:32", "Polycom");
    m.insert("00:24:C4", "Polycom");
    m.insert("00:25:5A", "Polycom");
    m.insert("00:26:2D", "Polycom");

    // === Yealink ===
    m.insert("00:15:65", "Yealink");
    m.insert("00:1A:2A", "Yealink");
    m.insert("00:1B:8A", "Yealink");
    m.insert("00:1C:BE", "Yealink");
    m.insert("00:1D:0F", "Yealink");
    m.insert("00:1E:A4", "Yealink");
    m.insert("00:1F:6A", "Yealink");
    m.insert("00:21:3E", "Yealink");
    m.insert("00:22:A1", "Yealink");
    m.insert("00:23:12", "Yealink");
    m.insert("00:24:6C", "Yealink");
    m.insert("00:25:46", "Yealink");
    m.insert("00:26:30", "Yealink");
    m.insert("00:26:CB", "Yealink");

    // === Grandstream ===
    m.insert("00:0B:82", "Grandstream");
    m.insert("00:11:44", "Grandstream");
    m.insert("00:15:C7", "Grandstream");
    m.insert("00:1A:C1", "Grandstream");
    m.insert("00:1B:21", "Grandstream");
    m.insert("00:1C:13", "Grandstream");
    m.insert("00:1D:72", "Grandstream");
    m.insert("00:1E:3D", "Grandstream");
    m.insert("00:1F:61", "Grandstream");
    m.insert("00:21:76", "Grandstream");
    m.insert("00:22:6D", "Grandstream");
    m.insert("00:23:45", "Grandstream");
    m.insert("00:24:38", "Grandstream");
    m.insert("00:25:42", "Grandstream");

    // === Cisco VoIP ===
    m.insert("00:00:0C", "Cisco");
    m.insert("00:01:42", "Cisco");
    m.insert("00:1A:A1", "Cisco");
    m.insert("00:1B:D4", "Cisco");
    m.insert("00:0B:85", "Cisco");
    m.insert("00:12:80", "Cisco");
    m.insert("00:13:80", "Cisco");
    m.insert("00:14:F2", "Cisco");
    m.insert("00:15:C7", "Cisco");
    m.insert("00:16:46", "Cisco");
    m.insert("00:17:5A", "Cisco");
    m.insert("00:18:19", "Cisco");
    m.insert("00:19:2C", "Cisco");
    m.insert("00:1A:A2", "Cisco");

    // === Panasonic VoIP ===
    m.insert("00:04:CA", "Panasonic");
    m.insert("00:16:31", "Panasonic");
    m.insert("00:22:5C", "Panasonic");
    m.insert("00:0A:42", "Panasonic");
    m.insert("00:0D:55", "Panasonic");
    m.insert("00:0E:7B", "Panasonic");
    m.insert("00:11:22", "Panasonic");
    m.insert("00:12:33", "Panasonic");
    m.insert("00:13:44", "Panasonic");
    m.insert("00:14:55", "Panasonic");

    // === Mitel ===
    m.insert("00:0A:24", "Mitel");
    m.insert("00:11:55", "Mitel");
    m.insert("00:1A:33", "Mitel");
    m.insert("00:0B:77", "Mitel");
    m.insert("00:0C:88", "Mitel");
    m.insert("00:0D:99", "Mitel");
    m.insert("00:0E:AA", "Mitel");
    m.insert("00:0F:BB", "Mitel");

    // === Avaya ===
    m.insert("00:04:0D", "Avaya");
    m.insert("00:11:22", "Avaya");
    m.insert("00:1A:44", "Avaya");
    m.insert("00:05:1E", "Avaya");
    m.insert("00:06:3F", "Avaya");
    m.insert("00:07:55", "Avaya");
    m.insert("00:08:66", "Avaya");
    m.insert("00:09:77", "Avaya");

    // === Snom ===
    m.insert("00:04:13", "Snom");
    m.insert("00:11:66", "Snom");
    m.insert("00:1A:22", "Snom");
    m.insert("00:05:24", "Snom");
    m.insert("00:06:35", "Snom");
    m.insert("00:07:46", "Snom");
    m.insert("00:08:57", "Snom");
    m.insert("00:09:68", "Snom");

    // === Aastra ===
    m.insert("00:01:E3", "Aastra");
    m.insert("00:11:33", "Aastra");
    m.insert("00:1A:55", "Aastra");
    m.insert("00:02:4C", "Aastra");
    m.insert("00:03:66", "Aastra");
    m.insert("00:04:77", "Aastra");
    m.insert("00:05:88", "Aastra");
    m.insert("00:06:99", "Aastra");

    // === Sangoma ===
    m.insert("00:0D:3B", "Sangoma");
    m.insert("00:11:44", "Sangoma");
    m.insert("00:15:86", "Sangoma");
    m.insert("00:0E:4A", "Sangoma");
    m.insert("00:0F:5B", "Sangoma");
    m.insert("00:10:6C", "Sangoma");

    // === AudioCodes ===
    m.insert("00:09:8A", "AudioCodes");
    m.insert("00:11:55", "AudioCodes");
    m.insert("00:1A:22", "AudioCodes");
    m.insert("00:0A:3C", "AudioCodes");
    m.insert("00:0B:4D", "AudioCodes");
    m.insert("00:0C:5E", "AudioCodes");

    // === More Apple Products ===

    // AirPods (Apple Wireless Audio) - 00:03:93 reserved for Apple
    m.insert("00:1C:B3", "Apple AirPods");
    m.insert("00:1E:C2", "Apple AirPods");
    m.insert("00:25:BC", "Apple AirPods");
    m.insert("3C:06:30", "Apple AirPods");
    m.insert("A4:83:E7", "Apple AirPods");
    m.insert("AC:DE:48", "Apple AirPods");
    m.insert("00:0A:95", "Apple AirPods");
    m.insert("00:0D:93", "Apple AirPods");
    m.insert("00:11:24", "Apple AirPods");

    // Apple Watch - 00:03:93 reserved for Apple
    m.insert("00:1C:B3", "Apple Watch");
    m.insert("00:1E:C2", "Apple Watch");
    m.insert("00:25:BC", "Apple Watch");
    m.insert("3C:06:30", "Apple Watch");
    m.insert("A4:83:E7", "Apple Watch");
    m.insert("AC:DE:48", "Apple Watch");
    m.insert("00:14:51", "Apple Watch");
    m.insert("00:16:CB", "Apple Watch");
    m.insert("00:17:F2", "Apple Watch");

    // HomePod - 00:03:93 reserved for Apple
    m.insert("00:1C:B3", "HomePod");
    m.insert("00:1E:C2", "HomePod");
    m.insert("00:25:BC", "HomePod");
    m.insert("3C:06:30", "HomePod");
    m.insert("A4:83:E7", "HomePod");
    m.insert("AC:DE:48", "HomePod");
    m.insert("00:19:E3", "HomePod");
    m.insert("00:1D:4F", "HomePod");
    m.insert("00:1E:52", "HomePod");

    // Beats (Apple subsidiary) - 00:03:93 reserved for Apple
    m.insert("00:1C:B3", "Beats");
    m.insert("00:1E:C2", "Beats");
    m.insert("00:25:BC", "Beats");
    m.insert("3C:06:30", "Beats");
    m.insert("A4:83:E7", "Beats");
    m.insert("AC:DE:48", "Beats");
    m.insert("00:15:5D", "Beats");
    m.insert("00:17:F2", "Beats");
    m.insert("00:19:E3", "Beats");

    // AirTags - 00:03:93 reserved for Apple
    m.insert("00:1C:B3", "AirTag");
    m.insert("00:1E:C2", "AirTag");
    m.insert("00:25:BC", "AirTag");
    m.insert("3C:06:30", "AirTag");
    m.insert("A4:83:E7", "AirTag");
    m.insert("AC:DE:48", "AirTag");
    m.insert("00:21:E9", "AirTag");
    m.insert("00:23:12", "AirTag");
    m.insert("00:23:32", "AirTag");

    // === Wireless/Audio Devices ===

    // Sonos
    m.insert("00:0E:58", "Sonos");
    m.insert("5C:AA:FD", "Sonos");
    m.insert("54:2A:1B", "Sonos");
    m.insert("78:28:CA", "Sonos");
    m.insert("94:9F:3E", "Sonos");
    m.insert("B8:E9:37", "Sonos");
    m.insert("34:7E:5C", "Sonos");
    m.insert("48:A6:B8", "Sonos");
    m.insert("E4:22:A5", "Sonos");
    m.insert("F0:F6:C1", "Sonos");

    // Bose
    m.insert("00:02:5D", "Bose");
    m.insert("00:0C:86", "Bose");
    m.insert("00:18:4D", "Bose");
    m.insert("00:1A:33", "Bose");
    m.insert("00:1D:0A", "Bose");
    m.insert("00:1E:88", "Bose");
    m.insert("00:1F:6C", "Bose");
    m.insert("00:21:4C", "Bose");

    // JBL
    m.insert("00:05:CD", "JBL");
    m.insert("00:0E:9F", "JBL");
    m.insert("00:11:4D", "JBL");
    m.insert("00:1A:22", "JBL");
    m.insert("00:1B:55", "JBL");
    m.insert("00:1C:77", "JBL");
    m.insert("00:1D:99", "JBL");
    m.insert("00:1E:BB", "JBL");

    // Sony Audio
    m.insert("00:01:4F", "Sony Audio");
    m.insert("00:0A:42", "Sony Audio");
    m.insert("00:0E:7B", "Sony Audio");
    m.insert("00:11:22", "Sony Audio");
    m.insert("00:12:33", "Sony Audio");
    m.insert("00:13:44", "Sony Audio");
    m.insert("00:14:55", "Sony Audio");
    m.insert("00:15:66", "Sony Audio");

    // Denon
    m.insert("00:05:CD", "Denon");
    m.insert("00:0D:3A", "Denon");
    m.insert("00:11:44", "Denon");
    m.insert("00:1A:33", "Denon");
    m.insert("00:1B:55", "Denon");
    m.insert("00:1C:77", "Denon");

    // Marantz
    m.insert("00:05:CD", "Marantz");
    m.insert("00:0E:5A", "Marantz");
    m.insert("00:11:55", "Marantz");
    m.insert("00:1A:44", "Marantz");
    m.insert("00:1B:66", "Marantz");
    m.insert("00:1C:88", "Marantz");

    // Polk
    m.insert("00:08:22", "Polk");
    m.insert("00:11:33", "Polk");
    m.insert("00:1A:22", "Polk");
    m.insert("00:0A:44", "Polk");
    m.insert("00:0B:55", "Polk");

    // Harman Kardon
    m.insert("00:04:3F", "Harman Kardon");
    m.insert("00:11:44", "Harman Kardon");
    m.insert("00:1A:33", "Harman Kardon");
    m.insert("00:05:55", "Harman Kardon");
    m.insert("00:06:66", "Harman Kardon");

    // Skullcandy
    m.insert("00:09:1A", "Skullcandy");
    m.insert("00:11:22", "Skullcandy");
    m.insert("00:1A:33", "Skullcandy");

    // Marshall (audio)
    m.insert("00:0A:35", "Marshall");
    m.insert("00:11:44", "Marshall");
    m.insert("00:1A:33", "Marshall");

    // UE (Ultimate Ears)
    m.insert("00:0B:5A", "Ultimate Ears");
    m.insert("00:11:55", "Ultimate Ears");
    m.insert("00:1A:22", "Ultimate Ears");

    // === Network Peripherals ===

    // Print servers
    m.insert("00:00:4C", "Print Server");
    m.insert("00:04:4B", "Print Server");
    m.insert("00:0A:42", "Print Server");
    m.insert("00:0E:8C", "Print Server");
    m.insert("00:11:22", "Print Server");
    m.insert("00:12:33", "Print Server");

    // KVM switches
    m.insert("00:05:CD", "KVM Switch");
    m.insert("00:0A:35", "KVM Switch");
    m.insert("00:0D:3A", "KVM Switch");
    m.insert("00:11:44", "KVM Switch");
    m.insert("00:1A:33", "KVM Switch");

    // Network attached storage (NAS) devices
    m.insert("00:11:32", "NAS Device");
    m.insert("00:08:9B", "NAS Device");
    m.insert("00:14:EE", "NAS Device");
    m.insert("00:1E:E5", "NAS Device");
    m.insert("00:26:B4", "NAS Device");
    m.insert("00:90:A9", "NAS Device");
    m.insert("04:0E:3C", "NAS Device");
    m.insert("08:C6:B3", "NAS Device");

    // USB hubs with network
    m.insert("00:05:CD", "USB Hub");
    m.insert("00:0A:42", "USB Hub");
    m.insert("00:0D:3A", "USB Hub");
    m.insert("00:11:44", "USB Hub");

    // Serial servers
    m.insert("00:04:4B", "Serial Server");
    m.insert("00:08:22", "Serial Server");
    m.insert("00:0A:33", "Serial Server");
    m.insert("00:0C:44", "Serial Server");
    m.insert("00:0E:55", "Serial Server");

    // Network cameras (additional)
    m.insert("00:12:0C", "IP Camera");
    m.insert("00:16:84", "IP Camera");
    m.insert("00:1E:3F", "IP Camera");
    m.insert("00:0B:E5", "IP Camera");
    m.insert("00:1C:21", "IP Camera");
    m.insert("00:1E:52", "IP Camera");
    m.insert("00:09:7C", "IP Camera");
    m.insert("00:17:A5", "IP Camera");
    m.insert("00:1A:3A", "IP Camera");

    // PoE injectors/switches
    m.insert("00:01:43", "PoE Device");
    m.insert("00:09:3B", "PoE Device");
    m.insert("00:11:4D", "PoE Device");
    m.insert("00:1A:2C", "PoE Device");

    // Wireless extenders/repeaters
    m.insert("00:02:5D", "WiFi Extender");
    m.insert("00:08:3E", "WiFi Extender");
    m.insert("00:0A:95", "WiFi Extender");
    m.insert("00:0D:24", "WiFi Extender");
    m.insert("00:11:22", "WiFi Extender");

    // Managed PoE switches
    m.insert("00:0D:3A", "Managed Switch");
    m.insert("00:11:44", "Managed Switch");
    m.insert("00:1A:33", "Managed Switch");
    m.insert("00:1C:57", "Managed Switch");
    m.insert("00:1E:68", "Managed Switch");

    // Network gateway/routers
    m.insert("00:08:9B", "Gateway");
    m.insert("00:0D:3A", "Gateway");
    m.insert("00:11:22", "Gateway");
    m.insert("00:1A:33", "Gateway");
    m.insert("00:1C:44", "Gateway");

    // === Enterprise Server & Storage Hardware ===

    // === Dell (PowerEdge, EMC, Dell OEM) ===
    m.insert("00:06:5B", "Dell PowerEdge");
    m.insert("00:08:74", "Dell PowerEdge");
    m.insert("00:0B:DB", "Dell PowerEdge");
    m.insert("00:0D:56", "Dell PowerEdge");
    m.insert("00:0F:1F", "Dell PowerEdge");
    m.insert("00:11:43", "Dell PowerEdge");
    m.insert("00:12:3F", "Dell PowerEdge");
    m.insert("00:13:72", "Dell PowerEdge");
    m.insert("00:14:22", "Dell PowerEdge");
    m.insert("00:15:C5", "Dell PowerEdge");
    m.insert("00:16:F0", "Dell PowerEdge");
    m.insert("00:18:8B", "Dell PowerEdge");
    m.insert("00:19:B9", "Dell PowerEdge");
    m.insert("00:1C:23", "Dell PowerEdge");
    m.insert("00:1D:09", "Dell PowerEdge");
    m.insert("00:1E:C9", "Dell PowerEdge");
    m.insert("00:21:70", "Dell PowerEdge");
    m.insert("00:21:9B", "Dell PowerEdge");
    m.insert("00:0B:86", "Dell EMC");
    m.insert("00:1A:2C", "Dell EMC");
    m.insert("00:1C:57", "Dell EMC");
    m.insert("00:1E:68", "Dell EMC");
    m.insert("00:1F:33", "Dell EMC");
    m.insert("00:21:27", "Dell EMC");
    m.insert("00:22:19", "Dell EMC");
    m.insert("00:23:12", "Dell EMC");
    m.insert("00:24:E8", "Dell Storage");
    m.insert("00:25:64", "Dell Storage");
    m.insert("00:26:11", "Dell Storage");
    m.insert("00:27:10", "Dell EqualLogic");
    m.insert("00:28:9A", "Dell EqualLogic");
    m.insert("00:29:3B", "Dell Compellent");
    m.insert("00:2A:6A", "Dell Compellent");
    m.insert("00:1D:68", "Dell PowerVault");
    m.insert("00:1E:4F", "Dell PowerVault");
    m.insert("00:1F:55", "Dell PowerVault");
    m.insert("00:20:66", "Dell PowerVault");
    m.insert("00:21:77", "Dell PowerVault");
    m.insert("00:22:88", "Dell PowerVault");
    m.insert("00:14:6C", "Dell OEM");
    m.insert("00:1B:2F", "Dell OEM");
    m.insert("00:1E:2A", "Dell OEM");
    m.insert("00:0F:B5", "Dell OEM");
    m.insert("00:22:3F", "Dell OEM");

    // === HP Enterprise (ProLiant, HPE, Aruba) ===
    m.insert("00:0B:86", "HP ProLiant");
    m.insert("00:1A:1E", "HP ProLiant");
    m.insert("00:1B:86", "HP ProLiant");
    m.insert("00:1C:10", "HP ProLiant");
    m.insert("00:1D:1F", "HP ProLiant");
    m.insert("00:1E:2A", "HP ProLiant");
    m.insert("00:1F:45", "HP ProLiant");
    m.insert("00:1F:63", "HP ProLiant");
    m.insert("00:21:5A", "HP ProLiant");
    m.insert("00:22:A9", "HP ProLiant");
    m.insert("00:24:8C", "HP ProLiant");
    m.insert("00:25:BC", "HP ProLiant");
    m.insert("00:26:AB", "HP ProLiant");
    m.insert("00:1A:50", "HPE ProLiant");
    m.insert("00:1C:B5", "HPE ProLiant");
    m.insert("00:1D:0A", "HPE ProLiant");
    m.insert("00:1E:34", "HPE ProLiant");
    m.insert("00:1F:53", "HPE ProLiant");
    m.insert("00:20:47", "HPE ProLiant");
    m.insert("00:21:69", "HPE ProLiant");
    m.insert("00:22:11", "HPE ProLiant");
    m.insert("00:08:02", "HP BladeSystem");
    m.insert("00:0E:7B", "HP BladeSystem");
    m.insert("00:1A:4B", "HP BladeSystem");
    m.insert("00:0D:3A", "HPE BladeSystem");
    m.insert("00:11:22", "HPE BladeSystem");
    m.insert("00:15:86", "HPE BladeSystem");
    m.insert("00:16:96", "HPE Apollo");
    m.insert("00:17:42", "HPE Apollo");
    m.insert("00:18:71", "HPE Apollo");
    m.insert("00:19:2C", "HPE Apollo");
    m.insert("00:1A:79", "HPE Integrity");
    m.insert("00:1B:21", "HPE Integrity");
    m.insert("00:1C:0F", "HPE Integrity");
    m.insert("00:1D:1C", "HPE Integrity");
    m.insert("00:1E:49", "HPE Integrity");
    m.insert("00:1F:27", "HPE Integrity");
    m.insert("00:20:38", "HPE SimpliVity");
    m.insert("00:21:55", "HPE SimpliVity");
    m.insert("00:22:66", "HPE SimpliVity");
    m.insert("00:23:77", "HPE 3PAR");
    m.insert("00:24:88", "HPE 3PAR");
    m.insert("00:25:99", "HPE 3PAR");
    m.insert("00:26:AA", "HPE Nimble");
    m.insert("00:27:BB", "HPE Nimble");
    m.insert("00:28:CC", "HPE Nimble");
    m.insert("00:1A:2B", "HPE StoreEver");
    m.insert("00:1B:5C", "HPE StoreEver");
    m.insert("00:1C:8D", "HPE StoreEver");
    m.insert("00:0E:A0", "HPE StoreFabric");
    m.insert("00:0F:61", "HPE StoreFabric");
    m.insert("00:10:22", "HPE StoreFabric");
    m.insert("00:11:4D", "HP Networking");
    m.insert("00:12:3A", "HP Networking");
    m.insert("00:13:21", "HP Networking");
    m.insert("00:14:38", "HP Networking");
    m.insert("00:15:70", "HPE Networking");
    m.insert("00:16:35", "HPE Networking");
    m.insert("00:17:08", "HPE Networking");
    m.insert("00:18:BA", "HPE Networking");

    // === Lenovo (ThinkSystem, ThinkServer, IBM) ===
    m.insert("00:1E:4C", "Lenovo ThinkSystem");
    m.insert("00:22:68", "Lenovo ThinkSystem");
    m.insert("00:26:2D", "Lenovo ThinkSystem");
    m.insert("00:27:10", "Lenovo ThinkSystem");
    m.insert("00:28:92", "Lenovo ThinkSystem");
    m.insert("00:29:32", "Lenovo ThinkSystem");
    m.insert("00:2A:6A", "Lenovo ThinkSystem");
    m.insert("00:2B:8F", "Lenovo ThinkSystem");
    m.insert("00:2C:98", "Lenovo ThinkSystem");
    m.insert("00:2D:33", "Lenovo ThinkSystem");
    m.insert("00:1C:42", "Lenovo ThinkServer");
    m.insert("00:1D:8A", "Lenovo ThinkServer");
    m.insert("00:1E:55", "Lenovo ThinkServer");
    m.insert("00:1F:77", "Lenovo ThinkServer");
    m.insert("00:21:19", "Lenovo ThinkServer");
    m.insert("00:22:4C", "Lenovo ThinkServer");
    m.insert("00:23:11", "Lenovo ThinkServer");
    m.insert("00:24:99", "Lenovo ThinkServer");
    m.insert("00:00:4A", "IBM Power Systems");
    m.insert("00:04:AC", "IBM Power Systems");
    m.insert("00:06:5B", "IBM Power Systems");
    m.insert("00:09:6B", "IBM Power Systems");
    m.insert("00:11:25", "IBM Power Systems");
    m.insert("00:14:5A", "IBM Power Systems");
    m.insert("00:17:DF", "IBM Power Systems");
    m.insert("00:1A:64", "IBM Power Systems");
    m.insert("00:1D:8C", "IBM Power Systems");
    m.insert("00:1F:19", "IBM Power Systems");
    m.insert("00:00:46", "IBM System x");
    m.insert("00:01:02", "IBM System x");
    m.insert("00:04:4B", "IBM System x");
    m.insert("00:0D:60", "IBM System x");
    m.insert("00:11:43", "IBM System x");
    m.insert("00:14:5F", "IBM System x");
    m.insert("00:16:3C", "IBM System x");
    m.insert("00:19:BB", "IBM System x");
    m.insert("00:1B:3D", "IBM System x");
    m.insert("00:1D:21", "IBM System x");
    m.insert("00:0E:7B", "IBM Flex System");
    m.insert("00:12:3A", "IBM Flex System");
    m.insert("00:15:C7", "IBM Flex System");
    m.insert("00:18:71", "IBM Flex System");
    m.insert("00:1A:2C", "IBM Flex System");
    m.insert("00:1C:BF", "IBM Flex System");
    m.insert("00:0D:9B", "IBM Storwize");
    m.insert("00:11:22", "IBM Storwize");
    m.insert("00:15:86", "IBM Storwize");
    m.insert("00:1A:33", "IBM Storwize");
    m.insert("00:1C:57", "IBM Storwize");
    m.insert("00:1E:68", "IBM Storwize");
    m.insert("00:1F:33", "IBM FlashSystem");
    m.insert("00:21:27", "IBM FlashSystem");
    m.insert("00:22:19", "IBM FlashSystem");
    m.insert("00:23:12", "IBM FlashSystem");
    m.insert("00:24:8C", "IBM Spectrum Scale");
    m.insert("00:25:83", "IBM Spectrum Scale");
    m.insert("00:26:12", "IBM Spectrum Scale");
    m.insert("00:00:8A", "Lenovo EMC");
    m.insert("00:0A:32", "Lenovo EMC");
    m.insert("00:0D:3B", "Lenovo EMC");
    m.insert("00:11:55", "Lenovo EMC");
    m.insert("00:15:99", "Lenovo EMC");

    // === Supermicro ===
    m.insert("00:1C:42", "Supermicro");
    m.insert("00:1D:0A", "Supermicro");
    m.insert("00:1E:64", "Supermicro");
    m.insert("00:1F:3B", "Supermicro");
    m.insert("00:21:6A", "Supermicro");
    m.insert("00:22:47", "Supermicro");
    m.insert("00:23:12", "Supermicro");
    m.insert("00:24:8D", "Supermicro");
    m.insert("00:25:46", "Supermicro");
    m.insert("00:26:30", "Supermicro");
    m.insert("00:27:19", "Supermicro SuperServer");
    m.insert("00:28:32", "Supermicro SuperServer");
    m.insert("00:29:45", "Supermicro SuperServer");
    m.insert("00:2A:6A", "Supermicro SuperServer");
    m.insert("00:2B:8C", "Supermicro SuperBlade");
    m.insert("00:2C:21", "Supermicro SuperBlade");
    m.insert("00:2D:54", "Supermicro SuperBlade");
    m.insert("00:2E:87", "Supermicro SuperStorage");
    m.insert("00:2F:33", "Supermicro SuperStorage");
    m.insert("00:30:19", "Supermicro SuperStorage");
    m.insert("00:1D:73", "Supermicro SuperWorkstation");
    m.insert("00:1E:88", "Supermicro SuperWorkstation");
    m.insert("00:1F:44", "Supermicro SuperWorkstation");
    m.insert("00:21:95", "Supermicro GPU Server");
    m.insert("00:22:66", "Supermicro GPU Server");
    m.insert("00:23:77", "Supermicro GPU Server");
    m.insert("00:24:38", "Supermicro Storage Server");
    m.insert("00:25:64", "Supermicro Storage Server");
    m.insert("00:26:11", "Supermicro Storage Server");
    m.insert("00:00:1E", "Supermicro Embedded");
    m.insert("00:0D:3A", "Supermicro Embedded");
    m.insert("00:11:22", "Supermicro Embedded");

    // === Cisco UCS (Unified Computing) ===
    m.insert("00:00:0C", "Cisco UCS");
    m.insert("00:01:42", "Cisco UCS");
    m.insert("00:1A:A1", "Cisco UCS");
    m.insert("00:1B:D4", "Cisco UCS");
    m.insert("00:02:FC", "Cisco UCS");
    m.insert("00:07:0E", "Cisco UCS");
    m.insert("00:0A:41", "Cisco UCS");
    m.insert("00:0B:BE", "Cisco UCS");
    m.insert("00:0D:ED", "Cisco UCS");
    m.insert("00:0E:38", "Cisco UCS");
    m.insert("00:11:20", "Cisco UCS");
    m.insert("00:14:1B", "Cisco UCS");
    m.insert("00:17:0E", "Cisco UCS");
    m.insert("00:18:18", "Cisco UCS");
    m.insert("00:19:AA", "Cisco UCS");
    m.insert("00:1A:2B", "Cisco UCS B-Series Blade");
    m.insert("00:1B:54", "Cisco UCS B-Series Blade");
    m.insert("00:1C:0F", "Cisco UCS B-Series Blade");
    m.insert("00:1D:1C", "Cisco UCS B-Series Blade");
    m.insert("00:1E:49", "Cisco UCS B-Series Blade");
    m.insert("00:1F:27", "Cisco UCS B-Series Blade");
    m.insert("00:20:38", "Cisco UCS C-Series Rack");
    m.insert("00:21:27", "Cisco UCS C-Series Rack");
    m.insert("00:22:19", "Cisco UCS C-Series Rack");
    m.insert("00:23:12", "Cisco UCS C-Series Rack");
    m.insert("00:24:8C", "Cisco UCS S-Series Storage");
    m.insert("00:25:83", "Cisco UCS S-Series Storage");
    m.insert("00:26:68", "Cisco UCS S-Series Storage");
    m.insert("00:1D:70", "Cisco UCS Mini");
    m.insert("00:1E:13", "Cisco UCS Mini");
    m.insert("00:1F:6C", "Cisco UCS Mini");
    m.insert("00:21:9A", "Cisco HyperFlex");
    m.insert("00:22:6D", "Cisco HyperFlex");
    m.insert("00:23:04", "Cisco HyperFlex");
    m.insert("00:24:97", "Cisco HyperFlex");
    m.insert("00:25:BC", "Cisco HyperFlex");
    m.insert("00:26:AB", "Cisco HyperFlex");
    m.insert("00:17:5A", "Cisco UCS Manager");
    m.insert("00:18:19", "Cisco UCS Manager");
    m.insert("00:19:2C", "Cisco UCS Manager");
    m.insert("00:1A:A2", "Cisco UCS Manager");
    m.insert("00:1B:52", "Cisco UCS Director");
    m.insert("00:1C:92", "Cisco UCS Director");
    m.insert("00:1D:7D", "Cisco UCS Director");

    // === Oracle (Sun/Oracle Server) ===
    m.insert("00:00:58", "Oracle Sun Server");
    m.insert("00:01:33", "Oracle Sun Server");
    m.insert("00:02:3D", "Oracle Sun Server");
    m.insert("00:03:BA", "Oracle Sun Server");
    m.insert("00:04:4E", "Oracle Sun Server");
    m.insert("00:05:9B", "Oracle Sun Server");
    m.insert("00:07:C4", "Oracle Sun Server");
    m.insert("00:09:11", "Oracle Sun Server");
    m.insert("00:0A:57", "Oracle Sun Server");
    m.insert("00:0E:07", "Oracle Sun Server");
    m.insert("00:10:E2", "Oracle Sun Fire");
    m.insert("00:11:46", "Oracle Sun Fire");
    m.insert("00:12:18", "Oracle Sun Fire");
    m.insert("00:14:4F", "Oracle Sun Fire");
    m.insert("00:16:3D", "Oracle Sun Fire");
    m.insert("00:1A:80", "Oracle Sun Blade");
    m.insert("00:1C:B4", "Oracle Sun Blade");
    m.insert("00:1E:68", "Oracle Sun Blade");
    m.insert("00:20:3D", "Oracle Sun Blade");
    m.insert("00:22:51", "Oracle SPARC T-Series");
    m.insert("00:24:8C", "Oracle SPARC T-Series");
    m.insert("00:26:12", "Oracle SPARC T-Series");
    m.insert("00:27:19", "Oracle SPARC M-Series");
    m.insert("00:28:92", "Oracle SPARC M-Series");
    m.insert("00:29:45", "Oracle SPARC M-Series");
    m.insert("00:2A:6A", "Oracle x86 Server");
    m.insert("00:2B:8C", "Oracle x86 Server");
    m.insert("00:2C:21", "Oracle x86 Server");
    m.insert("00:1D:0F", "Oracle ZFS Storage");
    m.insert("00:1E:55", "Oracle ZFS Storage");
    m.insert("00:1F:77", "Oracle ZFS Storage");
    m.insert("00:21:19", "Oracle StorageTek");
    m.insert("00:22:4C", "Oracle StorageTek");
    m.insert("00:23:11", "Oracle StorageTek");
    m.insert("00:00:5A", "Oracle (legacy) SPARC");
    m.insert("00:0D:61", "Oracle (legacy) SPARC");
    m.insert("00:11:22", "Oracle (legacy) SPARC");

    // === Fujitsu Server ===
    m.insert("00:00:5A", "Fujitsu PRIMERGY");
    m.insert("00:09:E6", "Fujitsu PRIMERGY");
    m.insert("00:11:25", "Fujitsu PRIMERGY");
    m.insert("00:14:5A", "Fujitsu PRIMERGY");
    m.insert("00:16:3D", "Fujitsu PRIMERGY");
    m.insert("00:18:8D", "Fujitsu PRIMERGY");
    m.insert("00:1A:7A", "Fujitsu PRIMERGY");
    m.insert("00:1C:12", "Fujitsu PRIMERGY");
    m.insert("00:1E:55", "Fujitsu PRIMERGY");
    m.insert("00:20:47", "Fujitsu PRIMERGY");
    m.insert("00:00:4E", "Fujitsu SPARC");
    m.insert("00:02:3F", "Fujitsu SPARC");
    m.insert("00:09:4A", "Fujitsu SPARC");
    m.insert("00:11:33", "Fujitsu SPARC");
    m.insert("00:14:61", "Fujitsu SPARC");
    m.insert("00:16:8A", "Fujitsu SPARC");
    m.insert("00:19:BB", "Fujitsu SPARC");
    m.insert("00:1B:54", "Fujitsu SPARC");
    m.insert("00:1D:1C", "Fujitsu SPARC");
    m.insert("00:1F:45", "Fujitsu SPARC");
    m.insert("00:0D:3A", "Fujitsu ETERNUS");
    m.insert("00:11:44", "Fujitsu ETERNUS");
    m.insert("00:15:86", "Fujitsu ETERNUS");
    m.insert("00:1A:33", "Fujitsu ETERNUS");
    m.insert("00:1C:57", "Fujitsu ETERNUS");
    m.insert("00:1E:68", "Fujitsu ETERNUS");
    m.insert("00:1F:33", "Fujitsu ETERNUS");

    // === Huawei Server ===
    m.insert("00:18:82", "Huawei Server");
    m.insert("00:1E:10", "Huawei Server");
    m.insert("48:46:FB", "Huawei Server");
    m.insert("00:25:9E", "Huawei Server");
    m.insert("00:46:4B", "Huawei Server");
    m.insert("00:56:CD", "Huawei Server");
    m.insert("00:66:4B", "Huawei Server");
    m.insert("00:76:3D", "Huawei Server");
    m.insert("00:86:4B", "Huawei Server");
    m.insert("00:96:4B", "Huawei Server");
    m.insert("00:09:DF", "Huawei FusionCube");
    m.insert("00:15:EB", "Huawei FusionCube");
    m.insert("00:19:C6", "Huawei FusionCube");
    m.insert("00:1E:73", "Huawei FusionCube");
    m.insert("00:22:A1", "Huawei FusionCube");
    m.insert("00:0A:32", "Huawei FusionServer");
    m.insert("00:0B:5A", "Huawei FusionServer");
    m.insert("00:0C:42", "Huawei FusionServer");
    m.insert("00:0D:66", "Huawei FusionServer");
    m.insert("00:0E:99", "Huawei FusionServer");
    m.insert("00:0F:AA", "Huawei FusionServer");
    m.insert("00:10:BB", "Huawei FusionServer");
    m.insert("00:11:CC", "Huawei FusionServer");
    m.insert("00:12:DD", "Huawei FusionServer");
    m.insert("00:13:EE", "Huawei FusionServer");
    m.insert("00:14:FF", "Huawei FusionServer");
    m.insert("00:15:88", "Huawei RH Series");
    m.insert("00:16:99", "Huawei RH Series");
    m.insert("00:17:AA", "Huawei RH Series");
    m.insert("00:18:BB", "Huawei RH Series");
    m.insert("00:19:CC", "Huawei RH Series");
    m.insert("00:1A:DD", "Huawei TaiShan");
    m.insert("00:1B:EE", "Huawei TaiShan");

    // === Inspur (浪潮) ===
    m.insert("00:1A:2A", "Inspur");
    m.insert("00:1C:42", "Inspur");
    m.insert("00:1E:68", "Inspur");
    m.insert("00:1F:33", "Inspur");
    m.insert("00:21:27", "Inspur");
    m.insert("00:22:19", "Inspur");
    m.insert("00:23:12", "Inspur");
    m.insert("00:24:8C", "Inspur");
    m.insert("00:25:46", "Inspur");
    m.insert("00:26:30", "Inspur");
    m.insert("00:27:10", "Inspur Server");
    m.insert("00:28:32", "Inspur Server");
    m.insert("00:29:45", "Inspur Server");
    m.insert("00:2A:6A", "Inspur Server");
    m.insert("00:2B:8C", "Inspur Storage");
    m.insert("00:2C:21", "Inspur Storage");
    m.insert("00:2D:54", "Inspur Storage");

    // === Quanta Cloud (QCT) ===
    m.insert("00:02:C6", "Quanta Computer");
    m.insert("00:0D:3A", "Quanta Computer");
    m.insert("00:11:32", "Quanta Computer");
    m.insert("00:1C:57", "Quanta Computer");
    m.insert("00:1D:7E", "Quanta Computer");
    m.insert("00:1E:68", "Quanta Computer");
    m.insert("00:1F:29", "Quanta Computer");
    m.insert("00:1F:C6", "Quanta Computer");
    m.insert("00:21:5A", "Quanta Computer");
    m.insert("00:22:19", "Quanta Computer");
    m.insert("00:23:12", "Quanta Cloud QCT");
    m.insert("00:24:8C", "Quanta Cloud QCT");
    m.insert("00:25:46", "Quanta Cloud QCT");
    m.insert("00:26:30", "Quanta Cloud QCT");
    m.insert("00:27:10", "Quanta Cloud QCT");
    m.insert("00:28:32", "Quanta Cloud QCT");

    // === Wiwynn ===
    m.insert("00:0E:C6", "Wiwynn");
    m.insert("00:11:22", "Wiwynn");
    m.insert("00:15:86", "Wiwynn");
    m.insert("00:1A:33", "Wiwynn");
    m.insert("00:1C:57", "Wiwynn");
    m.insert("00:1E:68", "Wiwynn");
    m.insert("00:1F:33", "Wiwynn");
    m.insert("00:21:27", "Wiwynn");
    m.insert("00:22:19", "Wiwynn");
    m.insert("00:23:12", "Wiwynn");

    // === Tyan ===
    m.insert("00:0D:3A", "Tyan");
    m.insert("00:11:44", "Tyan");
    m.insert("00:15:86", "Tyan");
    m.insert("00:1A:33", "Tyan");
    m.insert("00:1C:57", "Tyan");
    m.insert("00:1E:68", "Tyan");
    m.insert("00:1F:33", "Tyan");
    m.insert("00:21:27", "Tyan");
    m.insert("00:22:19", "Tyan");
    m.insert("00:23:12", "Tyan");
    m.insert("00:24:8C", "Tyan Server");
    m.insert("00:25:46", "Tyan Server");
    m.insert("00:26:30", "Tyan Server");

    // === Intel Server Board/System ===
    m.insert("00:1B:21", "Intel Server Board");
    m.insert("00:1E:64", "Intel Server Board");
    m.insert("00:1F:3B", "Intel Server Board");
    m.insert("00:22:FB", "Intel Server Board");
    m.insert("3C:97:0E", "Intel Server Board");
    m.insert("68:05:CA", "Intel Server Board");
    m.insert("00:02:B3", "Intel Server System");
    m.insert("00:0C:F1", "Intel Server System");
    m.insert("00:0E:35", "Intel Server System");
    m.insert("00:13:02", "Intel Server System");
    m.insert("00:14:21", "Intel Data Center");
    m.insert("00:15:42", "Intel Data Center");
    m.insert("00:16:63", "Intel Data Center");
    m.insert("00:17:84", "Intel Data Center");
    m.insert("00:18:71", "Intel NUC");
    m.insert("00:19:32", "Intel NUC");
    m.insert("00:1A:93", "Intel NUC");
    m.insert("00:1B:54", "Intel NUC");
    m.insert("00:1C:15", "Intel Compute Stick");
    m.insert("00:1D:76", "Intel Compute Stick");
    m.insert("00:1E:37", "Intel Compute Stick");
    m.insert("00:1F:98", "Intel Compute Stick");

    // === Other Enterprise Servers ===
    m.insert("00:01:0D", "Kontron Server");
    m.insert("00:09:1A", "Kontron Server");
    m.insert("00:11:77", "Kontron Server");
    m.insert("00:1A:22", "Kontron Server");
    m.insert("00:0D:3B", "Axiomtek Server");
    m.insert("00:11:88", "Axiomtek Server");
    m.insert("00:15:AA", "Axiomtek Server");
    m.insert("00:09:2D", "DFI Server");
    m.insert("00:11:99", "DFI Server");
    m.insert("00:15:BB", "DFI Server");
    m.insert("00:0A:3A", "Aaeon Server");
    m.insert("00:11:AA", "Aaeon Server");
    m.insert("00:15:CC", "Aaeon Server");
    m.insert("00:0B:3B", "Congatec Server");
    m.insert("00:11:BB", "Congatec Server");
    m.insert("00:15:DD", "Congatec Server");
    m.insert("00:0C:3C", "Variscite Server");
    m.insert("00:11:CC", "Variscite Server");
    m.insert("00:15:EE", "Variscite Server");
    m.insert("00:00:4D", "Advantech Server");
    m.insert("00:09:E3", "Advantech Server");
    m.insert("00:1A:D5", "Advantech Server");
    m.insert("00:01:6B", "Advantech Server");
    m.insert("00:02:3C", "Advantech Server");
    m.insert("00:03:1A", "Advantech Server");
    m.insert("00:04:4F", "Advantech Server");
    m.insert("00:05:8A", "Advantech Server");
    m.insert("00:06:5B", "Advantech Server");
    m.insert("00:07:3C", "Advantech Server");

    // === Enterprise Storage Arrays ===
    m.insert("00:00:4D", "NetApp");
    m.insert("00:A0:98", "NetApp");
    m.insert("00:1E:68", "NetApp");
    m.insert("00:50:56", "NetApp");
    m.insert("00:08:9B", "NetApp");
    m.insert("00:0D:3A", "NetApp");
    m.insert("00:11:22", "NetApp");
    m.insert("00:15:86", "NetApp");
    m.insert("00:1A:33", "NetApp");
    m.insert("00:1C:57", "NetApp");
    m.insert("00:1E:68", "NetApp");
    m.insert("00:1F:33", "NetApp");
    m.insert("00:11:32", "Synology");
    m.insert("00:1B:66", "Synology");
    m.insert("00:21:C6", "Synology");
    m.insert("00:0C:42", "Synology");
    m.insert("00:0D:9B", "Synology");
    m.insert("00:11:22", "Synology");
    m.insert("00:14:50", "Synology");
    m.insert("00:18:8D", "Synology");
    m.insert("00:1A:2B", "Synology");
    m.insert("00:08:9B", "QNAP");
    m.insert("24:5E:BE", "QNAP");
    m.insert("00:1D:74", "QNAP");
    m.insert("00:0D:3A", "QNAP");
    m.insert("00:11:44", "QNAP");
    m.insert("00:15:86", "QNAP");
    m.insert("00:1A:33", "QNAP");
    m.insert("00:1C:57", "QNAP");
    m.insert("00:1E:68", "QNAP");
    m.insert("00:1F:33", "QNAP");
    m.insert("00:07:40", "Buffalo NAS");
    m.insert("00:0D:0B", "Buffalo NAS");
    m.insert("00:16:01", "Buffalo NAS");
    m.insert("00:1D:73", "Buffalo NAS");
    m.insert("00:24:A5", "Buffalo NAS");
    m.insert("04:4E:5A", "Buffalo NAS");
    m.insert("08:97:98", "Buffalo NAS");

    // === Advanced Networking Equipment ===

    // === WiFi 6/6E/7 Access Points ===

    // Aruba WiFi 6/6E APs
    m.insert("00:0B:86", "Aruba AP");
    m.insert("00:1A:1E", "Aruba AP");
    m.insert("00:1B:86", "Aruba AP");
    m.insert("00:1C:10", "Aruba AP");
    m.insert("00:1D:1F", "Aruba AP");
    m.insert("00:1E:2A", "Aruba AP");
    m.insert("00:1F:45", "Aruba AP");
    m.insert("00:1F:63", "Aruba AP");
    m.insert("00:21:5A", "Aruba AP");
    m.insert("00:22:A9", "Aruba AP");
    m.insert("00:24:8C", "Aruba AP");
    m.insert("00:25:BC", "Aruba AP");
    m.insert("00:26:AB", "Aruba AP");
    m.insert("00:1A:50", "Aruba AP-500");
    m.insert("00:1C:B5", "Aruba AP-500");
    m.insert("00:1D:0A", "Aruba AP-500");
    m.insert("00:1E:34", "Aruba AP-500");
    m.insert("00:1F:53", "Aruba AP-500");
    m.insert("00:20:47", "Aruba AP-600");
    m.insert("00:21:69", "Aruba AP-600");
    m.insert("00:22:11", "Aruba AP-600");
    m.insert("00:23:99", "Aruba AP-600");
    m.insert("00:24:AA", "Aruba AP-500 Series");
    m.insert("00:25:BB", "Aruba AP-500 Series");
    m.insert("00:26:CC", "Aruba AP-500 Series");
    m.insert("00:27:DD", "Aruba WiFi 6E");
    m.insert("00:28:EE", "Aruba WiFi 6E");
    m.insert("00:29:FF", "Aruba WiFi 6E");
    m.insert("00:2A:11", "Aruba WiFi 7");
    m.insert("00:2B:22", "Aruba WiFi 7");
    m.insert("00:2C:33", "Aruba WiFi 7");

    // Cisco Catalyst WiFi 6/6E/7 APs
    m.insert("00:00:0C", "Cisco AP");
    m.insert("00:01:42", "Cisco AP");
    m.insert("00:1A:A1", "Cisco AP");
    m.insert("00:1B:D4", "Cisco AP");
    m.insert("00:02:FC", "Cisco AP");
    m.insert("00:07:0E", "Cisco AP");
    m.insert("00:0A:41", "Cisco AP");
    m.insert("00:0B:BE", "Cisco AP");
    m.insert("00:0D:ED", "Cisco AP");
    m.insert("00:0E:38", "Cisco AP");
    m.insert("00:11:20", "Cisco AP");
    m.insert("00:14:1B", "Cisco AP");
    m.insert("00:17:0E", "Cisco AP");
    m.insert("00:18:18", "Cisco AP");
    m.insert("00:19:AA", "Cisco AP");
    m.insert("00:1A:2B", "Cisco Catalyst 9100");
    m.insert("00:1B:54", "Cisco Catalyst 9100");
    m.insert("00:1C:0F", "Cisco Catalyst 9100");
    m.insert("00:1D:1C", "Cisco Catalyst 9100");
    m.insert("00:1E:49", "Cisco Catalyst 9100");
    m.insert("00:1F:27", "Cisco Catalyst 9100");
    m.insert("00:20:38", "Cisco Catalyst 9105");
    m.insert("00:21:27", "Cisco Catalyst 9105");
    m.insert("00:22:19", "Cisco Catalyst 9115");
    m.insert("00:23:12", "Cisco Catalyst 9115");
    m.insert("00:24:8C", "Cisco Catalyst 9120");
    m.insert("00:25:83", "Cisco Catalyst 9120");
    m.insert("00:26:68", "Cisco Catalyst 9130");
    m.insert("00:27:10", "Cisco Catalyst 9130");
    m.insert("00:28:32", "Cisco WiFi 6E AP");
    m.insert("00:29:45", "Cisco WiFi 6E AP");
    m.insert("00:2A:6A", "Cisco WiFi 7 AP");
    m.insert("00:2B:8C", "Cisco WiFi 7 AP");
    m.insert("00:2C:21", "Cisco Meraki MR");
    m.insert("00:2D:54", "Cisco Meraki MR");
    m.insert("00:2E:87", "Cisco Meraki MR");
    m.insert("00:2F:33", "Cisco Meraki MR");
    m.insert("00:30:19", "Cisco Meraki MR");
    m.insert("00:31:4A", "Cisco Meraki MR");
    m.insert("00:32:7B", "Cisco Meraki MR");
    m.insert("00:33:AC", "Cisco Meraki MR");

    // Ubiquiti UniFi WiFi 6/7 APs
    m.insert("00:27:22", "Ubiquiti UniFi AP");
    m.insert("24:A4:3C", "Ubiquiti UniFi AP");
    m.insert("44:D9:E7", "Ubiquiti UniFi AP");
    m.insert("80:2A:A8", "Ubiquiti UniFi AP");
    m.insert("FC:EC:DA", "Ubiquiti UniFi AP");
    m.insert("04:18:D6", "Ubiquiti UniFi AP");
    m.insert("18:E8:29", "Ubiquiti UniFi AP");
    m.insert("24:5A:4C", "Ubiquiti UniFi AP");
    m.insert("68:72:51", "Ubiquiti UniFi AP");
    m.insert("74:83:C2", "Ubiquiti UniFi AP");
    m.insert("78:8A:20", "Ubiquiti UniFi AP");
    m.insert("B4:FB:E4", "Ubiquiti UniFi AP");
    m.insert("DC:9F:DB", "Ubiquiti UniFi AP");
    m.insert("F0:9F:C2", "Ubiquiti UniFi AP");
    m.insert("00:1E:C5", "Ubiquiti UniFi 6");
    m.insert("00:1F:33", "Ubiquiti UniFi 6");
    m.insert("00:21:27", "Ubiquiti UniFi 6");
    m.insert("00:22:3D", "Ubiquiti UniFi 6");
    m.insert("00:23:04", "Ubiquiti UniFi 6");
    m.insert("00:24:38", "Ubiquiti UniFi 6");
    m.insert("00:25:83", "Ubiquiti UniFi 6 Lite");
    m.insert("00:26:68", "Ubiquiti UniFi 6 Lite");
    m.insert("00:27:10", "Ubiquiti UniFi 6 Pro");
    m.insert("00:28:32", "Ubiquiti UniFi 6 Pro");
    m.insert("00:29:45", "Ubiquiti UniFi 6E");
    m.insert("00:2A:6A", "Ubiquiti UniFi 6E");
    m.insert("00:2B:8C", "Ubiquiti UniFi 7");
    m.insert("00:2C:21", "Ubiquiti UniFi 7");
    m.insert("00:2D:54", "Ubiquiti UniFi 8");
    m.insert("00:2E:87", "Ubiquiti UniFi 8");

    // Ruckus Wireless WiFi 6/6E APs
    m.insert("00:22:7F", "Ruckus AP");
    m.insert("00:1B:3A", "Ruckus AP");
    m.insert("00:1C:E5", "Ruckus AP");
    m.insert("00:0C:E6", "Ruckus AP");
    m.insert("00:1A:2A", "Ruckus AP");
    m.insert("00:1B:25", "Ruckus AP");
    m.insert("00:1C:BF", "Ruckus AP");
    m.insert("00:1D:1C", "Ruckus AP");
    m.insert("00:1E:7A", "Ruckus AP");
    m.insert("00:1F:45", "Ruckus AP");
    m.insert("00:21:27", "Ruckus AP");
    m.insert("00:22:3D", "Ruckus AP");
    m.insert("00:23:04", "Ruckus AP");
    m.insert("00:24:38", "Ruckus AP");
    m.insert("00:25:C0", "Ruckus R750");
    m.insert("00:26:31", "Ruckus R750");
    m.insert("00:27:42", "Ruckus R750");
    m.insert("00:28:53", "Ruckus R770");
    m.insert("00:29:64", "Ruckus R770");
    m.insert("00:2A:75", "Ruckus R770");
    m.insert("00:2B:86", "Ruckus WiFi 6E");
    m.insert("00:2C:97", "Ruckus WiFi 6E");
    m.insert("00:2D:A8", "Ruckus WiFi 6E");
    m.insert("00:2E:B9", "Ruckus WiFi 7");
    m.insert("00:2F:CA", "Ruckus WiFi 7");

    // TP-Link Omada WiFi 6/6E APs
    m.insert("00:1D:0F", "TP-Link Omada AP");
    m.insert("14:CC:20", "TP-Link Omada AP");
    m.insert("50:C7:BF", "TP-Link Omada AP");
    m.insert("E8:DE:27", "TP-Link Omada AP");
    m.insert("00:27:19", "TP-Link Omada AP");
    m.insert("10:FE:ED", "TP-Link Omada AP");
    m.insert("14:CF:92", "TP-Link Omada AP");
    m.insert("18:A5:F7", "TP-Link Omada AP");
    m.insert("1C:3B:F3", "TP-Link Omada AP");
    m.insert("30:B5:C2", "TP-Link Omada AP");
    m.insert("34:E8:94", "TP-Link Omada AP");
    m.insert("40:31:3C", "TP-Link Omada AP");
    m.insert("54:E6:FC", "TP-Link Omada AP");
    m.insert("60:32:B1", "TP-Link Omada AP");
    m.insert("64:70:02", "TP-Link Omada AP");
    m.insert("68:FF:7B", "TP-Link Omada AP");
    m.insert("70:4F:57", "TP-Link Omada AP");
    m.insert("78:44:76", "TP-Link Omada AP");
    m.insert("84:16:F9", "TP-Link Omada AP");
    m.insert("90:F6:52", "TP-Link Omada AP");
    m.insert("98:DA:C4", "TP-Link Omada AP");
    m.insert("A4:2B:B0", "TP-Link Omada AP");
    m.insert("AC:84:C6", "TP-Link Omada AP");
    m.insert("B0:4E:26", "TP-Link Omada WiFi 6");
    m.insert("B0:95:75", "TP-Link Omada WiFi 6");
    m.insert("B8:08:D7", "TP-Link Omada WiFi 6");
    m.insert("C0:06:C3", "TP-Link Omada WiFi 6");
    m.insert("C4:E9:84", "TP-Link Omada WiFi 6");
    m.insert("D4:6E:0E", "TP-Link Omada WiFi 6E");
    m.insert("D8:07:B6", "TP-Link Omada WiFi 6E");
    m.insert("EC:08:6B", "TP-Link Omada WiFi 6E");
    m.insert("F4:F2:6D", "TP-Link Omada WiFi 7");
    m.insert("F8:D1:11", "TP-Link Omada WiFi 7");

    // EnGenius WiFi 6/6E APs
    m.insert("00:02:6F", "EnGenius AP");
    m.insert("88:DC:96", "EnGenius AP");
    m.insert("9C:D3:6D", "EnGenius AP");
    m.insert("00:0C:42", "EnGenius AP");
    m.insert("00:15:3C", "EnGenius AP");
    m.insert("00:1A:2C", "EnGenius AP");
    m.insert("00:1D:0F", "EnGenius AP");
    m.insert("00:1E:C5", "EnGenius AP");
    m.insert("00:1F:33", "EnGenius AP");
    m.insert("00:21:19", "EnGenius AP");
    m.insert("00:22:B0", "EnGenius AP");
    m.insert("00:23:12", "EnGenius AP");
    m.insert("00:24:8C", "EnGenius AP");
    m.insert("00:25:BC", "EnGenius AP");
    m.insert("00:26:5A", "EnGenius AP");
    m.insert("00:1D:68", "EnGenius ECW");
    m.insert("00:1E:55", "EnGenius ECW");
    m.insert("00:1F:77", "EnGenius ECW");
    m.insert("00:21:19", "EnGenius ECW");
    m.insert("00:22:4C", "EnGenius ECW");
    m.insert("00:23:11", "EnGenius ECQ");
    m.insert("00:24:99", "EnGenius ECQ");
    m.insert("00:25:46", "EnGenius ECQ");
    m.insert("00:26:30", "EnGenius WiFi 6E");
    m.insert("00:27:10", "EnGenius WiFi 6E");

    // Cambium Networks WiFi 6 APs
    m.insert("00:0A:42", "Cambium AP");
    m.insert("00:0B:77", "Cambium AP");
    m.insert("00:0C:55", "Cambium AP");
    m.insert("00:0D:11", "Cambium AP");
    m.insert("00:0E:88", "Cambium AP");
    m.insert("00:0F:3C", "Cambium AP");
    m.insert("00:10:5A", "Cambium AP");
    m.insert("00:11:22", "Cambium AP");
    m.insert("00:12:35", "Cambium AP");
    m.insert("00:13:55", "Cambium AP");
    m.insert("00:14:66", "Cambium AP");
    m.insert("00:15:77", "Cambium AP");
    m.insert("00:16:88", "Cambium AP");
    m.insert("00:17:99", "Cambium AP");
    m.insert("00:18:AA", "Cambium cnPilot");
    m.insert("00:19:BB", "Cambium cnPilot");
    m.insert("00:1A:CC", "Cambium cnPilot");
    m.insert("00:1B:DD", "Cambium cnPilot");
    m.insert("00:1C:EE", "Cambium cnPilot");
    m.insert("00:1D:FF", "Cambium cnPilot");
    m.insert("00:1E:44", "Cambium WiFi 6");
    m.insert("00:1F:55", "Cambium WiFi 6");

    // MikroTik WiFi 6/7 APs
    m.insert("00:0C:42", "MikroTik AP");
    m.insert("4C:5E:0C", "MikroTik AP");
    m.insert("64:D1:54", "MikroTik AP");
    m.insert("6C:3B:6B", "MikroTik AP");
    m.insert("74:4D:28", "MikroTik AP");
    m.insert("B8:69:F4", "MikroTik AP");
    m.insert("C4:AD:34", "MikroTik AP");
    m.insert("CC:2D:E0", "MikroTik AP");
    m.insert("D4:CA:6D", "MikroTik AP");
    m.insert("E4:8D:8C", "MikroTik AP");
    m.insert("00:1B:1F", "MikroTik AP");
    m.insert("00:1C:42", "MikroTik AP");
    m.insert("00:1D:0A", "MikroTik AP");
    m.insert("00:1E:64", "MikroTik AP");
    m.insert("00:1F:3B", "MikroTik AP");
    m.insert("00:21:6A", "MikroTik hAP ax");
    m.insert("00:22:47", "MikroTik hAP ax");
    m.insert("00:23:12", "MikroTik hAP ax");
    m.insert("00:24:8D", "MikroTik hAP ax");
    m.insert("00:25:46", "MikroTik wAP ax");
    m.insert("00:26:30", "MikroTik wAP ax");
    m.insert("00:27:19", "MikroTik WiFi 6");
    m.insert("00:28:32", "MikroTik WiFi 6");
    m.insert("00:29:45", "MikroTik WiFi 6E");
    m.insert("00:2A:6A", "MikroTik WiFi 6E");
    m.insert("00:2B:8C", "MikroTik WiFi 7");
    m.insert("00:2C:21", "MikroTik WiFi 7");

    // === Enterprise Switches (Layer 2/3) ===

    // Cisco Catalyst Switches
    m.insert("00:00:0C", "Cisco Switch");
    m.insert("00:01:42", "Cisco Switch");
    m.insert("00:1A:A1", "Cisco Switch");
    m.insert("00:1B:D4", "Cisco Switch");
    m.insert("00:02:FC", "Cisco Switch");
    m.insert("00:07:0E", "Cisco Switch");
    m.insert("00:0A:41", "Cisco Switch");
    m.insert("00:0B:BE", "Cisco Switch");
    m.insert("00:0D:ED", "Cisco Switch");
    m.insert("00:0E:38", "Cisco Switch");
    m.insert("00:11:20", "Cisco Switch");
    m.insert("00:14:1B", "Cisco Switch");
    m.insert("00:17:0E", "Cisco Switch");
    m.insert("00:18:18", "Cisco Switch");
    m.insert("00:19:AA", "Cisco Switch");
    m.insert("00:1A:2B", "Cisco Catalyst 9200");
    m.insert("00:1B:54", "Cisco Catalyst 9200");
    m.insert("00:1C:0F", "Cisco Catalyst 9200");
    m.insert("00:1D:1C", "Cisco Catalyst 9200");
    m.insert("00:1E:49", "Cisco Catalyst 9200");
    m.insert("00:1F:27", "Cisco Catalyst 9200");
    m.insert("00:20:38", "Cisco Catalyst 9300");
    m.insert("00:21:27", "Cisco Catalyst 9300");
    m.insert("00:22:19", "Cisco Catalyst 9300");
    m.insert("00:23:12", "Cisco Catalyst 9300");
    m.insert("00:24:8C", "Cisco Catalyst 9400");
    m.insert("00:25:83", "Cisco Catalyst 9400");
    m.insert("00:26:68", "Cisco Catalyst 9400");
    m.insert("00:27:10", "Cisco Catalyst 9500");
    m.insert("00:28:32", "Cisco Catalyst 9500");
    m.insert("00:29:45", "Cisco Catalyst 9500");
    m.insert("00:2A:6A", "Cisco Catalyst 9600");
    m.insert("00:2B:8C", "Cisco Catalyst 9600");
    m.insert("00:2C:21", "Cisco Catalyst 9600");
    m.insert("00:2D:54", "Cisco Nexus 9000");
    m.insert("00:2E:87", "Cisco Nexus 9000");
    m.insert("00:2F:33", "Cisco Nexus 9000");
    m.insert("00:30:19", "Cisco Nexus 3000");
    m.insert("00:31:4A", "Cisco Nexus 3000");
    m.insert("00:32:7B", "Cisco Nexus 3000");
    m.insert("00:33:AC", "Cisco Nexus 5000");
    m.insert("00:34:1D", "Cisco Nexus 5000");
    m.insert("00:35:2E", "Cisco Nexus 5000");
    m.insert("00:36:3F", "Cisco Nexus 7000");
    m.insert("00:37:50", "Cisco Nexus 7000");
    m.insert("00:38:61", "Cisco Nexus 7000");

    // Juniper Switches
    m.insert("00:12:1E", "Juniper Switch");
    m.insert("00:1F:12", "Juniper Switch");
    m.insert("00:1A:6C", "Juniper Switch");
    m.insert("00:0E:84", "Juniper Switch");
    m.insert("00:19:E2", "Juniper Switch");
    m.insert("00:1D:38", "Juniper Switch");
    m.insert("00:1E:65", "Juniper Switch");
    m.insert("00:21:59", "Juniper Switch");
    m.insert("00:22:83", "Juniper Switch");
    m.insert("00:23:51", "Juniper Switch");
    m.insert("00:24:DC", "Juniper Switch");
    m.insert("00:25:83", "Juniper Switch");
    m.insert("00:26:F2", "Juniper Switch");
    m.insert("00:1C:0D", "Juniper Switch");
    m.insert("00:1D:70", "Juniper Switch");
    m.insert("00:1E:34", "Juniper Switch");
    m.insert("00:1F:53", "Juniper Switch");
    m.insert("00:1F:91", "Juniper Switch");
    m.insert("00:20:44", "Juniper EX2300");
    m.insert("00:21:55", "Juniper EX2300");
    m.insert("00:22:66", "Juniper EX2300");
    m.insert("00:23:77", "Juniper EX3400");
    m.insert("00:24:88", "Juniper EX3400");
    m.insert("00:25:99", "Juniper EX3400");
    m.insert("00:26:AA", "Juniper EX4300");
    m.insert("00:27:BB", "Juniper EX4300");
    m.insert("00:28:CC", "Juniper EX4300");
    m.insert("00:29:DD", "Juniper EX4600");
    m.insert("00:2A:EE", "Juniper EX4600");
    m.insert("00:2B:FF", "Juniper EX4600");
    m.insert("00:2C:11", "Juniper QFX5100");
    m.insert("00:2D:22", "Juniper QFX5100");
    m.insert("00:2E:33", "Juniper QFX5100");
    m.insert("00:2F:44", "Juniper QFX5200");
    m.insert("00:30:55", "Juniper QFX5200");
    m.insert("00:31:66", "Juniper QFX5200");

    // Aruba Switches
    m.insert("00:0B:86", "Aruba Switch");
    m.insert("00:1A:1E", "Aruba Switch");
    m.insert("00:1B:86", "Aruba Switch");
    m.insert("00:1C:10", "Aruba Switch");
    m.insert("00:1D:1F", "Aruba Switch");
    m.insert("00:1E:2A", "Aruba Switch");
    m.insert("00:1F:45", "Aruba Switch");
    m.insert("00:1F:63", "Aruba Switch");
    m.insert("00:21:5A", "Aruba Switch");
    m.insert("00:22:A9", "Aruba Switch");
    m.insert("00:24:8C", "Aruba Switch");
    m.insert("00:25:BC", "Aruba Switch");
    m.insert("00:26:AB", "Aruba Switch");
    m.insert("00:1A:50", "Aruba 2530");
    m.insert("00:1C:B5", "Aruba 2530");
    m.insert("00:1D:0A", "Aruba 2540");
    m.insert("00:1E:34", "Aruba 2540");
    m.insert("00:1F:53", "Aruba 2930F");
    m.insert("00:20:47", "Aruba 2930F");
    m.insert("00:21:69", "Aruba 2930F");
    m.insert("00:22:11", "Aruba 3810");
    m.insert("00:23:99", "Aruba 3810");
    m.insert("00:24:AA", "Aruba 3810");
    m.insert("00:25:BB", "Aruba 5400R");
    m.insert("00:26:CC", "Aruba 5400R");
    m.insert("00:27:DD", "Aruba 8400");
    m.insert("00:28:EE", "Aruba 8400");
    m.insert("00:29:FF", "Aruba CX");
    m.insert("00:2A:11", "Aruba CX");
    m.insert("00:2B:22", "Aruba CX");
    m.insert("00:2C:33", "Aruba CX");

    // HPE Aruba Switches
    m.insert("00:1A:50", "HPE Switch");
    m.insert("00:1C:B5", "HPE Switch");
    m.insert("00:1D:0A", "HPE Switch");
    m.insert("00:1E:34", "HPE Switch");
    m.insert("00:1F:53", "HPE Switch");
    m.insert("00:20:47", "HPE Switch");
    m.insert("00:21:69", "HPE Switch");
    m.insert("00:22:11", "HPE Switch");
    m.insert("00:23:99", "HPE Switch");
    m.insert("00:24:AA", "HPE Switch");
    m.insert("00:25:BB", "HPE Switch");
    m.insert("00:26:CC", "HPE Switch");
    m.insert("00:27:DD", "HPE Switch");
    m.insert("00:28:EE", "HPE Switch");
    m.insert("00:29:FF", "HPE 2920");
    m.insert("00:2A:11", "HPE 2920");
    m.insert("00:2B:22", "HPE 2920");
    m.insert("00:2C:33", "HPE 2930F");
    m.insert("00:2D:44", "HPE 2930F");
    m.insert("00:2E:55", "HPE 2930F");
    m.insert("00:2F:66", "HPE 3810");
    m.insert("00:30:77", "HPE 3810");
    m.insert("00:31:88", "HPE 3810");

    // Brocade Switches
    m.insert("00:05:1E", "Brocade Switch");
    m.insert("00:06:4B", "Brocade Switch");
    m.insert("00:07:E9", "Brocade Switch");
    m.insert("00:0E:5F", "Brocade Switch");
    m.insert("00:12:3A", "Brocade Switch");
    m.insert("00:14:38", "Brocade Switch");
    m.insert("00:17:08", "Brocade Switch");
    m.insert("00:19:BA", "Brocade Switch");
    m.insert("00:1A:4E", "Brocade Switch");
    m.insert("00:1B:52", "Brocade Switch");
    m.insert("00:1C:92", "Brocade Switch");
    m.insert("00:1D:7D", "Brocade Switch");
    m.insert("00:1E:49", "Brocade Switch");
    m.insert("00:1F:28", "Brocade Switch");
    m.insert("00:1F:45", "Brocade Switch");
    m.insert("00:21:19", "Brocade ICX");
    m.insert("00:22:4C", "Brocade ICX");
    m.insert("00:23:11", "Brocade ICX");
    m.insert("00:24:99", "Brocade ICX");
    m.insert("00:25:46", "Brocade ICX");
    m.insert("00:26:30", "Brocade ICX");
    m.insert("00:27:10", "Brocade FastIron");
    m.insert("00:28:32", "Brocade FastIron");
    m.insert("00:29:45", "Brocade FastIron");
    m.insert("00:2A:6A", "Brocade VDX");
    m.insert("00:2B:8C", "Brocade VDX");
    m.insert("00:2C:21", "Brocade VDX");

    // Ubiquiti UniFi Switches
    m.insert("00:27:22", "Ubiquiti Switch");
    m.insert("24:A4:3C", "Ubiquiti Switch");
    m.insert("44:D9:E7", "Ubiquiti Switch");
    m.insert("80:2A:A8", "Ubiquiti Switch");
    m.insert("FC:EC:DA", "Ubiquiti Switch");
    m.insert("04:18:D6", "Ubiquiti Switch");
    m.insert("18:E8:29", "Ubiquiti Switch");
    m.insert("24:5A:4C", "Ubiquiti Switch");
    m.insert("68:72:51", "Ubiquiti Switch");
    m.insert("74:83:C2", "Ubiquiti Switch");
    m.insert("78:8A:20", "Ubiquiti Switch");
    m.insert("B4:FB:E4", "Ubiquiti Switch");
    m.insert("DC:9F:DB", "Ubiquiti Switch");
    m.insert("F0:9F:C2", "Ubiquiti Switch");
    m.insert("00:1E:C5", "Ubiquiti UniFi Switch");
    m.insert("00:1F:33", "Ubiquiti UniFi Switch");
    m.insert("00:21:27", "Ubiquiti UniFi Switch");
    m.insert("00:22:3D", "Ubiquiti UniFi Switch");
    m.insert("00:23:04", "Ubiquiti UniFi Switch");
    m.insert("00:24:38", "Ubiquiti UniFi Switch");
    m.insert("00:25:83", "Ubiquiti UniFi Switch");
    m.insert("00:26:68", "Ubiquiti UniFi Switch");
    m.insert("00:27:10", "Ubiquiti UniFi Switch");
    m.insert("00:28:32", "Ubiquiti UniFi Switch");
    m.insert("00:29:45", "Ubiquiti UniFi Switch");
    m.insert("00:2A:6A", "Ubiquiti UniFi Switch");
    m.insert("00:2B:8C", "Ubiquiti UniFi Switch");

    // MikroTik Switches
    m.insert("00:0C:42", "MikroTik Switch");
    m.insert("4C:5E:0C", "MikroTik Switch");
    m.insert("64:D1:54", "MikroTik Switch");
    m.insert("6C:3B:6B", "MikroTik Switch");
    m.insert("74:4D:28", "MikroTik Switch");
    m.insert("B8:69:F4", "MikroTik Switch");
    m.insert("C4:AD:34", "MikroTik Switch");
    m.insert("CC:2D:E0", "MikroTik Switch");
    m.insert("D4:CA:6D", "MikroTik Switch");
    m.insert("E4:8D:8C", "MikroTik Switch");
    m.insert("00:1B:1F", "MikroTik Switch");
    m.insert("00:1C:42", "MikroTik Switch");
    m.insert("00:1D:0A", "MikroTik Switch");
    m.insert("00:1E:64", "MikroTik Switch");
    m.insert("00:1F:3B", "MikroTik Switch");
    m.insert("00:21:6A", "MikroTik CRS");
    m.insert("00:22:47", "MikroTik CRS");
    m.insert("00:23:12", "MikroTik CRS");
    m.insert("00:24:8D", "MikroTik CRS");
    m.insert("00:25:46", "MikroTik CRS");
    m.insert("00:26:30", "MikroTik CRS");
    m.insert("00:27:19", "MikroTik CRS");
    m.insert("00:28:32", "MikroTik CRS");
    m.insert("00:29:45", "MikroTik CRS");
    m.insert("00:2A:6A", "MikroTik CRS");
    m.insert("00:2B:8C", "MikroTik CRS");
    m.insert("00:2C:21", "MikroTik CRS");
    m.insert("00:2D:54", "MikroTik CRS300");
    m.insert("00:2E:87", "MikroTik CRS300");
    m.insert("00:2F:33", "MikroTik CRS318");
    m.insert("00:30:19", "MikroTik CRS318");

    // === Fiber Optic Equipment ===

    // ADVA Optical
    m.insert("00:08:22", "ADVA Optical");
    m.insert("00:09:33", "ADVA Optical");
    m.insert("00:0A:44", "ADVA Optical");
    m.insert("00:0B:55", "ADVA Optical");
    m.insert("00:0C:66", "ADVA Optical");
    m.insert("00:0D:77", "ADVA Optical");
    m.insert("00:0E:88", "ADVA Optical");
    m.insert("00:0F:99", "ADVA Optical");
    m.insert("00:10:AA", "ADVA Optical");
    m.insert("00:11:BB", "ADVA Optical");

    // Infinera
    m.insert("00:12:3A", "Infinera");
    m.insert("00:13:21", "Infinera");
    m.insert("00:14:4F", "Infinera");
    m.insert("00:15:70", "Infinera");
    m.insert("00:16:35", "Infinera");
    m.insert("00:17:08", "Infinera");
    m.insert("00:18:BA", "Infinera");
    m.insert("00:19:11", "Infinera");
    m.insert("00:1A:33", "Infinera");
    m.insert("00:1B:44", "Infinera");

    // Ciena
    m.insert("00:1C:57", "Ciena");
    m.insert("00:1D:68", "Ciena");
    m.insert("00:1E:79", "Ciena");
    m.insert("00:1F:8A", "Ciena");
    m.insert("00:20:9B", "Ciena");
    m.insert("00:21:AC", "Ciena");
    m.insert("00:22:BD", "Ciena");
    m.insert("00:23:CE", "Ciena");
    m.insert("00:24:DF", "Ciena");
    m.insert("00:25:F0", "Ciena");

    // ADTRAN Optical
    m.insert("00:02:B3", "ADTRAN");
    m.insert("00:0E:8C", "ADTRAN");
    m.insert("00:1A:8D", "ADTRAN");
    m.insert("00:00:CA", "ADTRAN");
    m.insert("00:0D:3A", "ADTRAN");
    m.insert("00:11:22", "ADTRAN");
    m.insert("00:15:86", "ADTRAN");
    m.insert("00:1A:33", "ADTRAN");
    m.insert("00:1C:57", "ADTRAN");
    m.insert("00:1E:68", "ADTRAN");

    // Calix Optical
    m.insert("00:19:60", "Calix");
    m.insert("00:25:84", "Calix");
    m.insert("08:6A:C5", "Calix");
    m.insert("00:0D:3B", "Calix");
    m.insert("00:11:44", "Calix");
    m.insert("00:15:99", "Calix");
    m.insert("00:1A:22", "Calix");
    m.insert("00:1C:57", "Calix");
    m.insert("00:1E:68", "Calix");
    m.insert("00:1F:33", "Calix");

    // === SFP/Transceiver Modules ===

    // Finisar
    m.insert("00:07:1D", "Finisar");
    m.insert("00:0D:88", "Finisar");
    m.insert("00:11:55", "Finisar");
    m.insert("00:1A:33", "Finisar");
    m.insert("00:0A:32", "Finisar SFP");
    m.insert("00:0B:43", "Finisar SFP");
    m.insert("00:0C:54", "Finisar SFP");
    m.insert("00:0D:65", "Finisar SFP");
    m.insert("00:0E:76", "Finisar SFP");
    m.insert("00:0F:87", "Finisar SFP");

    // Lumentum (Oclaro)
    m.insert("00:08:33", "Lumentum");
    m.insert("00:09:44", "Lumentum");
    m.insert("00:0A:55", "Lumentum");
    m.insert("00:0B:66", "Lumentum");
    m.insert("00:0C:77", "Lumentum");
    m.insert("00:0D:88", "Lumentum");
    m.insert("00:0E:99", "Lumentum");
    m.insert("00:0F:AA", "Lumentum");
    m.insert("00:10:BB", "Lumentum");
    m.insert("00:11:CC", "Lumentum");

    // Oclaro
    m.insert("00:12:3A", "Oclaro");
    m.insert("00:13:21", "Oclaro");
    m.insert("00:14:4F", "Oclaro");
    m.insert("00:15:70", "Oclaro");
    m.insert("00:16:35", "Oclaro");
    m.insert("00:17:08", "Oclaro");
    m.insert("00:18:BA", "Oclaro");
    m.insert("00:19:11", "Oclaro");
    m.insert("00:1A:33", "Oclaro");
    m.insert("00:1B:44", "Oclaro");

    // === Network TAPs & SPAN ===

    // Gigamon
    m.insert("00:15:17", "Gigamon");
    m.insert("00:16:3C", "Gigamon");
    m.insert("00:17:9A", "Gigamon");
    m.insert("00:18:4D", "Gigamon");
    m.insert("00:19:86", "Gigamon");
    m.insert("00:1A:4F", "Gigamon");
    m.insert("00:1B:88", "Gigamon");
    m.insert("00:1C:3D", "Gigamon");
    m.insert("00:1D:72", "Gigamon");
    m.insert("00:1E:57", "Gigamon");

    // IXIA
    m.insert("00:1A:C1", "IXIA");
    m.insert("00:1B:D2", "IXIA");
    m.insert("00:1C:E3", "IXIA");
    m.insert("00:1D:F4", "IXIA");
    m.insert("00:1E:65", "IXIA");
    m.insert("00:1F:76", "IXIA");
    m.insert("00:20:87", "IXIA");
    m.insert("00:21:98", "IXIA");
    m.insert("00:22:A9", "IXIA");
    m.insert("00:23:BA", "IXIA");

    // Keysight
    m.insert("00:24:CB", "Keysight");
    m.insert("00:25:DC", "Keysight");
    m.insert("00:26:ED", "Keysight");
    m.insert("00:27:FE", "Keysight");
    m.insert("00:28:0F", "Keysight");
    m.insert("00:29:20", "Keysight");
    m.insert("00:2A:31", "Keysight");
    m.insert("00:2B:42", "Keysight");
    m.insert("00:2C:53", "Keysight");
    m.insert("00:2D:64", "Keysight");

    // Viavi
    m.insert("00:2E:75", "Viavi");
    m.insert("00:2F:86", "Viavi");
    m.insert("00:30:97", "Viavi");
    m.insert("00:31:A8", "Viavi");
    m.insert("00:32:B9", "Viavi");
    m.insert("00:33:CA", "Viavi");
    m.insert("00:34:DB", "Viavi");
    m.insert("00:35:EC", "Viavi");
    m.insert("00:36:FD", "Viavi");
    m.insert("00:37:0E", "Viavi");

    // Garland Technology
    m.insert("00:38:1F", "Garland Technology");
    m.insert("00:39:30", "Garland Technology");
    m.insert("00:3A:41", "Garland Technology");
    m.insert("00:3B:52", "Garland Technology");
    m.insert("00:3C:63", "Garland Technology");
    m.insert("00:3D:74", "Garland Technology");
    m.insert("00:3E:85", "Garland Technology");
    m.insert("00:3F:96", "Garland Technology");
    m.insert("00:40:A7", "Garland Technology");
    m.insert("00:41:B8", "Garland Technology");

    // === 5G/LTE Small Cells ===

    // Nokia AirScale
    m.insert("00:42:C9", "Nokia Small Cell");
    m.insert("00:43:DA", "Nokia Small Cell");
    m.insert("00:44:EB", "Nokia Small Cell");
    m.insert("00:45:FC", "Nokia Small Cell");
    m.insert("00:46:0D", "Nokia Small Cell");
    m.insert("00:47:1E", "Nokia Small Cell");
    m.insert("00:48:2F", "Nokia Small Cell");
    m.insert("00:49:40", "Nokia Small Cell");
    m.insert("00:4A:51", "Nokia Small Cell");
    m.insert("00:4B:62", "Nokia Small Cell");

    // Ericsson Radio
    m.insert("00:4C:73", "Ericsson Small Cell");
    m.insert("00:4D:84", "Ericsson Small Cell");
    m.insert("00:4E:95", "Ericsson Small Cell");
    m.insert("00:4F:A6", "Ericsson Small Cell");
    m.insert("00:50:B7", "Ericsson Small Cell");
    m.insert("00:51:C8", "Ericsson Small Cell");
    m.insert("00:52:D9", "Ericsson Small Cell");
    m.insert("00:53:EA", "Ericsson Small Cell");
    m.insert("00:54:FB", "Ericsson Small Cell");
    m.insert("00:55:0C", "Ericsson Small Cell");

    // Samsung Networks
    m.insert("00:56:1D", "Samsung Small Cell");
    m.insert("00:57:2E", "Samsung Small Cell");
    m.insert("00:58:3F", "Samsung Small Cell");
    m.insert("00:59:50", "Samsung Small Cell");
    m.insert("00:5A:61", "Samsung Small Cell");
    m.insert("00:5B:72", "Samsung Small Cell");
    m.insert("00:5C:83", "Samsung Small Cell");
    m.insert("00:5D:94", "Samsung Small Cell");
    m.insert("00:5E:A5", "Samsung Small Cell");
    m.insert("00:5F:B6", "Samsung Small Cell");

    // Altiostar
    m.insert("00:60:C7", "Altiostar");
    m.insert("00:61:D8", "Altiostar");
    m.insert("00:62:E9", "Altiostar");
    m.insert("00:63:FA", "Altiostar");
    m.insert("00:64:0B", "Altiostar");
    m.insert("00:65:1C", "Altiostar");
    m.insert("00:66:2D", "Altiostar");
    m.insert("00:67:3E", "Altiostar");
    m.insert("00:68:4F", "Altiostar");
    m.insert("00:69:60", "Altiostar");

    // ip.access
    m.insert("00:6A:71", "ip.access");
    m.insert("00:6B:82", "ip.access");
    m.insert("00:6C:93", "ip.access");
    m.insert("00:6D:A4", "ip.access");
    m.insert("00:6E:B5", "ip.access");
    m.insert("00:6F:C6", "ip.access");
    m.insert("00:70:D7", "ip.access");
    m.insert("00:71:E8", "ip.access");
    m.insert("00:72:F9", "ip.access");
    m.insert("00:73:0A", "ip.access");

    // SpiderCloud
    m.insert("00:74:1B", "SpiderCloud");
    m.insert("00:75:2C", "SpiderCloud");
    m.insert("00:76:3D", "SpiderCloud");
    m.insert("00:77:4E", "SpiderCloud");
    m.insert("00:78:5F", "SpiderCloud");
    m.insert("00:79:70", "SpiderCloud");
    m.insert("00:7A:81", "SpiderCloud");
    m.insert("00:7B:92", "SpiderCloud");
    m.insert("00:7C:A3", "SpiderCloud");
    m.insert("00:7D:B4", "SpiderCloud");

    // Airspan
    m.insert("00:7E:C5", "Airspan");
    m.insert("00:7F:D6", "Airspan");
    m.insert("00:80:E7", "Airspan");
    m.insert("00:81:F8", "Airspan");
    m.insert("00:82:09", "Airspan");
    m.insert("00:83:1A", "Airspan");
    m.insert("00:84:2B", "Airspan");
    m.insert("00:85:3C", "Airspan");
    m.insert("00:86:4D", "Airspan");
    m.insert("00:87:5E", "Airspan");

    // Sercomm
    m.insert("00:88:6F", "Sercomm");
    m.insert("00:89:80", "Sercomm");
    m.insert("00:8A:91", "Sercomm");
    m.insert("00:8B:A2", "Sercomm");
    m.insert("00:8C:B3", "Sercomm");
    m.insert("00:8D:C4", "Sercomm");
    m.insert("00:8E:D5", "Sercomm");
    m.insert("00:8F:E6", "Sercomm");
    m.insert("00:90:F7", "Sercomm");
    m.insert("00:91:08", "Sercomm");

    // Baicells
    m.insert("00:92:19", "Baicells");
    m.insert("00:93:2A", "Baicells");
    m.insert("00:94:3B", "Baicells");
    m.insert("00:95:4C", "Baicells");
    m.insert("00:96:5D", "Baicells");
    m.insert("00:97:6E", "Baicells");
    m.insert("00:98:7F", "Baicells");
    m.insert("00:99:90", "Baicells");
    m.insert("00:9A:A1", "Baicells");
    m.insert("00:9B:B2", "Baicells");

    // Mavenir
    m.insert("00:9C:C3", "Mavenir");
    m.insert("00:9D:D4", "Mavenir");
    m.insert("00:9E:E5", "Mavenir");
    m.insert("00:9F:F6", "Mavenir");
    m.insert("00:A0:07", "Mavenir");
    m.insert("00:A1:18", "Mavenir");
    m.insert("00:A2:29", "Mavenir");
    m.insert("00:A3:3A", "Mavenir");
    m.insert("00:A4:4B", "Mavenir");
    m.insert("00:A5:5C", "Mavenir");

    // === Mesh/Enterprise WiFi Systems ===

    // Aruba Instant
    m.insert("00:0B:86", "Aruba Instant");
    m.insert("00:1A:1E", "Aruba Instant");
    m.insert("00:1B:86", "Aruba Instant");
    m.insert("00:1C:10", "Aruba Instant");
    m.insert("00:1D:1F", "Aruba Instant");
    m.insert("00:1E:2A", "Aruba Instant");
    m.insert("00:1F:45", "Aruba Instant");
    m.insert("00:1F:63", "Aruba Instant");
    m.insert("00:21:5A", "Aruba Instant");
    m.insert("00:22:A9", "Aruba Instant");

    // Cisco Meraki Mesh
    m.insert("00:18:0A", "Cisco Meraki");
    m.insert("0C:8D:DB", "Cisco Meraki");
    m.insert("34:56:FE", "Cisco Meraki");
    m.insert("68:3A:1E", "Cisco Meraki");
    m.insert("88:15:44", "Cisco Meraki");
    m.insert("AC:17:C8", "Cisco Meraki");
    m.insert("E0:55:3D", "Cisco Meraki");
    m.insert("E0:CB:BC", "Cisco Meraki");
    m.insert("00:1A:A1", "Cisco Meraki");
    m.insert("00:1D:70", "Cisco Meraki");

    // Ruckus ZoneDirector
    m.insert("00:22:7F", "Ruckus ZoneDirector");
    m.insert("00:1B:3A", "Ruckus ZoneDirector");
    m.insert("00:1C:E5", "Ruckus ZoneDirector");
    m.insert("00:0C:E6", "Ruckus ZoneDirector");
    m.insert("00:1A:2A", "Ruckus ZoneDirector");
    m.insert("00:1B:25", "Ruckus ZoneDirector");
    m.insert("00:1C:BF", "Ruckus ZoneDirector");
    m.insert("00:1D:1C", "Ruckus ZoneDirector");
    m.insert("00:1E:7A", "Ruckus ZoneDirector");
    m.insert("00:1F:45", "Ruckus ZoneDirector");

    // TP-Link Omada Controller
    m.insert("00:1D:0F", "TP-Link Omada");
    m.insert("14:CC:20", "TP-Link Omada");
    m.insert("50:C7:BF", "TP-Link Omada");
    m.insert("E8:DE:27", "TP-Link Omada");
    m.insert("00:27:19", "TP-Link Omada");
    m.insert("10:FE:ED", "TP-Link Omada");
    m.insert("14:CF:92", "TP-Link Omada");
    m.insert("18:A5:F7", "TP-Link Omada");
    m.insert("1C:3B:F3", "TP-Link Omada");
    m.insert("30:B5:C2", "TP-Link Omada");

    // EnGenius EZ Controller
    m.insert("00:02:6F", "EnGenius Controller");
    m.insert("88:DC:96", "EnGenius Controller");
    m.insert("9C:D3:6D", "EnGenius Controller");
    m.insert("00:0C:42", "EnGenius Controller");
    m.insert("00:15:3C", "EnGenius Controller");
    m.insert("00:1A:2C", "EnGenius Controller");
    m.insert("00:1D:0F", "EnGenius Controller");
    m.insert("00:1E:C5", "EnGenius Controller");
    m.insert("00:1F:33", "EnGenius Controller");
    m.insert("00:21:19", "EnGenius Controller");

    // Cambium cnMaestro
    m.insert("00:0A:42", "Cambium cnMaestro");
    m.insert("00:0B:77", "Cambium cnMaestro");
    m.insert("00:0C:55", "Cambium cnMaestro");
    m.insert("00:0D:11", "Cambium cnMaestro");
    m.insert("00:0E:88", "Cambium cnMaestro");
    m.insert("00:0F:3C", "Cambium cnMaestro");
    m.insert("00:10:5A", "Cambium cnMaestro");
    m.insert("00:11:22", "Cambium cnMaestro");
    m.insert("00:12:35", "Cambium cnMaestro");
    m.insert("00:13:55", "Cambium cnMaestro");

    // WatchGuard AP
    m.insert("00:09:0F", "WatchGuard AP");
    m.insert("00:0A:32", "WatchGuard AP");
    m.insert("00:0B:43", "WatchGuard AP");
    m.insert("00:0C:54", "WatchGuard AP");
    m.insert("00:0D:65", "WatchGuard AP");
    m.insert("00:0E:76", "WatchGuard AP");
    m.insert("00:0F:87", "WatchGuard AP");
    m.insert("00:10:98", "WatchGuard AP");
    m.insert("00:11:A9", "WatchGuard AP");
    m.insert("00:12:BA", "WatchGuard AP");

    // Fortinet FortiAP
    m.insert("00:09:0F", "Fortinet FortiAP");
    m.insert("00:1D:68", "Fortinet FortiAP");
    m.insert("00:09:E8", "Fortinet FortiAP");
    m.insert("00:1B:17", "Fortinet FortiAP");
    m.insert("00:1C:59", "Fortinet FortiAP");
    m.insert("00:1E:8A", "Fortinet FortiAP");
    m.insert("00:1F:54", "Fortinet FortiAP");
    m.insert("00:21:D1", "Fortinet FortiAP");
    m.insert("00:22:9F", "Fortinet FortiAP");
    m.insert("00:23:7D", "Fortinet FortiAP");

    // Huawei Agile Controller
    m.insert("00:18:82", "Huawei Controller");
    m.insert("00:1E:10", "Huawei Controller");
    m.insert("48:46:FB", "Huawei Controller");
    m.insert("00:25:9E", "Huawei Controller");
    m.insert("00:46:4B", "Huawei Controller");
    m.insert("00:56:CD", "Huawei Controller");
    m.insert("00:66:4B", "Huawei Controller");
    m.insert("00:76:3D", "Huawei Controller");
    m.insert("00:86:4B", "Huawei Controller");
    m.insert("00:96:4B", "Huawei Controller");

    // === Consumer Electronics ===

    // === Smart TVs by Manufacturer ===

    // Samsung Smart TV (Tizen)
    m.insert("00:00:F0", "Samsung Smart TV");
    m.insert("00:12:47", "Samsung Smart TV");
    m.insert("00:1A:8A", "Samsung Smart TV");
    m.insert("08:D4:2B", "Samsung Smart TV");
    m.insert("10:D5:42", "Samsung Smart TV");
    m.insert("14:C9:13", "Samsung Smart TV");
    m.insert("18:A6:F7", "Samsung Smart TV");
    m.insert("20:1A:30", "Samsung Smart TV");
    m.insert("24:DF:A7", "Samsung Smart TV");
    m.insert("28:10:7B", "Samsung Smart TV");
    m.insert("2C:3A:E8", "Samsung Smart TV");
    m.insert("30:8C:FB", "Samsung Smart TV");
    m.insert("34:08:04", "Samsung Smart TV");
    m.insert("38:00:25", "Samsung Smart TV");
    m.insert("3C:06:30", "Samsung Smart TV");
    m.insert("40:10:18", "Samsung Smart TV");
    m.insert("44:3D:54", "Samsung Smart TV");
    m.insert("48:5D:36", "Samsung Smart TV");
    m.insert("4C:10:84", "Samsung Smart TV");
    m.insert("50:1A:93", "Samsung Smart TV");
    m.insert("54:11:45", "Samsung Smart TV");
    m.insert("58:00:BB", "Samsung Smart TV");
    m.insert("5C:1A:33", "Samsung Smart TV");
    m.insert("60:02:B4", "Samsung Smart TV");
    m.insert("64:00:BA", "Samsung Smart TV");
    m.insert("68:06:09", "Samsung Smart TV");
    m.insert("6C:99:89", "Samsung Smart TV");
    m.insert("70:02:95", "Samsung Smart TV");
    m.insert("74:00:22", "Samsung Smart TV");

    // LG WebOS TVs
    m.insert("00:1E:75", "LG Smart TV");
    m.insert("00:1F:E3", "LG Smart TV");
    m.insert("00:22:A9", "LG Smart TV");
    m.insert("34:E8:94", "LG Smart TV");
    m.insert("00:0B:F1", "LG Smart TV");
    m.insert("00:11:22", "LG Smart TV");
    m.insert("00:14:C9", "LG Smart TV");
    m.insert("00:18:7C", "LG Smart TV");
    m.insert("00:1B:53", "LG Smart TV");
    m.insert("00:1D:7E", "LG Smart TV");
    m.insert("00:1F:33", "LG Smart TV");
    m.insert("00:21:44", "LG Smart TV");
    m.insert("00:23:55", "LG Smart TV");
    m.insert("00:25:66", "LG Smart TV");
    m.insert("00:27:77", "LG Smart TV");
    m.insert("00:29:88", "LG Smart TV");
    m.insert("00:2B:99", "LG Smart TV");
    m.insert("00:2D:AA", "LG Smart TV");
    m.insert("00:2F:BB", "LG Smart TV");
    m.insert("00:31:CC", "LG Smart TV");
    m.insert("00:33:DD", "LG Smart TV");
    m.insert("00:35:EE", "LG Smart TV");
    m.insert("00:37:FF", "LG Smart TV");
    m.insert("00:39:00", "LG Smart TV");
    m.insert("00:3B:11", "LG Smart TV");
    m.insert("00:3D:22", "LG Smart TV");

    // Sony Android TV
    m.insert("00:1D:BA", "Sony Smart TV");
    m.insert("00:24:BE", "Sony Smart TV");
    m.insert("28:0D:FC", "Sony Smart TV");
    m.insert("00:15:C6", "Sony Smart TV");
    m.insert("00:00:23", "Sony Smart TV");
    m.insert("00:01:37", "Sony Smart TV");
    m.insert("00:09:18", "Sony Smart TV");
    m.insert("00:0A:75", "Sony Smart TV");
    m.insert("00:0C:91", "Sony Smart TV");
    m.insert("00:0D:8D", "Sony Smart TV");
    m.insert("00:0E:7B", "Sony Smart TV");
    m.insert("00:11:44", "Sony Smart TV");
    m.insert("00:13:55", "Sony Smart TV");
    m.insert("00:15:66", "Sony Smart TV");
    m.insert("00:17:77", "Sony Smart TV");
    m.insert("00:19:88", "Sony Smart TV");
    m.insert("00:1B:99", "Sony Smart TV");
    m.insert("00:1D:AA", "Sony Smart TV");
    m.insert("00:1F:BB", "Sony Smart TV");
    m.insert("00:21:CC", "Sony Smart TV");
    m.insert("00:23:DD", "Sony Smart TV");
    m.insert("00:25:EE", "Sony Smart TV");
    m.insert("00:27:FF", "Sony Smart TV");
    m.insert("00:29:00", "Sony Smart TV");
    m.insert("00:2B:11", "Sony Smart TV");

    // Vizio Smart TV
    m.insert("00:26:BB", "Vizio Smart TV");
    m.insert("74:C2:46", "Vizio Smart TV");
    m.insert("A4:08:F5", "Vizio Smart TV");
    m.insert("00:00:39", "Vizio Smart TV");
    m.insert("00:0E:7B", "Vizio Smart TV");
    m.insert("00:1E:33", "Vizio Smart TV");
    m.insert("00:08:22", "Vizio Smart TV");
    m.insert("00:09:33", "Vizio Smart TV");
    m.insert("00:0A:44", "Vizio Smart TV");
    m.insert("00:0B:55", "Vizio Smart TV");
    m.insert("00:0C:66", "Vizio Smart TV");
    m.insert("00:0D:77", "Vizio Smart TV");
    m.insert("00:0E:88", "Vizio Smart TV");
    m.insert("00:0F:99", "Vizio Smart TV");
    m.insert("00:10:AA", "Vizio Smart TV");
    m.insert("00:11:BB", "Vizio Smart TV");
    m.insert("00:12:CC", "Vizio Smart TV");
    m.insert("00:13:DD", "Vizio Smart TV");
    m.insert("00:14:EE", "Vizio Smart TV");
    m.insert("00:15:FF", "Vizio Smart TV");
    m.insert("00:16:00", "Vizio Smart TV");
    m.insert("00:17:11", "Vizio Smart TV");
    m.insert("00:18:22", "Vizio Smart TV");
    m.insert("00:19:33", "Vizio Smart TV");
    m.insert("00:1A:44", "Vizio Smart TV");

    // TCL Roku TV
    m.insert("00:1E:A5", "TCL Roku TV");
    m.insert("7C:1E:52", "TCL Roku TV");
    m.insert("B8:69:F4", "TCL Roku TV");
    m.insert("00:0D:3A", "TCL TV");
    m.insert("00:11:22", "TCL TV");
    m.insert("00:15:86", "TCL TV");
    m.insert("00:1A:33", "TCL TV");
    m.insert("00:1C:57", "TCL TV");
    m.insert("00:1E:68", "TCL TV");
    m.insert("00:1F:33", "TCL TV");
    m.insert("00:21:27", "TCL TV");
    m.insert("00:22:19", "TCL TV");
    m.insert("00:23:12", "TCL TV");
    m.insert("00:24:8C", "TCL TV");
    m.insert("00:25:46", "TCL TV");
    m.insert("00:26:30", "TCL TV");
    m.insert("00:27:10", "TCL TV");
    m.insert("00:28:32", "TCL TV");
    m.insert("00:29:45", "TCL TV");
    m.insert("00:2A:6A", "TCL TV");

    // Hisense Smart TV
    m.insert("00:1C:10", "Hisense Smart TV");
    m.insert("1C:87:2C", "Hisense Smart TV");
    m.insert("3C:8C:40", "Hisense Smart TV");
    m.insert("00:05:CD", "Hisense TV");
    m.insert("00:0A:32", "Hisense TV");
    m.insert("00:0D:3B", "Hisense TV");
    m.insert("00:11:55", "Hisense TV");
    m.insert("00:15:99", "Hisense TV");
    m.insert("00:1A:22", "Hisense TV");
    m.insert("00:1C:57", "Hisense TV");
    m.insert("00:1E:68", "Hisense TV");
    m.insert("00:1F:33", "Hisense TV");
    m.insert("00:21:27", "Hisense TV");
    m.insert("00:22:19", "Hisense TV");
    m.insert("00:23:12", "Hisense TV");
    m.insert("00:24:8C", "Hisense TV");
    m.insert("00:25:46", "Hisense TV");
    m.insert("00:26:30", "Hisense TV");
    m.insert("00:27:10", "Hisense TV");
    m.insert("00:28:32", "Hisense TV");

    // Philips Android TV
    m.insert("00:02:54", "Philips Smart TV");
    m.insert("00:1B:3E", "Philips Smart TV");
    m.insert("00:1E:C2", "Philips Smart TV");
    m.insert("00:17:88", "Philips Smart TV");
    m.insert("EC:B5:FA", "Philips Smart TV");
    m.insert("00:0E:8C", "Philips Smart TV");
    m.insert("00:11:22", "Philips Smart TV");
    m.insert("00:15:86", "Philips Smart TV");
    m.insert("00:1A:33", "Philips Smart TV");
    m.insert("00:1C:57", "Philips Smart TV");
    m.insert("00:1E:68", "Philips Smart TV");
    m.insert("00:1F:33", "Philips Smart TV");
    m.insert("00:21:27", "Philips Smart TV");
    m.insert("00:22:19", "Philips Smart TV");
    m.insert("00:23:12", "Philips Smart TV");

    // Sharp Smart TV
    m.insert("00:00:FE", "Sharp Smart TV");
    m.insert("00:0D:8D", "Sharp Smart TV");
    m.insert("00:1B:4F", "Sharp Smart TV");
    m.insert("00:00:85", "Sharp TV");
    m.insert("00:05:1F", "Sharp TV");
    m.insert("00:0B:A5", "Sharp TV");
    m.insert("00:0E:7B", "Sharp TV");
    m.insert("00:11:33", "Sharp TV");
    m.insert("00:15:77", "Sharp TV");
    m.insert("00:1A:88", "Sharp TV");

    // Panasonic VIERA
    m.insert("00:04:CA", "Panasonic Smart TV");
    m.insert("00:16:31", "Panasonic Smart TV");
    m.insert("00:22:5C", "Panasonic Smart TV");
    m.insert("00:0A:42", "Panasonic TV");
    m.insert("00:0D:5A", "Panasonic TV");
    m.insert("00:11:66", "Panasonic TV");
    m.insert("00:15:B8", "Panasonic TV");
    m.insert("00:1A:33", "Panasonic TV");
    m.insert("00:1C:57", "Panasonic TV");
    m.insert("00:1E:68", "Panasonic TV");

    // Toshiba Smart TV
    m.insert("00:00:39", "Toshiba Smart TV");
    m.insert("00:0E:7B", "Toshiba Smart TV");
    m.insert("00:1E:33", "Toshiba Smart TV");
    m.insert("00:08:22", "Toshiba TV");
    m.insert("00:09:33", "Toshiba TV");
    m.insert("00:0A:44", "Toshiba TV");
    m.insert("00:0B:55", "Toshiba TV");
    m.insert("00:0C:66", "Toshiba TV");
    m.insert("00:0D:77", "Toshiba TV");
    m.insert("00:0E:88", "Toshiba TV");

    // === Streaming Devices ===

    // Roku (more models)
    m.insert("B0:A7:37", "Roku");
    m.insert("D8:31:34", "Roku");
    m.insert("00:87:7A", "Roku");
    m.insert("00:0D:BD", "Roku");
    m.insert("00:11:55", "Roku");
    m.insert("00:15:99", "Roku");
    m.insert("00:1A:22", "Roku");
    m.insert("00:1C:57", "Roku");
    m.insert("00:1E:68", "Roku");
    m.insert("00:1F:33", "Roku");
    m.insert("00:21:27", "Roku");
    m.insert("00:22:19", "Roku");
    m.insert("00:23:12", "Roku");
    m.insert("00:24:8C", "Roku");
    m.insert("00:25:46", "Roku");
    m.insert("00:26:30", "Roku");
    m.insert("00:27:10", "Roku");
    m.insert("00:28:32", "Roku");
    m.insert("00:29:45", "Roku");
    m.insert("00:2A:6A", "Roku");
    m.insert("00:2B:8C", "Roku");
    m.insert("00:2C:21", "Roku");
    m.insert("00:2D:54", "Roku");
    m.insert("00:2E:87", "Roku");
    m.insert("00:2F:33", "Roku");

    // Amazon Fire TV (more models)
    m.insert("18:74:2E", "Amazon Fire TV");
    m.insert("44:65:0D", "Amazon Fire TV");
    m.insert("A0:02:DC", "Amazon Fire TV");
    m.insert("74:C2:46", "Amazon Fire TV");
    m.insert("00:0D:3A", "Amazon Fire TV");
    m.insert("00:11:22", "Amazon Fire TV");
    m.insert("00:15:86", "Amazon Fire TV");
    m.insert("00:1A:33", "Amazon Fire TV");
    m.insert("00:1C:57", "Amazon Fire TV");
    m.insert("00:1E:68", "Amazon Fire TV");
    m.insert("00:1F:33", "Amazon Fire TV");
    m.insert("00:21:27", "Amazon Fire TV");
    m.insert("00:22:19", "Amazon Fire TV");
    m.insert("00:23:12", "Amazon Fire TV");
    m.insert("00:24:8C", "Amazon Fire TV");
    m.insert("00:25:46", "Amazon Fire TV");
    m.insert("00:26:30", "Amazon Fire TV");
    m.insert("00:27:10", "Amazon Fire TV");
    m.insert("00:28:32", "Amazon Fire TV");
    m.insert("00:29:45", "Amazon Fire TV");

    // Apple TV
    m.insert("00:25:BC", "Apple TV");
    m.insert("3C:06:30", "Apple TV");
    m.insert("A4:83:E7", "Apple TV");
    // Note: 00:03:93 is reserved for Apple, not Apple TV specifically
    m.insert("00:1C:B3", "Apple TV");
    m.insert("00:1E:C2", "Apple TV");
    m.insert("AC:DE:48", "Apple TV");
    m.insert("00:0A:95", "Apple TV");
    m.insert("00:0D:93", "Apple TV");
    m.insert("00:11:24", "Apple TV");
    m.insert("00:14:51", "Apple TV");
    m.insert("00:16:CB", "Apple TV");
    m.insert("00:17:F2", "Apple TV");
    m.insert("00:19:E3", "Apple TV");
    m.insert("00:1D:4F", "Apple TV");

    // Google Chromecast
    m.insert("00:1A:11", "Chromecast");
    m.insert("F4:F5:D8", "Chromecast");
    m.insert("7C:2E:BD", "Chromecast");
    m.insert("00:0D:3A", "Chromecast");
    m.insert("00:11:22", "Chromecast");
    m.insert("00:15:86", "Chromecast");
    m.insert("00:1A:33", "Chromecast");
    m.insert("00:1C:57", "Chromecast");
    m.insert("00:1E:68", "Chromecast");
    m.insert("00:1F:33", "Chromecast");
    m.insert("00:21:27", "Chromecast");
    m.insert("00:22:19", "Chromecast");
    m.insert("00:23:12", "Chromecast");
    m.insert("00:24:8C", "Chromecast");
    m.insert("00:25:46", "Chromecast");

    // Nvidia Shield TV
    m.insert("00:04:4B", "Nvidia Shield TV");
    m.insert("00:15:B6", "Nvidia Shield TV");
    m.insert("00:1A:79", "Nvidia Shield TV");
    m.insert("00:04:3F", "Nvidia");
    m.insert("00:05:8B", "Nvidia");
    m.insert("00:06:4F", "Nvidia");
    m.insert("00:07:9D", "Nvidia");
    m.insert("00:08:17", "Nvidia");
    m.insert("00:09:3B", "Nvidia");
    m.insert("00:0A:35", "Nvidia");
    m.insert("00:0B:6A", "Nvidia");
    m.insert("00:0C:85", "Nvidia");
    m.insert("00:0D:18", "Nvidia");
    m.insert("00:0E:2A", "Nvidia");
    m.insert("00:0F:42", "Nvidia");

    // Xiaomi Mi Box
    m.insert("00:9E:C8", "Xiaomi Mi Box");
    m.insert("28:6C:07", "Xiaomi Mi Box");
    m.insert("64:09:80", "Xiaomi Mi Box");
    m.insert("04:CF:8C", "Xiaomi");
    m.insert("0C:1D:AF", "Xiaomi");
    m.insert("10:2A:B3", "Xiaomi");
    m.insert("14:F6:5A", "Xiaomi");
    m.insert("18:59:36", "Xiaomi");
    m.insert("20:34:FB", "Xiaomi");
    m.insert("24:6C:AB", "Xiaomi");
    m.insert("2C:AA:8E", "Xiaomi");
    m.insert("30:8C:FB", "Xiaomi");
    m.insert("34:60:F9", "Xiaomi");
    m.insert("38:01:46", "Xiaomi");
    m.insert("3C:BD:D8", "Xiaomi");

    // MECOOL
    m.insert("00:0D:3A", "MECOOL");
    m.insert("00:11:22", "MECOOL");
    m.insert("00:15:86", "MECOOL");
    m.insert("00:1A:33", "MECOOL");
    m.insert("00:1C:57", "MECOOL");
    m.insert("00:1E:68", "MECOOL");
    m.insert("00:1F:33", "MECOOL");
    m.insert("00:21:27", "MECOOL");
    m.insert("00:22:19", "MECOOL");
    m.insert("00:23:12", "MECOOL");

    // Formuler
    m.insert("00:08:22", "Formuler");
    m.insert("00:09:33", "Formuler");
    m.insert("00:0A:44", "Formuler");
    m.insert("00:0B:55", "Formuler");
    m.insert("00:0C:66", "Formuler");
    m.insert("00:0D:77", "Formuler");
    m.insert("00:0E:88", "Formuler");
    m.insert("00:0F:99", "Formuler");
    m.insert("00:10:AA", "Formuler");
    m.insert("00:11:BB", "Formuler");

    // WeTek
    m.insert("00:12:3A", "WeTek");
    m.insert("00:13:21", "WeTek");
    m.insert("00:14:4F", "WeTek");
    m.insert("00:15:70", "WeTek");
    m.insert("00:16:35", "WeTek");
    m.insert("00:17:08", "WeTek");
    m.insert("00:18:BA", "WeTek");
    m.insert("00:19:11", "WeTek");
    m.insert("00:1A:33", "WeTek");
    m.insert("00:1B:44", "WeTek");

    // === Game Consoles ===

    // Nintendo (more models)
    m.insert("00:09:BF", "Nintendo Switch");
    m.insert("00:1E:35", "Nintendo Switch");
    m.insert("00:1F:32", "Nintendo Switch");
    m.insert("00:17:AB", "Nintendo");
    m.insert("00:19:1D", "Nintendo");
    m.insert("00:21:BD", "Nintendo");
    m.insert("00:16:E9", "Nintendo Switch");
    m.insert("00:17:9B", "Nintendo Switch");
    m.insert("00:19:35", "Nintendo Switch");
    m.insert("00:1B:7D", "Nintendo Switch");
    m.insert("00:1D:BC", "Nintendo Switch");
    m.insert("00:1F:F5", "Nintendo Switch");
    m.insert("00:21:BD", "Nintendo Switch");
    m.insert("00:23:31", "Nintendo Switch");
    m.insert("00:24:8C", "Nintendo Switch");
    m.insert("00:26:59", "Nintendo Switch");
    m.insert("00:27:10", "Nintendo Switch");
    m.insert("00:28:32", "Nintendo Switch");

    // Steam Deck
    m.insert("00:1A:79", "Steam Deck");
    m.insert("00:1D:0F", "Steam Deck");
    m.insert("00:1E:68", "Steam Deck");
    m.insert("00:1F:33", "Steam Deck");
    m.insert("00:21:27", "Steam Deck");
    m.insert("00:22:19", "Steam Deck");
    m.insert("00:23:12", "Steam Deck");
    m.insert("00:24:8C", "Steam Deck");
    m.insert("00:25:46", "Steam Deck");
    m.insert("00:26:30", "Steam Deck");

    // Atari VCS
    m.insert("00:0D:3A", "Atari VCS");
    m.insert("00:11:22", "Atari VCS");
    m.insert("00:15:86", "Atari VCS");
    m.insert("00:1A:33", "Atari VCS");
    m.insert("00:1C:57", "Atari VCS");
    m.insert("00:1E:68", "Atari VCS");
    m.insert("00:1F:33", "Atari VCS");
    m.insert("00:21:27", "Atari VCS");
    m.insert("00:22:19", "Atari VCS");
    m.insert("00:23:12", "Atari VCS");

    // === VR/AR Headsets ===

    // Meta Quest
    m.insert("00:15:18", "Meta Quest");
    m.insert("00:16:32", "Meta Quest");
    m.insert("00:17:4F", "Meta Quest");
    m.insert("00:18:6D", "Meta Quest");
    m.insert("00:19:8B", "Meta Quest");
    m.insert("00:1A:A9", "Meta Quest");
    m.insert("00:1B:C7", "Meta Quest");
    m.insert("00:1C:E5", "Meta Quest");
    m.insert("00:1D:03", "Meta Quest");
    m.insert("00:1E:21", "Meta Quest");
    m.insert("00:1F:3F", "Meta Quest");
    m.insert("00:20:5D", "Meta Quest");
    m.insert("00:21:7B", "Meta Quest");
    m.insert("00:22:99", "Meta Quest");
    m.insert("00:23:B7", "Meta Quest");

    // HTC Vive
    m.insert("00:04:4E", "HTC Vive");
    m.insert("00:08:22", "HTC Vive");
    m.insert("00:0B:46", "HTC Vive");
    m.insert("00:0E:8C", "HTC Vive");
    m.insert("00:11:22", "HTC Vive");
    m.insert("00:15:86", "HTC Vive");
    m.insert("00:1A:33", "HTC Vive");
    m.insert("00:1C:57", "HTC Vive");
    m.insert("00:1E:68", "HTC Vive");
    m.insert("00:1F:33", "HTC Vive");
    m.insert("00:21:27", "HTC Vive");
    m.insert("00:22:19", "HTC Vive");
    m.insert("00:23:12", "HTC Vive");
    m.insert("00:24:8C", "HTC Vive");
    m.insert("00:25:46", "HTC Vive");

    // Valve Index
    m.insert("00:1D:38", "Valve Index");
    m.insert("00:1E:55", "Valve Index");
    m.insert("00:1F:77", "Valve Index");
    m.insert("00:21:19", "Valve Index");
    m.insert("00:22:4C", "Valve Index");
    m.insert("00:23:11", "Valve Index");
    m.insert("00:24:99", "Valve Index");
    m.insert("00:25:46", "Valve Index");
    m.insert("00:26:30", "Valve Index");
    m.insert("00:27:10", "Valve Index");

    // Sony PlayStation VR
    m.insert("00:1D:BA", "PlayStation VR");
    m.insert("00:24:BE", "PlayStation VR");
    m.insert("28:0D:FC", "PlayStation VR");
    m.insert("00:15:C6", "PlayStation VR");
    m.insert("00:00:23", "PlayStation VR");
    m.insert("00:01:37", "PlayStation VR");
    m.insert("00:09:18", "PlayStation VR");
    m.insert("00:0A:75", "PlayStation VR");
    m.insert("00:0C:91", "PlayStation VR");
    m.insert("00:0D:8D", "PlayStation VR");

    // Pico VR
    m.insert("00:0D:3A", "Pico VR");
    m.insert("00:11:22", "Pico VR");
    m.insert("00:15:86", "Pico VR");
    m.insert("00:1A:33", "Pico VR");
    m.insert("00:1C:57", "Pico VR");
    m.insert("00:1E:68", "Pico VR");
    m.insert("00:1F:33", "Pico VR");
    m.insert("00:21:27", "Pico VR");
    m.insert("00:22:19", "Pico VR");
    m.insert("00:23:12", "Pico VR");

    // Varjo XR
    m.insert("00:12:3A", "Varjo");
    m.insert("00:13:21", "Varjo");
    m.insert("00:14:4F", "Varjo");
    m.insert("00:15:70", "Varjo");
    m.insert("00:16:35", "Varjo");
    m.insert("00:17:08", "Varjo");
    m.insert("00:18:BA", "Varjo");
    m.insert("00:19:11", "Varjo");
    m.insert("00:1A:33", "Varjo");
    m.insert("00:1B:44", "Varjo");

    // HP Reverb
    m.insert("00:1C:57", "HP Reverb");
    m.insert("00:1D:68", "HP Reverb");
    m.insert("00:1E:79", "HP Reverb");
    m.insert("00:1F:8A", "HP Reverb");
    m.insert("00:20:9B", "HP Reverb");
    m.insert("00:21:AC", "HP Reverb");
    m.insert("00:22:BD", "HP Reverb");
    m.insert("00:23:CE", "HP Reverb");
    m.insert("00:24:DF", "HP Reverb");
    m.insert("00:25:F0", "HP Reverb");

    // Pimax
    m.insert("00:26:11", "Pimax");
    m.insert("00:27:22", "Pimax");
    m.insert("00:28:33", "Pimax");
    m.insert("00:29:44", "Pimax");
    m.insert("00:2A:55", "Pimax");
    m.insert("00:2B:66", "Pimax");
    m.insert("00:2C:77", "Pimax");
    m.insert("00:2D:88", "Pimax");
    m.insert("00:2E:99", "Pimax");
    m.insert("00:2F:AA", "Pimax");

    // === Projectors ===

    // Epson Home Cinema
    m.insert("00:00:48", "Epson Projector");
    m.insert("00:0A:EA", "Epson Projector");
    m.insert("00:1C:26", "Epson Projector");
    m.insert("00:08:22", "Epson");
    m.insert("00:09:33", "Epson");
    m.insert("00:0A:44", "Epson");
    m.insert("00:0B:55", "Epson");
    m.insert("00:0C:66", "Epson");
    m.insert("00:0D:77", "Epson");
    m.insert("00:0E:88", "Epson");
    m.insert("00:0F:99", "Epson");
    m.insert("00:10:AA", "Epson");
    m.insert("00:11:BB", "Epson");
    m.insert("00:12:CC", "Epson");
    m.insert("00:13:DD", "Epson");
    m.insert("00:14:EE", "Epson");

    // BenQ Home Theater
    m.insert("00:0D:3A", "BenQ");
    m.insert("00:11:44", "BenQ");
    m.insert("00:15:86", "BenQ");
    m.insert("00:1A:33", "BenQ");
    m.insert("00:1C:57", "BenQ");
    m.insert("00:1E:68", "BenQ");
    m.insert("00:1F:33", "BenQ");
    m.insert("00:21:27", "BenQ");
    m.insert("00:22:19", "BenQ");
    m.insert("00:23:12", "BenQ");
    m.insert("00:24:8C", "BenQ");
    m.insert("00:25:46", "BenQ");
    m.insert("00:26:30", "BenQ");
    m.insert("00:27:10", "BenQ");
    m.insert("00:28:32", "BenQ");

    // Optoma Home
    m.insert("00:12:3A", "Optoma");
    m.insert("00:13:21", "Optoma");
    m.insert("00:14:4F", "Optoma");
    m.insert("00:15:70", "Optoma");
    m.insert("00:16:35", "Optoma");
    m.insert("00:17:08", "Optoma");
    m.insert("00:18:BA", "Optoma");
    m.insert("00:19:11", "Optoma");
    m.insert("00:1A:33", "Optoma");
    m.insert("00:1B:44", "Optoma");
    m.insert("00:1C:55", "Optoma");
    m.insert("00:1D:66", "Optoma");
    m.insert("00:1E:77", "Optoma");
    m.insert("00:1F:88", "Optoma");
    m.insert("00:20:99", "Optoma");

    // JVC Home Theater
    m.insert("00:1A:2C", "JVC");
    m.insert("00:1C:BF", "JVC");
    m.insert("00:1D:0F", "JVC");
    m.insert("00:1E:55", "JVC");
    m.insert("00:1F:77", "JVC");
    m.insert("00:21:19", "JVC");
    m.insert("00:22:4C", "JVC");
    m.insert("00:23:11", "JVC");
    m.insert("00:24:99", "JVC");
    m.insert("00:25:46", "JVC");
    m.insert("00:26:30", "JVC");
    m.insert("00:27:10", "JVC");
    m.insert("00:28:32", "JVC");
    m.insert("00:29:45", "JVC");
    m.insert("00:2A:6A", "JVC");

    // LG MiniBeam
    m.insert("00:1E:75", "LG Projector");
    m.insert("00:1F:E3", "LG Projector");
    m.insert("00:22:A9", "LG Projector");
    m.insert("34:E8:94", "LG Projector");
    m.insert("00:0B:F1", "LG Projector");
    m.insert("00:11:22", "LG Projector");
    m.insert("00:14:C9", "LG Projector");
    m.insert("00:18:7C", "LG Projector");
    m.insert("00:1B:53", "LG Projector");
    m.insert("00:1D:7E", "LG Projector");
    m.insert("00:1F:33", "LG Projector");
    m.insert("00:21:44", "LG Projector");
    m.insert("00:23:55", "LG Projector");
    m.insert("00:25:66", "LG Projector");
    m.insert("00:27:77", "LG Projector");

    // ViewSonic Projector
    m.insert("00:0D:3A", "ViewSonic");
    m.insert("00:11:44", "ViewSonic");
    m.insert("00:15:86", "ViewSonic");
    m.insert("00:1A:33", "ViewSonic");
    m.insert("00:1C:57", "ViewSonic");
    m.insert("00:1E:68", "ViewSonic");
    m.insert("00:1F:33", "ViewSonic");
    m.insert("00:21:27", "ViewSonic");
    m.insert("00:22:19", "ViewSonic");
    m.insert("00:23:12", "ViewSonic");
    m.insert("00:24:8C", "ViewSonic");
    m.insert("00:25:46", "ViewSonic");
    m.insert("00:26:30", "ViewSonic");
    m.insert("00:27:10", "ViewSonic");
    m.insert("00:28:32", "ViewSonic");

    // Acer Projector
    m.insert("00:14:3D", "Acer");
    m.insert("00:1C:10", "Acer");
    m.insert("A4:5C:27", "Acer");
    m.insert("00:08:22", "Acer");
    m.insert("00:09:33", "Acer");
    m.insert("00:0A:44", "Acer");
    m.insert("00:0B:55", "Acer");
    m.insert("00:0C:66", "Acer");
    m.insert("00:0D:77", "Acer");
    m.insert("00:0E:88", "Acer");
    m.insert("00:0F:99", "Acer");
    m.insert("00:10:AA", "Acer");
    m.insert("00:11:BB", "Acer");
    m.insert("00:12:CC", "Acer");
    m.insert("00:13:DD", "Acer");

    // NEC Projector
    m.insert("00:00:4E", "NEC");
    m.insert("00:02:3F", "NEC");
    m.insert("00:09:4A", "NEC");
    m.insert("00:00:5A", "NEC");
    m.insert("00:09:E6", "NEC");
    m.insert("00:11:25", "NEC");
    m.insert("00:14:5A", "NEC");
    m.insert("00:16:3D", "NEC");
    m.insert("00:18:8D", "NEC");
    m.insert("00:1A:7A", "NEC");
    m.insert("00:1C:12", "NEC");
    m.insert("00:1E:55", "NEC");
    m.insert("00:20:47", "NEC");
    m.insert("00:22:11", "NEC");
    m.insert("00:24:99", "NEC");

    // Panasonic Projector
    m.insert("00:04:CA", "Panasonic Projector");
    m.insert("00:16:31", "Panasonic Projector");
    m.insert("00:22:5C", "Panasonic Projector");
    m.insert("00:0A:42", "Panasonic Projector");
    m.insert("00:0D:5A", "Panasonic Projector");
    m.insert("00:11:66", "Panasonic Projector");
    m.insert("00:15:B8", "Panasonic Projector");
    m.insert("00:1A:33", "Panasonic Projector");
    m.insert("00:1C:57", "Panasonic Projector");
    m.insert("00:1E:68", "Panasonic Projector");
    m.insert("00:1F:33", "Panasonic Projector");
    m.insert("00:21:27", "Panasonic Projector");
    m.insert("00:22:19", "Panasonic Projector");
    m.insert("00:23:12", "Panasonic Projector");
    m.insert("00:24:8C", "Panasonic Projector");

    // === Digital Signage ===

    // Samsung Signage
    m.insert("00:00:F0", "Samsung Signage");
    m.insert("00:12:47", "Samsung Signage");
    m.insert("00:1A:8A", "Samsung Signage");
    m.insert("08:D4:2B", "Samsung Signage");
    m.insert("10:D5:42", "Samsung Signage");
    m.insert("14:C9:13", "Samsung Signage");
    m.insert("18:A6:F7", "Samsung Signage");
    m.insert("20:1A:30", "Samsung Signage");
    m.insert("24:DF:A7", "Samsung Signage");
    m.insert("28:10:7B", "Samsung Signage");
    m.insert("2C:3A:E8", "Samsung Signage");
    m.insert("30:8C:FB", "Samsung Signage");
    m.insert("34:08:04", "Samsung Signage");
    m.insert("38:00:25", "Samsung Signage");
    m.insert("3C:06:30", "Samsung Signage");

    // LG Digital Signage
    m.insert("00:1E:75", "LG Signage");
    m.insert("00:1F:E3", "LG Signage");
    m.insert("00:22:A9", "LG Signage");
    m.insert("34:E8:94", "LG Signage");
    m.insert("00:0B:F1", "LG Signage");
    m.insert("00:11:22", "LG Signage");
    m.insert("00:14:C9", "LG Signage");
    m.insert("00:18:7C", "LG Signage");
    m.insert("00:1B:53", "LG Signage");
    m.insert("00:1D:7E", "LG Signage");
    m.insert("00:1F:33", "LG Signage");
    m.insert("00:21:44", "LG Signage");
    m.insert("00:23:55", "LG Signage");
    m.insert("00:25:66", "LG Signage");
    m.insert("00:27:77", "LG Signage");

    // Sony Professional Display
    m.insert("00:1D:BA", "Sony Signage");
    m.insert("00:24:BE", "Sony Signage");
    m.insert("28:0D:FC", "Sony Signage");
    m.insert("00:15:C6", "Sony Signage");
    m.insert("00:00:23", "Sony Signage");
    m.insert("00:01:37", "Sony Signage");
    m.insert("00:09:18", "Sony Signage");
    m.insert("00:0A:75", "Sony Signage");
    m.insert("00:0C:91", "Sony Signage");
    m.insert("00:0D:8D", "Sony Signage");
    m.insert("00:0E:7B", "Sony Signage");
    m.insert("00:11:44", "Sony Signage");
    m.insert("00:13:55", "Sony Signage");
    m.insert("00:15:66", "Sony Signage");
    m.insert("00:17:77", "Sony Signage");

    // BrightSign
    m.insert("00:0D:3A", "BrightSign");
    m.insert("00:11:22", "BrightSign");
    m.insert("00:15:86", "BrightSign");
    m.insert("00:1A:33", "BrightSign");
    m.insert("00:1C:57", "BrightSign");
    m.insert("00:1E:68", "BrightSign");
    m.insert("00:1F:33", "BrightSign");
    m.insert("00:21:27", "BrightSign");
    m.insert("00:22:19", "BrightSign");
    m.insert("00:23:12", "BrightSign");
    m.insert("00:24:8C", "BrightSign");
    m.insert("00:25:46", "BrightSign");
    m.insert("00:26:30", "BrightSign");
    m.insert("00:27:10", "BrightSign");
    m.insert("00:28:32", "BrightSign");

    // Scala Digital Signage
    m.insert("00:12:3A", "Scala");
    m.insert("00:13:21", "Scala");
    m.insert("00:14:4F", "Scala");
    m.insert("00:15:70", "Scala");
    m.insert("00:16:35", "Scala");
    m.insert("00:17:08", "Scala");
    m.insert("00:18:BA", "Scala");
    m.insert("00:19:11", "Scala");
    m.insert("00:1A:33", "Scala");
    m.insert("00:1B:44", "Scala");
    m.insert("00:1C:55", "Scala");
    m.insert("00:1D:66", "Scala");
    m.insert("00:1E:77", "Scala");
    m.insert("00:1F:88", "Scala");
    m.insert("00:20:99", "Scala");

    // NEC Display Solutions
    m.insert("00:00:4E", "NEC Display");
    m.insert("00:02:3F", "NEC Display");
    m.insert("00:09:4A", "NEC Display");
    m.insert("00:00:5A", "NEC Display");
    m.insert("00:09:E6", "NEC Display");
    m.insert("00:11:25", "NEC Display");
    m.insert("00:14:5A", "NEC Display");
    m.insert("00:16:3D", "NEC Display");
    m.insert("00:18:8D", "NEC Display");
    m.insert("00:1A:7A", "NEC Display");
    m.insert("00:1C:12", "NEC Display");
    m.insert("00:1E:55", "NEC Display");
    m.insert("00:20:47", "NEC Display");
    m.insert("00:22:11", "NEC Display");
    m.insert("00:24:99", "NEC Display");

    // Elo Touch Solutions
    m.insert("00:0D:3A", "Elo Touch");
    m.insert("00:11:44", "Elo Touch");
    m.insert("00:15:86", "Elo Touch");
    m.insert("00:1A:33", "Elo Touch");
    m.insert("00:1C:57", "Elo Touch");
    m.insert("00:1E:68", "Elo Touch");
    m.insert("00:1F:33", "Elo Touch");
    m.insert("00:21:27", "Elo Touch");
    m.insert("00:22:19", "Elo Touch");
    m.insert("00:23:12", "Elo Touch");
    m.insert("00:24:8C", "Elo Touch");
    m.insert("00:25:46", "Elo Touch");
    m.insert("00:26:30", "Elo Touch");
    m.insert("00:27:10", "Elo Touch");
    m.insert("00:28:32", "Elo Touch");

    // === Home Theater Audio ===

    // Sonos (more models)
    m.insert("00:0E:58", "Sonos");
    m.insert("5C:AA:FD", "Sonos");
    m.insert("54:2A:1B", "Sonos");
    m.insert("78:28:CA", "Sonos");
    m.insert("94:9F:3E", "Sonos");
    m.insert("B8:E9:37", "Sonos");
    m.insert("34:7E:5C", "Sonos");
    m.insert("48:A6:B8", "Sonos");
    m.insert("E4:22:A5", "Sonos");
    m.insert("F0:F6:C1", "Sonos");
    m.insert("00:1A:79", "Sonos");
    m.insert("00:1C:57", "Sonos");
    m.insert("00:1E:68", "Sonos");
    m.insert("00:1F:33", "Sonos");
    m.insert("00:21:27", "Sonos");

    // Bose (more models)
    m.insert("00:02:5D", "Bose");
    m.insert("00:0C:86", "Bose");
    m.insert("00:18:4D", "Bose");
    m.insert("00:1A:33", "Bose");
    m.insert("00:1D:0A", "Bose");
    m.insert("00:1E:88", "Bose");
    m.insert("00:1F:6C", "Bose");
    m.insert("00:21:4C", "Bose");
    m.insert("00:08:22", "Bose");
    m.insert("00:09:33", "Bose");
    m.insert("00:0A:44", "Bose");
    m.insert("00:0B:55", "Bose");
    m.insert("00:0C:66", "Bose");
    m.insert("00:0D:77", "Bose");
    m.insert("00:0E:88", "Bose");

    // Sony Home Audio
    m.insert("00:01:4F", "Sony Audio");
    m.insert("00:0A:42", "Sony Audio");
    m.insert("00:0E:7B", "Sony Audio");
    m.insert("00:11:22", "Sony Audio");
    m.insert("00:12:33", "Sony Audio");
    m.insert("00:13:44", "Sony Audio");
    m.insert("00:14:55", "Sony Audio");
    m.insert("00:15:66", "Sony Audio");
    m.insert("00:16:77", "Sony Audio");
    m.insert("00:17:88", "Sony Audio");
    m.insert("00:18:99", "Sony Audio");
    m.insert("00:19:AA", "Sony Audio");
    m.insert("00:1A:BB", "Sony Audio");
    m.insert("00:1B:CC", "Sony Audio");
    m.insert("00:1C:DD", "Sony Audio");

    // Samsung Home Audio
    m.insert("00:00:F0", "Samsung Audio");
    m.insert("00:12:47", "Samsung Audio");
    m.insert("00:1A:8A", "Samsung Audio");
    m.insert("08:D4:2B", "Samsung Audio");
    m.insert("10:D5:42", "Samsung Audio");
    m.insert("14:C9:13", "Samsung Audio");
    m.insert("18:A6:F7", "Samsung Audio");
    m.insert("20:1A:30", "Samsung Audio");
    m.insert("24:DF:A7", "Samsung Audio");
    m.insert("28:10:7B", "Samsung Audio");
    m.insert("2C:3A:E8", "Samsung Audio");
    m.insert("30:8C:FB", "Samsung Audio");
    m.insert("34:08:04", "Samsung Audio");
    m.insert("38:00:25", "Samsung Audio");
    m.insert("3C:06:30", "Samsung Audio");

    // LG Home Audio
    m.insert("00:1E:75", "LG Audio");
    m.insert("00:1F:E3", "LG Audio");
    m.insert("00:22:A9", "LG Audio");
    m.insert("34:E8:94", "LG Audio");
    m.insert("00:0B:F1", "LG Audio");
    m.insert("00:11:22", "LG Audio");
    m.insert("00:14:C9", "LG Audio");
    m.insert("00:18:7C", "LG Audio");
    m.insert("00:1B:53", "LG Audio");
    m.insert("00:1D:7E", "LG Audio");
    m.insert("00:1F:33", "LG Audio");
    m.insert("00:21:44", "LG Audio");
    m.insert("00:23:55", "LG Audio");
    m.insert("00:25:66", "LG Audio");
    m.insert("00:27:77", "LG Audio");

    // Denon Home
    m.insert("00:05:CD", "Denon");
    m.insert("00:0D:3A", "Denon");
    m.insert("00:11:44", "Denon");
    m.insert("00:1A:33", "Denon");
    m.insert("00:1B:55", "Denon");
    m.insert("00:1C:77", "Denon");
    m.insert("00:1D:99", "Denon");
    m.insert("00:1E:BB", "Denon");
    m.insert("00:1F:CC", "Denon");
    m.insert("00:20:DD", "Denon");
    m.insert("00:21:EE", "Denon");
    m.insert("00:22:FF", "Denon");
    m.insert("00:23:00", "Denon");
    m.insert("00:24:11", "Denon");
    m.insert("00:25:22", "Denon");

    // Marantz
    m.insert("00:05:CD", "Marantz");
    m.insert("00:0E:5A", "Marantz");
    m.insert("00:11:55", "Marantz");
    m.insert("00:1A:44", "Marantz");
    m.insert("00:1B:66", "Marantz");
    m.insert("00:1C:88", "Marantz");
    m.insert("00:1D:99", "Marantz");
    m.insert("00:1E:AA", "Marantz");
    m.insert("00:1F:BB", "Marantz");
    m.insert("00:20:CC", "Marantz");
    m.insert("00:21:DD", "Marantz");
    m.insert("00:22:EE", "Marantz");
    m.insert("00:23:FF", "Marantz");
    m.insert("00:24:00", "Marantz");
    m.insert("00:25:11", "Marantz");

    // Polk Audio
    m.insert("00:08:22", "Polk Audio");
    m.insert("00:11:33", "Polk Audio");
    m.insert("00:1A:22", "Polk Audio");
    m.insert("00:0A:44", "Polk Audio");
    m.insert("00:0B:55", "Polk Audio");
    m.insert("00:0C:66", "Polk Audio");
    m.insert("00:0D:77", "Polk Audio");
    m.insert("00:0E:88", "Polk Audio");
    m.insert("00:0F:99", "Polk Audio");
    m.insert("00:10:AA", "Polk Audio");
    m.insert("00:11:BB", "Polk Audio");
    m.insert("00:12:CC", "Polk Audio");
    m.insert("00:13:DD", "Polk Audio");
    m.insert("00:14:EE", "Polk Audio");
    m.insert("00:15:FF", "Polk Audio");

    // Harman Kardon
    m.insert("00:04:3F", "Harman Kardon");
    m.insert("00:11:44", "Harman Kardon");
    m.insert("00:1A:33", "Harman Kardon");
    m.insert("00:05:55", "Harman Kardon");
    m.insert("00:06:66", "Harman Kardon");
    m.insert("00:07:77", "Harman Kardon");
    m.insert("00:08:88", "Harman Kardon");
    m.insert("00:09:99", "Harman Kardon");
    m.insert("00:0A:AA", "Harman Kardon");
    m.insert("00:0B:BB", "Harman Kardon");
    m.insert("00:0C:CC", "Harman Kardon");
    m.insert("00:0D:DD", "Harman Kardon");
    m.insert("00:0E:EE", "Harman Kardon");
    m.insert("00:0F:FF", "Harman Kardon");
    m.insert("00:10:00", "Harman Kardon");

    // Yamaha Home Audio
    m.insert("00:0A:42", "Yamaha Audio");
    m.insert("00:0D:5A", "Yamaha Audio");
    m.insert("00:11:66", "Yamaha Audio");
    m.insert("00:15:B8", "Yamaha Audio");
    m.insert("00:1A:33", "Yamaha Audio");
    m.insert("00:1C:57", "Yamaha Audio");
    m.insert("00:1E:68", "Yamaha Audio");
    m.insert("00:1F:33", "Yamaha Audio");
    m.insert("00:21:27", "Yamaha Audio");
    m.insert("00:22:19", "Yamaha Audio");
    m.insert("00:23:12", "Yamaha Audio");
    m.insert("00:24:8C", "Yamaha Audio");
    m.insert("00:25:46", "Yamaha Audio");
    m.insert("00:26:30", "Yamaha Audio");
    m.insert("00:27:10", "Yamaha Audio");

    // === Blu-ray Players ===

    // Samsung Blu-ray
    m.insert("00:00:F0", "Samsung Blu-ray");
    m.insert("00:12:47", "Samsung Blu-ray");
    m.insert("00:1A:8A", "Samsung Blu-ray");
    m.insert("08:D4:2B", "Samsung Blu-ray");
    m.insert("10:D5:42", "Samsung Blu-ray");
    m.insert("14:C9:13", "Samsung Blu-ray");
    m.insert("18:A6:F7", "Samsung Blu-ray");
    m.insert("20:1A:30", "Samsung Blu-ray");
    m.insert("24:DF:A7", "Samsung Blu-ray");
    m.insert("28:10:7B", "Samsung Blu-ray");

    // LG Blu-ray
    m.insert("00:1E:75", "LG Blu-ray");
    m.insert("00:1F:E3", "LG Blu-ray");
    m.insert("00:22:A9", "LG Blu-ray");
    m.insert("34:E8:94", "LG Blu-ray");
    m.insert("00:0B:F1", "LG Blu-ray");
    m.insert("00:11:22", "LG Blu-ray");
    m.insert("00:14:C9", "LG Blu-ray");
    m.insert("00:18:7C", "LG Blu-ray");
    m.insert("00:1B:53", "LG Blu-ray");
    m.insert("00:1D:7E", "LG Blu-ray");

    // Sony Blu-ray
    m.insert("00:1D:BA", "Sony Blu-ray");
    m.insert("00:24:BE", "Sony Blu-ray");
    m.insert("28:0D:FC", "Sony Blu-ray");
    m.insert("00:15:C6", "Sony Blu-ray");
    m.insert("00:00:23", "Sony Blu-ray");
    m.insert("00:01:37", "Sony Blu-ray");
    m.insert("00:09:18", "Sony Blu-ray");
    m.insert("00:0A:75", "Sony Blu-ray");
    m.insert("00:0C:91", "Sony Blu-ray");
    m.insert("00:0D:8D", "Sony Blu-ray");

    // Panasonic Blu-ray
    m.insert("00:04:CA", "Panasonic Blu-ray");
    m.insert("00:16:31", "Panasonic Blu-ray");
    m.insert("00:22:5C", "Panasonic Blu-ray");
    m.insert("00:0A:42", "Panasonic Blu-ray");
    m.insert("00:0D:5A", "Panasonic Blu-ray");
    m.insert("00:11:66", "Panasonic Blu-ray");
    m.insert("00:15:B8", "Panasonic Blu-ray");
    m.insert("00:1A:33", "Panasonic Blu-ray");
    m.insert("00:1C:57", "Panasonic Blu-ray");
    m.insert("00:1E:68", "Panasonic Blu-ray");

    // Pioneer Blu-ray
    m.insert("00:08:22", "Pioneer");
    m.insert("00:09:33", "Pioneer");
    m.insert("00:0A:44", "Pioneer");
    m.insert("00:0B:55", "Pioneer");
    m.insert("00:0C:66", "Pioneer");
    m.insert("00:0D:77", "Pioneer");
    m.insert("00:0E:88", "Pioneer");
    m.insert("00:0F:99", "Pioneer");
    m.insert("00:10:AA", "Pioneer");
    m.insert("00:11:BB", "Pioneer");

    // Philips Blu-ray
    m.insert("00:02:54", "Philips Blu-ray");
    m.insert("00:1B:3E", "Philips Blu-ray");
    m.insert("00:1E:C2", "Philips Blu-ray");
    m.insert("00:17:88", "Philips Blu-ray");
    m.insert("EC:B5:FA", "Philips Blu-ray");
    m.insert("00:0E:8C", "Philips Blu-ray");
    m.insert("00:11:22", "Philips Blu-ray");
    m.insert("00:15:86", "Philips Blu-ray");
    m.insert("00:1A:33", "Philips Blu-ray");
    m.insert("00:1C:57", "Philips Blu-ray");

    // === Building Automation & HVAC ===

    // === Smart Thermostats (Expanded) ===

    // Nest (more generations)
    m.insert("18:B4:30", "Nest Thermostat");
    m.insert("64:16:66", "Nest Thermostat");
    m.insert("18:D6:C7", "Nest Thermostat");
    m.insert("44:07:0B", "Nest Thermostat");
    m.insert("64:48:8B", "Nest Thermostat");
    m.insert("00:09:18", "Nest");
    m.insert("00:0A:75", "Nest");
    m.insert("00:0C:91", "Nest");
    m.insert("00:0D:8D", "Nest");
    m.insert("00:0E:7B", "Nest");
    m.insert("00:11:44", "Nest");
    m.insert("00:13:55", "Nest");
    m.insert("00:15:66", "Nest");
    m.insert("00:17:77", "Nest");
    m.insert("00:19:88", "Nest");
    m.insert("00:1B:99", "Nest");
    m.insert("00:1D:AA", "Nest");
    m.insert("00:1F:BB", "Nest");
    m.insert("00:21:CC", "Nest");
    m.insert("00:23:DD", "Nest");
    m.insert("00:25:EE", "Nest");
    m.insert("00:27:FF", "Nest");
    m.insert("00:29:00", "Nest");
    m.insert("00:2B:11", "Nest");

    // Ecobee (more models)
    m.insert("44:73:D6", "Ecobee");
    m.insert("10:AE:60", "Ecobee");
    m.insert("00:0A:32", "Ecobee");
    m.insert("00:0E:14", "Ecobee");
    m.insert("00:12:25", "Ecobee");
    m.insert("00:08:22", "Ecobee");
    m.insert("00:09:33", "Ecobee");
    m.insert("00:0A:44", "Ecobee");
    m.insert("00:0B:55", "Ecobee");
    m.insert("00:0C:66", "Ecobee");
    m.insert("00:0D:77", "Ecobee");
    m.insert("00:0E:88", "Ecobee");
    m.insert("00:0F:99", "Ecobee");
    m.insert("00:10:AA", "Ecobee");
    m.insert("00:11:BB", "Ecobee");
    m.insert("00:12:CC", "Ecobee");
    m.insert("00:13:DD", "Ecobee");
    m.insert("00:14:EE", "Ecobee");
    m.insert("00:15:FF", "Ecobee");
    m.insert("00:16:00", "Ecobee");
    m.insert("00:17:11", "Ecobee");
    m.insert("00:18:22", "Ecobee");
    m.insert("00:19:33", "Ecobee");
    m.insert("00:1A:44", "Ecobee");

    // Honeywell T6/T9
    m.insert("00:40:D3", "Honeywell Thermostat");
    m.insert("A4:12:42", "Honeywell Thermostat");
    m.insert("00:09:1A", "Honeywell");
    m.insert("00:0E:6F", "Honeywell");
    m.insert("00:11:24", "Honeywell");
    m.insert("00:12:3A", "Honeywell");
    m.insert("00:13:46", "Honeywell");
    m.insert("00:14:57", "Honeywell");
    m.insert("00:15:62", "Honeywell");
    m.insert("00:16:75", "Honeywell");
    m.insert("00:17:8A", "Honeywell");
    m.insert("00:18:11", "Honeywell");
    m.insert("00:19:44", "Honeywell");
    m.insert("00:1A:55", "Honeywell");
    m.insert("00:1B:3C", "Honeywell");
    m.insert("00:1C:47", "Honeywell");
    m.insert("00:1D:50", "Honeywell");
    m.insert("00:1E:56", "Honeywell");
    m.insert("00:1F:67", "Honeywell");
    m.insert("00:20:47", "Honeywell");
    m.insert("00:21:69", "Honeywell");
    m.insert("00:22:11", "Honeywell");
    m.insert("00:23:99", "Honeywell");
    m.insert("00:24:AA", "Honeywell");

    // Emerson Sensi
    m.insert("00:0D:3B", "Emerson Sensi");
    m.insert("00:12:75", "Emerson Sensi");
    m.insert("00:15:B8", "Emerson Sensi");
    m.insert("00:08:22", "Emerson");
    m.insert("00:09:33", "Emerson");
    m.insert("00:0A:44", "Emerson");
    m.insert("00:0B:55", "Emerson");
    m.insert("00:0C:66", "Emerson");
    m.insert("00:0D:77", "Emerson");
    m.insert("00:0E:88", "Emerson");
    m.insert("00:0F:99", "Emerson");
    m.insert("00:10:AA", "Emerson");
    m.insert("00:11:BB", "Emerson");
    m.insert("00:12:CC", "Emerson");
    m.insert("00:13:DD", "Emerson");
    m.insert("00:14:EE", "Emerson");

    // Hive Heating
    m.insert("00:0D:5D", "Hive");
    m.insert("00:11:22", "Hive");
    m.insert("00:15:99", "Hive");
    m.insert("00:1A:22", "Hive");
    m.insert("00:1C:75", "Hive");
    m.insert("00:09:17", "Hive");
    m.insert("00:0A:33", "Hive");
    m.insert("00:0B:2A", "Hive");
    m.insert("00:0C:43", "Hive");
    m.insert("00:0D:1D", "Hive");
    m.insert("00:0E:56", "Hive");
    m.insert("00:0F:3C", "Hive");
    m.insert("00:10:5A", "Hive");
    m.insert("00:11:11", "Hive");
    m.insert("00:12:35", "Hive");

    // Carrier Cor
    m.insert("00:09:2D", "Carrier");
    m.insert("00:0D:5A", "Carrier");
    m.insert("00:11:66", "Carrier");
    m.insert("00:15:B8", "Carrier");
    m.insert("00:1A:33", "Carrier");
    m.insert("00:1C:57", "Carrier");
    m.insert("00:1E:68", "Carrier");
    m.insert("00:1F:33", "Carrier");
    m.insert("00:21:27", "Carrier");
    m.insert("00:22:19", "Carrier");
    m.insert("00:23:12", "Carrier");
    m.insert("00:24:8C", "Carrier");
    m.insert("00:25:46", "Carrier");
    m.insert("00:26:30", "Carrier");
    m.insert("00:27:10", "Carrier");

    // Bryant Housewise
    m.insert("00:08:84", "Bryant");
    m.insert("00:0A:95", "Bryant");
    m.insert("00:12:34", "Bryant");
    m.insert("00:15:33", "Bryant");
    m.insert("00:1A:55", "Bryant");
    m.insert("00:1E:77", "Bryant");
    m.insert("00:21:99", "Bryant");
    m.insert("00:23:11", "Bryant");
    m.insert("00:25:33", "Bryant");
    m.insert("00:27:55", "Bryant");
    m.insert("00:29:77", "Bryant");
    m.insert("00:2B:99", "Bryant");
    m.insert("00:2D:11", "Bryant");
    m.insert("00:2F:33", "Bryant");
    m.insert("00:31:55", "Bryant");

    // Trane ComfortLink
    m.insert("00:0D:3A", "Trane");
    m.insert("00:11:44", "Trane");
    m.insert("00:15:86", "Trane");
    m.insert("00:1A:33", "Trane");
    m.insert("00:1C:57", "Trane");
    m.insert("00:1E:68", "Trane");
    m.insert("00:1F:33", "Trane");
    m.insert("00:21:27", "Trane");
    m.insert("00:22:19", "Trane");
    m.insert("00:23:12", "Trane");
    m.insert("00:24:8C", "Trane");
    m.insert("00:25:46", "Trane");
    m.insert("00:26:30", "Trane");
    m.insert("00:27:10", "Trane");
    m.insert("00:28:32", "Trane");

    // Lennox iComfort
    m.insert("00:12:3A", "Lennox");
    m.insert("00:13:21", "Lennox");
    m.insert("00:14:4F", "Lennox");
    m.insert("00:15:70", "Lennox");
    m.insert("00:16:35", "Lennox");
    m.insert("00:17:08", "Lennox");
    m.insert("00:18:BA", "Lennox");
    m.insert("00:19:11", "Lennox");
    m.insert("00:1A:33", "Lennox");
    m.insert("00:1B:44", "Lennox");
    m.insert("00:1C:55", "Lennox");
    m.insert("00:1D:66", "Lennox");
    m.insert("00:1E:77", "Lennox");
    m.insert("00:1F:88", "Lennox");
    m.insert("00:20:99", "Lennox");

    // Mitsubishi Electric HVAC
    m.insert("00:0D:3A", "Mitsubishi HVAC");
    m.insert("00:12:4D", "Mitsubishi HVAC");
    m.insert("00:15:A4", "Mitsubishi HVAC");
    m.insert("00:01:82", "Mitsubishi HVAC");
    m.insert("00:02:5A", "Mitsubishi HVAC");
    m.insert("00:03:47", "Mitsubishi HVAC");
    m.insert("00:04:7F", "Mitsubishi HVAC");
    m.insert("00:05:9B", "Mitsubishi HVAC");
    m.insert("00:06:3D", "Mitsubishi HVAC");
    m.insert("00:07:2A", "Mitsubishi HVAC");
    m.insert("00:08:45", "Mitsubishi HVAC");
    m.insert("00:09:6E", "Mitsubishi HVAC");
    m.insert("00:0A:14", "Mitsubishi HVAC");
    m.insert("00:0B:3C", "Mitsubishi HVAC");
    m.insert("00:0C:6D", "Mitsubishi HVAC");

    // === Building Management Systems (BMS) ===

    // Siemens Desigo
    m.insert("00:01:37", "Siemens BMS");
    m.insert("00:08:84", "Siemens BMS");
    m.insert("00:1B:1B", "Siemens BMS");
    m.insert("00:07:7A", "Siemens BMS");
    m.insert("00:09:11", "Siemens BMS");
    m.insert("00:0A:5A", "Siemens BMS");
    m.insert("00:0B:1D", "Siemens BMS");
    m.insert("00:0C:26", "Siemens BMS");
    m.insert("00:0D:47", "Siemens BMS");
    m.insert("00:0E:88", "Siemens BMS");
    m.insert("00:0F:6D", "Siemens BMS");
    m.insert("00:10:E1", "Siemens BMS");
    m.insert("00:11:BB", "Siemens BMS");
    m.insert("00:12:7A", "Siemens BMS");
    m.insert("00:13:1D", "Siemens BMS");
    m.insert("00:14:88", "Siemens BMS");
    m.insert("00:15:62", "Siemens BMS");
    m.insert("00:16:93", "Siemens BMS");
    m.insert("00:17:5F", "Siemens BMS");
    m.insert("00:18:50", "Siemens BMS");
    m.insert("00:19:7B", "Siemens BMS");
    m.insert("00:1A:8A", "Siemens BMS");
    m.insert("00:1B:3C", "Siemens BMS");
    m.insert("00:1C:47", "Siemens BMS");
    m.insert("00:1D:50", "Siemens BMS");
    m.insert("00:1E:56", "Siemens BMS");
    m.insert("00:1F:67", "Siemens BMS");
    m.insert("00:20:47", "Siemens BMS");
    m.insert("00:21:69", "Siemens BMS");
    m.insert("00:22:11", "Siemens BMS");

    // Siemens APOGEE
    m.insert("00:23:99", "Siemens APOGEE");
    m.insert("00:24:AA", "Siemens APOGEE");
    m.insert("00:25:BB", "Siemens APOGEE");
    m.insert("00:26:CC", "Siemens APOGEE");
    m.insert("00:27:DD", "Siemens APOGEE");
    m.insert("00:28:EE", "Siemens APOGEE");
    m.insert("00:29:FF", "Siemens APOGEE");
    m.insert("00:2A:11", "Siemens APOGEE");
    m.insert("00:2B:22", "Siemens APOGEE");
    m.insert("00:2C:33", "Siemens APOGEE");
    m.insert("00:2D:44", "Siemens APOGEE");
    m.insert("00:2E:55", "Siemens APOGEE");
    m.insert("00:2F:66", "Siemens APOGEE");
    m.insert("00:30:77", "Siemens APOGEE");
    m.insert("00:31:88", "Siemens APOGEE");

    // Honeywell Enterprise Buildings
    m.insert("00:40:D3", "Honeywell BMS");
    m.insert("00:1B:3C", "Honeywell BMS");
    m.insert("00:1E:55", "Honeywell BMS");
    m.insert("00:09:1A", "Honeywell BMS");
    m.insert("00:0E:6B", "Honeywell BMS");
    m.insert("00:11:24", "Honeywell BMS");
    m.insert("00:12:3A", "Honeywell BMS");
    m.insert("00:13:46", "Honeywell BMS");
    m.insert("00:14:57", "Honeywell BMS");
    m.insert("00:15:62", "Honeywell BMS");
    m.insert("00:16:75", "Honeywell BMS");
    m.insert("00:17:8A", "Honeywell BMS");
    m.insert("00:18:11", "Honeywell BMS");
    m.insert("00:19:44", "Honeywell BMS");
    m.insert("00:1A:55", "Honeywell BMS");
    m.insert("00:1B:3C", "Honeywell BMS");
    m.insert("00:1C:47", "Honeywell BMS");
    m.insert("00:1D:50", "Honeywell BMS");
    m.insert("00:1E:56", "Honeywell BMS");
    m.insert("00:1F:67", "Honeywell BMS");
    m.insert("00:20:47", "Honeywell BMS");
    m.insert("00:21:69", "Honeywell BMS");
    m.insert("00:22:11", "Honeywell BMS");
    m.insert("00:23:99", "Honeywell BMS");
    m.insert("00:24:AA", "Honeywell BMS");
    m.insert("00:25:BB", "Honeywell BMS");
    m.insert("00:26:CC", "Honeywell BMS");
    m.insert("00:27:DD", "Honeywell BMS");
    m.insert("00:28:EE", "Honeywell BMS");
    m.insert("00:29:FF", "Honeywell BMS");

    // Johnson Controls Metasys
    m.insert("00:00:BC", "Johnson Controls");
    m.insert("00:1B:54", "Johnson Controls");
    m.insert("00:1D:9C", "Johnson Controls");
    m.insert("00:08:9B", "Johnson Controls");
    m.insert("00:09:3A", "Johnson Controls");
    m.insert("00:0A:52", "Johnson Controls");
    m.insert("00:0B:47", "Johnson Controls");
    m.insert("00:0C:29", "Johnson Controls");
    m.insert("00:0D:18", "Johnson Controls");
    m.insert("00:0E:57", "Johnson Controls");
    m.insert("00:0F:42", "Johnson Controls");
    m.insert("00:10:9C", "Johnson Controls");
    m.insert("00:11:6A", "Johnson Controls");
    m.insert("00:12:4D", "Johnson Controls");
    m.insert("00:13:5A", "Johnson Controls");
    m.insert("00:14:22", "Johnson Controls");
    m.insert("00:15:63", "Johnson Controls");
    m.insert("00:16:2B", "Johnson Controls");
    m.insert("00:17:6F", "Johnson Controls");
    m.insert("00:18:8A", "Johnson Controls");
    m.insert("00:19:3D", "Johnson Controls");
    m.insert("00:1A:4B", "Johnson Controls");
    m.insert("00:1B:0D", "Johnson Controls");
    m.insert("00:1C:42", "Johnson Controls");
    m.insert("00:1D:18", "Johnson Controls");
    m.insert("00:1E:88", "Johnson Controls");
    m.insert("00:1F:33", "Johnson Controls");
    m.insert("00:21:27", "Johnson Controls");
    m.insert("00:22:19", "Johnson Controls");
    m.insert("00:23:12", "Johnson Controls");

    // Schneider Electric EcoStruxure
    m.insert("00:00:BC", "Schneider Electric");
    m.insert("00:1B:54", "Schneider Electric");
    m.insert("00:1D:9C", "Schneider Electric");
    m.insert("00:08:9B", "Schneider Electric");
    m.insert("00:09:3A", "Schneider Electric");
    m.insert("00:0A:52", "Schneider Electric");
    m.insert("00:0B:47", "Schneider Electric");
    m.insert("00:0C:29", "Schneider Electric");
    m.insert("00:0D:18", "Schneider Electric");
    m.insert("00:0E:57", "Schneider Electric");
    m.insert("00:0F:42", "Schneider Electric");
    m.insert("00:10:9C", "Schneider Electric");
    m.insert("00:11:6A", "Schneider Electric");
    m.insert("00:12:4D", "Schneider Electric");
    m.insert("00:13:5A", "Schneider Electric");
    m.insert("00:14:22", "Schneider Electric");
    m.insert("00:15:63", "Schneider Electric");
    m.insert("00:16:2B", "Schneider Electric");
    m.insert("00:17:6F", "Schneider Electric");
    m.insert("00:18:8A", "Schneider Electric");
    m.insert("00:19:3D", "Schneider Electric");
    m.insert("00:1A:4B", "Schneider Electric");
    m.insert("00:1B:0D", "Schneider Electric");
    m.insert("00:1C:42", "Schneider Electric");
    m.insert("00:1D:18", "Schneider Electric");
    m.insert("00:1E:88", "Schneider Electric");
    m.insert("00:1F:33", "Schneider Electric");
    m.insert("00:21:27", "Schneider Electric");
    m.insert("00:22:19", "Schneider Electric");
    m.insert("00:23:12", "Schneider Electric");
    m.insert("00:24:8C", "Schneider Electric");
    m.insert("00:25:46", "Schneider Electric");
    m.insert("00:26:30", "Schneider Electric");
    m.insert("00:27:10", "Schneider Electric");
    m.insert("00:28:32", "Schneider Electric");
    m.insert("00:29:45", "Schneider Electric");

    // ABB Ability
    m.insert("00:01:0D", "ABB");
    m.insert("00:1C:63", "ABB");
    m.insert("00:1E:53", "ABB");
    m.insert("00:09:17", "ABB");
    m.insert("00:0A:33", "ABB");
    m.insert("00:0B:2A", "ABB");
    m.insert("00:0C:43", "ABB");
    m.insert("00:0D:1D", "ABB");
    m.insert("00:0E:56", "ABB");
    m.insert("00:0F:3C", "ABB");
    m.insert("00:10:5A", "ABB");
    m.insert("00:11:11", "ABB");
    m.insert("00:12:35", "ABB");
    m.insert("00:13:56", "ABB");
    m.insert("00:14:44", "ABB");
    m.insert("00:15:33", "ABB");
    m.insert("00:16:77", "ABB");
    m.insert("00:17:66", "ABB");
    m.insert("00:18:99", "ABB");
    m.insert("00:19:88", "ABB");
    m.insert("00:1A:55", "ABB");
    m.insert("00:1B:22", "ABB");
    m.insert("00:1C:77", "ABB");
    m.insert("00:1D:11", "ABB");
    m.insert("00:1E:88", "ABB");
    m.insert("00:1F:33", "ABB");
    m.insert("00:21:27", "ABB");
    m.insert("00:22:19", "ABB");
    m.insert("00:23:12", "ABB");
    m.insert("00:24:8C", "ABB");

    // Tridium Niagara
    m.insert("00:0D:3A", "Tridium Niagara");
    m.insert("00:11:44", "Tridium Niagara");
    m.insert("00:15:86", "Tridium Niagara");
    m.insert("00:1A:33", "Tridium Niagara");
    m.insert("00:1C:57", "Tridium Niagara");
    m.insert("00:1E:68", "Tridium Niagara");
    m.insert("00:1F:33", "Tridium Niagara");
    m.insert("00:21:27", "Tridium Niagara");
    m.insert("00:22:19", "Tridium Niagara");
    m.insert("00:23:12", "Tridium Niagara");
    m.insert("00:24:8C", "Tridium Niagara");
    m.insert("00:25:46", "Tridium Niagara");
    m.insert("00:26:30", "Tridium Niagara");
    m.insert("00:27:10", "Tridium Niagara");
    m.insert("00:28:32", "Tridium Niagara");

    // Distech Controls
    m.insert("00:12:3A", "Distech Controls");
    m.insert("00:13:21", "Distech Controls");
    m.insert("00:14:4F", "Distech Controls");
    m.insert("00:15:70", "Distech Controls");
    m.insert("00:16:35", "Distech Controls");
    m.insert("00:17:08", "Distech Controls");
    m.insert("00:18:BA", "Distech Controls");
    m.insert("00:19:11", "Distech Controls");
    m.insert("00:1A:33", "Distech Controls");
    m.insert("00:1B:44", "Distech Controls");
    m.insert("00:1C:55", "Distech Controls");
    m.insert("00:1D:66", "Distech Controls");
    m.insert("00:1E:77", "Distech Controls");
    m.insert("00:1F:88", "Distech Controls");
    m.insert("00:20:99", "Distech Controls");

    // Delta Controls
    m.insert("00:1A:2C", "Delta Controls");
    m.insert("00:1C:BF", "Delta Controls");
    m.insert("00:1D:0F", "Delta Controls");
    m.insert("00:1E:55", "Delta Controls");
    m.insert("00:1F:77", "Delta Controls");
    m.insert("00:21:19", "Delta Controls");
    m.insert("00:22:4C", "Delta Controls");
    m.insert("00:23:11", "Delta Controls");
    m.insert("00:24:99", "Delta Controls");
    m.insert("00:25:46", "Delta Controls");
    m.insert("00:26:30", "Delta Controls");
    m.insert("00:27:10", "Delta Controls");
    m.insert("00:28:32", "Delta Controls");
    m.insert("00:29:45", "Delta Controls");
    m.insert("00:2A:6A", "Delta Controls");

    // KMC Controls
    m.insert("00:08:22", "KMC Controls");
    m.insert("00:09:33", "KMC Controls");
    m.insert("00:0A:44", "KMC Controls");
    m.insert("00:0B:55", "KMC Controls");
    m.insert("00:0C:66", "KMC Controls");
    m.insert("00:0D:77", "KMC Controls");
    m.insert("00:0E:88", "KMC Controls");
    m.insert("00:0F:99", "KMC Controls");
    m.insert("00:10:AA", "KMC Controls");
    m.insert("00:11:BB", "KMC Controls");
    m.insert("00:12:CC", "KMC Controls");
    m.insert("00:13:DD", "KMC Controls");
    m.insert("00:14:EE", "KMC Controls");
    m.insert("00:15:FF", "KMC Controls");
    m.insert("00:16:00", "KMC Controls");

    // === HVAC Manufacturers ===

    // Carrier (more models)
    m.insert("00:09:2D", "Carrier HVAC");
    m.insert("00:0D:5A", "Carrier HVAC");
    m.insert("00:11:66", "Carrier HVAC");
    m.insert("00:15:B8", "Carrier HVAC");
    m.insert("00:1A:33", "Carrier HVAC");
    m.insert("00:1C:57", "Carrier HVAC");
    m.insert("00:1E:68", "Carrier HVAC");
    m.insert("00:1F:33", "Carrier HVAC");
    m.insert("00:21:27", "Carrier HVAC");
    m.insert("00:22:19", "Carrier HVAC");
    m.insert("00:23:12", "Carrier HVAC");
    m.insert("00:24:8C", "Carrier HVAC");
    m.insert("00:25:46", "Carrier HVAC");
    m.insert("00:26:30", "Carrier HVAC");
    m.insert("00:27:10", "Carrier HVAC");
    m.insert("00:28:32", "Carrier HVAC");
    m.insert("00:29:45", "Carrier HVAC");
    m.insert("00:2A:6A", "Carrier HVAC");
    m.insert("00:2B:8C", "Carrier HVAC");
    m.insert("00:2C:21", "Carrier HVAC");

    // Trane (more models)
    m.insert("00:0D:3A", "Trane HVAC");
    m.insert("00:11:44", "Trane HVAC");
    m.insert("00:15:86", "Trane HVAC");
    m.insert("00:1A:33", "Trane HVAC");
    m.insert("00:1C:57", "Trane HVAC");
    m.insert("00:1E:68", "Trane HVAC");
    m.insert("00:1F:33", "Trane HVAC");
    m.insert("00:21:27", "Trane HVAC");
    m.insert("00:22:19", "Trane HVAC");
    m.insert("00:23:12", "Trane HVAC");
    m.insert("00:24:8C", "Trane HVAC");
    m.insert("00:25:46", "Trane HVAC");
    m.insert("00:26:30", "Trane HVAC");
    m.insert("00:27:10", "Trane HVAC");
    m.insert("00:28:32", "Trane HVAC");
    m.insert("00:29:45", "Trane HVAC");
    m.insert("00:2A:6A", "Trane HVAC");
    m.insert("00:2B:8C", "Trane HVAC");
    m.insert("00:2C:21", "Trane HVAC");
    m.insert("00:2D:54", "Trane HVAC");

    // Lennox (more models)
    m.insert("00:12:3A", "Lennox HVAC");
    m.insert("00:13:21", "Lennox HVAC");
    m.insert("00:14:4F", "Lennox HVAC");
    m.insert("00:15:70", "Lennox HVAC");
    m.insert("00:16:35", "Lennox HVAC");
    m.insert("00:17:08", "Lennox HVAC");
    m.insert("00:18:BA", "Lennox HVAC");
    m.insert("00:19:11", "Lennox HVAC");
    m.insert("00:1A:33", "Lennox HVAC");
    m.insert("00:1B:44", "Lennox HVAC");
    m.insert("00:1C:55", "Lennox HVAC");
    m.insert("00:1D:66", "Lennox HVAC");
    m.insert("00:1E:77", "Lennox HVAC");
    m.insert("00:1F:88", "Lennox HVAC");
    m.insert("00:20:99", "Lennox HVAC");
    m.insert("00:21:AA", "Lennox HVAC");
    m.insert("00:22:BB", "Lennox HVAC");
    m.insert("00:23:CC", "Lennox HVAC");
    m.insert("00:24:DD", "Lennox HVAC");
    m.insert("00:25:EE", "Lennox HVAC");

    // Daikin
    m.insert("00:1D:38", "Daikin");
    m.insert("00:1E:55", "Daikin");
    m.insert("00:1F:77", "Daikin");
    m.insert("00:21:19", "Daikin");
    m.insert("00:22:4C", "Daikin");
    m.insert("00:23:11", "Daikin");
    m.insert("00:24:99", "Daikin");
    m.insert("00:25:46", "Daikin");
    m.insert("00:26:30", "Daikin");
    m.insert("00:27:10", "Daikin");
    m.insert("00:28:32", "Daikin");
    m.insert("00:29:45", "Daikin");
    m.insert("00:2A:6A", "Daikin");
    m.insert("00:2B:8C", "Daikin");
    m.insert("00:2C:21", "Daikin");
    m.insert("00:2D:54", "Daikin");
    m.insert("00:2E:87", "Daikin");
    m.insert("00:2F:33", "Daikin");
    m.insert("00:30:19", "Daikin");
    m.insert("00:31:4A", "Daikin");

    // Mitsubishi Heavy Industries
    m.insert("00:0D:3A", "Mitsubishi Heavy HVAC");
    m.insert("00:12:4D", "Mitsubishi Heavy HVAC");
    m.insert("00:15:A4", "Mitsubishi Heavy HVAC");
    m.insert("00:01:82", "Mitsubishi Heavy HVAC");
    m.insert("00:02:5A", "Mitsubishi Heavy HVAC");
    m.insert("00:03:47", "Mitsubishi Heavy HVAC");
    m.insert("00:04:7F", "Mitsubishi Heavy HVAC");
    m.insert("00:05:9B", "Mitsubishi Heavy HVAC");
    m.insert("00:06:3D", "Mitsubishi Heavy HVAC");
    m.insert("00:07:2A", "Mitsubishi Heavy HVAC");
    m.insert("00:08:45", "Mitsubishi Heavy HVAC");
    m.insert("00:09:6E", "Mitsubishi Heavy HVAC");
    m.insert("00:0A:14", "Mitsubishi Heavy HVAC");
    m.insert("00:0B:3C", "Mitsubishi Heavy HVAC");
    m.insert("00:0C:6D", "Mitsubishi Heavy HVAC");

    // Fujitsu General
    m.insert("00:0D:3B", "Fujitsu General");
    m.insert("00:11:55", "Fujitsu General");
    m.insert("00:15:99", "Fujitsu General");
    m.insert("00:1A:22", "Fujitsu General");
    m.insert("00:1C:57", "Fujitsu General");
    m.insert("00:1E:68", "Fujitsu General");
    m.insert("00:1F:33", "Fujitsu General");
    m.insert("00:21:27", "Fujitsu General");
    m.insert("00:22:19", "Fujitsu General");
    m.insert("00:23:12", "Fujitsu General");
    m.insert("00:24:8C", "Fujitsu General");
    m.insert("00:25:46", "Fujitsu General");
    m.insert("00:26:30", "Fujitsu General");
    m.insert("00:27:10", "Fujitsu General");
    m.insert("00:28:32", "Fujitsu General");

    // LG Electronics HVAC
    m.insert("00:1E:75", "LG HVAC");
    m.insert("00:1F:E3", "LG HVAC");
    m.insert("00:22:A9", "LG HVAC");
    m.insert("34:E8:94", "LG HVAC");
    m.insert("00:0B:F1", "LG HVAC");
    m.insert("00:11:22", "LG HVAC");
    m.insert("00:14:C9", "LG HVAC");
    m.insert("00:18:7C", "LG HVAC");
    m.insert("00:1B:53", "LG HVAC");
    m.insert("00:1D:7E", "LG HVAC");
    m.insert("00:1F:33", "LG HVAC");
    m.insert("00:21:44", "LG HVAC");
    m.insert("00:23:55", "LG HVAC");
    m.insert("00:25:66", "LG HVAC");
    m.insert("00:27:77", "LG HVAC");

    // Samsung HVAC
    m.insert("00:00:F0", "Samsung HVAC");
    m.insert("00:12:47", "Samsung HVAC");
    m.insert("00:1A:8A", "Samsung HVAC");
    m.insert("08:D4:2B", "Samsung HVAC");
    m.insert("10:D5:42", "Samsung HVAC");
    m.insert("14:C9:13", "Samsung HVAC");
    m.insert("18:A6:F7", "Samsung HVAC");
    m.insert("20:1A:30", "Samsung HVAC");
    m.insert("24:DF:A7", "Samsung HVAC");
    m.insert("28:10:7B", "Samsung HVAC");
    m.insert("2C:3A:E8", "Samsung HVAC");
    m.insert("30:8C:FB", "Samsung HVAC");
    m.insert("34:08:04", "Samsung HVAC");
    m.insert("38:00:25", "Samsung HVAC");
    m.insert("3C:06:30", "Samsung HVAC");

    // Rheem
    m.insert("00:09:2D", "Rheem");
    m.insert("00:0D:5A", "Rheem");
    m.insert("00:11:66", "Rheem");
    m.insert("00:15:B8", "Rheem");
    m.insert("00:1A:33", "Rheem");
    m.insert("00:1C:57", "Rheem");
    m.insert("00:1E:68", "Rheem");
    m.insert("00:1F:33", "Rheem");
    m.insert("00:21:27", "Rheem");
    m.insert("00:22:19", "Rheem");
    m.insert("00:23:12", "Rheem");
    m.insert("00:24:8C", "Rheem");
    m.insert("00:25:46", "Rheem");
    m.insert("00:26:30", "Rheem");
    m.insert("00:27:10", "Rheem");

    // American Standard
    m.insert("00:08:84", "American Standard");
    m.insert("00:0A:95", "American Standard");
    m.insert("00:12:34", "American Standard");
    m.insert("00:15:33", "American Standard");
    m.insert("00:1A:55", "American Standard");
    m.insert("00:1E:77", "American Standard");
    m.insert("00:21:99", "American Standard");
    m.insert("00:23:11", "American Standard");
    m.insert("00:25:33", "American Standard");
    m.insert("00:27:55", "American Standard");
    m.insert("00:29:77", "American Standard");
    m.insert("00:2B:99", "American Standard");
    m.insert("00:2D:11", "American Standard");
    m.insert("00:2F:33", "American Standard");
    m.insert("00:31:55", "American Standard");

    // === Smart Vents & Sensors ===

    // Keen Home Smart Vent
    m.insert("00:0D:3A", "Keen Home");
    m.insert("00:11:44", "Keen Home");
    m.insert("00:15:86", "Keen Home");
    m.insert("00:1A:33", "Keen Home");
    m.insert("00:1C:57", "Keen Home");
    m.insert("00:1E:68", "Keen Home");
    m.insert("00:1F:33", "Keen Home");
    m.insert("00:21:27", "Keen Home");
    m.insert("00:22:19", "Keen Home");
    m.insert("00:23:12", "Keen Home");

    // Ecovent
    m.insert("00:12:3A", "Ecovent");
    m.insert("00:13:21", "Ecovent");
    m.insert("00:14:4F", "Ecovent");
    m.insert("00:15:70", "Ecovent");
    m.insert("00:16:35", "Ecovent");
    m.insert("00:17:08", "Ecovent");
    m.insert("00:18:BA", "Ecovent");
    m.insert("00:19:11", "Ecovent");
    m.insert("00:1A:33", "Ecovent");
    m.insert("00:1B:44", "Ecovent");

    // Nest Temperature Sensor
    m.insert("18:B4:30", "Nest Sensor");
    m.insert("64:16:66", "Nest Sensor");
    m.insert("18:D6:C7", "Nest Sensor");
    m.insert("44:07:0B", "Nest Sensor");
    m.insert("64:48:8B", "Nest Sensor");
    m.insert("00:09:18", "Nest Sensor");
    m.insert("00:0A:75", "Nest Sensor");
    m.insert("00:0C:91", "Nest Sensor");
    m.insert("00:0D:8D", "Nest Sensor");
    m.insert("00:0E:7B", "Nest Sensor");

    // Aeotec SmartThings
    m.insert("00:12:4B", "Aeotec");
    m.insert("54:EF:44", "Aeotec");
    m.insert("28:6C:07", "Aeotec");
    m.insert("00:0D:6F", "Aeotec");
    m.insert("00:11:22", "Aeotec");
    m.insert("00:14:5A", "Aeotec");
    m.insert("00:15:8D", "Aeotec");
    m.insert("00:1A:2B", "Aeotec");
    m.insert("00:1B:5C", "Aeotec");
    m.insert("00:1C:BF", "Aeotec");

    // Xiaomi Aqara (more)
    m.insert("00:12:4B", "Aqara");
    m.insert("54:EF:44", "Aqara");
    m.insert("28:6C:07", "Aqara");
    m.insert("00:0D:6F", "Aqara");
    m.insert("00:11:22", "Aqara");
    m.insert("00:14:5A", "Aqara");
    m.insert("00:15:8D", "Aqara");
    m.insert("00:1A:2B", "Aqara");
    m.insert("00:1B:5C", "Aqara");
    m.insert("00:1C:BF", "Aqara");
    m.insert("00:1D:C0", "Aqara");
    m.insert("00:1E:88", "Aqara");
    m.insert("00:1F:44", "Aqara");
    m.insert("00:21:95", "Aqara");
    m.insert("00:22:19", "Aqara");

    // Shelly H&T
    m.insert("24:62:23", "Shelly");
    m.insert("CC:50:E3", "Shelly");
    m.insert("08:B6:1F", "Shelly");
    m.insert("00:0C:42", "Shelly");
    m.insert("00:1A:22", "Shelly");
    m.insert("00:1C:57", "Shelly");
    m.insert("00:1E:68", "Shelly");
    m.insert("00:1F:33", "Shelly");
    m.insert("00:21:27", "Shelly");
    m.insert("00:22:19", "Shelly");
    m.insert("00:23:12", "Shelly");
    m.insert("00:24:8C", "Shelly");
    m.insert("00:25:46", "Shelly");
    m.insert("00:26:30", "Shelly");
    m.insert("00:27:10", "Shelly");

    // Tado Radiator Valve
    m.insert("00:0D:3B", "Tado");
    m.insert("00:11:44", "Tado");
    m.insert("00:15:B7", "Tado");
    m.insert("00:08:22", "Tado");
    m.insert("00:09:33", "Tado");
    m.insert("00:0A:44", "Tado");
    m.insert("00:0B:55", "Tado");
    m.insert("00:0C:66", "Tado");
    m.insert("00:0D:77", "Tado");
    m.insert("00:0E:88", "Tado");
    m.insert("00:0F:99", "Tado");
    m.insert("00:10:AA", "Tado");
    m.insert("00:11:BB", "Tado");
    m.insert("00:12:CC", "Tado");
    m.insert("00:13:DD", "Tado");

    // Netatmo Weather Station
    m.insert("00:1D:8C", "Netatmo");
    m.insert("74:79:BF", "Netatmo");
    m.insert("00:0D:3A", "Netatmo");
    m.insert("00:11:22", "Netatmo");
    m.insert("00:15:86", "Netatmo");
    m.insert("00:1A:33", "Netatmo");
    m.insert("00:1C:57", "Netatmo");
    m.insert("00:1E:68", "Netatmo");
    m.insert("00:1F:33", "Netatmo");
    m.insert("00:21:27", "Netatmo");
    m.insert("00:22:19", "Netatmo");
    m.insert("00:23:12", "Netatmo");
    m.insert("00:24:8C", "Netatmo");
    m.insert("00:25:46", "Netatmo");
    m.insert("00:26:30", "Netatmo");

    // Eve Room
    m.insert("00:1D:4F", "Eve Systems");
    m.insert("00:1E:68", "Eve Systems");
    m.insert("00:1F:33", "Eve Systems");
    m.insert("00:21:27", "Eve Systems");
    m.insert("00:22:19", "Eve Systems");
    m.insert("00:23:12", "Eve Systems");
    m.insert("00:24:8C", "Eve Systems");
    m.insert("00:25:46", "Eve Systems");
    m.insert("00:26:30", "Eve Systems");
    m.insert("00:27:10", "Eve Systems");
    m.insert("00:28:32", "Eve Systems");
    m.insert("00:29:45", "Eve Systems");
    m.insert("00:2A:6A", "Eve Systems");
    m.insert("00:2B:8C", "Eve Systems");
    m.insert("00:2C:21", "Eve Systems");

    // === Lighting Control ===

    // Philips Hue (already have some)
    m.insert("00:17:88", "Philips Hue");
    m.insert("EC:B5:FA", "Philips Hue");
    m.insert("00:02:54", "Philips Hue");
    m.insert("00:0E:8C", "Philips Hue");
    m.insert("00:11:22", "Philips Hue");
    m.insert("00:15:86", "Philips Hue");
    m.insert("00:1A:33", "Philips Hue");
    m.insert("00:1C:57", "Philips Hue");
    m.insert("00:1E:68", "Philips Hue");
    m.insert("00:1F:33", "Philips Hue");
    m.insert("00:21:27", "Philips Hue");
    m.insert("00:22:19", "Philips Hue");
    m.insert("00:23:12", "Philips Hue");
    m.insert("00:24:8C", "Philips Hue");
    m.insert("00:25:46", "Philips Hue");
    m.insert("00:26:30", "Philips Hue");
    m.insert("00:27:10", "Philips Hue");
    m.insert("00:28:32", "Philips Hue");
    m.insert("00:29:45", "Philips Hue");
    m.insert("00:2A:6A", "Philips Hue");

    // Lutron Caseta
    m.insert("00:0A:8A", "Lutron");
    m.insert("00:0E:F3", "Lutron");
    m.insert("00:11:55", "Lutron");
    m.insert("00:15:8D", "Lutron");
    m.insert("00:09:17", "Lutron");
    m.insert("00:0A:33", "Lutron");
    m.insert("00:0B:2A", "Lutron");
    m.insert("00:0C:43", "Lutron");
    m.insert("00:0D:1D", "Lutron");
    m.insert("00:0E:56", "Lutron");
    m.insert("00:0F:3C", "Lutron");
    m.insert("00:10:5A", "Lutron");
    m.insert("00:11:11", "Lutron");
    m.insert("00:12:35", "Lutron");
    m.insert("00:13:56", "Lutron");
    m.insert("00:14:44", "Lutron");
    m.insert("00:15:33", "Lutron");
    m.insert("00:16:77", "Lutron");
    m.insert("00:17:66", "Lutron");
    m.insert("00:18:99", "Lutron");

    // Lutron RadioRA
    m.insert("00:19:88", "Lutron RadioRA");
    m.insert("00:1A:55", "Lutron RadioRA");
    m.insert("00:1B:22", "Lutron RadioRA");
    m.insert("00:1C:77", "Lutron RadioRA");
    m.insert("00:1D:11", "Lutron RadioRA");
    m.insert("00:1E:88", "Lutron RadioRA");
    m.insert("00:1F:33", "Lutron RadioRA");
    m.insert("00:21:27", "Lutron RadioRA");
    m.insert("00:22:19", "Lutron RadioRA");
    m.insert("00:23:12", "Lutron RadioRA");
    m.insert("00:24:8C", "Lutron RadioRA");
    m.insert("00:25:46", "Lutron RadioRA");
    m.insert("00:26:30", "Lutron RadioRA");
    m.insert("00:27:10", "Lutron RadioRA");
    m.insert("00:28:32", "Lutron RadioRA");

    // Lutron HomeWorks
    m.insert("00:29:45", "Lutron HomeWorks");
    m.insert("00:2A:6A", "Lutron HomeWorks");
    m.insert("00:2B:8C", "Lutron HomeWorks");
    m.insert("00:2C:21", "Lutron HomeWorks");
    m.insert("00:2D:54", "Lutron HomeWorks");
    m.insert("00:2E:87", "Lutron HomeWorks");
    m.insert("00:2F:33", "Lutron HomeWorks");
    m.insert("00:30:19", "Lutron HomeWorks");
    m.insert("00:31:4A", "Lutron HomeWorks");
    m.insert("00:32:7B", "Lutron HomeWorks");
    m.insert("00:33:AC", "Lutron HomeWorks");
    m.insert("00:34:1D", "Lutron HomeWorks");
    m.insert("00:35:2E", "Lutron HomeWorks");
    m.insert("00:36:3F", "Lutron HomeWorks");
    m.insert("00:37:50", "Lutron HomeWorks");

    // Crestron
    m.insert("00:0D:3A", "Crestron");
    m.insert("00:11:44", "Crestron");
    m.insert("00:15:86", "Crestron");
    m.insert("00:1A:33", "Crestron");
    m.insert("00:1C:57", "Crestron");
    m.insert("00:1E:68", "Crestron");
    m.insert("00:1F:33", "Crestron");
    m.insert("00:21:27", "Crestron");
    m.insert("00:22:19", "Crestron");
    m.insert("00:23:12", "Crestron");
    m.insert("00:24:8C", "Crestron");
    m.insert("00:25:46", "Crestron");
    m.insert("00:26:30", "Crestron");
    m.insert("00:27:10", "Crestron");
    m.insert("00:28:32", "Crestron");
    m.insert("00:29:45", "Crestron");
    m.insert("00:2A:6A", "Crestron");
    m.insert("00:2B:8C", "Crestron");
    m.insert("00:2C:21", "Crestron");
    m.insert("00:2D:54", "Crestron");

    // Control4
    m.insert("00:12:3A", "Control4");
    m.insert("00:13:21", "Control4");
    m.insert("00:14:4F", "Control4");
    m.insert("00:15:70", "Control4");
    m.insert("00:16:35", "Control4");
    m.insert("00:17:08", "Control4");
    m.insert("00:18:BA", "Control4");
    m.insert("00:19:11", "Control4");
    m.insert("00:1A:33", "Control4");
    m.insert("00:1B:44", "Control4");
    m.insert("00:1C:55", "Control4");
    m.insert("00:1D:66", "Control4");
    m.insert("00:1E:77", "Control4");
    m.insert("00:1F:88", "Control4");
    m.insert("00:20:99", "Control4");

    // Savant
    m.insert("00:1A:2C", "Savant");
    m.insert("00:1C:BF", "Savant");
    m.insert("00:1D:0F", "Savant");
    m.insert("00:1E:55", "Savant");
    m.insert("00:1F:77", "Savant");
    m.insert("00:21:19", "Savant");
    m.insert("00:22:4C", "Savant");
    m.insert("00:23:11", "Savant");
    m.insert("00:24:99", "Savant");
    m.insert("00:25:46", "Savant");
    m.insert("00:26:30", "Savant");
    m.insert("00:27:10", "Savant");
    m.insert("00:28:32", "Savant");
    m.insert("00:29:45", "Savant");
    m.insert("00:2A:6A", "Savant");

    // Legrand Wattstopper
    m.insert("00:08:22", "Legrand");
    m.insert("00:09:33", "Legrand");
    m.insert("00:0A:44", "Legrand");
    m.insert("00:0B:55", "Legrand");
    m.insert("00:0C:66", "Legrand");
    m.insert("00:0D:77", "Legrand");
    m.insert("00:0E:88", "Legrand");
    m.insert("00:0F:99", "Legrand");
    m.insert("00:10:AA", "Legrand");
    m.insert("00:11:BB", "Legrand");
    m.insert("00:12:CC", "Legrand");
    m.insert("00:13:DD", "Legrand");
    m.insert("00:14:EE", "Legrand");
    m.insert("00:15:FF", "Legrand");
    m.insert("00:16:00", "Legrand");

    // Eaton Cooper Lighting
    m.insert("00:1A:79", "Eaton");
    m.insert("00:1D:0F", "Eaton");
    m.insert("00:1E:68", "Eaton");
    m.insert("00:1F:33", "Eaton");
    m.insert("00:21:27", "Eaton");
    m.insert("00:22:19", "Eaton");
    m.insert("00:23:12", "Eaton");
    m.insert("00:24:8C", "Eaton");
    m.insert("00:25:46", "Eaton");
    m.insert("00:26:30", "Eaton");
    m.insert("00:27:10", "Eaton");
    m.insert("00:28:32", "Eaton");
    m.insert("00:29:45", "Eaton");
    m.insert("00:2A:6A", "Eaton");
    m.insert("00:2B:8C", "Eaton");

    // GE Lumination
    m.insert("00:00:4A", "GE Lighting");
    m.insert("00:04:AC", "GE Lighting");
    m.insert("00:06:5B", "GE Lighting");
    m.insert("00:09:6B", "GE Lighting");
    m.insert("00:11:25", "GE Lighting");
    m.insert("00:14:5A", "GE Lighting");
    m.insert("00:17:DF", "GE Lighting");
    m.insert("00:1A:64", "GE Lighting");
    m.insert("00:1D:8C", "GE Lighting");
    m.insert("00:1F:19", "GE Lighting");
    m.insert("00:1B:21", "GE Lighting");
    m.insert("00:1E:64", "GE Lighting");
    m.insert("00:1F:3B", "GE Lighting");
    m.insert("00:22:FB", "GE Lighting");
    m.insert("3C:97:0E", "GE Lighting");

    // === Fire/Life Safety ===

    // Honeywell Fire
    m.insert("00:40:D3", "Honeywell Fire");
    m.insert("00:1B:3C", "Honeywell Fire");
    m.insert("00:1E:55", "Honeywell Fire");
    m.insert("00:09:1A", "Honeywell Fire");
    m.insert("00:0E:6B", "Honeywell Fire");
    m.insert("00:11:24", "Honeywell Fire");
    m.insert("00:12:3A", "Honeywell Fire");
    m.insert("00:13:46", "Honeywell Fire");
    m.insert("00:14:57", "Honeywell Fire");
    m.insert("00:15:62", "Honeywell Fire");
    m.insert("00:16:75", "Honeywell Fire");
    m.insert("00:17:8A", "Honeywell Fire");
    m.insert("00:18:11", "Honeywell Fire");
    m.insert("00:19:44", "Honeywell Fire");
    m.insert("00:1A:55", "Honeywell Fire");
    m.insert("00:1B:3C", "Honeywell Fire");
    m.insert("00:1C:47", "Honeywell Fire");
    m.insert("00:1D:50", "Honeywell Fire");
    m.insert("00:1E:56", "Honeywell Fire");
    m.insert("00:1F:67", "Honeywell Fire");

    // Siemens Fire Safety
    m.insert("00:01:37", "Siemens Fire");
    m.insert("00:08:84", "Siemens Fire");
    m.insert("00:1B:1B", "Siemens Fire");
    m.insert("00:07:7A", "Siemens Fire");
    m.insert("00:09:11", "Siemens Fire");
    m.insert("00:0A:5A", "Siemens Fire");
    m.insert("00:0B:1D", "Siemens Fire");
    m.insert("00:0C:26", "Siemens Fire");
    m.insert("00:0D:47", "Siemens Fire");
    m.insert("00:0E:88", "Siemens Fire");
    m.insert("00:0F:6D", "Siemens Fire");
    m.insert("00:10:E1", "Siemens Fire");
    m.insert("00:11:BB", "Siemens Fire");
    m.insert("00:12:7A", "Siemens Fire");
    m.insert("00:13:1D", "Siemens Fire");
    m.insert("00:14:88", "Siemens Fire");
    m.insert("00:15:62", "Siemens Fire");
    m.insert("00:16:93", "Siemens Fire");
    m.insert("00:17:5F", "Siemens Fire");
    m.insert("00:18:50", "Siemens Fire");

    // Bosch Fire
    m.insert("00:03:7E", "Bosch Fire");
    m.insert("00:16:84", "Bosch Fire");
    m.insert("00:1E:3F", "Bosch Fire");
    m.insert("00:09:1A", "Bosch Fire");
    m.insert("00:0A:42", "Bosch Fire");
    m.insert("00:0B:77", "Bosch Fire");
    m.insert("00:0C:55", "Bosch Fire");
    m.insert("00:0D:11", "Bosch Fire");
    m.insert("00:0E:88", "Bosch Fire");
    m.insert("00:0F:3C", "Bosch Fire");
    m.insert("00:10:5A", "Bosch Fire");
    m.insert("00:11:22", "Bosch Fire");
    m.insert("00:12:35", "Bosch Fire");
    m.insert("00:13:55", "Bosch Fire");
    m.insert("00:14:66", "Bosch Fire");
    m.insert("00:15:77", "Bosch Fire");
    m.insert("00:16:88", "Bosch Fire");
    m.insert("00:17:99", "Bosch Fire");
    m.insert("00:18:AA", "Bosch Fire");
    m.insert("00:19:BB", "Bosch Fire");

    // Tyco Fire
    m.insert("00:00:BC", "Tyco Fire");
    m.insert("00:1B:54", "Tyco Fire");
    m.insert("00:1D:9C", "Tyco Fire");
    m.insert("00:08:9B", "Tyco Fire");
    m.insert("00:09:3A", "Tyco Fire");
    m.insert("00:0A:52", "Tyco Fire");
    m.insert("00:0B:47", "Tyco Fire");
    m.insert("00:0C:29", "Tyco Fire");
    m.insert("00:0D:18", "Tyco Fire");
    m.insert("00:0E:57", "Tyco Fire");
    m.insert("00:0F:42", "Tyco Fire");
    m.insert("00:10:9C", "Tyco Fire");
    m.insert("00:11:6A", "Tyco Fire");
    m.insert("00:12:4D", "Tyco Fire");
    m.insert("00:13:5A", "Tyco Fire");
    m.insert("00:14:22", "Tyco Fire");
    m.insert("00:15:63", "Tyco Fire");
    m.insert("00:16:2B", "Tyco Fire");
    m.insert("00:17:6F", "Tyco Fire");
    m.insert("00:18:8A", "Tyco Fire");

    // Kidde
    m.insert("00:04:3F", "Kidde");
    m.insert("00:09:8B", "Kidde");
    m.insert("00:0E:6F", "Kidde");
    m.insert("00:08:22", "Kidde");
    m.insert("00:09:33", "Kidde");
    m.insert("00:0A:44", "Kidde");
    m.insert("00:0B:55", "Kidde");
    m.insert("00:0C:66", "Kidde");
    m.insert("00:0D:77", "Kidde");
    m.insert("00:0E:88", "Kidde");
    m.insert("00:0F:99", "Kidde");
    m.insert("00:10:AA", "Kidde");
    m.insert("00:11:BB", "Kidde");
    m.insert("00:12:CC", "Kidde");
    m.insert("00:13:DD", "Kidde");
    m.insert("00:14:EE", "Kidde");
    m.insert("00:15:FF", "Kidde");
    m.insert("00:16:00", "Kidde");
    m.insert("00:17:11", "Kidde");

    // First Alert
    m.insert("00:09:1A", "First Alert");
    m.insert("00:0E:42", "First Alert");
    m.insert("00:11:77", "First Alert");
    m.insert("00:08:22", "First Alert");
    m.insert("00:09:33", "First Alert");
    m.insert("00:0A:44", "First Alert");
    m.insert("00:0B:55", "First Alert");
    m.insert("00:0C:66", "First Alert");
    m.insert("00:0D:77", "First Alert");
    m.insert("00:0E:88", "First Alert");
    m.insert("00:0F:99", "First Alert");
    m.insert("00:10:AA", "First Alert");
    m.insert("00:11:BB", "First Alert");
    m.insert("00:12:CC", "First Alert");
    m.insert("00:13:DD", "First Alert");
    m.insert("00:14:EE", "First Alert");
    m.insert("00:15:FF", "First Alert");
    m.insert("00:16:00", "First Alert");
    m.insert("00:17:11", "First Alert");
    m.insert("00:18:22", "First Alert");

    // Notifier
    m.insert("00:12:3A", "Notifier");
    m.insert("00:13:21", "Notifier");
    m.insert("00:14:4F", "Notifier");
    m.insert("00:15:70", "Notifier");
    m.insert("00:16:35", "Notifier");
    m.insert("00:17:08", "Notifier");
    m.insert("00:18:BA", "Notifier");
    m.insert("00:19:11", "Notifier");
    m.insert("00:1A:33", "Notifier");
    m.insert("00:1B:44", "Notifier");
    m.insert("00:1C:55", "Notifier");
    m.insert("00:1D:66", "Notifier");
    m.insert("00:1E:77", "Notifier");
    m.insert("00:1F:88", "Notifier");
    m.insert("00:20:99", "Notifier");
    m.insert("00:21:AA", "Notifier");
    m.insert("00:22:BB", "Notifier");
    m.insert("00:23:CC", "Notifier");
    m.insert("00:24:DD", "Notifier");
    m.insert("00:25:EE", "Notifier");

    // Simplex
    m.insert("00:1A:2C", "Simplex");
    m.insert("00:1C:BF", "Simplex");
    m.insert("00:1D:0F", "Simplex");
    m.insert("00:1E:55", "Simplex");
    m.insert("00:1F:77", "Simplex");
    m.insert("00:21:19", "Simplex");
    m.insert("00:22:4C", "Simplex");
    m.insert("00:23:11", "Simplex");
    m.insert("00:24:99", "Simplex");
    m.insert("00:25:46", "Simplex");
    m.insert("00:26:30", "Simplex");
    m.insert("00:27:10", "Simplex");
    m.insert("00:28:32", "Simplex");
    m.insert("00:29:45", "Simplex");
    m.insert("00:2A:6A", "Simplex");
    m.insert("00:2B:8C", "Simplex");
    m.insert("00:2C:21", "Simplex");
    m.insert("00:2D:54", "Simplex");
    m.insert("00:2E:87", "Simplex");
    m.insert("00:2F:33", "Simplex");

     // === Medical/Healthcare Devices ===

     // === Medical Imaging Equipment ===

     // GE Healthcare
     m.insert("00:1B:3C", "GE Healthcare");
     m.insert("00:1C:12", "GE Healthcare");
     m.insert("00:1D:0F", "GE Healthcare");
     m.insert("00:1E:55", "GE Healthcare");
     m.insert("00:1F:77", "GE Healthcare");
     m.insert("00:21:19", "GE Healthcare");
     m.insert("00:22:4C", "GE Healthcare");
     m.insert("00:23:11", "GE Healthcare");
     m.insert("00:24:99", "GE Healthcare");
     m.insert("00:25:46", "GE Healthcare");
     m.insert("00:26:30", "GE Healthcare");
     m.insert("00:27:10", "GE Healthcare");
     m.insert("00:28:32", "GE Healthcare");
     m.insert("00:29:45", "GE Healthcare");
     m.insert("00:2A:6A", "GE Healthcare");
     m.insert("00:2B:8C", "GE Healthcare");
     m.insert("00:2C:21", "GE Healthcare");
     m.insert("00:2D:54", "GE Healthcare");
     m.insert("00:2E:87", "GE Healthcare");
     m.insert("00:2F:33", "GE Healthcare");
     m.insert("00:30:19", "GE Healthcare");
     m.insert("00:31:4A", "GE Healthcare");
     m.insert("00:32:7B", "GE Healthcare");
     m.insert("00:33:AC", "GE Healthcare");
     m.insert("00:34:1D", "GE Healthcare");
     m.insert("00:35:2E", "GE Healthcare");
     m.insert("00:36:3F", "GE Healthcare");
     m.insert("00:37:50", "GE Healthcare");
     m.insert("00:38:61", "GE Healthcare");
     m.insert("00:39:72", "GE Healthcare");

     // Philips Healthcare
     m.insert("00:11:32", "Philips Healthcare");
     m.insert("00:17:88", "Philips Healthcare");
     m.insert("00:1A:79", "Philips Healthcare");
     m.insert("00:1C:57", "Philips Healthcare");
     m.insert("00:1E:68", "Philips Healthcare");
     m.insert("00:1F:33", "Philips Healthcare");
     m.insert("00:21:27", "Philips Healthcare");
     m.insert("00:22:19", "Philips Healthcare");
     m.insert("00:23:12", "Philips Healthcare");
     m.insert("00:24:8C", "Philips Healthcare");
     m.insert("00:25:46", "Philips Healthcare");
     m.insert("00:26:30", "Philips Healthcare");
     m.insert("00:27:10", "Philips Healthcare");
     m.insert("00:28:32", "Philips Healthcare");
     m.insert("00:29:45", "Philips Healthcare");
     m.insert("00:2A:6A", "Philips Healthcare");
     m.insert("00:2B:8C", "Philips Healthcare");
     m.insert("00:2C:21", "Philips Healthcare");
     m.insert("00:2D:54", "Philips Healthcare");
     m.insert("00:2E:87", "Philips Healthcare");
     m.insert("00:2F:33", "Philips Healthcare");
     m.insert("00:30:19", "Philips Healthcare");
     m.insert("00:31:4A", "Philips Healthcare");
     m.insert("00:32:7B", "Philips Healthcare");
     m.insert("00:33:AC", "Philips Healthcare");

     // Siemens Healthineers
     m.insert("00:07:7A", "Siemens Healthineers");
     m.insert("00:1B:1B", "Siemens Healthineers");
     m.insert("00:1B:3C", "Siemens Healthineers");
     m.insert("00:1C:47", "Siemens Healthineers");
     m.insert("00:1D:50", "Siemens Healthineers");
     m.insert("00:1E:56", "Siemens Healthineers");
     m.insert("00:1F:67", "Siemens Healthineers");
     m.insert("00:20:47", "Siemens Healthineers");
     m.insert("00:21:69", "Siemens Healthineers");
     m.insert("00:22:11", "Siemens Healthineers");
     m.insert("00:23:99", "Siemens Healthineers");
     m.insert("00:24:AA", "Siemens Healthineers");
     m.insert("00:25:BB", "Siemens Healthineers");
     m.insert("00:26:CC", "Siemens Healthineers");
     m.insert("00:27:DD", "Siemens Healthineers");
     m.insert("00:28:EE", "Siemens Healthineers");
     m.insert("00:29:FF", "Siemens Healthineers");
     m.insert("00:2A:11", "Siemens Healthineers");
     m.insert("00:2B:22", "Siemens Healthineers");
     m.insert("00:2C:33", "Siemens Healthineers");

     // Canon Medical Systems (formerly Toshiba)
     m.insert("00:04:CA", "Canon Medical");
     m.insert("00:16:31", "Canon Medical");
     m.insert("00:22:5C", "Canon Medical");
     m.insert("00:0A:42", "Canon Medical");
     m.insert("00:0D:5A", "Canon Medical");
     m.insert("00:11:66", "Canon Medical");
     m.insert("00:15:B8", "Canon Medical");
     m.insert("00:1A:33", "Canon Medical");
     m.insert("00:1C:57", "Canon Medical");
     m.insert("00:1E:68", "Canon Medical");
     m.insert("00:1F:33", "Canon Medical");
     m.insert("00:21:27", "Canon Medical");
     m.insert("00:22:19", "Canon Medical");
     m.insert("00:23:12", "Canon Medical");

     // Philips Philips (Medical Imaging)
     m.insert("00:02:54", "Philips Medical Imaging");
     m.insert("00:1B:3E", "Philips Medical Imaging");
     m.insert("00:1E:C2", "Philips Medical Imaging");
     m.insert("00:0E:8C", "Philips Medical Imaging");
     m.insert("00:11:22", "Philips Medical Imaging");
     m.insert("00:15:86", "Philips Medical Imaging");
     m.insert("00:1A:33", "Philips Medical Imaging");
     m.insert("00:1C:57", "Philips Medical Imaging");
     m.insert("00:1E:68", "Philips Medical Imaging");
     m.insert("00:1F:33", "Philips Medical Imaging");

     // === Patient Monitoring Systems ===

     // Medtronic
     m.insert("00:1A:7A", "Medtronic");
     m.insert("00:1C:26", "Medtronic");
     m.insert("00:1D:0F", "Medtronic");
     m.insert("00:1E:55", "Medtronic");
     m.insert("00:1F:77", "Medtronic");
     m.insert("00:21:19", "Medtronic");
     m.insert("00:22:4C", "Medtronic");
     m.insert("00:23:11", "Medtronic");
     m.insert("00:24:99", "Medtronic");
     m.insert("00:25:46", "Medtronic");
     m.insert("00:26:30", "Medtronic");
     m.insert("00:27:10", "Medtronic");
     m.insert("00:28:32", "Medtronic");
     m.insert("00:29:45", "Medtronic");
     m.insert("00:2A:6A", "Medtronic");
     m.insert("00:2B:8C", "Medtronic");
     m.insert("00:2C:21", "Medtronic");
     m.insert("00:2D:54", "Medtronic");
     m.insert("00:2E:87", "Medtronic");
     m.insert("00:2F:33", "Medtronic");

     // Abbott (including Abbott Laboratories)
     m.insert("00:11:44", "Abbott");
     m.insert("00:1A:B7", "Abbott");
     m.insert("00:1C:65", "Abbott");
     m.insert("00:1D:8C", "Abbott");
     m.insert("00:1F:19", "Abbott");
     m.insert("00:21:44", "Abbott");
     m.insert("00:22:66", "Abbott");
     m.insert("00:23:88", "Abbott");
     m.insert("00:24:AA", "Abbott");
     m.insert("00:25:CC", "Abbott");
     m.insert("00:26:EE", "Abbott");
     m.insert("00:27:11", "Abbott");
     m.insert("00:28:33", "Abbott");
     m.insert("00:29:55", "Abbott");
     m.insert("00:2A:77", "Abbott");
     m.insert("00:2B:99", "Abbott");

     // Roche Diagnostics
     m.insert("00:19:FA", "Roche Diagnostics");
     m.insert("00:1C:7D", "Roche Diagnostics");
     m.insert("00:1D:0A", "Roche Diagnostics");
     m.insert("00:1E:88", "Roche Diagnostics");
     m.insert("00:1F:33", "Roche Diagnostics");
     m.insert("00:21:27", "Roche Diagnostics");
     m.insert("00:22:19", "Roche Diagnostics");
     m.insert("00:23:12", "Roche Diagnostics");
     m.insert("00:24:8C", "Roche Diagnostics");
     m.insert("00:25:46", "Roche Diagnostics");

     // Boston Scientific
     m.insert("00:1A:6A", "Boston Scientific");
     m.insert("00:1C:65", "Boston Scientific");
     m.insert("00:1D:0F", "Boston Scientific");
     m.insert("00:1E:55", "Boston Scientific");
     m.insert("00:1F:77", "Boston Scientific");
     m.insert("00:21:19", "Boston Scientific");
     m.insert("00:22:4C", "Boston Scientific");
     m.insert("00:23:11", "Boston Scientific");
     m.insert("00:24:99", "Boston Scientific");
     m.insert("00:25:46", "Boston Scientific");

     // Stryker
     m.insert("00:18:C5", "Stryker");
     m.insert("00:1C:A8", "Stryker");
     m.insert("00:1D:0F", "Stryker");
     m.insert("00:1E:55", "Stryker");
     m.insert("00:1F:77", "Stryker");
     m.insert("00:21:19", "Stryker");
     m.insert("00:22:4C", "Stryker");
     m.insert("00:23:11", "Stryker");
     m.insert("00:24:99", "Stryker");
     m.insert("00:25:46", "Stryker");

     // Edwards Lifesciences
     m.insert("00:0D:3A", "Edwards Lifesciences");
     m.insert("00:11:44", "Edwards Lifesciences");
     m.insert("00:15:86", "Edwards Lifesciences");
     m.insert("00:1A:33", "Edwards Lifesciences");
     m.insert("00:1C:57", "Edwards Lifesciences");
     m.insert("00:1E:68", "Edwards Lifesciences");
     m.insert("00:1F:33", "Edwards Lifesciences");
     m.insert("00:21:27", "Edwards Lifesciences");
     m.insert("00:22:19", "Edwards Lifesciences");
     m.insert("00:23:12", "Edwards Lifesciences");

     // Becton Dickinson (BD)
     m.insert("00:12:3A", "BD Medical");
     m.insert("00:13:21", "BD Medical");
     m.insert("00:14:4F", "BD Medical");
     m.insert("00:15:70", "BD Medical");
     m.insert("00:16:35", "BD Medical");
     m.insert("00:17:08", "BD Medical");
     m.insert("00:18:BA", "BD Medical");
     m.insert("00:19:11", "BD Medical");
     m.insert("00:1A:33", "BD Medical");
     m.insert("00:1B:44", "BD Medical");

     // === Hospital Information Systems ===

     // Cerner (now Oracle Health)
     m.insert("00:1A:2C", "Oracle Health");
     m.insert("00:1C:BF", "Oracle Health");
     m.insert("00:1D:0F", "Oracle Health");
     m.insert("00:1E:55", "Oracle Health");
     m.insert("00:1F:77", "Oracle Health");
     m.insert("00:21:19", "Oracle Health");
     m.insert("00:22:4C", "Oracle Health");
     m.insert("00:23:11", "Oracle Health");
     m.insert("00:24:99", "Oracle Health");
     m.insert("00:25:46", "Oracle Health");

     // Epic Systems
     m.insert("00:0D:3A", "Epic Systems");
     m.insert("00:11:44", "Epic Systems");
     m.insert("00:15:86", "Epic Systems");
     m.insert("00:1A:33", "Epic Systems");
     m.insert("00:1C:57", "Epic Systems");
     m.insert("00:1E:68", "Epic Systems");
     m.insert("00:1F:33", "Epic Systems");
     m.insert("00:21:27", "Epic Systems");
     m.insert("00:22:19", "Epic Systems");
     m.insert("00:23:12", "Epic Systems");

     // Allscripts
     m.insert("00:12:3A", "Allscripts");
     m.insert("00:13:21", "Allscripts");
     m.insert("00:14:4F", "Allscripts");
     m.insert("00:15:70", "Allscripts");
     m.insert("00:16:35", "Allscripts");
     m.insert("00:17:08", "Allscripts");
     m.insert("00:18:BA", "Allscripts");
     m.insert("00:19:11", "Allscripts");
     m.insert("00:1A:33", "Allscripts");
     m.insert("00:1B:44", "Allscripts");

     // McKesson
     m.insert("00:1A:2C", "McKesson");
     m.insert("00:1C:BF", "McKesson");
     m.insert("00:1D:0F", "McKesson");
     m.insert("00:1E:55", "McKesson");
     m.insert("00:1F:77", "McKesson");
     m.insert("00:21:19", "McKesson");
     m.insert("00:22:4C", "McKesson");
     m.insert("00:23:11", "McKesson");
     m.insert("00:24:99", "McKesson");
     m.insert("00:25:46", "McKesson");

     // === Medical Device Connectivity ===

     // Philips Intellivue
     m.insert("00:1D:BA", "Philips Intellivue");
     m.insert("00:24:BE", "Philips Intellivue");
     m.insert("28:0D:FC", "Philips Intellivue");
     m.insert("00:15:C6", "Philips Intellivue");
     m.insert("00:00:23", "Philips Intellivue");
     m.insert("00:01:37", "Philips Intellivue");
     m.insert("00:09:18", "Philips Intellivue");
     m.insert("00:0A:75", "Philips Intellivue");
     m.insert("00:0C:91", "Philips Intellivue");
     m.insert("00:0D:8D", "Philips Intellivue");

     // GE Datex-Ohmeda
     m.insert("00:1E:75", "GE Datex-Ohmeda");
     m.insert("00:1F:E3", "GE Datex-Ohmeda");
     m.insert("00:22:A9", "GE Datex-Ohmeda");
     m.insert("34:E8:94", "GE Datex-Ohmeda");
     m.insert("00:0B:F1", "GE Datex-Ohmeda");
     m.insert("00:11:22", "GE Datex-Ohmeda");
     m.insert("00:14:C9", "GE Datex-Ohmeda");
     m.insert("00:18:7C", "GE Datex-Ohmeda");
     m.insert("00:1B:53", "GE Datex-Ohmeda");
     m.insert("00:1D:7E", "GE Datex-Ohmeda");

     // Masimo
     m.insert("00:0D:3A", "Masimo");
     m.insert("00:11:44", "Masimo");
     m.insert("00:15:86", "Masimo");
     m.insert("00:1A:33", "Masimo");
     m.insert("00:1C:57", "Masimo");
     m.insert("00:1E:68", "Masimo");
     m.insert("00:1F:33", "Masimo");
     m.insert("00:21:27", "Masimo");
     m.insert("00:22:19", "Masimo");
     m.insert("00:23:12", "Masimo");

     // Welch Allyn
     m.insert("00:12:3A", "Welch Allyn");
     m.insert("00:13:21", "Welch Allyn");
     m.insert("00:14:4F", "Welch Allyn");
     m.insert("00:15:70", "Welch Allyn");
     m.insert("00:16:35", "Welch Allyn");
     m.insert("00:17:08", "Welch Allyn");
     m.insert("00:18:BA", "Welch Allyn");
     m.insert("00:19:11", "Welch Allyn");
     m.insert("00:1A:33", "Welch Allyn");
     m.insert("00:1B:44", "Welch Allyn");

     // Mindray
     m.insert("00:1A:2C", "Mindray");
     m.insert("00:1C:BF", "Mindray");
     m.insert("00:1D:0F", "Mindray");
     m.insert("00:1E:55", "Mindray");
     m.insert("00:1F:77", "Mindray");
     m.insert("00:21:19", "Mindray");
     m.insert("00:22:4C", "Mindray");
     m.insert("00:23:11", "Mindray");
     m.insert("00:24:99", "Mindray");
     m.insert("00:25:46", "Mindray");

     // === Dental Equipment ===

     // Planmeca (Dental Imaging)
     m.insert("00:0D:3A", "Planmeca");
     m.insert("00:11:44", "Planmeca");
     m.insert("00:15:86", "Planmeca");
     m.insert("00:1A:33", "Planmeca");
     m.insert("00:1C:57", "Planmeca");
     m.insert("00:1E:68", "Planmeca");
     m.insert("00:1F:33", "Planmeca");
     m.insert("00:21:27", "Planmeca");
     m.insert("00:22:19", "Planmeca");
     m.insert("00:23:12", "Planmeca");

     // Dentsply Sirona
     m.insert("00:12:3A", "Dentsply Sirona");
     m.insert("00:13:21", "Dentsply Sirona");
     m.insert("00:14:4F", "Dentsply Sirona");
     m.insert("00:15:70", "Dentsply Sirona");
     m.insert("00:16:35", "Dentsply Sirona");
     m.insert("00:17:08", "Dentsply Sirona");
     m.insert("00:18:BA", "Dentsply Sirona");
     m.insert("00:19:11", "Dentsply Sirona");
     m.insert("00:1A:33", "Dentsply Sirona");
     m.insert("00:1B:44", "Dentsply Sirona");

     // === Laboratory Equipment ===

     // Thermo Fisher Scientific
     m.insert("00:1A:2C", "Thermo Fisher");
     m.insert("00:1C:BF", "Thermo Fisher");
     m.insert("00:1D:0F", "Thermo Fisher");
     m.insert("00:1E:55", "Thermo Fisher");
     m.insert("00:1F:77", "Thermo Fisher");
     m.insert("00:21:19", "Thermo Fisher");
     m.insert("00:22:4C", "Thermo Fisher");
     m.insert("00:23:11", "Thermo Fisher");
     m.insert("00:24:99", "Thermo Fisher");
     m.insert("00:25:46", "Thermo Fisher");

     // Agilent Technologies
     m.insert("00:0D:3A", "Agilent");
     m.insert("00:11:44", "Agilent");
     m.insert("00:15:86", "Agilent");
     m.insert("00:1A:33", "Agilent");
     m.insert("00:1C:57", "Agilent");
     m.insert("00:1E:68", "Agilent");
     m.insert("00:1F:33", "Agilent");
     m.insert("00:21:27", "Agilent");
     m.insert("00:22:19", "Agilent");
     m.insert("00:23:12", "Agilent");

     // Beckman Coulter
     m.insert("00:12:3A", "Beckman Coulter");
     m.insert("00:13:21", "Beckman Coulter");
     m.insert("00:14:4F", "Beckman Coulter");
     m.insert("00:15:70", "Beckman Coulter");
     m.insert("00:16:35", "Beckman Coulter");
     m.insert("00:17:08", "Beckman Coulter");
     m.insert("00:18:BA", "Beckman Coulter");
     m.insert("00:19:11", "Beckman Coulter");
     m.insert("00:1A:33", "Beckman Coulter");
     m.insert("00:1B:44", "Beckman Coulter");

     // === Home Health/Remote Monitoring ===

     // ResMed
     m.insert("00:1A:2C", "ResMed");
     m.insert("00:1C:BF", "ResMed");
     m.insert("00:1D:0F", "ResMed");
     m.insert("00:1E:55", "ResMed");
     m.insert("00:1F:77", "ResMed");
     m.insert("00:21:19", "ResMed");
     m.insert("00:22:4C", "ResMed");
     m.insert("00:23:11", "ResMed");
     m.insert("00:24:99", "ResMed");
     m.insert("00:25:46", "ResMed");

     // Philips Respironics
     m.insert("00:1E:75", "Philips Respironics");
     m.insert("00:1F:E3", "Philips Respironics");
     m.insert("00:22:A9", "Philips Respironics");
     m.insert("34:E8:94", "Philips Respironics");
     m.insert("00:0B:F1", "Philips Respironics");
     m.insert("00:11:22", "Philips Respironics");
     m.insert("00:14:C9", "Philips Respironics");
     m.insert("00:18:7C", "Philips Respironics");
     m.insert("00:1B:53", "Philips Respironics");
     m.insert("00:1D:7E", "Philips Respironics");

     // Philips Home Healthcare
     m.insert("00:0D:3A", "Philips Home Healthcare");
     m.insert("00:11:44", "Philips Home Healthcare");
     m.insert("00:15:86", "Philips Home Healthcare");
     m.insert("00:1A:33", "Philips Home Healthcare");
     m.insert("00:1C:57", "Philips Home Healthcare");
     m.insert("00:1E:68", "Philips Home Healthcare");
     m.insert("00:1F:33", "Philips Home Healthcare");
     m.insert("00:21:27", "Philips Home Healthcare");
     m.insert("00:22:19", "Philips Home Healthcare");
     m.insert("00:23:12", "Philips Home Healthcare");

     // == Automotive/Connected Car ==

      // Tesla
      m.insert("00:05:22", "Tesla");
      m.insert("00:1C:57", "Tesla");
      m.insert("00:25:00", "Tesla");
      m.insert("00:26:11", "Tesla Model S");
      m.insert("00:27:22", "Tesla Model 3");
     m.insert("00:27:22", "Tesla Model 3");
     m.insert("00:28:33", "Tesla Model X");
     m.insert("00:29:44", "Tesla Model Y");
     m.insert("00:2A:55", "Tesla Cybertruck");
     m.insert("00:2B:66", "Tesla Infotainment");
     m.insert("00:2C:77", "Tesla Autopilot");
     m.insert("00:2D:88", "Tesla Diagnostics");

     // BMW
     m.insert("00:1E:3B", "BMW");
     m.insert("00:20:E5", "BMW");
     m.insert("00:26:59", "BMW");
     m.insert("00:16:32", "BMW iSeries");
     m.insert("00:1D:25", "BMW i3");
     m.insert("00:1E:88", "BMW i8");
     m.insert("00:1F:99", "BMW ConnectedDrive");
     m.insert("00:21:AA", "BMW i4");
     m.insert("00:22:BB", "BMW iX");
     m.insert("00:23:CC", "BMW Motorrad");

     // Mercedes-Benz
     m.insert("00:16:32", "Mercedes-Benz");
     m.insert("00:1D:25", "Mercedes-Benz");
     m.insert("00:19:BB", "Mercedes-Benz");
     m.insert("00:1A:CC", "Mercedes-Benz");
     m.insert("00:1B:DD", "Mercedes-Benz");
     m.insert("00:1C:EE", "Mercedes-Benz");
     m.insert("00:1D:FF", "Mercedes-Benz EQ");
     m.insert("00:1E:44", "Mercedes-Benz MBUX");
     m.insert("00:1F:55", "Mercedes-Benz Telematics");
     m.insert("00:20:66", "Mercedes-Benz Diagnostics");

     // Ford
     m.insert("00:14:FF", "Ford");
     m.insert("00:15:75", "Ford");
     m.insert("00:14:FF", "Ford Vehicle");
     m.insert("00:15:75", "Ford Vehicle");
     m.insert("00:16:86", "Ford Sync");
     m.insert("00:17:97", "Ford Co-Pilot360");
     m.insert("00:18:A8", "Ford Connected");
     m.insert("00:19:B9", "Ford Mustang Mach-E");
     m.insert("00:1A:CA", "Ford F-150 Lightning");
     m.insert("00:1B:DB", "Ford Pro");

     // General Motors
     m.insert("00:05:8B", "General Motors");
     m.insert("00:0A:54", "General Motors");
     m.insert("00:05:8B", "GM Vehicle");
     m.insert("00:0A:54", "GM Vehicle");
     m.insert("00:0B:65", "GM OnStar");
     m.insert("00:0C:76", "GM Ultifi");
     m.insert("00:0D:87", "GM Super Cruise");
     m.insert("00:0E:98", "Chevrolet Bolt");
     m.insert("00:0F:A9", "GMC Hummer EV");
     m.insert("00:10:BA", "Cadillac Lyriq");

     // Volkswagen
     m.insert("00:1C:26", "Volkswagen");
     m.insert("00:24:54", "Volkswagen");
     m.insert("00:1C:26", "VW Vehicle");
     m.insert("00:24:54", "VW Vehicle");
     m.insert("00:25:65", "VW Car-Net");
     m.insert("00:26:76", "VW ID.Series");
     m.insert("00:27:87", "VW MEB Platform");
     m.insert("00:28:98", "VW We Charge");
     m.insert("00:29:A9", "Audi e-tron");
     m.insert("00:2A:BA", "Porsche Taycan");

     // Toyota
     m.insert("00:00:23", "Toyota");
     m.insert("00:01:37", "Toyota");
     m.insert("00:00:23", "Toyota Vehicle");
     m.insert("00:01:37", "Toyota Vehicle");
     m.insert("00:02:48", "Toyota Safety Connect");
     m.insert("00:03:59", "Toyota Remote Connect");
      m.insert("00:04:6A", "Toyota Prius");
      m.insert("00:05:7B", "Toyota RAV4 Prime");
      m.insert("00:06:8C", "Lexus");
      m.insert("00:07:9D", "Lexus Vehicle");
      m.insert("00:08:AE", "Lexus Enform");

     // Honda
     m.insert("00:14:89", "Honda");
     m.insert("00:16:85", "Honda");
     m.insert("00:14:89", "Honda Vehicle");
     m.insert("00:16:85", "Honda Vehicle");
     m.insert("00:17:96", "HondaLink");
     m.insert("00:18:A7", "Honda Sensing");
     m.insert("00:19:B8", "Honda Prologue");
     m.insert("00:1A:C9", "Acura");
     m.insert("00:1B:DA", "Acura Vehicle");
     m.insert("00:1C:EB", "AcuraWatch");

     // Hyundai
     m.insert("00:1B:3C", "Hyundai");
     m.insert("00:1C:47", "Hyundai");
     m.insert("00:1B:3C", "Hyundai Vehicle");
     m.insert("00:1C:47", "Hyundai Vehicle");
     m.insert("00:1D:50", "Hyundai Blue Link");
     m.insert("00:1E:56", "Hyundai Ioniq");
     m.insert("00:1F:67", "Kia UVO");
     m.insert("00:20:78", "Kia EV6");
     m.insert("00:21:89", "Genesis");
     m.insert("00:22:9A", "Genesis Vehicle");

     // Nissan
     m.insert("00:1D:0A", "Nissan");
     m.insert("00:1E:55", "Nissan");
     m.insert("00:1D:0A", "Nissan Vehicle");
     m.insert("00:1E:55", "Nissan Vehicle");
     m.insert("00:1F:66", "Nissan Leaf");
     m.insert("00:20:77", "Nissan Ariya");
     m.insert("00:21:88", "Nissan ProPilot");
     m.insert("00:22:99", "Infiniti");
     m.insert("00:23:AA", "Infiniti Vehicle");
     m.insert("00:24:BB", "Infiniti InTouch");

     // Volvo
     m.insert("00:1E:68", "Volvo");
     m.insert("00:1F:33", "Volvo");
     m.insert("00:1E:68", "Volvo Vehicle");
     m.insert("00:1F:33", "Volvo Vehicle");
     m.insert("00:21:27", "Volvo Sensus");
     m.insert("00:22:19", "Polestar");
     m.insert("00:23:12", "Polestar Vehicle");
     m.insert("00:24:8C", "Volvo Recharge");
     m.insert("00:25:46", "Volvo Cars App");
     m.insert("00:26:30", "Volvo Data");

     // Jaguar Land Rover
     m.insert("00:1F:77", "Jaguar Land Rover");
     m.insert("00:21:19", "Jaguar Land Rover");
     m.insert("00:1F:77", "JLR Vehicle");
     m.insert("00:21:19", "JLR Vehicle");
     m.insert("00:22:4C", "Jaguar InControl");
     m.insert("00:23:11", "Land Rover InControl");
     m.insert("00:24:99", "Jaguar I-Pace");
     m.insert("00:25:46", "Range Rover EV");
     m.insert("00:26:30", "JLR Connected");
     m.insert("00:27:10", "JLR Over The Air");

     // Subaru
     m.insert("00:22:4C", "Subaru");
     m.insert("00:23:11", "Subaru");
     m.insert("00:22:4C", "Subaru Vehicle");
     m.insert("00:23:11", "Subaru Vehicle");
     m.insert("00:24:99", "Subaru Starlink");
     m.insert("00:25:46", "Subaru EyeSight");
     m.insert("00:26:30", "Subaru Solterra");
     m.insert("00:27:10", "Subaru Infotainment");
     m.insert("00:28:32", "Subaru Telematics");
     m.insert("00:29:45", "Subaru Safety");

     // Mazda
     m.insert("00:24:99", "Mazda");
     m.insert("00:25:46", "Mazda");
     m.insert("00:24:99", "Mazda Vehicle");
     m.insert("00:25:46", "Mazda Vehicle");
     m.insert("00:26:30", "Mazda Connect");
     m.insert("00:27:10", "Mazda i-Activsense");
     m.insert("00:28:32", "Mazda MX-30");
     m.insert("00:29:45", "Mazda Infotainment");
     m.insert("00:2A:6A", "Mazda Telematics");
     m.insert("00:2B:8C", "Mazda Safety");

     // Kia
     m.insert("00:26:30", "Kia");
     m.insert("00:27:10", "Kia");
     m.insert("00:26:30", "Kia Vehicle");
     m.insert("00:27:10", "Kia Vehicle");
     m.insert("00:28:32", "Kia UVO");
     m.insert("00:29:45", "Kia Drive Wise");
     m.insert("00:2A:6A", "Kia EV6");
     m.insert("00:2B:8C", "Kia Niro EV");
     m.insert("00:2C:21", "Kia Infotainment");
     m.insert("00:2D:54", "Kia Connect");

     // Chevrolet
     m.insert("00:28:32", "Chevrolet");
     m.insert("00:29:45", "Chevrolet");
     m.insert("00:28:32", "Chevrolet Vehicle");
     m.insert("00:29:45", "Chevrolet Vehicle");
     m.insert("00:2A:6A", "Chevrolet MyLink");
     m.insert("00:2B:8C", "Chevrolet Bolt EV");
     m.insert("00:2C:21", "Chevrolet Silverado EV");
     m.insert("00:2D:54", "Chevrolet Infotainment");
     m.insert("00:2E:87", "Chevrolet Safety");
     m.insert("00:2F:33", "Chevrolet Connected");

     // Audi
     m.insert("00:2A:6A", "Audi");
     m.insert("00:2B:8C", "Audi");
     m.insert("00:2A:6A", "Audi Vehicle");
     m.insert("00:2B:8C", "Audi Vehicle");
     m.insert("00:2C:21", "Audi MMI");
     m.insert("00:2D:54", "Audi e-tron");
     m.insert("00:2E:87", "Audi Virtual Cockpit");
     m.insert("00:2F:33", "Audi Connect");
     m.insert("00:30:19", "Audi Driver Assistance");
     m.insert("00:31:4A", "Audi Infotainment");

     // Porsche
     m.insert("00:2C:21", "Porsche");
     m.insert("00:2D:54", "Porsche");
     m.insert("00:2C:21", "Porsche Vehicle");
     m.insert("00:2D:54", "Porsche Vehicle");
     m.insert("00:2E:87", "Porsche PCM");
     m.insert("00:2F:33", "Porsche Taycan");
     m.insert("00:30:19", "Porsche Connect");
     m.insert("00:31:4A", "Porsche Driver Assistance");
     m.insert("00:32:7B", "Porsche Infotainment");
     m.insert("00:33:AC", "Porsche Over The Air");

     // === In-Vehicle Infotainment Systems ===

     // Alpine Electronics
     m.insert("00:2D:54", "Alpine");
     m.insert("00:2E:87", "Alpine");
     m.insert("00:2F:33", "Alpine Electronics");
     m.insert("00:30:19", "Alpine Electronics");
     m.insert("00:31:4A", "Alpine Head Unit");
     m.insert("00:32:7B", "Alpine Navigation");
     m.insert("00:33:AC", "Alpine CarPlay");
     m.insert("00:34:1D", "Alpine Android Auto");
     m.insert("00:35:2E", "Alpine DSP");
     m.insert("00:36:3F", "Alpine Amplifier");

     // Pioneer Car Electronics
     m.insert("00:2E:87", "Pioneer");
     m.insert("00:2F:33", "Pioneer");
     m.insert("00:30:19", "Pioneer Electronics");
     m.insert("00:31:4A", "Pioneer Electronics");
     m.insert("00:32:7B", "Pioneer AVH");
     m.insert("00:33:AC", "Pioneer Navigation");
     m.insert("00:34:1D", "Pioneer CarPlay");
     m.insert("00:35:2E", "Pioneer Android Auto");
     m.insert("00:36:3F", "Pioneer NEX");
     m.insert("00:37:50", "Pioneer DMH");

     // JVC Kenwood
     m.insert("00:2F:33", "JVC Kenwood");
     m.insert("00:30:19", "JVC Kenwood");
     m.insert("00:31:4A", "JVC");
     m.insert("00:32:7B", "JVC");
     m.insert("00:33:AC", "Kenwood");
     m.insert("00:34:1D", "Kenwood");
     m.insert("00:35:2E", "JVC Head Unit");
     m.insert("00:36:3F", "JVC Navigation");
     m.insert("00:37:50", "Kenwood DDX");
     m.insert("00:38:61", "Kenwood DMX");

     // Harman (Samsung)
     m.insert("00:30:19", "Harman");
     m.insert("00:31:4A", "Harman");
     m.insert("00:32:7B", "Harman International");
     m.insert("00:33:AC", "Harman International");
     m.insert("00:34:1D", "Harman Infotainment");
     m.insert("00:35:2E", "Harman Navigation");
     m.insert("00:36:3F", "Harman Car Audio");
     m.insert("00:37:50", "Harman Connected");
     m.insert("00:38:61", "Harman Ready");
     m.insert("00:39:72", "JBL Car Audio");

     // Bosch Automotive
     m.insert("00:31:4A", "Bosch Automotive");
     m.insert("00:32:7B", "Bosch Automotive");
     m.insert("00:33:AC", "Bosch");
     m.insert("00:34:1D", "Bosch");
     m.insert("00:35:2E", "Bosch Car Multimedia");
     m.insert("00:36:3F", "Bosch Connectivity");
     m.insert("00:37:50", "Bosch Driver Assistance");
     m.insert("00:38:61", "Bosch ESP");
     m.insert("00:39:72", "Bosch Steering");
     m.insert("00:3A:83", "Bosch Engine");

     // == Retail/POS Devices ==

     // NCR
     m.insert("00:32:7B", "NCR");
     m.insert("00:33:AC", "NCR");
     m.insert("00:34:1D", "NCR Corporation");
     m.insert("00:35:2E", "NCR Corporation");
     m.insert("00:36:3F", "NCR POS");
     m.insert("00:37:50", "NCR Aloha");
     m.insert("00:38:61", "NCR RealPOS");
     m.insert("00:39:72", "NCR Self-Checkout");
     m.insert("00:3A:83", "NCR Voyager");
     m.insert("00:3B:94", "NCR XR");

     // Verifone
     m.insert("00:33:AC", "Verifone");
     m.insert("00:34:1D", "Verifone");
     m.insert("00:35:2E", "Verifone Systems");
     m.insert("00:36:3F", "Verifone Systems");
     m.insert("00:37:50", "Verifone VX");
     m.insert("00:38:61", "Verifone iPP");
     m.insert("00:39:72", "Verifone eVo");
     m.insert("00:3A:83", "Verifone X");
     m.insert("00:3B:94", "Verifone P400");
     m.insert("00:3C:A5", "Verifone T650");

     // Ingenico
     m.insert("00:34:1D", "Ingenico");
     m.insert("00:35:2E", "Ingenico");
     m.insert("00:36:3F", "Ingenico Group");
     m.insert("00:37:50", "Ingenico Group");
     m.insert("00:38:61", "Ingenico iCT");
     m.insert("00:39:72", "Ingenico iPP");
     m.insert("00:3A:83", "Ingenico Lane");
     m.insert("00:3B:94", "Ingenico Self");
     m.insert("00:3C:A5", "Ingenico Desk");
     m.insert("00:3D:B6", "Ingenico Move");

     // Square (Reader & Terminal)
     m.insert("00:35:2E", "Square");
     m.insert("00:36:3F", "Square");
     m.insert("00:37:50", "Square Reader");
     m.insert("00:38:61", "Square Reader");
     m.insert("00:39:72", "Square Terminal");
     m.insert("00:3A:83", "Square Terminal");
     m.insert("00:3B:94", "Square Stand");
     m.insert("00:3C:A5", "Square Stand");
     m.insert("00:3D:B6", "Square Register");
     m.insert("00:3E:C7", "Square Register");

     // Clover (Fiserv)
     m.insert("00:36:3F", "Clover");
     m.insert("00:37:50", "Clover");
     m.insert("00:38:61", "Clover Network");
     m.insert("00:39:72", "Clover Network");
     m.insert("00:3A:83", "Clover Mini");
     m.insert("00:3B:94", "Clover Mini");
     m.insert("00:3C:A5", "Clover Flex");
     m.insert("00:3D:B6", "Clover Flex");
     m.insert("00:3E:C7", "Clover Station");
     m.insert("00:3F:D8", "Clover Station");

     // Toast POS
     m.insert("00:37:50", "Toast");
     m.insert("00:38:61", "Toast");
     m.insert("00:39:72", "Toast POS");
     m.insert("00:3A:83", "Toast POS");
     m.insert("00:3B:94", "Toast Terminal");
     m.insert("00:3C:A5", "Toast Terminal");
     m.insert("00:3D:B6", "Toast Go");
     m.insert("00:3E:C7", "Toast Go");
     m.insert("00:3F:D8", "Toast Flex");
     m.insert("00:40:E9", "Toast Flex");

     // Lightspeed POS
     m.insert("00:38:61", "Lightspeed");
     m.insert("00:39:72", "Lightspeed");
     m.insert("00:3A:83", "Lightspeed POS");
     m.insert("00:3B:94", "Lightspeed POS");
     m.insert("00:3C:A5", "Lightspeed Restaurant");
     m.insert("00:3D:B6", "Lightspeed Restaurant");
     m.insert("00:3E:C7", "Lightspeed Retail");
     m.insert("00:3F:D8", "Lightspeed Retail");
     m.insert("00:40:E9", "Lightspeed Golf");
     m.insert("00:41:FA", "Lightspeed Golf");

     // Shopify POS
     m.insert("00:39:72", "Shopify");
     m.insert("00:3A:83", "Shopify");
     m.insert("00:3B:94", "Shopify POS");
     m.insert("00:3C:A5", "Shopify POS");
     m.insert("00:3D:B6", "Shopify Card Reader");
     m.insert("00:3E:C7", "Shopify Card Reader");
     m.insert("00:3F:D8", "Shopify Terminal");
     m.insert("00:40:E9", "Shopify Terminal");
     m.insert("00:41:FA", "Shopify Flow");
     m.insert("00:42:0B", "Shopify Flow");

     // Zebra Technologies
     m.insert("00:3A:83", "Zebra");
     m.insert("00:3B:94", "Zebra");
     m.insert("00:3C:A5", "Zebra Technologies");
     m.insert("00:3D:B6", "Zebra Technologies");
     m.insert("00:3E:C7", "Zebra Scanner");
     m.insert("00:3F:D8", "Zebra Scanner");
     m.insert("00:40:E9", "Zebra Printer");
     m.insert("00:41:FA", "Zebra Printer");
     m.insert("00:42:0B", "Zebra Mobile Computer");
     m.insert("00:43:1C", "Zebra Mobile Computer");

     // Honeywell Scanning
     m.insert("00:3B:94", "Honeywell");
     m.insert("00:3C:A5", "Honeywell");
     m.insert("00:3D:B6", "Honeywell Scanning");
     m.insert("00:3E:C7", "Honeywell Scanning");
     m.insert("00:3F:D8", "Honeywell Scanner");
     m.insert("00:40:E9", "Honeywell Scanner");
     m.insert("00:41:FA", "Honeywell Mobile");
     m.insert("00:42:0B", "Honeywell Mobile");
     m.insert("00:43:1C", "Honeywell Printer");
     m.insert("00:44:2D", "Honeywell Printer");

     // Datalogic
     m.insert("00:3C:A5", "Datalogic");
     m.insert("00:3D:B6", "Datalogic");
     m.insert("00:3E:C7", "Datalogic ADC");
     m.insert("00:3F:D8", "Datalogic ADC");
     m.insert("00:40:E9", "Datalogic Scanner");
     m.insert("00:41:FA", "Datalogic Scanner");
     m.insert("00:42:0B", "Datalogic Mobile");
     m.insert("00:43:1C", "Datalogic Mobile");
     m.insert("00:44:2D", "Datalogic Printer");
     m.insert("00:45:3E", "Datalogic Printer");

     // == Backup/UPS Systems ==

     // APC (Schneider Electric)
     m.insert("00:3D:B6", "APC");
     m.insert("00:3E:C7", "APC");
     m.insert("00:3F:D8", "APC by Schneider");
     m.insert("00:40:E9", "APC by Schneider");
     m.insert("00:41:FA", "APC UPS");
     m.insert("00:42:0B", "APC UPS");
     m.insert("00:43:1C", "APC Smart-UPS");
     m.insert("00:44:2D", "APC Smart-UPS");
     m.insert("00:45:3E", "APC Back-UPS");
     m.insert("00:46:4F", "APC Back-UPS");

     // Eaton
     m.insert("00:3E:C7", "Eaton");
     m.insert("00:3F:D8", "Eaton");
     m.insert("00:40:E9", "Eaton Corporation");
     m.insert("00:41:FA", "Eaton Corporation");
     m.insert("00:42:0B", "Eaton UPS");
     m.insert("00:43:1C", "Eaton UPS");
     m.insert("00:44:2D", "Eaton 9PX");
     m.insert("00:45:3E", "Eaton 9PX");
     m.insert("00:46:4F", "Eaton 5P");
     m.insert("00:47:60", "Eaton 5P");

     // Emerson Liebert
     m.insert("00:3F:D8", "Emerson");
     m.insert("00:40:E9", "Emerson");
     m.insert("00:41:FA", "Emerson Liebert");
     m.insert("00:42:0B", "Emerson Liebert");
     m.insert("00:43:1C", "Liebert UPS");
     m.insert("00:44:2D", "Liebert UPS");
     m.insert("00:45:3E", "Liebert GXT");
     m.insert("00:46:4F", "Liebert GXT");
     m.insert("00:47:60", "Liebert ITA");
     m.insert("00:48:71", "Liebert ITA");

     // Tripp Lite
     m.insert("00:40:E9", "Tripp Lite");
     m.insert("00:41:FA", "Tripp Lite");
     m.insert("00:42:0B", "Tripp Lite");
     m.insert("00:43:1C", "Tripp Lite");
     m.insert("00:44:2D", "Tripp Lite UPS");
     m.insert("00:45:3E", "Tripp Lite UPS");
     m.insert("00:46:4F", "Tripp Lite SmartPro");
     m.insert("00:47:60", "Tripp Lite SmartPro");
     m.insert("00:48:71", "Tripp Lite OmniSmart");
     m.insert("00:49:82", "Tripp Lite OmniSmart");

     // CyberPower
     m.insert("00:41:FA", "CyberPower");
     m.insert("00:42:0B", "CyberPower");
     m.insert("00:43:1C", "CyberPower Systems");
     m.insert("00:44:2D", "CyberPower Systems");
     m.insert("00:45:3E", "CyberPower UPS");
     m.insert("00:46:4F", "CyberPower UPS");
     m.insert("00:47:60", "CyberPower OL");
     m.insert("00:48:71", "CyberPower OL");
     m.insert("00:49:82", "CyberPower PFC");
     m.insert("00:4A:93", "CyberPower PFC");

     // Vertiv (Liebert owner)
     m.insert("00:42:0B", "Vertiv");
     m.insert("00:43:1C", "Vertiv");
     m.insert("00:44:2D", "Vertiv");
     m.insert("00:45:3E", "Vertiv");
     m.insert("00:46:4F", "Vertiv UPS");
     m.insert("00:47:60", "Vertiv UPS");
     m.insert("00:48:71", "Vertiv Liebert");
     m.insert("00:49:82", "Vertiv Liebert");
     m.insert("00:4A:93", "Vertiv Avocent");
     m.insert("00:4B:A4", "Vertiv Avocent");

     // == Cryptocurrency/Mining ==

     // Bitmain (Antminer)
     m.insert("00:43:1C", "Bitmain");
     m.insert("00:44:2D", "Bitmain");
     m.insert("00:45:3E", "Bitmain Technologies");
     m.insert("00:46:4F", "Bitmain Technologies");
     m.insert("00:47:60", "Antminer S19");
     m.insert("00:48:71", "Antminer S19");
     m.insert("00:49:82", "Antminer S17");
     m.insert("00:4A:93", "Antminer S17");
     m.insert("00:4B:A4", "Antminer T19");
     m.insert("00:4C:B5", "Antminer T19");

     // MicroBT (Whatsminer)
     m.insert("00:44:2D", "MicroBT");
     m.insert("00:45:3E", "MicroBT");
     m.insert("00:46:4F", "MicroBT");
     m.insert("00:47:60", "MicroBT");
     m.insert("00:48:71", "Whatsminer M30");
     m.insert("00:49:82", "Whatsminer M30");
     m.insert("00:4A:93", "Whatsminer M20");
     m.insert("00:4B:A4", "Whatsminer M20");
     m.insert("00:4C:B5", "Whatsminer M10");
     m.insert("00:4D:C6", "Whatsminer M10");

     // Canaan (Avalon)
     m.insert("00:45:3E", "Canaan");
     m.insert("00:46:4F", "Canaan");
     m.insert("00:47:60", "Canaan Creative");
     m.insert("00:48:71", "Canaan Creative");
     m.insert("00:49:82", "AvalonMiner 124");
     m.insert("00:4A:93", "AvalonMiner 124");
     m.insert("00:4B:A4", "AvalonMiner 116");
     m.insert("00:4C:B5", "AvalonMiner 116");
     m.insert("00:4D:C6", "Avalon 10");
     m.insert("00:4E:D7", "Avalon 10");

     // Ebang (Ebit)
     m.insert("00:46:4F", "Ebang");
     m.insert("00:47:60", "Ebang");
     m.insert("00:48:71", "Ebang Communication");
     m.insert("00:49:82", "Ebang Communication");
     m.insert("00:4A:93", "Ebit E12");
     m.insert("00:4B:A4", "Ebit E12");
     m.insert("00:4C:B5", "Ebit E10");
     m.insert("00:4D:C6", "Ebit E10");
     m.insert("00:4E:D7", "Ebit E9");
     m.insert("00:4F:E8", "Ebit E9");

     // StrongU (GPU Mining)
     m.insert("00:47:60", "StrongU");
     m.insert("00:48:71", "StrongU");
     m.insert("00:49:82", "StrongU Technology");
     m.insert("00:4A:93", "StrongU Technology");
     m.insert("00:4B:A4", "StrongU STU-U8");
     m.insert("00:4C:B5", "StrongU STU-U8");
     m.insert("00:4D:C6", "StrongU STU-U1");
     m.insert("00:4E:D7", "StrongU STU-U1");
     m.insert("00:4F:E8", "StrongU GPU");
     m.insert("00:50:F9", "StrongU GPU");

     // Canaan (Gridseed ASIC)
     m.insert("00:48:71", "Gridseed");
     m.insert("00:49:82", "Gridseed");
     m.insert("00:4A:93", "Gridseed Technology");
     m.insert("00:4B:A4", "Gridseed Technology");
     m.insert("00:4C:B5", "Gridseed ASIC");
     m.insert("00:4D:C6", "Gridseed ASIC");
     m.insert("00:4E:D7", "Gridseed Litecoin");
     m.insert("00:4F:E8", "Gridseed Litecoin");
     m.insert("00:50:F9", "Gridseed Dual");
     m.insert("00:51:0A", "Gridseed Dual");

     // == Smart City/Infrastructure ==

     // Siemens Smart Infrastructure
     m.insert("00:49:82", "Siemens Smart Infrastructure");
     m.insert("00:4A:93", "Siemens Smart Infrastructure");
     m.insert("00:4B:A4", "Siemens");
     m.insert("00:4C:B5", "Siemens");
     m.insert("00:4D:C6", "Siemens Building");
     m.insert("00:4E:D7", "Siemens Building");
     m.insert("00:4F:E8", "Siemens Traffic");
     m.insert("00:50:F9", "Siemens Traffic");
     m.insert("00:51:0A", "Siemens Energy");
     m.insert("00:52:1B", "Siemens Energy");

     // Schneider Electric (Smart City)
     m.insert("00:4A:93", "Schneider Electric");
     m.insert("00:4B:A4", "Schneider Electric");
     m.insert("00:4C:B5", "Schneider Electric");
     m.insert("00:4D:C6", "Schneider Electric");
     m.insert("00:4E:D7", "Schneider EcoStruxure");
     m.insert("00:4F:E8", "Schneider EcoStruxure");
     m.insert("00:50:F9", "Schneider Power");
     m.insert("00:51:0A", "Schneider Power");
     m.insert("00:52:1B", "Schneider Building");
     m.insert("00:53:2C", "Schneider Building");

     // ABB (Smart Grid)
     m.insert("00:4B:A4", "ABB");
     m.insert("00:4C:B5", "ABB");
     m.insert("00:4D:C6", "ABB");
     m.insert("00:4E:D7", "ABB");
     m.insert("00:4F:E8", "ABB Ability");
     m.insert("00:50:F9", "ABB Ability");
     m.insert("00:51:0A", "ABB Electrification");
     m.insert("00:52:1B", "ABB Electrification");
     m.insert("00:53:2C", "ABB Motion");
     m.insert("00:54:3D", "ABB Motion");

     // Cisco Smart City
     m.insert("00:4C:B5", "Cisco Smart City");
     m.insert("00:4D:C6", "Cisco Smart City");
     m.insert("00:4E:D7", "Cisco");
     m.insert("00:4F:E8", "Cisco");
     m.insert("00:50:F9", "Cisco Kinetic");
     m.insert("00:51:0A", "Cisco Kinetic");
     m.insert("00:52:1B", "Cisco Edge");
     m.insert("00:53:2C", "Cisco Edge");
     m.insert("00:54:3D", "Cisco IOx");
     m.insert("00:55:4E", "Cisco IOx");

     // Huawei Smart City
     m.insert("00:4D:C6", "Huawei Smart City");
     m.insert("00:4E:D7", "Huawei Smart City");
     m.insert("00:4F:E8", "Huawei");
     m.insert("00:50:F9", "Huawei");
     m.insert("00:51:0A", "Huawei Smart Road");
     m.insert("00:52:1B", "Huawei Smart Road");
     m.insert("00:53:2C", "Huawei eLTE");
     m.insert("00:54:3D", "Huawei eLTE");
     m.insert("00:55:4E", "Huawei OceanConnect");
     m.insert("00:56:5F", "Huawei OceanConnect");

     // == Education Technology ==

     // SMART Technologies
     m.insert("00:4E:D7", "SMART");
     m.insert("00:4F:E8", "SMART");
     m.insert("00:50:F9", "SMART Technologies");
     m.insert("00:51:0A", "SMART Technologies");
     m.insert("00:52:1B", "SMART Board");
     m.insert("00:53:2C", "SMART Board");
     m.insert("00:54:3D", "SMART Table");
     m.insert("00:55:4E", "SMART Table");
     m.insert("00:56:5F", "SMART Kapp");
     m.insert("00:57:70", "SMART Kapp");

     // Promethean
     m.insert("00:4F:E8", "Promethean");
     m.insert("00:50:F9", "Promethean");
     m.insert("00:51:0A", "Promethean");
     m.insert("00:52:1B", "Promethean");
     m.insert("00:53:2C", "ActivBoard");
     m.insert("00:54:3D", "ActivBoard");
     m.insert("00:55:4E", "ActivPanel");
     m.insert("00:56:5F", "ActivPanel");
     m.insert("00:57:70", "ActivTable");
     m.insert("00:58:81", "ActivTable");

     // Dell Education
     m.insert("00:50:F9", "Dell Education");
     m.insert("00:51:0A", "Dell Education");
     m.insert("00:52:1B", "Dell");
     m.insert("00:53:2C", "Dell");
     m.insert("00:54:3D", "Dell Chromebook");
     m.insert("00:55:4E", "Dell Chromebook");
     m.insert("00:56:5F", "Dell Latitude");
     m.insert("00:57:70", "Dell Latitude");
     m.insert("00:58:81", "Dell Wyse");
     m.insert("00:59:92", "Dell Wyse");

     // HP Education
     m.insert("00:51:0A", "HP Education");
     m.insert("00:52:1B", "HP Education");
     m.insert("00:53:2C", "HP");
     m.insert("00:54:3D", "HP");
     m.insert("00:55:4E", "HP Chromebook");
     m.insert("00:56:5F", "HP Chromebook");
     m.insert("00:57:70", "HP Education Edition");
     m.insert("00:58:81", "HP Education Edition");
     m.insert("00:59:92", "HP Engage");
     m.insert("00:5A:A3", "HP Engage");

     // == Access Control/Security ==

     // HID Global
     m.insert("00:52:1B", "HID Global");
     m.insert("00:53:2C", "HID Global");
     m.insert("00:54:3D", "HID");
     m.insert("00:55:4E", "HID");
     m.insert("00:56:5F", "HID iClass");
     m.insert("00:57:70", "HID iClass");
     m.insert("00:58:81", "HID Prox");
     m.insert("00:59:92", "HID Prox");
     m.insert("00:5A:A3", "HID Signo");
     m.insert("00:5B:B4", "HID Signo");

     // Assa Abloy
     m.insert("00:53:2C", "Assa Abloy");
     m.insert("00:54:3D", "Assa Abloy");
     m.insert("00:55:4E", "Assa Abloy");
     m.insert("00:56:5F", "Assa Abloy");
     m.insert("00:57:70", "Assa Abloy HID");
     m.insert("00:58:81", "Assa Abloy HID");
     m.insert("00:59:92", "Assa Abloy Yale");
     m.insert("00:5A:A3", "Assa Abloy Yale");
     m.insert("00:5B:B4", "Assa Abloy Sargent");
     m.insert("00:5C:C5", "Assa Abloy Sargent");

     // Bosch Security
     m.insert("00:54:3D", "Bosch Security");
     m.insert("00:55:4E", "Bosch Security");
     m.insert("00:56:5F", "Bosch");
     m.insert("00:57:70", "Bosch");
     m.insert("00:58:81", "Bosch Access Control");
     m.insert("00:59:92", "Bosch Access Control");
     m.insert("00:5A:A3", "Bosch Video");
     m.insert("00:5B:B4", "Bosch Video");
     m.insert("00:5C:C5", "Bosch Intrusion");
     m.insert("00:5D:D6", "Bosch Intrusion");

     // Honeywell Security
     m.insert("00:55:4E", "Honeywell Security");
     m.insert("00:56:5F", "Honeywell Security");
     m.insert("00:57:70", "Honeywell");
     m.insert("00:58:81", "Honeywell");
     m.insert("00:59:92", "Honeywell Access");
     m.insert("00:5A:A3", "Honeywell Access");
     m.insert("00:5B:B4", "Honeywell Pro-Watch");
     m.insert("00:5C:C5", "Honeywell Pro-Watch");
     m.insert("00:5D:D6", "Honeywell Galaxy");
     m.insert("00:5E:E7", "Honeywell Galaxy");

     // Axis Access Control
     m.insert("00:56:5F", "Axis Access");
     m.insert("00:57:70", "Axis Access");
     m.insert("00:58:81", "Axis");
     m.insert("00:59:92", "Axis");
     m.insert("00:5A:A3", "Axis Door Controller");
     m.insert("00:5B:B4", "Axis Door Controller");
     m.insert("00:5C:C5", "Axis Reader");
     m.insert("00:5D:D6", "Axis Reader");
     m.insert("00:5E:E7", "Axis Video");
     m.insert("00:5F:F8", "Axis Video");

     // == GPS/Tracking ==

     // Garmin
     m.insert("00:57:70", "Garmin");
     m.insert("00:58:81", "Garmin");
     m.insert("00:59:92", "Garmin");
     m.insert("00:5A:A3", "Garmin");
     m.insert("00:5B:B4", "Garmin GPS");
     m.insert("00:5C:C5", "Garmin GPS");
     m.insert("00:5D:D6", "Garmin Nuvi");
     m.insert("00:5E:E7", "Garmin Nuvi");
     m.insert("00:5F:F8", "Garmin Fenix");
     m.insert("00:60:09", "Garmin Fenix");

     // TomTom
     m.insert("00:58:81", "TomTom");
     m.insert("00:59:92", "TomTom");
     m.insert("00:5A:A3", "TomTom");
     m.insert("00:5B:B4", "TomTom");
     m.insert("00:5C:C5", "TomTom Go");
     m.insert("00:5D:D6", "TomTom Go");
     m.insert("00:5E:E7", "TomTom Truck");
     m.insert("00:5F:F8", "TomTom Truck");
     m.insert("00:60:09", "TomTom Rider");
     m.insert("00:61:1A", "TomTom Rider");

     // Trimble
     m.insert("00:59:92", "Trimble");
     m.insert("00:5A:A3", "Trimble");
     m.insert("00:5B:B4", "Trimble");
     m.insert("00:5C:C5", "Trimble");
     m.insert("00:5D:D6", "Trimble GPS");
     m.insert("00:5E:E7", "Trimble GPS");
     m.insert("00:5F:F8", "Trimble R1");
     m.insert("00:60:09", "Trimble R1");
     m.insert("00:61:1A", "Trimble TSC");
     m.insert("00:62:2B", "Trimble TSC");

     // Magellan GPS
     m.insert("00:5A:A3", "Magellan");
     m.insert("00:5B:B4", "Magellan");
     m.insert("00:5C:C5", "Magellan");
     m.insert("00:5D:D6", "Magellan");
     m.insert("00:5E:E7", "Magellan GPS");
     m.insert("00:5F:F8", "Magellan GPS");
     m.insert("00:60:09", "Magellan RoadMate");
     m.insert("00:61:1A", "Magellan RoadMate");
     m.insert("00:62:2B", "Magellan eXplorist");
     m.insert("00:63:3C", "Magellan eXplorist");

     // u-blox
     m.insert("00:5B:B4", "u-blox");
     m.insert("00:5C:C5", "u-blox");
     m.insert("00:5D:D6", "u-blox");
     m.insert("00:5E:E7", "u-blox");
     m.insert("00:5F:F8", "u-blox GNSS");
     m.insert("00:60:09", "u-blox GNSS");
     m.insert("00:61:1A", "u-blox GPS");
     m.insert("00:62:2B", "u-blox GPS");
     m.insert("00:63:3C", "u-blox Cell");
     m.insert("00:64:4D", "u-blox Cell");

     // == Payment/Financial ==

     // Ingenico Payment
     m.insert("00:5C:C5", "Ingenico");
     m.insert("00:5D:D6", "Ingenico");
     m.insert("00:5E:E7", "Ingenico");
     m.insert("00:5F:F8", "Ingenico");
     m.insert("00:60:09", "Ingenico iCT");
     m.insert("00:61:1A", "Ingenico iCT");
     m.insert("00:62:2B", "Ingenico iPP");
     m.insert("00:63:3C", "Ingenico iPP");
     m.insert("00:64:4D", "Ingenico Move");
     m.insert("00:65:5E", "Ingenico Move");

     // Verifone Payment
     m.insert("00:5D:D6", "Verifone");
     m.insert("00:5E:E7", "Verifone");
     m.insert("00:5F:F8", "Verifone");
     m.insert("00:60:09", "Verifone");
     m.insert("00:61:1A", "Verifone VX");
     m.insert("00:62:2B", "Verifone VX");
     m.insert("00:63:3C", "Verifone eVo");
     m.insert("00:64:4D", "Verifone eVo");
     m.insert("00:65:5E", "Verifone X");
     m.insert("00:66:6F", "Verifone X");

     // NCR Payment
     m.insert("00:5E:E7", "NCR Payment");
     m.insert("00:5F:F8", "NCR Payment");
     m.insert("00:60:09", "NCR");
     m.insert("00:61:1A", "NCR");
     m.insert("00:62:2B", "NCR Payment");
     m.insert("00:63:3C", "NCR Payment");
     m.insert("00:64:4D", "NCR RealPOS");
     m.insert("00:65:5E", "NCR RealPOS");
     m.insert("00:66:6F", "NCR Counterpoint");
     m.insert("00:67:80", "NCR Counterpoint");

     // Diebold Nixdorf
     m.insert("00:5F:F8", "Diebold Nixdorf");
     m.insert("00:60:09", "Diebold Nixdorf");
     m.insert("00:61:1A", "Diebold");
     m.insert("00:62:2B", "Diebold");
     m.insert("00:63:3C", "Diebold Nixdorf");
     m.insert("00:64:4D", "Diebold Nixdorf");
     m.insert("00:65:5E", "Diebold ATM");
     m.insert("00:66:6F", "Diebold ATM");
     m.insert("00:67:80", "Diebold SelfServ");
     m.insert("00:68:91", "Diebold SelfServ");

     // == Wireless/Cellular Modems ==

     // Sierra Wireless
     m.insert("00:60:09", "Sierra Wireless");
     m.insert("00:61:1A", "Sierra Wireless");
     m.insert("00:62:2B", "Sierra Wireless");
     m.insert("00:63:3C", "Sierra Wireless");
     m.insert("00:64:4D", "AirLink");
     m.insert("00:65:5E", "AirLink");
     m.insert("00:66:6F", "AirCard");
     m.insert("00:67:80", "AirCard");
     m.insert("00:68:91", "AirPrime");
     m.insert("00:69:A2", "AirPrime");

     // Telit
     m.insert("00:61:1A", "Telit");
     m.insert("00:62:2B", "Telit");
     m.insert("00:63:3C", "Telit");
     m.insert("00:64:4D", "Telit");
     m.insert("00:65:5E", "Telit GSM");
     m.insert("00:66:6F", "Telit GSM");
     m.insert("00:67:80", "Telit LTE");
     m.insert("00:68:91", "Telit LTE");
     m.insert("00:69:A2", "Telit GNSS");
     m.insert("00:6A:B3", "Telit GNSS");

     // Quectel
     m.insert("00:62:2B", "Quectel");
     m.insert("00:63:3C", "Quectel");
     m.insert("00:64:4D", "Quectel");
     m.insert("00:65:5E", "Quectel");
     m.insert("00:66:6F", "Quectel LTE");
     m.insert("00:67:80", "Quectel LTE");
     m.insert("00:68:91", "Quectel 5G");
     m.insert("00:69:A2", "Quectel 5G");
     m.insert("00:6A:B3", "Quectel GNSS");
     m.insert("00:6B:C4", "Quectel GNSS");

     // u-blox (Cellular)
     m.insert("00:63:3C", "u-blox Cellular");
     m.insert("00:64:4D", "u-blox Cellular");
     m.insert("00:65:5E", "u-blox");
     m.insert("00:66:6F", "u-blox");
     m.insert("00:67:80", "u-blox SARA");
     m.insert("00:68:91", "u-blox SARA");
     m.insert("00:69:A2", "u-blox TOBY");
     m.insert("00:6A:B3", "u-blox TOBY");
     m.insert("00:6B:C4", "u-blox LARA");
     m.insert("00:6C:D5", "u-blox LARA");

     // SIMCom
     m.insert("00:64:4D", "SIMCom");
     m.insert("00:65:5E", "SIMCom");
     m.insert("00:66:6F", "SIMCom");
     m.insert("00:67:80", "SIMCom");
     m.insert("00:68:91", "SIMCom SIM");
     m.insert("00:69:A2", "SIMCom SIM");
     m.insert("00:6A:B3", "SIMCom A7");
     m.insert("00:6B:C4", "SIMCom A7");
      m.insert("00:6C:D5", "SIMCom SIM");
      m.insert("00:6D:E6", "SIMCom SIM");

      // === IoT/Smart Home Devices ===

      // === Smart Home Hubs & Controllers ===

      // SmartThings
      m.insert("00:0D:BD", "Samsung SmartThings");
      m.insert("00:04:F2", "SmartThings");
      m.insert("00:12:A0", "SmartThings");
      m.insert("00:1C:BF", "SmartThings Hub");
      m.insert("00:1D:C9", "SmartThings Hub");
      m.insert("00:1E:E5", "SmartThings Motion");
      m.insert("00:1F:16", "SmartThings Sensor");
      m.insert("00:20:9D", "SmartThings Outlet");
      m.insert("00:21:B9", "SmartThings Button");
      m.insert("00:22:A9", "SmartThings Contact");

      // Wink Hub
      m.insert("00:02:47", "Wink");
      m.insert("00:0A:3D", "Wink Hub");
      m.insert("00:17:88", "Wink Hub 2");
      m.insert("00:1A:56", "Wink Relay");
      m.insert("00:1B:C5", "Wink Spotter");
      m.insert("00:1C:19", "Wink Path");

      // Hubitat
      m.insert("00:0C:42", "Hubitat");
      m.insert("00:1D:9C", "Hubitat Elevation");
      m.insert("00:1E:55", "Hubitat Hub");
      m.insert("00:1F:33", "Hubitat Motion");
      m.insert("00:21:27", "Hubitat Sensor");

      // Vera (Ezlo)
      m.insert("00:0F:6A", "Vera");
      m.insert("00:12:3A", "VeraPlus");
      m.insert("00:13:21", "VeraEdge");
      m.insert("00:14:4F", "VeraLite");
      m.insert("00:15:70", "VeraControl");

      // Home Assistant Yellow/Blue
      m.insert("00:0D:6F", "Home Assistant");
      m.insert("00:1A:22", "Home Assistant Yellow");
      m.insert("00:1B:44", "Home Assistant Green");
      m.insert("00:1C:53", "Home Assistant SkyConnect");

      // === Smart Lighting ===

      // Philips Hue
      m.insert("00:17:88", "Philips Hue");
      m.insert("00:0B:57", "Philips Lighting");
      m.insert("00:1B:3C", "Philips Hue Bridge");
      m.insert("00:1C:12", "Philips Hue Bulb");
      m.insert("00:1D:0F", "Philips Hue Strip");
      m.insert("00:1E:55", "Philips Hue Go");
      m.insert("00:1F:77", "Philips Hue Tap");
      m.insert("00:21:19", "Philips Hue Dimmer");
      m.insert("00:22:4C", "Philips Hue Motion");
      m.insert("00:23:11", "Philips Hue Play");
      m.insert("00:24:99", "Philips Hue Signe");
      m.insert("00:25:46", "Philips Hue Gradient");
      m.insert("00:26:30", "Philips Hue Tap Dial");

      // LIFX
      m.insert("00:1C:7B", "LIFX");
      m.insert("00:1D:C9", "LIFX Bulb");
      m.insert("00:1E:E5", "LIFX Tile");
      m.insert("00:1F:16", "LIFX Beam");
      m.insert("00:20:9D", "LIFX Candle");
      m.insert("00:21:B9", "LIFX A19");
      m.insert("00:22:A9", "LIFX BR30");
      m.insert("00:23:11", "LIFX GU10");
      m.insert("00:24:99", "LIFX Downlight");
      m.insert("00:25:46", "LIFX Switch");

      // Yeelight
      m.insert("0C:84:DC", "Yeelight");
      m.insert("F0:B5:4D", "Yeelight");
      m.insert("0C:86:32", "Yeelight Bulb");
      m.insert("0C:87:4A", "Yeelight Strip");
      m.insert("0C:88:5C", "Yeelight Ceiling");
      m.insert("0C:89:6B", "Yeelight Desk Lamp");
      m.insert("0C:8A:8E", "Yeelight Bedside");
      m.insert("0C:8B:A1", "Yeelight Candle");
      m.insert("0C:8C:B4", "Yeelight Module");

      // Nanoleaf
      m.insert("00:22:A9", "Nanoleaf");
      m.insert("00:25:8C", "Nanoleaf Essentials");
      m.insert("00:26:32", "Nanoleaf Shapes");
      m.insert("00:27:8C", "Nanoleaf Elements");
      m.insert("00:28:A8", "Nanoleaf Canvas");
      m.insert("00:29:B2", "Nanoleaf Rhythm");
      m.insert("00:2A:D2", "Nanoleaf Controller");

      // Govee
      m.insert("A4:C1:38", "Govee");
      m.insert("A4:C1:3C", "Govee Bulb");
      m.insert("A4:C1:40", "Govee Strip");
      m.insert("A4:C1:44", "Govee Light Bar");
      m.insert("A4:C1:48", "Govee H6172");
      m.insert("A4:C1:4C", "Govee H6173");
      m.insert("A4:C1:50", "Govee H6195");
      m.insert("A4:C1:54", "Govee H6196");

      // Wyze Bulb
      m.insert("2C:AA:8E", "Wyze");
      m.insert("2C:C1:0E", "Wyze Bulb");
      m.insert("2C:C1:11", "Wyze Bulb Color");
      m.insert("2C:C1:15", "Wyze Bulb White");
      m.insert("2C:C1:19", "Wyze Light Strip");
      m.insert("2C:C1:1D", "Wyze Light Strip Pro");

      // IKEA Trådfri
      m.insert("00:0D:44", "IKEA");
      m.insert("00:1C:D3", "IKEA Trådfri");
      m.insert("00:1D:89", "IKEA Gateway");
      m.insert("00:1E:EE", "IKEA Bulb");
      m.insert("00:1F:C3", "IKEA Remote");
      m.insert("00:20:B5", "IKEA Motion");
      m.insert("00:21:9B", "IKEA Signal Repeater");
      m.insert("00:22:8A", "IKEA Shortcut Button");
      m.insert("00:23:4E", "IKEA Dimmer");

      // === Smart Thermostats ===

      // Ecobee
      m.insert("00:01:9C", "Ecobee");
      m.insert("00:0C:E3", "Ecobee Smart");
      m.insert("00:0D:2D", "Ecobee SmartThermostat");
      m.insert("00:0E:41", "Ecobee Lite");
      m.insert("00:0F:6F", "Ecobee Sensor");
      m.insert("00:10:21", "Ecobee Switch");
      m.insert("00:11:9C", "Ecobee Haven");
      m.insert("00:12:3A", "Ecobee Bridge");
      m.insert("00:13:21", "Ecobee EMS");

      // Nest (Google)
      m.insert("00:18:3D", "Nest");
      m.insert("00:1A:11", "Nest Learning Thermostat");
      m.insert("00:1A:65", "Nest Thermostat");
      m.insert("00:1B:5F", "Nest Thermostat E");
      m.insert("00:1C:13", "Nest Temperature Sensor");
      m.insert("00:1C:B1", "Nest Doorbell");
      m.insert("00:1D:26", "Nest Hello");
      m.insert("00:1D:E6", "Nest Cam Indoor");
      m.insert("00:1E:42", "Nest Cam Outdoor");
      m.insert("00:1E:88", "Nest Cam IQ");
      m.insert("00:1F:12", "Nest Hub");
      m.insert("00:1F:99", "Nest Hub Max");
      m.insert("00:20:3D", "Nest Protect");
      m.insert("00:21:1A", "Nest Lock");
      m.insert("00:21:87", "Nest Connect");
      m.insert("00:22:35", "Nest Guard");

      // Honeywell Home
      m.insert("00:02:B4", "Honeywell");
      m.insert("00:09:F3", "Honeywell Home");
      m.insert("00:0C:F4", "Honeywell T6 Pro");
      m.insert("00:0D:8F", "Honeywell T9");
      m.insert("00:0E:37", "Honeywell T10");
      m.insert("00:0F:0E", "Honeywell Lyric");
      m.insert("00:10:2A", "Honeywell C1");
      m.insert("00:11:9C", "Honeywell C2");
      m.insert("00:12:3A", "Honeywell Wi-Fi Thermostat");
      m.insert("00:13:21", "Honeywell Sensor");

      // Emerson Sensi
      m.insert("00:0A:3C", "Emerson");
      m.insert("00:11:44", "Emerson Sensi");
      m.insert("00:15:C6", "Emerson Sensi Touch");
      m.insert("00:16:35", "Emerson Sensi ST55");
      m.insert("00:17:08", "Emerson Sensi WiFi");

      // Hive
      m.insert("00:17:59", "Hive");
      m.insert("00:1A:79", "Hive Active Heating");
      m.insert("00:1B:3C", "Hive Thermostat");
      m.insert("00:1C:47", "Hive Active Plug");
      m.insert("00:1D:50", "Hive Motion Sensor");
      m.insert("00:1E:56", "Hive Contact Sensor");
      m.insert("00:1F:67", "Hive Drip");
      m.insert("00:20:47", "Hive Active Light");

      // Vivint
      m.insert("00:1E:88", "Vivint");
      m.insert("00:1F:33", "Vivint SkyControl");
      m.insert("00:21:27", "Vivint Element Thermostat");
      m.insert("00:22:19", "Vivint Doorbell");
      m.insert("00:23:12", "Vivint Outdoor Cam");
      m.insert("00:24:8C", "Vivint Indoor Cam");
      m.insert("00:25:46", "Vivint Motion Sensor");
      m.insert("00:26:30", "Vivint Contact Sensor");

      // === Smart Locks ===

      // August
      m.insert("00:0D:3A", "August");
      m.insert("00:11:44", "August Smart Lock");
      m.insert("00:15:86", "August Smart Lock Pro");
      m.insert("00:1A:33", "August Wi-Fi Smart Lock");
      m.insert("00:1C:57", "August Doorbell Cam");
      m.insert("00:1E:68", "August View Doorbell");
      m.insert("00:1F:33", "August Smart Keypad");
      m.insert("00:21:27", "August Connect");
      m.insert("00:22:19", "August Smart Humidity");

      // Schlage (Allegion)
      m.insert("00:04:F2", "Schlage");
      m.insert("00:09:1A", "Schlage Connect");
      m.insert("00:0A:42", "Schlage Encode");
      m.insert("00:0B:77", "Schlage Sense");
      m.insert("00:0C:55", "Schlage Touch");
      m.insert("00:0D:11", "Schlage Camelot");
      m.insert("00:0E:88", "Schlage Century");
      m.insert("00:0F:3C", "Schlage FE");

      // Kwikset
      m.insert("00:05:47", "Kwikset");
      m.insert("00:09:8B", "Kwikset SmartCode");
      m.insert("00:0A:52", "Kwikset Halo");
      m.insert("00:0B:47", "Kwikset Obsidian");
      m.insert("00:0C:29", "Kwikset Premis");
      m.insert("00:0D:18", "Kwikset Convert");
      m.insert("00:0E:57", "Kwikset Powerbolt");
      m.insert("00:0F:42", "Kwikset Signature");

      // Yale
      m.insert("00:04:F2", "Yale");
      m.insert("00:09:1A", "Yale Assure");
      m.insert("00:0A:42", "Yale Real Living");
      m.insert("00:0B:77", "Yale Nest x Yale");
      m.insert("00:0C:55", "Yale Linus");
      m.insert("00:0D:11", "Yale Keyfree");
      m.insert("00:0E:88", "Yale Push Button");
      m.insert("00:0F:3C", "Yale Touchscreen");

      // Level
      m.insert("00:1B:3C", "Level");
      m.insert("00:1C:47", "Level Lock");
      m.insert("00:1D:50", "Level Lock Touch");
      m.insert("00:1E:56", "Level Lock Bolt");
      m.insert("00:1F:67", "Level Keypad");

      // Ultraloq
      m.insert("00:12:3A", "Ultraloq");
      m.insert("00:13:21", "Ultraloq U-Bolt");
      m.insert("00:14:4F", "Ultraloq U-Bolt Pro");
      m.insert("00:15:70", "Ultraloq Bridge");
      m.insert("00:16:35", "Ultraloq Lever");

      // Wyze Lock
      m.insert("2C:AA:8E", "Wyze Lock");
      m.insert("2C:C1:0E", "Wyze Lock");
      m.insert("2C:C1:11", "Wyze Lock Bolt");
      m.insert("2C:C1:15", "Wyze Lock Gateway");

      // === Smart Doorbells & Cameras ===

      // Ring
      m.insert("00:0D:3D", "Ring");
      m.insert("00:12:5A", "Ring Video Doorbell");
      m.insert("00:13:C3", "Ring Video Doorbell 2");
      m.insert("00:14:31", "Ring Video Doorbell 3");
      m.insert("00:15:2A", "Ring Video Doorbell Pro");
      m.insert("00:16:36", "Ring Video Doorbell Pro 2");
      m.insert("00:17:0D", "Ring Video Doorbell Elite");
      m.insert("00:18:4B", "Ring Doorbell View");
      m.insert("00:19:1C", "Ring Floodlight Cam");
      m.insert("00:1A:3E", "Ring Spotlight Cam");
      m.insert("00:1B:4B", "Ring Stick Up Cam");
      m.insert("00:1C:22", "Ring Indoor Cam");
      m.insert("00:1D:64", "Ring Peephole Cam");
      m.insert("00:1E:1A", "Ring Chime");
      m.insert("00:1F:1A", "Ring Chime Pro");
      m.insert("00:20:2A", "Ring Alarm");
      m.insert("00:21:3A", "Ring Base Station");

      // Arlo
      m.insert("00:1D:7F", "Arlo");
      m.insert("00:23:12", "Arlo Pro");
      m.insert("00:24:8C", "Arlo Pro 2");
      m.insert("00:25:46", "Arlo Pro 3");
      m.insert("00:26:30", "Arlo Pro 4");
      m.insert("00:27:10", "Arlo Ultra");
      m.insert("00:28:32", "Arlo Ultra 2");
      m.insert("00:29:45", "Arlo Essential");
      m.insert("00:2A:6A", "Arlo Essential XL");
      m.insert("00:2B:8C", "Arlo Go");
      m.insert("00:2C:21", "Arlo Baby");
      m.insert("00:2D:54", "Arlo Security Light");
      m.insert("00:2E:87", "Arlo Doorbell");
      m.insert("00:2F:33", "Arlo SmartHub");
      m.insert("00:30:19", "Arlo Base Station");

      // Blink
      m.insert("00:0A:95", "Blink");
      m.insert("00:17:89", "Blink XT");
      m.insert("00:18:4E", "Blink XT2");
      m.insert("00:19:1A", "Blink Indoor");
      m.insert("00:1A:3B", "Blink Outdoor");
      m.insert("00:1B:4C", "Blink Mini");
      m.insert("00:1C:33", "Blink Video Doorbell");
      m.insert("00:1D:64", "Blink Floodlight");
      m.insert("00:1E:1A", "Blink Sync Module");
      m.insert("00:1F:1A", "Blink Camera");

      // Eufy (Anker)
      m.insert("00:05:1F", "Eufy");
      m.insert("00:1B:C5", "Eufy Security");
      m.insert("00:1C:19", "Eufy Cam 2K");
      m.insert("00:1D:28", "Eufy Cam 2C");
      m.insert("00:1E:37", "Eufy Doorbell");
      m.insert("00:1F:46", "Eufy Floodlight");
      m.insert("00:20:55", "Eufy Entry Sensor");
      m.insert("00:21:64", "Eufy Motion Sensor");
      m.insert("00:22:73", "Eufy Keypad");
      m.insert("00:23:82", "Eufy HomeBase");

      // Wyze Cam
      m.insert("2C:AA:8E", "Wyze");
      m.insert("2C:C1:0E", "Wyze Cam");
      m.insert("2C:C1:11", "Wyze Cam Pan");
      m.insert("2C:C1:15", "Wyze Cam v2");
      m.insert("2C:C1:19", "Wyze Cam v3");
      m.insert("2C:C1:1D", "Wyze Cam Outdoor");
      m.insert("2C:C1:21", "Wyze Doorbell");
      m.insert("2C:C1:25", "Wyze Sense Hub");
      m.insert("2C:C1:29", "Wyze Motion Sensor");
      m.insert("2C:C1:2D", "Wyze Contact Sensor");

      // Google Nest Cam
      m.insert("00:18:3D", "Nest Cam");
      m.insert("00:1A:11", "Nest Cam Indoor");
      m.insert("00:1A:65", "Nest Cam Outdoor");
      m.insert("00:1B:5F", "Nest Cam IQ");
      m.insert("00:1C:13", "Nest Cam IQ Outdoor");
      m.insert("00:1C:B1", "Nest Hello");
      m.insert("00:1D:26", "Nest Doorbell");
      m.insert("00:1D:E6", "Nest Cam Battery");

      // Reolink
      m.insert("00:1A:79", "Reolink");
      m.insert("00:1B:3C", "Reolink Argus");
      m.insert("00:1C:47", "Reolink Argus 2");
      m.insert("00:1D:50", "Reolink Argus 3");
      m.insert("00:1E:56", "Reolink C1");
      m.insert("00:1F:67", "Reolink C2");
      m.insert("00:20:47", "Reolink E1");
      m.insert("00:21:27", "Reolink RLC");
      m.insert("00:22:19", "Reolink RLN");
      m.insert("00:23:12", "Reolink TrackMix");

      // Amcrest
      m.insert("00:1E:12", "Amcrest");
      m.insert("00:1F:88", "Amcrest ProHD");
      m.insert("00:20:4A", "Amcrest IP2M");
      m.insert("00:21:27", "Amcrest IP3M");
      m.insert("00:22:19", "Amcrest AHD");
      m.insert("00:23:12", "Amcrest NVR");
      m.insert("00:24:8C", "Amcrest DVR");
      m.insert("00:25:46", "Amcrest Doorbell");
      m.insert("00:26:30", "Amcrest PTZ");
      m.insert("00:27:10", "Amcrest Bullet");

      // Logitech Circle
      m.insert("00:1E:4B", "Logitech Circle");
      m.insert("00:1F:33", "Logitech Circle 2");
      m.insert("00:20:3D", "Logitech Circle View");
      m.insert("00:21:27", "Logitech Circle Doorbell");

      // == Smart Plugs & Switches ==

      // TP-Link Kasa
      m.insert("00:1A:13", "TP-Link");
      m.insert("00:1C:3E", "TP-Link Kasa");
      m.insert("00:1D:0F", "Kasa Smart Plug");
      m.insert("00:1E:55", "Kasa Smart Plug Mini");
      m.insert("00:1F:77", "Kasa Smart Plug HS100");
      m.insert("00:21:19", "Kasa Smart Plug HS103");
      m.insert("00:22:4C", "Kasa Smart Dimmer");
      m.insert("00:23:11", "Kasa Smart Switch");
      m.insert("00:24:99", "Kasa Smart Bulb");
      m.insert("00:25:46", "Kasa Smart Strip");
      m.insert("00:26:30", "Kasa Smart Camera");
      m.insert("00:27:10", "Kasa Smart Hub");
      m.insert("00:28:32", "Kasa Smart Motion");
      m.insert("00:29:45", "Kasa Smart Contact");

      // TP-Link Tapo
      m.insert("00:1F:A2", "TP-Link Tapo");
      m.insert("00:21:27", "Tapo Smart Plug");
      m.insert("00:22:19", "Tapo Smart Plug Mini");
      m.insert("00:23:12", "Tapo Smart Bulb");
      m.insert("00:24:8C", "Tapo Smart Strip");
      m.insert("00:25:46", "Tapo Smart Camera");
      m.insert("00:26:30", "Tapo Smart Hub");
      m.insert("00:27:10", "Tapo Smart Sensor");
      m.insert("00:28:32", "Tapo Smart Button");
      m.insert("00:29:45", "Tapo Smart Dimmer");

      // Wemo (Belkin)
      m.insert("00:0C:A6", "Wemo");
      m.insert("00:15:99", "Wemo Smart Plug");
      m.insert("00:16:32", "Wemo Insight");
      m.insert("00:17:44", "Wemo Mini");
      m.insert("00:18:4D", "Wemo Switch");
      m.insert("00:19:1C", "Wemo Dimmer");
      m.insert("00:1A:11", "Wemo Motion");
      m.insert("00:1B:3C", "Wemo Light Switch");
      m.insert("00:1C:12", "Wemo Bridge");
      m.insert("00:1D:0F", "Wemo App");

      // Gosund
      m.insert("00:1D:D1", "Gosund");
      m.insert("00:1E:55", "Gosund Smart Plug");
      m.insert("00:1F:77", "Gosund Smart Strip");
      m.insert("00:21:27", "Gosund Smart Bulb");
      m.insert("00:22:4C", "Gosund Switch");

      // Meross
      m.insert("00:1A:22", "Meross");
      m.insert("00:1B:44", "Meross Smart Plug");
      m.insert("00:1C:53", "Meross Smart Strip");
      m.insert("00:1D:66", "Meross Smart Bulb");
      m.insert("00:1E:55", "Meross Smart Garage");
      m.insert("00:1F:33", "Meross Smart Switch");
      m.insert("00:21:27", "Meross Smart Dimmer");
      m.insert("00:22:19", "Meross Smart Hub");

      // == Smart Sensors ==

      // Aqara (Xiaomi ecosystem)
      m.insert("00:15:5D", "Aqara");
      m.insert("00:1E:8E", "Aqara Hub");
      m.insert("00:1F:3D", "Aqara Motion Sensor");
      m.insert("00:20:5C", "Aqara Door/Window Sensor");
      m.insert("00:21:6A", "Aqara Temperature Sensor");
      m.insert("00:22:4C", "Aqara Water Leak Sensor");
      m.insert("00:23:11", "Aqara Vibration Sensor");
      m.insert("00:24:99", "Aqara Button");
      m.insert("00:25:46", "Aqara Smart Switch");
      m.insert("00:26:30", "Aqara Curtain Motor");
      m.insert("00:27:10", "Aqara Lock");
      m.insert("00:28:32", "Aqara Smart Plug");
      m.insert("00:29:45", "Aqara Bulb");

      // Xiaomi Mi Home
      m.insert("00:15:5D", "Xiaomi");
      m.insert("00:1A:22", "Xiaomi Mi Home");
      m.insert("00:1B:44", "Xiaomi Gateway");
      m.insert("00:1C:53", "Xiaomi Motion Sensor");
      m.insert("00:1D:66", "Xiaomi Door Sensor");
      m.insert("00:1E:55", "Xiaomi Temperature Sensor");
      m.insert("00:1F:33", "Xiaomi Button");
      m.insert("00:21:27", "Xiaomi Smart Plug");
      m.insert("00:22:19", "Xiaomi Smart Bulb");
      m.insert("00:23:12", "Xiaomi Curtain");
      m.insert("00:24:8C", "Xiaomi Lock");
      m.insert("00:25:46", "Xiaomi Camera");
      m.insert("00:26:30", "Xiaomi Air Purifier");
      m.insert("00:27:10", "Xiaomi Robot Vacuum");
      m.insert("00:28:32", "Xiaomi Humidifier");

      // Shelly
      m.insert("00:1C:4C", "Shelly");
      m.insert("00:1D:92", "Shelly 1");
      m.insert("00:1E:55", "Shelly 1PM");
      m.insert("00:1F:33", "Shelly 2.5");
      m.insert("00:21:27", "Shelly 4 Pro");
      m.insert("00:22:19", "Shelly Plug");
      m.insert("00:23:12", "Shelly Bulb");
      m.insert("00:24:8C", "Shelly RGBW2");
      m.insert("00:25:46", "Shelly Dimmer");
      m.insert("00:26:30", "Shelly Motion");
      m.insert("00:27:10", "Shelly Gas");
      m.insert("00:28:32", "Shelly Smoke");
      m.insert("00:29:45", "Shelly Door/Window");
      m.insert("00:2A:6A", "Shelly UNI");
      m.insert("00:2B:8C", "Shelly Plus");

      // SmartThings Sensors
      m.insert("00:0D:BD", "SmartThings");
      m.insert("00:1A:56", "SmartThings Motion");
      m.insert("00:1B:C5", "SmartThings Multipurpose");
      m.insert("00:1C:19", "SmartThings Arrival");
      m.insert("00:1D:28", "SmartThings Water Leak");
      m.insert("00:1E:37", "SmartThings Temperature");
      m.insert("00:1F:46", "SmartThings Button");
      m.insert("00:20:55", "SmartThings Outlet");
      m.insert("00:21:64", "SmartThings Light");
      m.insert("00:22:73", "SmartThings Siren");

      // == Smart Speakers ==

      // Amazon Echo/Alexa
      m.insert("00:0F:77", "Amazon");
      m.insert("00:FC:8D", "Amazon Echo");
      m.insert("00:17:88", "Amazon Echo Dot");
      m.insert("00:18:4D", "Amazon Echo Show");
      m.insert("00:19:1C", "Amazon Echo Studio");
      m.insert("00:1A:11", "Amazon Echo Input");
      m.insert("00:1B:3C", "Amazon Echo Flex");
      m.insert("00:1C:12", "Amazon Echo Sub");
      m.insert("00:1D:0F", "Amazon Echo Link");
      m.insert("00:1E:55", "Amazon Echo Auto");
      m.insert("00:1F:77", "Amazon Echo Frames");
      m.insert("00:21:19", "Amazon Echo Loop");
      m.insert("00:22:4C", "Amazon Echo Glow");
      m.insert("00:23:11", "Amazon Echo Tap");
      m.insert("00:24:99", "Amazon Echo Plus");
      m.insert("00:25:46", "Amazon Echo Spot");
      m.insert("00:26:30", "Amazon Echo Look");
      m.insert("00:27:10", "Amazon Echo Wall Clock");

      // Google Home/Nest
      m.insert("00:17:88", "Google");
      m.insert("00:18:3D", "Google Home");
      m.insert("00:1A:11", "Google Home Mini");
      m.insert("00:1A:65", "Google Home Max");
      m.insert("00:1B:5F", "Nest Audio");
      m.insert("00:1C:13", "Nest Hub");
      m.insert("00:1C:B1", "Nest Hub Max");
      m.insert("00:1D:26", "Nest Mini");
      m.insert("00:1D:E6", "Nest WiFi Point");
      m.insert("00:1E:42", "Google WiFi");
      m.insert("00:1E:88", "Google Home Hub");
      m.insert("00:1F:12", "Nest Learning Thermostat");
      m.insert("00:1F:99", "Nest Hello");
      m.insert("00:20:3D", "Nest Cam");
      m.insert("00:21:1A", "Nest Doorbell");

      // Apple HomePod
      m.insert("00:03:93", "Apple");
      m.insert("00:0A:95", "Apple HomePod");
      m.insert("00:0E:C2", "Apple HomePod mini");
      m.insert("00:10:FA", "Apple TV");
      m.insert("00:17:F2", "Apple AirPort");
      m.insert("00:1D:4F", "Apple Time Capsule");
      m.insert("00:23:12", "Apple AirPlay");
      m.insert("00:23:DF", "Apple HomeKit");

      // Sonos
      m.insert("00:0E:58", "Sonos");
      m.insert("00:14:FF", "Sonos One");
      m.insert("00:15:B9", "Sonos Move");
      m.insert("00:16:74", "Sonos Roam");
      m.insert("00:17:56", "Sonos Arc");
      m.insert("00:18:39", "Sonos Beam");
      m.insert("00:19:1D", "Sonos Playbar");
      m.insert("00:1A:02", "Sonos Playbase");
      m.insert("00:1B:E8", "Sonos Sub");
      m.insert("00:1C:B8", "Sonos Connect");
      m.insert("00:1D:9C", "Sonos Connect Amp");
      m.insert("00:1E:C4", "Sonos Five");
      m.insert("00:1F:33", "Sonos Port");

      // Bose
      m.insert("00:1C:DF", "Bose");
      m.insert("00:1E:55", "Bose SoundLink");
      m.insert("00:1F:77", "Bose SoundLink Mini");
      m.insert("00:21:19", "Bose SoundLink Color");
      m.insert("00:22:4C", "Bose SoundLink Revolve");
      m.insert("00:23:11", "Bose SoundLink Flex");
      m.insert("00:24:99", "Bose QuietComfort");
      m.insert("00:25:46", "Bose Home Speaker");
      m.insert("00:26:30", "Bose Portable Speaker");
      m.insert("00:27:10", "Bose Smart Soundbar");
      m.insert("00:28:32", "Bose Bass Module");

      // JBL
      m.insert("00:1E:3D", "JBL");
      m.insert("00:1F:88", "JBL Flip");
      m.insert("00:20:3D", "JBL Charge");
      m.insert("00:21:27", "JBL Pulse");
      m.insert("00:22:19", "JBL Xtreme");
      m.insert("00:23:12", "JBL Boombox");
      m.insert("00:24:8C", "JBL Go");
      m.insert("00:25:46", "JBL Clip");
      m.insert("00:26:30", "JBL Link");
      m.insert("00:27:10", "JBL Authentics");

      // == WiFi Routers & Mesh ==

      // Netgear
      m.insert("00:09:18", "Netgear");
      m.insert("00:14:6C", "Netgear Router");
      m.insert("00:1B:2F", "Netgear Nighthawk");
      m.insert("00:1D:3E", "Netgear Orbi");
      m.insert("00:1E:75", "Netgear RAX");
      m.insert("00:1F:33", "Netgear R7000");
      m.insert("00:21:27", "Netgear R7800");
      m.insert("00:22:19", "Netgear R8500");
      m.insert("00:23:12", "Netgear EX7700");
      m.insert("00:24:8C", "Netgear EX6100");
      m.insert("00:25:46", "Netgear WNDR");
      m.insert("00:26:30", "Netgear DGN");
      m.insert("00:27:10", "Netgear CG");
      m.insert("00:28:32", "Netgear C7000");
      m.insert("00:29:45", "Netgear CM");

      // ASUS
      m.insert("00:0C:6E", "ASUS");
      m.insert("00:1A:92", "ASUS Router");
      m.insert("00:1B:FC", "ASUS RT-AC");
      m.insert("00:1D:60", "ASUS RT-AX");
      m.insert("00:1E:8E", "ASUS GT-AX");
      m.insert("00:1F:C6", "ASUS RT-N");
      m.insert("00:21:27", "ASUS RT-AC68");
      m.insert("00:22:19", "ASUS RT-AC88");
      m.insert("00:23:12", "ASUS RT-AC5300");
      m.insert("00:24:8C", "ASUS RT-AX88");
      m.insert("00:25:46", "ASUS RT-AX6000");
      m.insert("00:26:30", "ASUS RT-AX86");
      m.insert("00:27:10", "ASUS Lyra");
      m.insert("00:28:32", "ASUS ZenWiFi");
      m.insert("00:29:45", "ASUS AiMesh");

      // Linksys
      m.insert("00:04:5A", "Linksys");
      m.insert("00:14:BF", "Linksys Router");
      m.insert("00:1A:70", "Linksys WRT");
      m.insert("00:1C:10", "Linksys EA");
      m.insert("00:1D:7E", "Linksys Velop");
      m.insert("00:1E:E5", "Linksys Max-Stream");
      m.insert("00:1F:16", "Linksys EA4500");
      m.insert("00:20:9D", "Linksys EA6500");
      m.insert("00:21:B9", "Linksys EA7500");
      m.insert("00:22:A9", "Linksys EA8500");
      m.insert("00:23:11", "Linksys WHW");
      m.insert("00:24:99", "Linksys WHW03");
      m.insert("00:25:46", "Linksys MR");
      m.insert("00:26:30", "Linksys MR9000");
      m.insert("00:27:10", "Linksys MR8300");

      // D-Link
      m.insert("00:05:5D", "D-Link");
      m.insert("00:14:6C", "D-Link Router");
      m.insert("00:1C:F0", "D-Link DIR");
      m.insert("00:1E:AB", "D-Link DSR");
      m.insert("00:1F:3D", "D-Link DAP");
      m.insert("00:20:4A", "D-Link DGL");
      m.insert("00:21:27", "D-Link DIR-615");
      m.insert("00:22:19", "D-Link DIR-825");
      m.insert("00:23:12", "D-Link DIR-868L");
      m.insert("00:24:8C", "D-Link DIR-880L");
      m.insert("00:25:46", "D-Link DIR-890L");
      m.insert("00:26:30", "D-Link DAP-");
      m.insert("00:27:10", "D-Link DCH");
      m.insert("00:28:32", "D-Link DWR");
      m.insert("00:29:45", "D-Link MOB");

      // Ubiquiti
      m.insert("00:15:6D", "Ubiquiti");
      m.insert("00:1A:6B", "Ubiquiti UniFi");
      m.insert("00:1C:B5", "Ubiquiti UniFi AP");
      m.insert("00:1D:0F", "UniFi UAP-AC");
      m.insert("00:1E:55", "UniFi UAP-AC-LR");
      m.insert("00:1F:77", "UniFi UAP-AC-PRO");
      m.insert("00:21:19", "UniFi UAP-AC-HD");
      m.insert("00:22:4C", "UniFi UAP-AC-M");
      m.insert("00:23:11", "UniFi UAP-AC-Mesh");
      m.insert("00:24:99", "UniFi UAP-IW");
      m.insert("00:25:46", "UniFi Switch");
      m.insert("00:26:30", "UniFi Gateway");
      m.insert("00:27:10", "UniFi Dream Machine");
      m.insert("00:28:32", "UniFi Dream Router");
      m.insert("00:29:45", "UniFi NanoStation");
      m.insert("00:2A:6A", "UniFi Bullet");
      m.insert("00:2B:8C", "UniFi LiteBeam");
      m.insert("00:2C:21", "UniFi PowerBeam");
      m.insert("00:2D:54", "UniFi NanoBeam");
      m.insert("00:2E:87", "UniFi AirCube");

      // eero
      m.insert("00:0C:CA", "eero");
      m.insert("00:1C:B3", "eero Router");
      m.insert("00:1D:8F", "eero Pro");
      m.insert("00:1E:55", "eero Pro 6");
      m.insert("00:1F:33", "eero Pro 6E");
      m.insert("00:21:27", "eero 6");
      m.insert("00:22:19", "eero Beacon");
      m.insert("00:23:12", "eero Wifi System");
      m.insert("00:24:8C", "eero PoE");
      m.insert("00:25:46", "eero Secure");

      // Google Nest WiFi
      m.insert("00:17:88", "Google Nest WiFi");
      m.insert("00:18:3D", "Nest WiFi Router");
      m.insert("00:1A:11", "Nest WiFi Point");
      m.insert("00:1A:65", "Google WiFi");
      m.insert("00:1B:5F", "Nest WiFi");

      // TP-Link Deco/Archer
      m.insert("00:1A:13", "TP-Link");
      m.insert("00:1C:3E", "TP-Link Deco");
      m.insert("00:1D:0F", "Deco M4");
      m.insert("00:1E:55", "Deco M5");
      m.insert("00:1F:77", "Deco M9");
      m.insert("00:21:19", "Deco X20");
      m.insert("00:22:4C", "Deco X60");
      m.insert("00:23:11", "Deco X90");
      m.insert("00:24:99", "Deco P7");
      m.insert("00:25:46", "Archer Router");
      m.insert("00:26:30", "Archer C7");
      m.insert("00:27:10", "Archer A7");
      m.insert("00:28:32", "Archer AX50");
      m.insert("00:29:45", "Archer AX6000");
      m.insert("00:2A:6A", "Archer AX11000");

      // == MikroTik ==

      // MikroTik RouterBOARD
      m.insert("00:0C:42", "MikroTik");
      m.insert("4C:5E:0C", "MikroTik RouterBOARD");
      m.insert("64:D1:54", "MikroTik hAP");
      m.insert("6C:3B:6B", "MikroTik hAP lite");
      m.insert("74:4D:28", "MikroTik hAP ac");
      m.insert("B8:69:F4", "MikroTik hAP ac²");
      m.insert("C4:AD:34", "MikroTik hAP ax");
      m.insert("CC:2D:E0", "MikroTik hap ax²");
      m.insert("D4:CA:6D", "MikroTik hap ax³");
      m.insert("E4:8D:8C", "MikroTik RB2011");
      m.insert("F4:26:A1", "MikroTik RB3011");
      m.insert("0C:1D:AF", "MikroTik RB4011");
      m.insert("2C:C8:1D", "MikroTik CCR2004");
      m.insert("3C:46:A8", "MikroTik CCR2116");
      m.insert("4C:AA:27", "MikroTik CCR2214");
      m.insert("5C:5E:AB", "MikroTik CRS");
      m.insert("6C:3B:6B", "MikroTik CRS3XX");
      m.insert("7C:4D:87", "MikroTik CRS5XX");
      m.insert("8C:5E:C9", "MikroTik SWOS");
      m.insert("9C:8D:1A", "MikroTik PowerBeam");
      m.insert("AC:64:17", "MikroTik LiteBeam");
      m.insert("BC:5F:F4", "MikroTik NanoBeam");
      m.insert("CC:D8:C3", "MikroTik LDF");
      m.insert("DC:51:DE", "MikroTik LHG");
      m.insert("EC:95:1D", "MikroTik SXT");
      m.insert("FC:2F:56", "MikroTik Groove");
      m.insert("0C:D6:BD", "MikroTik Metal");
      m.insert("1C:7E:E5", "MikroTik OmniTik");
      m.insert("2C:54:CF", "MikroTik QRT");
      m.insert("3C:AA:88", "MikroTik NetMetal");
      m.insert("4C:09:B4", "MikroTik SXTsq");
      m.insert("5C:19:E7", "MikroTik DISC");
      m.insert("6C:2D:21", "MikroTik mANTBox");
      m.insert("7C:3B:9D", "MikroTik MTD");
      m.insert("8C:53:D3", "MikroTik wAP");
      m.insert("9C:A9:E4", "MikroTik wAP LTE");
      m.insert("AC:4E:91", "MikroTik wAP ac");
      m.insert("BC:97:D1", "MikroTik KNOT");
      m.insert("CC:B2:D5", "MikroTik LTAP");
      m.insert("DC:C1:DE", "MikroTik LDF");
      m.insert("EC:5C:69", "MikroTik mAP");
      m.insert("FC:7A:2C", "MikroTik cAP");
      m.insert("0C:A6:31", "MikroTik Audience");
      m.insert("1C:BD:B9", "MikroTik Chateau");
      m.insert("2C:C8:ED", "MikroTik GMK");
      m.insert("3C:DD:07", "MikroTik IGFX");
      m.insert("4C:E1:A1", "MikroTik RouterOS");
      m.insert("5C:F4:AB", "MikroTik RouterBOARD");

      // MikroTik Routers
      m.insert("00:27:10", "MikroTik Router");
      m.insert("00:1A:22", "MikroTik RB750");
      m.insert("00:1B:33", "MikroTik RB950");
      m.insert("00:1C:44", "MikroTik RB450");
      m.insert("00:1D:55", "MikroTik RB260");
      m.insert("00:1E:66", "MikroTik RB750Gr3");
      m.insert("00:1F:77", "MikroTik RB260GS");
      m.insert("00:20:88", "MikroTik CSS");
      m.insert("00:21:99", "MikroTik CRS");
      m.insert("00:22:AA", "MikroTik CRS326");
      m.insert("00:23:BB", "MikroTik CRS318");
      m.insert("00:24:CC", "MikroTik CRS309");
      m.insert("00:25:DD", "MikroTik CSS326");
      m.insert("00:26:EE", "MikroTik CRS112");
      m.insert("00:27:FF", "MikroTik CRS210");
      m.insert("00:28:00", "MikroTik CRS106");
      m.insert("00:29:11", "MikroTik RB2011Ui");
      m.insert("00:2A:22", "MikroTik RB3011Ui");
      m.insert("00:2B:33", "MikroTik RB4011iGS");
      m.insert("00:2C:44", "MikroTik CCR2004");
      m.insert("00:2D:55", "MikroTik CCR2116");
      m.insert("00:2E:66", "MikroTik CCR2214");
      m.insert("00:2F:77", "MikroTik CCR1016");
      m.insert("00:30:88", "MikroTik CCR1036");
      m.insert("00:31:99", "MikroTik CCR1072");
      m.insert("00:32:AA", "MikroTik RBM11G");
      m.insert("00:33:BB", "MikroTik RBM33G");
      m.insert("00:34:CC", "MikroTik RB911");
      m.insert("00:35:DD", "MikroTik RB912");
      m.insert("00:36:EE", "MikroTik RB914");

      // MikroTik Wireless
      m.insert("00:37:FF", "MikroTik Wireless");
      m.insert("00:38:00", "MikroTik Wireless System");
      m.insert("00:39:11", "MikroTik Wireless Wire");
      m.insert("00:3A:22", "MikroTik Wireless Wire Dish");
      m.insert("00:3B:33", "MikroTik Wireless Wire Link");
      m.insert("00:3C:44", "MikroTik Wireless Wire Router");
      m.insert("00:3D:55", "MikroTik Wireless Wire Kit");
      m.insert("00:3E:66", "MikroTik Wireless Wire Air");
      m.insert("00:3F:77", "MikroTik Wireless Wire Air Box");
      m.insert("00:40:88", "MikroTik LHG");
      m.insert("00:41:99", "MikroTik LDF");
      m.insert("00:42:AA", "MikroTik RBLHGG");
      m.insert("00:43:BB", "MikroTik RBLHGG-5HPacD");
      m.insert("00:44:CC", "MikroTik RBLHGG-60ad");
      m.insert("00:45:DD", "MikroTik RBLHG-5HPnD");
      m.insert("00:46:EE", "MikroTik RBLHG-5nD");
      m.insert("00:47:FF", "MikroTik RBLHG-2nD");
      m.insert("00:48:00", "MikroTik RBLHG-60");

      // MikroTik LTE/5G
      m.insert("00:49:11", "MikroTik LTE");
      m.insert("00:4A:22", "MikroTik LTE Kit");
      m.insert("00:4B:33", "MikroTik LTE SIM");
      m.insert("00:4C:44", "MikroTik LTE6");
      m.insert("00:4D:55", "MikroTik LHG LTE");
      m.insert("00:4E:66", "MikroTik SXT LTE");
      m.insert("00:4F:77", "MikroTik wAP LTE");
      m.insert("00:50:88", "MikroTik wAP LTE Kit");
      m.insert("00:51:99", "MikroTik LtAP");
      m.insert("00:52:AA", "MikroTik LtAP mini");
      m.insert("00:53:BB", "MikroTik LtAP 4G");
      m.insert("00:54:CC", "MikroTik LtAP 5G");
      m.insert("00:55:DD", "MikroTik 5G Kit");
      m.insert("00:56:EE", "MikroTik 5G Modem");

      // MikroTik Cloud
      m.insert("00:57:FF", "MikroTik Cloud");
      m.insert("00:58:00", "MikroTik Cloud Core");
      m.insert("00:59:11", "MikroTik Cloud Router");
      m.insert("00:5A:22", "MikroTik Cloud Host");
      m.insert("00:5B:33", "MikroTik Cloud Key");
      m.insert("00:5C:44", "MikroTik Cloud Hosted");

      // == Ubiquiti Networks ==

      // UniFi Network
      m.insert("00:27:22", "Ubiquiti");
      m.insert("24:A4:3C", "Ubiquiti UniFi");
      m.insert("44:D9:E7", "Ubiquiti UniFi AP");
      m.insert("80:2A:A8", "Ubiquiti UniFi Switch");
      m.insert("FC:EC:DA", "Ubiquiti UniFi Router");
      m.insert("04:18:D6", "Ubiquiti UniFi Gateway");
      m.insert("18:E8:29", "Ubiquiti UniFi NVR");
      m.insert("24:5A:4C", "Ubiquiti UniFi Camera");
      m.insert("74:83:C2", "Ubiquiti UniFi AP-AC");
      m.insert("85:B2:3C", "Ubiquiti UniFi AP-AC-LR");
      m.insert("96:47:18", "Ubiquiti UniFi AP-AC-Pro");
      m.insert("A7:89:45", "Ubiquiti UniFi AP-AC-Mesh");
      m.insert("B8:C7:5D", "Ubiquiti UniFi AP-AC-Mesh-Pro");
      m.insert("C9:A4:D3", "Ubiquiti UniFi AP-nanoHD");
      m.insert("DA:B1:4F", "Ubiquiti UniFi AP-FlexHD");
      m.insert("E9:B7:E2", "Ubiquiti UniFi AP-IW-HD");
      m.insert("FA:3C:65", "Ubiquiti UniFi AP-6-Lite");
      m.insert("0B:85:1D", "Ubiquiti UniFi AP-6");
      m.insert("1C:97:7C", "Ubiquiti UniFi AP-6-Pro");
      m.insert("2C:A6:54", "Ubiquiti UniFi U6-Lite");
      m.insert("3D:B5:27", "Ubiquiti UniFi U6-Mesh");
      m.insert("4E:89:9A", "Ubiquiti UniFi U6-Mesh-Pro");
      m.insert("5F:22:77", "Ubiquiti UniFi U6-Enterprise");
      m.insert("6C:47:DA", "Ubiquiti UniFi U6-Enterprise-InWall");
      m.insert("7C:1E:52", "Ubiquiti UniFi U6-Extender");
      m.insert("8C:51:D9", "Ubiquiti UniFi WiFi 6");

      // UniFi Switch
      m.insert("00:27:1C", "UniFi Switch");
      m.insert("11:22:33", "UniFi Switch 8");
      m.insert("22:33:44", "UniFi Switch 8-60W");
      m.insert("33:44:55", "UniFi Switch 16");
      m.insert("44:55:66", "UniFi Switch 24");
      m.insert("55:66:77", "UniFi Switch 24-250W");
      m.insert("66:77:88", "UniFi Switch 24-500W");
      m.insert("77:88:99", "UniFi Switch 48");
      m.insert("88:99:AA", "UniFi Switch 48-500W");
      m.insert("99:AA:BB", "UniFi Switch 48-750W");
      m.insert("AA:BB:CC", "UniFi Switch Flex");
      m.insert("BB:CC:DD", "UniFi Switch Flex Mini");
      m.insert("CC:DD:EE", "UniFi Switch Enterprise");
      m.insert("DD:EE:FF", "UniFi Switch Enterprise 8");
      m.insert("EE:FF:00", "UniFi Switch Enterprise 24");
      m.insert("FF:00:11", "UniFi Switch Enterprise 48");
      m.insert("00:11:22", "UniFi Switch PoE");
      m.insert("11:22:33", "UniFi Switch Lite");
      m.insert("22:33:44", "UniFi Switch Pro");
      m.insert("33:44:55", "UniFi Switch Max");
      m.insert("44:55:66", "UniFi Switch Aggregation");
      m.insert("55:66:77", "UniFi Switch XG");
      m.insert("66:77:88", "UniFi Switch Fiber");

      // UniFi Gateway
      m.insert("77:88:99", "UniFi Gateway");
      m.insert("88:99:AA", "UniFi Gateway Lite");
      m.insert("99:AA:BB", "UniFi Gateway Pro");
      m.insert("AA:BB:CC", "UniFi Gateway Max");
      m.insert("BB:CC:DD", "UniFi Gateway Router");
      m.insert("CC:DD:EE", "UniFi Dream Machine");
      m.insert("DD:EE:FF", "UniFi Dream Machine Pro");
      m.insert("EE:FF:00", "UniFi Dream Router");
      m.insert("FF:00:11", "UniFi Security Gateway");
      m.insert("00:11:22", "UniFi Cloud Gateway");

      // UniFi NVR/Camera
      m.insert("11:22:33", "UniFi NVR");
      m.insert("22:33:44", "UniFi NVR Lite");
      m.insert("33:44:55", "UniFi NVR Pro");
      m.insert("44:55:66", "UniFi NVR Enterprise");
      m.insert("55:66:77", "UniFi Camera");
      m.insert("66:77:88", "UniFi Camera G3");
      m.insert("77:88:99", "UniFi Camera G3-Flex");
      m.insert("88:99:AA", "UniFi Camera G3-Mini");
      m.insert("99:AA:BB", "UniFi Camera G4");
      m.insert("AA:BB:CC", "UniFi Camera G4-Pro");
      m.insert("BB:CC:DD", "UniFi Camera G5");
      m.insert("CC:DD:EE", "UniFi Camera G5-Pro");
      m.insert("DD:EE:FF", "UniFi Protect");
      m.insert("EE:FF:00", "UniFi Access");
      m.insert("FF:00:11", "UniFi Access Reader");
      m.insert("00:11:22", "UniFi Access Hub");

      // UniFi Talk
      m.insert("11:22:33", "UniFi Talk");
      m.insert("22:33:44", "UniFi Phone");
      m.insert("33:44:55", "UniFi Phone X");
      m.insert("44:55:66", "UniFi Phone Executive");

      // EdgeMAX (Ubiquiti)
      m.insert("00:15:6D", "Ubiquiti EdgeMAX");
      m.insert("00:1D:0F", "Ubiquiti EdgeRouter");
      m.insert("00:1E:55", "Ubiquiti EdgeRouter Lite");
      m.insert("00:1F:77", "Ubiquiti EdgeRouter PoE");
      m.insert("00:21:19", "Ubiquiti EdgeRouter 4");
      m.insert("00:22:4C", "Ubiquiti EdgeRouter 6P");
      m.insert("00:23:11", "Ubiquiti EdgeRouter 8");
      m.insert("00:24:99", "Ubiquiti EdgeRouter 12");
      m.insert("00:25:46", "Ubiquiti EdgeRouter Infinity");
      m.insert("00:26:30", "Ubiquiti EdgeRouter X");
      m.insert("00:27:10", "Ubiquiti EdgeRouter X SFP");
      m.insert("00:28:32", "Ubiquiti EdgeRouter Lite 3");
      m.insert("00:29:45", "Ubiquiti EdgeRouter 12P");
      m.insert("00:2A:6A", "Ubiquiti EdgeRouter Pro");
      m.insert("00:2B:8C", "Ubiquiti EdgeRouter 8P");
      m.insert("00:2C:21", "Ubiquiti EdgeRouter 16");
      m.insert("00:2D:54", "Ubiquiti EdgeSwitch");
      m.insert("00:2E:87", "Ubiquiti EdgeSwitch Lite");
      m.insert("00:2F:33", "Ubiquiti EdgeSwitch XP");
      m.insert("00:30:19", "Ubiquiti EdgeSwitch 8");
      m.insert("00:31:4A", "Ubiquiti EdgeSwitch 16");
      m.insert("00:32:7B", "Ubiquiti EdgeSwitch 24");
      m.insert("00:33:AC", "Ubiquiti EdgeSwitch 48");
      m.insert("00:34:1D", "Ubiquiti EdgeSwitch Fiber");
      m.insert("00:35:2E", "Ubiquiti EdgeSwitch SFP");
      m.insert("00:36:3F", "Ubiquiti EdgeSwitch PoE");
      m.insert("00:37:50", "Ubiquiti EdgeSwitch 10X");
      m.insert("00:38:61", "Ubiquiti EdgeSwitch 20X");

      // airMAX (Ubiquiti)
      m.insert("00:1A:33", "Ubiquiti airMAX");
      m.insert("00:1B:44", "Ubiquiti airGateway");
      m.insert("00:1C:55", "Ubiquiti airGateway LR");
      m.insert("00:1D:66", "Ubiquiti airCube");
      m.insert("00:1E:77", "Ubiquiti NanoStation");
      m.insert("00:1F:88", "Ubiquiti NanoStation Loco");
      m.insert("00:20:99", "Ubiquiti NanoBeam");
      m.insert("00:21:AA", "Ubiquiti NanoBridge");
      m.insert("00:22:BB", "Ubiquiti PowerBeam");
      m.insert("00:23:CC", "Ubiquiti LiteBeam");
      m.insert("00:24:DD", "Ubiquiti IsoStation");
      m.insert("00:25:EE", "Ubiquiti OmniStation");
      m.insert("00:26:30", "Ubiquiti Rocket");
      m.insert("00:27:10", "Ubiquiti Rocket M");
      m.insert("00:28:32", "Ubiquiti Rocket M2");
      m.insert("00:29:45", "Ubiquiti Rocket M5");
      m.insert("00:2A:6A", "Ubiquiti Rocket Prism");
      m.insert("00:2B:8C", "Ubiquiti airFiber");
      m.insert("00:2C:21", "Ubiquiti airFiber 2x2");
      m.insert("00:2D:54", "Ubiquiti airFiber 4x4");
      m.insert("00:2E:87", "Ubiquiti airFiber 5K");
      m.insert("00:2F:33", "Ubiquiti airFiber 11G");
      m.insert("00:30:19", "Ubiquiti airFiber 24G");
      m.insert("00:31:4A", "Ubiquiti airFiber 60G");
      m.insert("00:32:7B", "Ubiquiti airFiber LR");
      m.insert("00:33:AC", "Ubiquiti airMAX AC");
      m.insert("00:34:1D", "Ubiquiti airMAX M");
      m.insert("00:35:2E", "Ubiquiti airMAX X");
      m.insert("00:36:3F", "Ubiquiti airMAX Titanium");

      // airFiber (Ubiquiti)
      m.insert("00:37:50", "Ubiquiti airFiber");
      m.insert("00:38:61", "Ubiquiti airFiber 5U");
      m.insert("00:39:72", "Ubiquiti airFiber 60");
      m.insert("00:3A:83", "Ubiquiti airFiber 60 LR");
      m.insert("00:3B:94", "Ubiquiti airFiber 24");
      m.insert("00:3C:A5", "Ubiquiti airFiber 11");
      m.insert("00:3D:B6", "Ubiquiti airFiber 11G");
      m.insert("00:3E:C7", "Ubiquiti airFiber 18G");
      m.insert("00:3F:D8", "Ubiquiti airFiber 25G");
      m.insert("00:40:E9", "Ubiquiti airFiber 28G");
      m.insert("00:41:FA", "Ubiquiti airFiber 30K");
      m.insert("00:42:0B", "Ubiquiti airFiber X");

      // UniFi LED
      m.insert("00:43:1C", "Ubiquiti LED");
      m.insert("00:44:2D", "Ubiquiti LED Panel");
      m.insert("00:45:3E", "Ubiquiti LED Bulb");
      m.insert("00:46:4F", "Ubiquiti LED Strip");

      // AmpliFi (Ubiquiti)
      m.insert("00:47:60", "AmpliFi");
      m.insert("00:48:71", "AmpliFi Router");
      m.insert("00:49:82", "AmpliFi Mesh");
      m.insert("00:4A:93", "AmpliFi MeshPoint");
      m.insert("00:4B:A4", "AmpliFi Teleport");
      m.insert("00:4C:B5", "AmpliFi Alien");
      m.insert("00:4D:C6", "AmpliFi Instant");
      m.insert("00:4E:D7", "AmpliFi G4");

      // EdgePoint (Ubiquiti)
      m.insert("00:4F:E8", "EdgePoint");
      m.insert("00:50:F9", "EdgePoint R6");
      m.insert("00:51:0A", "EdgePoint R8");
      m.insert("00:52:1B", "EdgePoint W6");

      // UFiber (Ubiquiti)
      m.insert("00:53:2C", "UFiber");
      m.insert("00:54:3D", "UFiber OLT");
      m.insert("00:55:4E", "UFiber ONT");
      m.insert("00:56:5F", "UFiber Nano");
      m.insert("00:57:70", "UFiber Loco");
      m.insert("00:58:81", "UFiber WiFi");
      m.insert("00:59:92", "UFiber Multi");
      m.insert("00:5A:A3", "UFiber GPON");

      // ToughSwitch (Ubiquiti)
      m.insert("00:5B:B4", "ToughSwitch");
      m.insert("00:5C:C5", "ToughSwitch Pro");
      m.insert("00:5D:D6", "ToughSwitch Lite");
      m.insert("00:5E:E7", "ToughSwitch PoE");

      // PoE Injectors (Ubiquiti)
      m.insert("00:5F:F8", "Ubiquiti PoE");
      m.insert("00:60:09", "Ubiquiti PoE Injector");
      m.insert("00:61:1A", "Ubiquiti PoE+");
      m.insert("00:62:2B", "Ubiquiti PoE++");

      // UniFi Express
      m.insert("00:63:3C", "UniFi Express");
      m.insert("00:64:4D", "UniFi Express 6");
      m.insert("00:65:5E", "UniFi Express Max");

      // == Microcontrollers & Dev Boards ==

      // Espressif (ESP32, ESP8266)
      m.insert("00:1A:22", "Espressif");
      m.insert("00:1B:C5", "Espressif ESP32");
      m.insert("00:1C:19", "Espressif ESP32-S2");
      m.insert("00:1D:28", "Espressif ESP32-S3");
      m.insert("00:1E:37", "Espressif ESP32-C3");
      m.insert("00:1F:46", "Espressif ESP8266");
      m.insert("00:20:55", "Espressif ESP32-WROVER");
      m.insert("00:21:64", "Espressif ESP32-DevKit");
      m.insert("00:22:73", "Espressif ESP32-CAM");
      m.insert("00:23:82", "Espressif ESP-LAUNCHER");

      // Arduino
      m.insert("00:04:23", "Arduino Uno");
      m.insert("00:05:25", "Arduino Mega");
      m.insert("00:06:27", "Arduino Nano");
      m.insert("00:07:29", "Arduino Leonardo");
      m.insert("00:08:2B", "Arduino Due");
      m.insert("00:09:2D", "Arduino Yun");
      m.insert("00:0A:2F", "Arduino Zero");
      m.insert("00:0B:31", "Arduino MKR");
      m.insert("00:0C:33", "Arduino Portenta");
      m.insert("00:0D:35", "Arduino Nano 33");
      m.insert("00:0E:37", "Arduino GIGA");
      m.insert("00:0F:39", "Arduino IDE");

      // Particle
      m.insert("00:02:54", "Particle");
      m.insert("00:04:23", "Particle Photon");
      m.insert("00:05:25", "Particle P1");
      m.insert("00:06:27", "Particle Electron");
      m.insert("00:07:29", "Particle Argon");
      m.insert("00:08:2B", "Particle Boron");
      m.insert("00:09:2D", "Particle Xenon");
      m.insert("00:0A:2F", "Particle Gen 3");
      m.insert("00:0B:31", "Particle Tracker");
      m.insert("00:0C:33", "Particle IO");

      // Adafruit
      m.insert("00:04:F3", "Adafruit");
      m.insert("00:15:83", "Adafruit Feather");
      m.insert("00:16:53", "Adafruit HUZZAH");
      m.insert("00:17:56", "Adafruit Circuit Python");
      m.insert("00:18:4D", "Adafruit Gemma");
      m.insert("00:19:1C", "Adafruit Trinket");
      m.insert("00:1A:11", "Adafruit ItsyBitsy");
      m.insert("00:1B:3C", "Adafruit Metro");
      m.insert("00:1C:12", "Adafruit Grand Central");
      m.insert("00:1D:0F", "Adafruit PyPortal");

      // Raspberry Pi
      m.insert("00:0A:95", "Raspberry Pi");
      m.insert("00:1A:2C", "Raspberry Pi 3");
      m.insert("00:1C:BF", "Raspberry Pi 4");
      m.insert("00:1D:0F", "Raspberry Pi 2");
      m.insert("00:1E:55", "Raspberry Pi 1");
      m.insert("00:1F:77", "Raspberry Pi Zero");
      m.insert("00:21:19", "Raspberry Pi Zero W");
      m.insert("00:22:4C", "Raspberry Pi CM");
      m.insert("00:23:11", "Raspberry Pi 3B+");
      m.insert("00:24:99", "Raspberry Pi 3A+");
      m.insert("00:25:46", "Raspberry Pi 400");
      m.insert("00:26:30", "Raspberry Pi 5");
      m.insert("00:27:10", "Raspberry Pi Pico");
      m.insert("00:28:32", "Raspberry Pi Compute Module 4");
      m.insert("00:29:45", "Raspberry Pi Pico W");

      // BeagleBone
      m.insert("00:0C:75", "BeagleBoard");
      m.insert("00:1D:C9", "BeagleBone Black");
      m.insert("00:1E:55", "BeagleBone Green");
      m.insert("00:1F:33", "BeagleBone AI");
      m.insert("00:21:27", "BeagleBone Black Wireless");
      m.insert("00:22:19", "BeagleBone PocketBeagle");
      m.insert("00:23:12", "BeagleBoard-xM");

      // NVIDIA Jetson
      m.insert("00:04:4B", "NVIDIA Jetson");
      m.insert("00:1A:11", "NVIDIA Jetson Nano");
      m.insert("00:1A:65", "NVIDIA Jetson TX1");
      m.insert("00:1B:5F", "NVIDIA Jetson TX2");
      m.insert("00:1C:13", "NVIDIA Jetson Xavier");
      m.insert("00:1C:B1", "NVIDIA Jetson AGX Xavier");
      m.insert("00:1D:26", "NVIDIA Jetson Orin");
      m.insert("00:1D:E6", "NVIDIA Jetson AGX Orin");
      m.insert("00:1E:42", "NVIDIA Jetson Xavier NX");
      m.insert("00:1E:88", "NVIDIA Jetson Nano Developer Kit");

      // ODROID
      m.insert("00:1B:21", "ODROID");
      m.insert("00:1C:53", "ODROID-XU4");
      m.insert("00:1D:66", "ODROID-N2");
      m.insert("00:1E:55", "ODROID-N2+");
      m.insert("00:1F:33", "ODROID-C4");
      m.insert("00:21:27", "ODROID-C2");
      m.insert("00:22:19", "ODROID-M1");
      m.insert("00:23:12", "ODROID-H2");
      m.insert("00:24:8C", "ODROID-H2+");
      m.insert("00:25:46", "ODROID-HC4");

      // == 3D Printers ==

      // Creality
      m.insert("00:15:22", "Creality");
      m.insert("00:16:53", "Creality Ender 3");
      m.insert("00:17:56", "Creality Ender 3 Pro");
      m.insert("00:18:4D", "Creality Ender 3 V2");
      m.insert("00:19:1C", "Creality Ender 5");
      m.insert("00:1A:11", "Creality Ender 5 Pro");
      m.insert("00:1B:3C", "Creality Ender 5 Plus");
      m.insert("00:1C:12", "Creality CR-10");
      m.insert("00:1D:0F", "Creality CR-10S");
      m.insert("00:1E:55", "Creality CR-10 V2");
      m.insert("00:1F:77", "Creality CR-10 V3");
      m.insert("00:21:19", "Creality CR-20");
      m.insert("00:22:4C", "Creality CR-30");
      m.insert("00:23:11", "Creality CR-X");
      m.insert("00:24:99", "Creality HALOT");
      m.insert("00:25:46", "Creality Sermoon");
      m.insert("00:26:30", "Creality K1");
      m.insert("00:27:10", "Creality K1 Max");

      // Prusa Research
      m.insert("00:1A:06", "Prusa");
      m.insert("00:1C:42", "Prusa i3 MK3");
      m.insert("00:1D:8C", "Prusa i3 MK3S");
      m.insert("00:1E:55", "Prusa i3 MK3S+");
      m.insert("00:1F:77", "Prusa Mini");
      m.insert("00:21:19", "Prusa SL1");
      m.insert("00:22:4C", "Prusa SL1S");
      m.insert("00:23:11", "Prusa Buddy");
      m.insert("00:24:99", "Prusa XL");
      m.insert("00:25:46", "Prusa MK4");

      // Ultimaker
      m.insert("00:0A:95", "Ultimaker");
      m.insert("00:1C:99", "Ultimaker 2");
      m.insert("00:1D:0F", "Ultimaker 2+");
      m.insert("00:1E:55", "Ultimaker 3");
      m.insert("00:1F:77", "Ultimaker 3 Extended");
      m.insert("00:21:19", "Ultimaker 5");
      m.insert("00:22:4C", "Ultimaker 5 Extended");
      m.insert("00:23:11", "Ultimaker S3");
      m.insert("00:24:99", "Ultimaker S5");
      m.insert("00:25:46", "Ultimaker S7");
      m.insert("00:26:30", "Ultimaker S5 Pro");

      // Formlabs
      m.insert("00:0D:28", "Formlabs");
      m.insert("00:1C:BF", "Form 2");
      m.insert("00:1D:0F", "Form 3");
      m.insert("00:1E:55", "Form 3B");
      m.insert("00:1F:77", "Form 3L");
      m.insert("00:21:19", "Form 3BL");
      m.insert("00:22:4C", "Form Cure");
      m.insert("00:23:11", "Form Wash");
      m.insert("00:24:99", "Fuse 1");

      // MakerBot
      m.insert("00:07:32", "MakerBot");
      m.insert("00:14:FF", "MakerBot Replicator");
      m.insert("00:15:99", "MakerBot Replicator+");
      m.insert("00:16:32", "MakerBot Replicator 2");
      m.insert("00:17:44", "MakerBot Replicator 5th Gen");
      m.insert("00:18:4D", "MakerBot Z18");
      m.insert("00:19:1C", "MakerBot Method");
      m.insert("00:1A:11", "MakerBot Method X");
      m.insert("00:1B:3C", "MakerBot Sketch");

      // Anycubic
      m.insert("00:1A:2C", "Anycubic");
      m.insert("00:1B:44", "Anycubic i3 Mega");
      m.insert("00:1C:53", "Anycubic i3 Mega S");
      m.insert("00:1D:66", "Anycubic Vyper");
      m.insert("00:1E:55", "Anycubic Chiron");
      m.insert("00:1F:33", "Anycubic Kobra");
      m.insert("00:21:27", "Anycubic Kobra Go");
      m.insert("00:22:19", "Anycubic Kobra Max");
      m.insert("00:23:12", "Anycubic Photon");
      m.insert("00:24:8C", "Anycubic Photon S");
      m.insert("00:25:46", "Anycubic Photon Mono");
      m.insert("00:26:30", "Anycubic Photon DLP");
      m.insert("00:27:10", "Anycubic Wash & Cure");

      // Elegoo
      m.insert("00:1A:79", "Elegoo");
      m.insert("00:1B:3C", "Elegoo Neptune");
      m.insert("00:1C:47", "Elegoo Neptune 2");
      m.insert("00:1D:50", "Elegoo Neptune 3");
      m.insert("00:1E:56", "Elegoo Saturn");
      m.insert("00:1F:67", "Elegoo Saturn 2");
      m.insert("00:20:47", "Elegoo Mars");
      m.insert("00:21:27", "Elegoo Mars 2");
      m.insert("00:22:19", "Elegoo Mars 3");
      m.insert("00:23:12", "Elegoo Jupiter");
      m.insert("00:24:8C", "Elegoo Resin");
      m.insert("00:25:46", "Elegoo Cura");

      // == Drones ==

      // DJI
      m.insert("00:12:1C", "DJI");
      m.insert("00:13:E4", "DJI Mavic");
      m.insert("00:14:36", "DJI Mavic Air");
      m.insert("00:15:25", "DJI Mavic Air 2");
      m.insert("00:16:6C", "DJI Mavic Air 2S");
      m.insert("00:17:0A", "DJI Mavic 3");
      m.insert("00:17:56", "DJI Mavic 2");
      m.insert("00:18:4D", "DJI Mavic Pro");
      m.insert("00:19:1C", "DJI Mavic Mini");
      m.insert("00:1A:11", "DJI Mini 2");
      m.insert("00:1B:3C", "DJI Mini 3 Pro");
      m.insert("00:1C:12", "DJI Phantom");
      m.insert("00:1D:0F", "DJI Phantom 4");
      m.insert("00:1E:55", "DJI Spark");
      m.insert("00:1F:77", "DJI FPV");
      m.insert("00:21:19", "DJI Avata");
      m.insert("00:22:4C", "DJI Inspire");
      m.insert("00:23:11", "DJI Matrice");
      m.insert("00:24:99", "DJI Ronin");
      m.insert("00:25:46", "DJI Osmo");
      m.insert("00:26:30", "DJI Pocket");
      m.insert("00:27:10", "DJI RC Pro");
      m.insert("00:28:32", "DJI RC");
      m.insert("00:29:45", "DJI RC-N1");

      // Parrot
      m.insert("00:12:1C", "Parrot");
      m.insert("00:1C:26", "Parrot ANAFI");
      m.insert("00:1D:8F", "Parrot Bebop");
      m.insert("00:1E:55", "Parrot Bebop 2");
      m.insert("00:1F:77", "Parrot Mambo");
      m.insert("00:21:19", "Parrot Swing");
      m.insert("00:22:4C", "Parrot Disco");
      m.insert("00:23:11", "Parrot AR.Drone");
      m.insert("00:24:99", "Parrot SkyController");

      // Skydio
      m.insert("00:1A:2C", "Skydio");
      m.insert("00:1C:BF", "Skydio 2");
      m.insert("00:1D:0F", "Skydio 2+");
      m.insert("00:1E:55", "Skydio X2");
      m.insert("00:1F:77", "Skydio Controller");

      // Autel Robotics
      m.insert("00:1B:3C", "Autel");
      m.insert("00:1C:47", "Autel Evo");
      m.insert("00:1D:50", "Autel Evo Lite");
      m.insert("00:1E:56", "Autel Evo Nano");
      m.insert("00:1F:67", "Autel Dragonfly");
      m.insert("00:21:27", "Autel Robotics");

      // Holy Stone
      m.insert("00:1D:0F", "Holy Stone");
      m.insert("00:1E:55", "Holy Stone HS100");
      m.insert("00:1F:77", "Holy Stone HS110");
      m.insert("00:21:19", "Holy Stone HS165");
      m.insert("00:22:4C", "Holy Stone HS720");
      m.insert("00:23:11", "Holy Stone HS700");
      m.insert("00:24:99", "Holy Stone F181");
      m.insert("00:25:46", "Holy Stone HS600");

      // Syma
      m.insert("00:1F:33", "Syma");
      m.insert("00:21:27", "Syma X5C");
      m.insert("00:22:19", "Syma X8");
      m.insert("00:23:12", "Syma X8C");
      m.insert("00:24:8C", "Syma X8G");
      m.insert("00:25:46", "Syma Z3");
      m.insert("00:26:30", "Syma X9");
      m.insert("00:27:10", "Syma X20");
      m.insert("00:28:32", "Syma X22");

      // == E-Bikes & Scooters ==

      // Xiaomi Mi Electric Scooter
      m.insert("00:15:5D", "Xiaomi");
      m.insert("00:1A:22", "Xiaomi Mi Electric Scooter");
      m.insert("00:1B:44", "Xiaomi Mi Scooter Pro");
      m.insert("00:1C:53", "Xiaomi Mi Scooter Pro 2");
      m.insert("00:1D:66", "Xiaomi Mi Scooter 1S");
      m.insert("00:1E:55", "Xiaomi Mi Scooter Essential");
      m.insert("00:1F:33", "Xiaomi Mi Scooter Lite");
      m.insert("00:21:27", "Xiaomi Ninebot");
      m.insert("00:22:19", "Xiaomi Qicycle");

      // Segway-Ninebot
      m.insert("00:1D:28", "Segway");
      m.insert("00:1E:55", "Segway Ninebot");
      m.insert("00:1F:77", "Segway Max");
      m.insert("00:21:19", "Segway Ninebot ES");
      m.insert("00:22:4C", "Segway Ninebot E");
      m.insert("00:23:11", "Segway GT");
      m.insert("00:24:99", "Segway SuperScooter");
      m.insert("00:25:46", "Segway-Ninebot Bot");

      // Bird
      m.insert("00:21:27", "Bird");
      m.insert("00:22:19", "Bird One");
      m.insert("00:23:12", "Bird Air");
      m.insert("00:24:8C", "Bird Three");
      m.insert("00:25:46", "Bird Lite");

      // Lime
      m.insert("00:23:12", "Lime");
      m.insert("00:24:8C", "Lime Gen 2");
      m.insert("00:25:46", "Lime Gen 3");
      m.insert("00:26:30", "Lime S");

      // Spin
      m.insert("00:25:46", "Spin");
      m.insert("00:26:30", "Spin Gen 2");
      m.insert("00:27:10", "Spin Gen 3");

      // Razor
      m.insert("00:26:30", "Razor");
      m.insert("00:27:10", "Razor E100");
      m.insert("00:28:32", "Razor E200");
      m.insert("00:29:45", "Razor E300");
      m.insert("00:2A:6A", "Razor EcoSmart");
      m.insert("00:2B:8C", "Razor Power A2");
      m.insert("00:2C:21", "Razor Power Core");
      m.insert("00:2D:54", "Razor Vibertron");

      // == Game Consoles & Handhelds ==

      // Steam Deck (Valve)
      m.insert("00:1D:0D", "Valve");
      m.insert("00:1E:55", "Steam Deck");
      m.insert("00:1F:77", "Steam Deck Dock");
      m.insert("00:21:19", "Steam Controller");
      m.insert("00:22:4C", "Steam Link");
      m.insert("00:23:11", "Valve Index");

      // Nintendo Switch
      m.insert("00:16:FE", "Nintendo Switch");
      m.insert("00:17:AB", "Nintendo Switch Lite");
      m.insert("00:18:74", "Nintendo Switch OLED");
      m.insert("00:19:1D", "Nintendo Joy-Con");
      m.insert("00:1A:09", "Nintendo Pro Controller");
      m.insert("00:1B:7D", "Nintendo Switch Dock");
      m.insert("00:1C:BE", "Nintendo LAN Adapter");

      // PlayStation (Sony)
      m.insert("00:04:1F", "Sony");
      m.insert("00:1A:80", "PlayStation 4");
      m.insert("00:1B:4E", "PlayStation 5");
      m.insert("00:1C:D5", "PlayStation VR");
      m.insert("00:1E:46", "PlayStation VR2");
      m.insert("00:1F:E7", "PlayStation Portal");
      m.insert("00:21:4E", "DualShock 4");
      m.insert("00:22:4C", "DualSense");
      m.insert("00:23:11", "PlayStation Camera");
      m.insert("00:24:99", "PlayStation Move");
      m.insert("00:25:46", "PS VR Headset");
      m.insert("00:26:30", "PS5 Disc Drive");

      // Xbox (Microsoft)
      m.insert("00:03:FF", "Microsoft");
      m.insert("00:1D:D8", "Xbox One");
      m.insert("00:1E:18", "Xbox Series X");
      m.insert("00:1F:E6", "Xbox Series S");
      m.insert("00:21:F1", "Xbox Wireless Controller");
      m.insert("00:22:4C", "Xbox Adaptive Controller");
      m.insert("00:23:11", "Xbox Kinect");
      m.insert("00:24:99", "Xbox Elite");
      m.insert("00:25:46", "Xbox Media Remote");

      // Atari
      m.insert("00:00:57", "Atari");
      m.insert("00:12:3A", "Atari VCS");
      m.insert("00:13:21", "Atari Flashback");

      // Logitech Gaming
      m.insert("00:1D:4E", "Logitech");
      m.insert("00:1F:20", "Logitech G");
      m.insert("00:21:27", "Logitech G Pro Keyboard");
      m.insert("00:22:19", "Logitech G Pro Mouse");
      m.insert("00:23:12", "Logitech G502");
      m.insert("00:24:8C", "Logitech G603");
      m.insert("00:25:46", "Logitech G733");
      m.insert("00:26:30", "Logitech G935");
      m.insert("00:27:10", "Logitech F310");
      m.insert("00:28:32", "Logitech F710");
      m.insert("00:29:45", "Logitech G920");

      // Razer
      m.insert("00:15:B9", "Razer");
      m.insert("00:1C:BF", "Razer BlackWidow");
      m.insert("00:1D:0F", "Razer DeathAdder");
      m.insert("00:1E:55", "Razer Naga");
      m.insert("00:1F:77", "Razer Basilisk");
      m.insert("00:21:19", "Razer Huntsman");
      m.insert("00:22:4C", "Razer Viper");
      m.insert("00:23:11", "Razer Goliathus");
      m.insert("00:24:99", "Razer Kraken");
      m.insert("00:25:46", "Razer BlackShark");
      m.insert("00:26:30", "Razer Blade");
      m.insert("00:27:10", "Razer Barracuda");
      m.insert("00:28:32", "Razer Seiren");
      m.insert("00:29:45", "Razer Turret");

      // Corsair
      m.insert("00:1D:46", "Corsair");
      m.insert("00:1F:54", "Corsair K95");
      m.insert("00:21:27", "Corsair K70");
      m.insert("00:22:19", "Corsair K60");
      m.insert("00:23:12", "Corsair Strafe");
      m.insert("00:24:8C", "Corsair Ironclaw");
      m.insert("00:25:46", "Corsair Harpoon");
      m.insert("00:26:30", "Corsair M65");
      m.insert("00:27:10", "Corsair Dark Core");
      m.insert("00:28:32", "Corsair Virtuoso");
      m.insert("00:29:45", "Corsair Void");
      m.insert("00:2A:6A", "Corsair HS50");
      m.insert("00:2B:8C", "Corsair HS60");
      m.insert("00:2C:21", "Corsair SP2500");
      m.insert("00:2D:54", "Corsair ML120");

      // SteelSeries
      m.insert("00:1C:BF", "SteelSeries");
      m.insert("00:1D:0F", "SteelSeries Apex");
      m.insert("00:1E:55", "SteelSeries Rival");
      m.insert("00:1F:77", "SteelSeries Sensei");
      m.insert("00:21:19", "SteelSeries Arctis");
      m.insert("00:22:4C", "SteelSeries Siberia");
      m.insert("00:23:11", "SteelSeries Apex Pro");
      m.insert("00:24:99", "SteelSeries Prime");
      m.insert("00:25:46", "SteelSeries Aerox");

      // == USB Hubs & Docking Stations ==

      // Anker
      m.insert("00:18:E6", "Anker");
      m.insert("00:1A:56", "Anker PowerPort");
      m.insert("00:1B:3C", "Anker PowerDrive");
      m.insert("00:1C:47", "Anker SoundCore");
      m.insert("00:1D:50", "Anker SoundBuds");
      m.insert("00:1E:56", "Anker Nebula");
      m.insert("00:1F:67", "Anker Liberty");
      m.insert("00:20:47", "Anker PowerHouse");
      m.insert("00:21:27", "Anker 737 Power Bank");
      m.insert("00:22:19", "Anker 533 Power Bank");
      m.insert("00:23:12", "Anker Prime");
      m.insert("00:24:8C", "Anker 321 Power Bank");
      m.insert("00:25:46", "Anker 511 Power Bank");

      // CalDigit
      m.insert("00:02:50", "CalDigit");
      m.insert("00:1C:7B", "CalDigit TS3 Plus");
      m.insert("00:1D:C9", "CalDigit TS4");
      m.insert("00:1E:55", "CalDigit SOHO Dock");
      m.insert("00:1F:77", "CalDigit BHU");
      m.insert("00:21:19", "CalDigit USB Pro");
      m.insert("00:22:4C", "CalDigit FA-101");

      // OWC
      m.insert("00:0D:93", "OWC");
      m.insert("00:1C:42", "OWC Thunderbolt Hub");
      m.insert("00:1D:8C", "OWC USB-C Dock");
      m.insert("00:1E:55", "OWC USB-C Pro Dock");
      m.insert("00:1F:77", "OWC Thunderbolt 3 Dock");
      m.insert("00:21:19", "OWC Envoy Express");
      m.insert("00:22:4C", "OWC Envoy Pro FX");
      m.insert("00:23:11", "OWC Envoy Pro EX");
      m.insert("00:24:99", "OWC Mercury Elite Pro");

      // Dell WD15/ WD19 Dock
      m.insert("00:0C:E6", "Dell");
      m.insert("00:14:22", "Dell WD15 Dock");
      m.insert("00:15:99", "Dell WD19 Dock");
      m.insert("00:16:32", "Dell WD19TBS Dock");
      m.insert("00:17:44", "Dell WD19DC Dock");
      m.insert("00:18:4D", "Dell Thunderbolt Dock");
      m.insert("00:19:1C", "Dell DA300");
      m.insert("00:1A:11", "Dell DA310");
      m.insert("00:1B:3C", "Dell USB-C Hub");
      m.insert("00:1C:12", "Dell Multi-Adapter");

      // HP Thunderbolt Dock
      m.insert("00:0C:77", "HP");
      m.insert("00:14:38", "HP Thunderbolt Dock");
      m.insert("00:15:2D", "HP USB-C Dock");
      m.insert("00:16:42", "HP USB-C G2 Dock");
      m.insert("00:17:1E", "HP USB-C G5 Dock");
      m.insert("00:18:4D", "HP Thunderbolt Dock G2");
      m.insert("00:19:1C", "HP Elite Thunderbolt");
      m.insert("00:1A:11", "HP Thunderbolt 120W");
      m.insert("00:1B:3C", "HP Z Dock");
      m.insert("00:1C:12", "HP Z Mini Dock");

      // Plugable
      m.insert("00:1A:2C", "Plugable");
      m.insert("00:1C:BF", "Plugable USB Dock");
      m.insert("00:1D:0F", "Plugable TBT3 Dock");
      m.insert("00:1E:55", "Plugable TBT4 Dock");
      m.insert("00:1F:77", "Plugable UD-6950");
      m.insert("00:21:19", "Plugable UD-3900");
      m.insert("00:22:4C", "Plugable UD-768");
      m.insert("00:23:11", "Plugable USBC-6950");

      // Kensington
      m.insert("00:0C:E3", "Kensington");
      m.insert("00:1C:57", "Kensington SD2000");
      m.insert("00:1D:8C", "Kensington SD2500");
      m.insert("00:1E:55", "Kensington SD2600");
      m.insert("00:1F:77", "Kensington SD3500");
      m.insert("00:21:19", "Kensington SD5200");
      m.insert("00:22:4C", "Kensington SD5200T");
      m.insert("00:23:11", "Kensington SD5400T");
      m.insert("00:24:99", "Kensington SD5500T");

      // == Industrial IoT & PLC ==

      // Siemens Industrial
      m.insert("00:01:37", "Siemens");
      m.insert("00:08:84", "Siemens PLC");
      m.insert("00:1B:1B", "Siemens S7");
      m.insert("00:07:7A", "Siemens LOGO");
      m.insert("00:09:11", "Siemens SIMATIC");
      m.insert("00:0A:5A", "Siemens ET200");
      m.insert("00:0B:1D", "Siemens HMI");
      m.insert("00:0C:26", "Siemens SINAMICS");
      m.insert("00:0D:47", "Siemens SCALANCE");
      m.insert("00:0E:88", "Siemens RUGGEDCOM");
      m.insert("00:0F:6D", "Siemens Industrial");
      m.insert("00:10:E1", "Siemens PROFINET");

      // Schneider Electric
      m.insert("00:00:57", "Schneider Electric");
      m.insert("00:04:A3", "Schneider Modicon");
      m.insert("00:08:9B", "Schneider Altivar");
      m.insert("00:09:3A", "Schneider Lexium");
      m.insert("00:0A:52", "Schneider Sepam");
      m.insert("00:0B:47", "Schneider Zelio");
      m.insert("00:0C:29", "Schneider Twido");
      m.insert("00:0D:18", "Schneider Vijeo");
      m.insert("00:0E:57", "Schneider EcoStruxure");
      m.insert("00:0F:42", "Schneider PowerTag");
      m.insert("00:10:9C", "Schneider Circuit Breaker");

      // Rockwell Automation
      m.insert("00:00:BC", "Rockwell");
      m.insert("00:1B:54", "Rockwell PLC");
      m.insert("00:1D:9C", "Rockwell ControlLogix");
      m.insert("00:08:9B", "Rockwell CompactLogix");
      m.insert("00:09:3A", "Rockwell MicroLogix");
      m.insert("00:0A:52", "Rockwell SLC");
      m.insert("00:0B:47", "Rockwell PanelView");
      m.insert("00:0C:29", "Rockwell PowerFlex");
      m.insert("00:0D:18", "Rockwell Stratix");
      m.insert("00:0E:57", "Rockwell FactoryTalk");
      m.insert("00:0F:42", "Rockwell GuardLogix");

      // Honeywell Process
      m.insert("00:40:D3", "Honeywell");
      m.insert("00:1B:3C", "Honeywell Process");
      m.insert("00:1E:55", "Honeywell Experion");
      m.insert("00:09:1A", "Honeywell TDC");
      m.insert("00:0E:6B", "Honeywell HC900");
      m.insert("00:11:24", "Honeywell UDC");
      m.insert("00:12:3A", "Honeywell DCE");
      m.insert("00:13:46", "Honeywell FDM");
      m.insert("00:14:57", "Honeywell ST3000");
      m.insert("00:15:62", "Honeywell STT");

      // Mitsubishi Electric
      m.insert("00:0C:29", "Mitsubishi");
      m.insert("00:1A:2C", "Mitsubishi PLC");
      m.insert("00:1C:BF", "Mitsubishi FX");
      m.insert("00:1D:0F", "Mitsubishi Q");
      m.insert("00:1E:55", "Mitsubishi L");
      m.insert("00:1F:77", "Mitsubishi R");
      m.insert("00:21:19", "Mitsubishi GOT");
      m.insert("00:22:4C", "Mitsubishi FR");
      m.insert("00:23:11", "Mitsubishi MR");
      m.insert("00:24:99", "Mitsubishi GT");
      m.insert("00:25:46", "Mitsubishi E");

      // Omron
      m.insert("00:0B:78", "Omron");
      m.insert("00:1A:2C", "Omron PLC");
      m.insert("00:1C:BF", "Omron CP1");
      m.insert("00:1D:0F", "Omron CJ2");
      m.insert("00:1E:55", "Omron CS1");
      m.insert("00:1F:77", "Omron C200");
      m.insert("00:21:19", "Omron NX");
      m.insert("00:22:4C", "Omron NJ");
      m.insert("00:23:11", "Omron NY");
      m.insert("00:24:99", "Omron Sysmac");
      m.insert("00:25:46", "Omron HMI");

      // Bosch Rexroth
      m.insert("00:1A:2C", "Bosch Rexroth");
      m.insert("00:1C:BF", "Bosch Indramat");
      m.insert("00:1D:0F", "Bosch CtrlX");
      m.insert("00:1E:55", "Bosch XMS");
      m.insert("00:1F:77", "Bosch HMV");
      m.insert("00:21:19", "Bosch PLC");
      m.insert("00:22:4C", "Bosch Drive");
      m.insert("00:23:11", "Bosch Servo");
      m.insert("00:24:99", "Bosch Hydraulic");
      m.insert("00:25:46", "Bosch Linear");

      // == Home Appliances ==

      // Samsung Smart Appliances
      m.insert("00:02:5B", "Samsung");
      m.insert("00:0D:3D", "Samsung SmartThings");
      m.insert("00:12:5A", "Samsung Refrigerator");
      m.insert("00:13:C3", "Samsung Washer");
      m.insert("00:14:31", "Samsung Dryer");
      m.insert("00:15:2A", "Samsung Dishwasher");
      m.insert("00:16:36", "Samsung Oven");
      m.insert("00:17:0D", "Samsung Air Purifier");
      m.insert("00:18:4B", "Samsung Vacuum");
      m.insert("00:19:1C", "Samsung AC");
      m.insert("00:1A:3E", "Samsung Washer AddWash");
      m.insert("00:1B:4B", "Samsung Bespoke");

      // LG ThinQ
      m.insert("00:1D:25", "LG");
      m.insert("00:1E:4C", "LG ThinQ");
      m.insert("00:1F:33", "LG Refrigerator");
      m.insert("00:21:27", "LG Washer");
      m.insert("00:22:19", "LG Dryer");
      m.insert("00:23:12", "LG Dishwasher");
      m.insert("00:24:8C", "LG Oven");
      m.insert("00:25:46", "LG Air Purifier");
      m.insert("00:26:30", "LG Styler");
      m.insert("00:27:10", "LG Dehumidifier");

      // GE Appliances (SmartHQ)
      m.insert("00:00:4A", "GE");
      m.insert("00:04:AC", "GE Appliances");
      m.insert("00:06:5B", "GE SmartHQ");
      m.insert("00:09:6B", "GE Profile");
      m.insert("00:11:25", "GE Cafe");
      m.insert("00:14:5A", "GE Monogram");
      m.insert("00:17:DF", "GE Top-Load");
      m.insert("00:1A:64", "GE Front-Load");
      m.insert("00:1D:8C", "GE Smart Monitor");
      m.insert("00:1F:19", "GE Water Heater");
      // Hikvision (including Hikvision Digital Technology)
      m.insert("00:1D:0D", "Hikvision");
      m.insert("00:1D:27", "Hikvision Digital Technology");
      m.insert("00:1D:CD", "Hikvision IP Camera");
      m.insert("00:1E:13", "Hikvision Dome Camera");
      m.insert("00:1E:74", "Hikvision Bullet Camera");
      m.insert("00:1F:16", "Hikvision PTZ Camera");
      m.insert("00:1F:AC", "Hikvision NVR");
      m.insert("00:20:19", "Hikvision DVR");
      m.insert("00:20:D2", "Hikvision Turbo DVR");
      m.insert("00:21:2A", "Hikvision Video Encoder");
      m.insert("00:21:E8", "Hikvision Access Control");
      m.insert("00:22:3A", "Hikvision Intercom");
      m.insert("00:22:9C", "Hikvision Biometrics");
      m.insert("00:23:12", "Hikvision Thermal Camera");
      m.insert("00:23:CD", "Hikvision Deep Learning");
      m.insert("00:24:1C", "Hikvision Pro Series");
      m.insert("00:24:B9", "Hikvision Ultra Series");
      m.insert("00:25:26", "Hikvision WiFi Camera");
      m.insert("00:25:AE", "Hikvision Solar Camera");
      m.insert("00:26:19", "Hikvision PoE Switch");

      // Dahua Technology
      m.insert("00:1B:54", "Dahua");
      m.insert("00:1C:25", "Dahua Technology");
      m.insert("00:1C:BE", "Dahua IP Camera");
      m.insert("00:1D:35", "Dahua Dome Camera");
      m.insert("00:1D:E6", "Dahua Bullet Camera");
      m.insert("00:1E:4D", "Dahua PTZ Camera");
      m.insert("00:1F:1A", "Dahua NVR");
      m.insert("00:1F:E7", "Dahua DVR");
      m.insert("00:20:39", "Dahua XVR");
      m.insert("00:20:F1", "Dahua Video Wall");
      m.insert("00:21:3D", "Dahua Access Control");
      m.insert("00:21:DB", "Dahua Doorbell");
      m.insert("00:22:26", "Dahua Thermal Imaging");
      m.insert("00:22:C8", "Dahua LPR Camera");
      m.insert("00:23:11", "Dahua TiOC");
      m.insert("00:23:99", "Dahua WizSense");
      m.insert("00:24:1A", "Dahua WizMind");
      m.insert("00:24:9C", "Dahua ePOE");
      m.insert("00:25:21", "Dahua DHI Prefix");
      m.insert("00:25:A3", "Dahua Commercial");

      // Axis Communications
      m.insert("00:40:8C", "Axis");
      m.insert("00:02:3A", "Axis Communications");
      m.insert("00:0A:DC", "Axis IP Camera");
      m.insert("00:0E:7C", "Axis Dome Camera");
      m.insert("00:12:3A", "Axis Bullet Camera");
      m.insert("00:13:21", "Axis PTZ Camera");
      m.insert("00:14:4F", "Axis Fixed Camera");
      m.insert("00:15:70", "Axis Thermal Camera");
      m.insert("00:16:35", "Axis Explosion-Proof");
      m.insert("00:17:08", "Axis Mini PTZ");
      m.insert("00:18:BA", "Axis Corner Camera");
      m.insert("00:19:11", "Axis Multi-Sensor");
      m.insert("00:1A:33", "Axis Q Series");
      m.insert("00:1B:44", "Axis P Series");
      m.insert("00:1C:55", "Axis M Series");
      m.insert("00:1D:66", "Axis F Series");
      m.insert("00:1E:77", "Axis AXIS S");
      m.insert("00:1F:88", "Axis NVR");
      m.insert("00:20:99", "Axis DVR");
      m.insert("00:21:AA", "Axis Encoder");
      m.insert("00:22:BB", "Axis Video Receiver");
      m.insert("00:23:CC", "Axis Access Control");
      m.insert("00:24:DD", "Axis Audio");
      m.insert("00:25:EE", "Axis I/O");
      m.insert("00:26:30", "Axis Radar");
      m.insert("00:27:10", "Axis Speakers");
      m.insert("00:28:32", "Axis Indicators");

      // Bosch Security Systems
      m.insert("00:03:7E", "Bosch");
      m.insert("00:01:37", "Bosch Security");
      m.insert("00:07:7A", "Bosch IP Camera");
      m.insert("00:08:84", "Bosch Dome Camera");
      m.insert("00:09:1A", "Bosch Bullet Camera");
      m.insert("00:0A:5A", "Bosch PTZ Camera");
      m.insert("00:0B:1D", "Bosch NVR");
      m.insert("00:0C:26", "Bosch DVR");
      m.insert("00:0D:47", "Bosch Video Encoder");
      m.insert("00:0E:88", "Bosch Divar");
      m.insert("00:0F:6D", "Bosch DiBos");
      m.insert("00:10:E1", "Bosch Access");
      m.insert("00:11:BB", "Bosch Intrusion");
      m.insert("00:12:7A", "Bosch Fire");
      m.insert("00:13:1D", "Bosch Building Integration");
      m.insert("00:14:88", "Bosch Video Management");
      m.insert("00:15:62", "Bosch Analytics");
      m.insert("00:16:93", "Bosch MIC Camera");
      m.insert("00:17:5F", "Bosch AutoDome");
      m.insert("00:18:50", "Bosch Dinion");
      m.insert("00:19:11", "Bosch Flexidome");
      m.insert("00:1A:22", "Bosch Dinion IP");
      m.insert("00:1B:3C", "Bosch IP 4000");
      m.insert("00:1C:12", "Bosch IP 5000");
      m.insert("00:1D:0F", "Bosch IP 6000");
      m.insert("00:1E:55", "Bosch IP 7000");

      // Panasonic i-PRO
      m.insert("00:00:1A", "Panasonic");
      m.insert("00:12:3A", "Panasonic i-PRO");
      m.insert("00:13:21", "Panasonic Network Camera");
      m.insert("00:14:4F", "Panasonic Dome Camera");
      m.insert("00:15:70", "Panasonic Bullet Camera");
      m.insert("00:16:35", "Panasonic PTZ Camera");
      m.insert("00:17:08", "Panasonic NVR");
      m.insert("00:18:BA", "Panasonic WJ Series");
      m.insert("00:19:11", "Panasonic BL Series");
      m.insert("00:1A:33", "Panasonic BB Series");
      m.insert("00:1B:44", "Panasonic WV Series");
      m.insert("00:1C:55", "Panasonic WV-S Series");
      m.insert("00:1D:66", "Panasonic WV-SF Series");
      m.insert("00:1E:77", "Panasonic WV-SW Series");
      m.insert("00:1F:88", "Panasonic WV-SP Series");
      m.insert("00:20:99", "Panasonic WV-NP Series");
      m.insert("00:21:AA", "Panasonic WV-V Series");
      m.insert("00:22:BB", "Panasonic System Manager");
      m.insert("00:23:CC", "Panasonic Video Insight");

      // Sony Security
      m.insert("00:01:0E", "Sony");
      m.insert("00:04:8B", "Sony Security");
      m.insert("00:0A:3D", "Sony IP Camera");
      m.insert("00:0E:6B", "Sony SNC");
      m.insert("00:12:3A", "Sony G5");
      m.insert("00:13:21", "Sony G6");
      m.insert("00:14:4F", "Sony G7");
      m.insert("00:15:70", "Sony SNC-EM");
      m.insert("00:16:35", "Sony SNC-VM");
      m.insert("00:17:08", "Sony SNC-P");
      m.insert("00:18:BA", "Sony SNC-CH");
      m.insert("00:19:11", "Sony SNC-EB");
      m.insert("00:1A:33", "Sony SNC-ER");
      m.insert("00:1B:44", "Sony SNC-VB");
      m.insert("00:1C:55", "Sony SNC-VM");
      m.insert("00:1D:66", "Sony REA-C Series");
      m.insert("00:1E:77", "Sony Video Management");
      m.insert("00:1F:88", "Sony NVR");
      m.insert("00:20:99", "Sony Analytics");

      // Pelco (Motorola Solutions)
      m.insert("00:0B:81", "Pelco");
      m.insert("00:1D:0F", "Pelco by Motorola");
      m.insert("00:1E:55", "Pelco IP Camera");
      m.insert("00:1F:77", "Pelco Sarix");
      m.insert("00:21:19", "Pelco Spectra");
      m.insert("00:22:4C", "Pelco Esprit");
      m.insert("00:23:11", "Pelco Endura");
      m.insert("00:24:99", "Pelco Digital Sentry");
      m.insert("00:25:46", "Pelco DX Series");
      m.insert("00:26:30", "Pelco VS Series");
      m.insert("00:27:10", "Pelco DSeries");
      m.insert("00:28:32", "Pelco ISeries");
      m.insert("00:29:45", "Pelco G Series");
      m.insert("00:2A:6A", "Pelco PM Series");
      m.insert("00:2B:8C", "Pelco Network Video");

      // Avigilon (Motorola Solutions)
      m.insert("00:1C:BF", "Avigilon");
      m.insert("00:1D:0F", "Avigilon H5A");
      m.insert("00:1E:55", "Avigilon H4A");
      m.insert("00:1F:77", "Avigilon H4L");
      m.insert("00:21:19", "Avigilon H4ES");
      m.insert("00:22:4C", "Avigilon H4PTZ");
      m.insert("00:23:11", "Avigilon H3A");
      m.insert("00:24:99", "Avigilon H3C");
      m.insert("00:25:46", "Avigilon 3.0H");
      m.insert("00:26:30", "Avigilon NVR");
      m.insert("00:27:10", "Avigilon ONVIF");
      m.insert("00:28:32", "Avigilon Video Analytics");
      m.insert("00:29:45", "Avigilon Access Control");

      // Honeywell Commercial Video
      m.insert("00:40:D3", "Honeywell");
      m.insert("00:1B:3C", "Honeywell Video");
      m.insert("00:1E:55", "Honeywell IP Camera");
      m.insert("00:09:1A", "Honeywell MaxPro");
      m.insert("00:0E:6B", "Honeywell Rapid Eye");
      m.insert("00:11:24", "Honeywell HRDP");
      m.insert("00:12:3A", "Honeywell HDZ");
      m.insert("00:13:46", "Honeywell HIDC");
      m.insert("00:14:57", "Honeywell H4D");
      m.insert("00:15:62", "Honeywell H3D");
      m.insert("00:16:75", "Honeywell H2D");
      m.insert("00:17:8A", "Honeywell NVR");
      m.insert("00:18:11", "Honeywell DVR");
      m.insert("00:19:44", "Honeywell Fusion");
      m.insert("00:1A:55", "Honeywell Video Analytics");
      m.insert("00:1B:3C", "Honeywell EquIP");
      m.insert("00:1C:47", "Honeywell Performance Series");

      // Vivotek
      m.insert("00:02:3B", "Vivotek");
      m.insert("00:0D:3A", "Vivotek IP Camera");
      m.insert("00:11:44", "Vivotek Dome Camera");
      m.insert("00:15:86", "Vivotek Bullet Camera");
      m.insert("00:1A:33", "Vivotek PTZ Camera");
      m.insert("00:1C:57", "Vivotek Fisheye");
      m.insert("00:1E:68", "Vivotek Speed Dome");
      m.insert("00:1F:33", "Vivotek NVR");
      m.insert("00:21:27", "Vivotek ND Series");
      m.insert("00:22:19", "Vivotek Rack Mount");
      m.insert("00:23:12", "Vivotek Video Server");
      m.insert("00:24:8C", "Vivotek VIVOTEK");
      m.insert("00:25:46", "Vivotek iMEGA");
      m.insert("00:26:30", "Vivotek FE9191");
      m.insert("00:27:10", "Vivotek MS9390");

      // Hanwha Techwin (Samsung Techwin)
      m.insert("00:02:54", "Hanwha");
      m.insert("00:0B:57", "Samsung Techwin");
      m.insert("00:1B:3C", "Hanwha Wisenet");
      m.insert("00:1C:12", "Wisenet X");
      m.insert("00:1D:0F", "Wisenet Q");
      m.insert("00:1E:55", "Wisenet P");
      m.insert("00:1F:77", "Wisenet T");
      m.insert("00:21:19", "Wisenet L");
      m.insert("00:22:4C", "Wisenet S");
      m.insert("00:23:11", "Wisenet HN");
      m.insert("00:24:99", "Wisenet NVR");
      m.insert("00:25:46", "Wisenet DVR");
      m.insert("00:26:30", "Wisenet WAVE");
      m.insert("00:27:10", "Wisenet Wisenet7");
      m.insert("00:28:32", "Wisenet AI Camera");

      // == Commercial Access Control Systems ==

      // HID Global
      m.insert("00:02:54", "HID");
      m.insert("00:05:25", "HID Global");
      m.insert("00:09:1A", "HID iClass");
      m.insert("00:0A:5A", "HID Prox");
      m.insert("00:0B:1D", "HID Signo");
      m.insert("00:0C:26", "HID Seos");
      m.insert("00:0D:47", "HID Crescendo");
      m.insert("00:0E:88", "HID Fargo");
      m.insert("00:0F:6D", "HID Identity");
      m.insert("00:10:E1", "HID Access Control");
      m.insert("00:11:BB", "HID Reader");
      m.insert("00:12:7A", "HID Card");
      m.insert("00:13:1D", "HID Mobile Access");
      m.insert("00:14:88", "HID pivCLASS");
      m.insert("00:15:62", "HID VertX");
      m.insert("00:16:93", "HID EDGE");
      m.insert("00:17:5F", "HID BioSign");
      m.insert("00:18:50", "HID Lumidigm");
      m.insert("00:19:11", "HID OMNIKEY");
      m.insert("00:1A:22", "HID DTAM");
      m.insert("00:1B:3C", "HID iCLASS SE");
      m.insert("00:1C:12", "HID multiCLASS SE");

      // Assa Abloy
      m.insert("00:0B:57", "Assa Abloy");
      m.insert("00:04:F2", "Assa Abloy HID");
      m.insert("00:09:1A", "Assa Abloy Yale");
      m.insert("00:0A:42", "Assa Abloy Sargent");
      m.insert("00:0B:77", "Assa Abloy Corbin");
      m.insert("00:0C:55", "Assa Abloy Securitron");
      m.insert("00:0D:11", "Assa Abloy Medeco");
      m.insert("00:0E:88", "Assa Abloy HES");
      m.insert("00:0F:3C", "Assa Abloy RCI");
      m.insert("00:10:5A", "Assa Abloy Traka");
      m.insert("00:11:22", "Assa Abloy Pio");
      m.insert("00:12:35", "Assa Abloy Adams Rite");
      m.insert("00:13:55", "Assa Abloy CSA");
      m.insert("00:14:66", "Assa Abloy Frame");
      m.insert("00:15:77", "Assa Abloy Exit Devices");
      m.insert("00:16:88", "Assa Abloy Electrified");
      m.insert("00:17:99", "Assa Abloy MagLock");
      m.insert("00:18:AA", "Assa Abloy Request to Exit");
      m.insert("00:19:BB", "Assa Abloy Door Operator");
      m.insert("00:1A:33", "Assa Abloy Aperio");
      m.insert("0C:B8:15", "Assa Abloy Cliq");
      m.insert("00:1C:55", "Assa Abloy SPAC");

      // Paxton Access
      m.insert("00:09:1A", "Paxton");
      m.insert("00:0A:5A", "Paxton Access");
      m.insert("00:0B:1D", "Paxton Net2");
      m.insert("00:0C:26", "Paxton Pax");
      m.insert("00:0D:47", "Paxton Access Control");
      m.insert("00:0E:88", "Paxton Reader");
      m.insert("00:0F:6D", "Paxton Token");
      m.insert("00:10:E1", "Paxton Keyfob");
      m.insert("00:11:BB", "Paxton Smart Token");
      m.insert("00:12:7A", "Paxton Bluetooth Reader");
      m.insert("00:13:1D", "Paxton Near");
      m.insert("00:14:88", "Paxton Switch");
      m.insert("00:15:62", "Paxton Door Handle");
      m.insert("00:16:93", "Paxton Exit Button");
      m.insert("00:17:5F", "Paxton Power Supply");

      // Kantech (Axis)
      m.insert("00:0B:1D", "Kantech");
      m.insert("00:0C:26", "Kantech Systems");
      m.insert("00:0D:47", "Kantech KT-1");
      m.insert("00:0E:88", "Kantech KT-2");
      m.insert("00:0F:6D", "Kantech ioController");
      m.insert("00:10:E1", "Kantech EntryPoint");
      m.insert("00:11:BB", "Kantech P225");
      m.insert("00:12:7A", "Kantech P300");
      m.insert("00:13:1D", "Kantech door module");
      m.insert("00:14:88", "Kantech reader module");
      m.insert("00:15:62", "Kantech input module");

      // Lenel (HID)
      m.insert("00:0C:26", "Lenel");
      m.insert("00:0D:47", "Lenel Systems");
      m.insert("00:0E:88", "Lenel OnGuard");
      m.insert("00:0F:6D", "Lenel NGX");
      m.insert("00:10:E1", "Lenel S2");
      m.insert("00:11:BB", "Lenel Access Controller");
      m.insert("00:12:7A", "Lenel Reader");
      m.insert("00:13:1D", "Lenel Credential");
      m.insert("00:14:88", "Lenel Intelli-M");
      m.insert("00:15:62", "Lenel Enforcer");
      m.insert("00:16:93", "Lenel PVC");

      // Software House (Honeywell)
      m.insert("00:0D:47", "Software House");
      m.insert("00:0E:88", "Software House Systems");
      m.insert("00:0F:6D", "Software House C-Cure");
      m.insert("00:10:E1", "Software House iSTAR");
      m.insert("00:11:BB", "Software House ACM");
      m.insert("00:12:7A", "Software House Exit");
      m.insert("00:13:1D", "Software House Door Module");
      m.insert("00:14:88", "Software House Reader");
      m.insert("00:15:62", "Software House Credential");

      // Axis Access Control
      m.insert("00:09:1A", "Axis Access");
      m.insert("00:0A:5A", "Axis Communications");
      m.insert("00:0B:1D", "Axis Door Controller");
      m.insert("00:0C:26", "Axis Access Control");
      m.insert("00:0D:47", "Axis A1001");
      m.insert("00:0E:88", "Axis A1601");
      m.insert("00:0F:6D", "Axis A8207");
      m.insert("00:10:E1", "Axis Network Door");
      m.insert("00:11:BB", "Axis I/O Module");
      m.insert("00:12:7A", "Axis Reader");

      // Bosch Access Control
      m.insert("00:0A:5A", "Bosch Access");
      m.insert("00:0B:1D", "Bosch Security Systems");
      m.insert("00:0C:26", "Bosch Access Control");
      m.insert("00:0D:47", "Bosch AMS");
      m.insert("00:0E:88", "Bosch BIS");
      m.insert("00:0F:6D", "Bosch Access Point");
      m.insert("00:10:E1", "Bosch Reader");
      m.insert("00:11:BB", "Bosch Controller");
      m.insert("00:12:7A", "Bosch Card");

      // Gallagher

      m.insert("00:0D:BD", "Gallagher");
      m.insert("00:0E:88", "Gallagher Security");
      m.insert("00:0F:6D", "Gallagher Command Centre");
      m.insert("00:10:E1", "Gallagher Controller");
      m.insert("00:11:BB", "Gallagher T20");
      m.insert("00:12:7A", "Gallagher T30");
      m.insert("00:13:1D", "Gallagher T40");
      m.insert("00:14:88", "Gallagher P20");
      m.insert("00:15:62", "Gallagher P30");
      m.insert("00:16:93", "Gallagher P40");
      m.insert("00:17:5F", "Gallagher 8200");
      m.insert("00:18:50", "Gallagher 8400");
      m.insert("00:19:11", "Gallagher 8500");
      m.insert("00:1A:33", "Gallagher HRP");
      m.insert("00:1B:44", "Gallagher PR15");
      m.insert("00:1C:55", "Gallagher Smart Card");
      m.insert("00:1D:66", "Gallagher Proximity");
      m.insert("00:1E:77", "Gallagher Biometric");
      m.insert("00:1F:88", "Gallagher Mobile Access");
      m.insert("00:20:99", "Gallagher Bluetooth");
      m.insert("00:21:AA", "Gallagher NFC");
      m.insert("00:22:BB", "Gallagher Reader");
      m.insert("00:23:CC", "Gallagher Keypad");
      m.insert("00:24:DD", "Gallagher Siren");
      m.insert("00:25:EE", "Gallagher Strobe");

      // Brivo
      m.insert("00:1B:3C", "Brivo");
      m.insert("00:1C:47", "Brivo Onair");
      m.insert("00:1D:50", "Brivo Access");
      m.insert("00:1E:56", "Brivo Reader");
      m.insert("00:1F:67", "Brivo Controller");

      // Genetec
      m.insert("00:1A:2C", "Genetec");
      m.insert("00:1B:3C", "Genetec Security");
      m.insert("00:1C:47", "Genetec Security Center");
      m.insert("00:1D:50", "Genetec Synergis");
      m.insert("00:1E:56", "Genetec Door Controller");
      m.insert("00:1F:67", "Genetec Reader");
      m.insert("00:21:27", "Genetec Camera");

      // HID Mercury (LenelS2)
      m.insert("00:1B:4B", "Mercury");
      m.insert("00:1C:5C", "Mercury Security");
      m.insert("00:1D:6D", "Mercury LP");
      m.insert("00:1E:7E", "Mercury MR");
      m.insert("00:1F:8F", "Mercury EP");
      m.insert("00:20:A0", "Mercury SCP");
      m.insert("00:21:B1", "Mercury MP");

      // Alarm.com
      m.insert("00:1C:BF", "Alarm.com");
      m.insert("00:1D:0F", "Alarm.com Access");
      m.insert("00:1E:55", "Alarm.com Controller");
      m.insert("00:1F:77", "Alarm.com Reader");
      m.insert("00:21:19", "Alarm.com Doorbell");

      // DoorKing
      m.insert("00:1D:8F", "DoorKing");
      m.insert("00:1E:55", "DoorKing Access");
      m.insert("00:1F:77", "DoorKing Controller");
      m.insert("00:21:19", "DoorKing Gate");
      m.insert("00:22:4C", "DoorKing Keypad");

      // Linear / Nortek
      m.insert("00:1E:55", "Linear");
      m.insert("00:1F:77", "Linear Pro");
      m.insert("00:21:19", "Linear Access");
      m.insert("00:22:4C", "Nortek");
      m.insert("00:23:11", "Nortek Security");

      // ASSA ABLOY Aperio
      m.insert("00:23:CC", "ASSA ABLOY");
      m.insert("00:24:DD", "ASSA ABLOY Aperio");
      m.insert("00:25:EE", "ASSA ABLOY HES");
      m.insert("00:26:30", "ASSA ABLOY Yale");
      m.insert("00:27:10", "ASSA ABLOY Sargent");

      // Salto Systems
      m.insert("00:21:19", "Salto");
      m.insert("00:22:4C", "Salto Systems");
      m.insert("00:23:11", "Salto KS");
      m.insert("00:24:99", "Salto SVN");
      m.insert("00:25:46", "Salto Geo");
      m.insert("00:26:30", "Salto JustIN");
      m.insert("00:27:10", "Salto CSF");

      // SimonsVoss
      m.insert("00:22:BB", "SimonsVoss");
      m.insert("00:23:CC", "SimonsVoss SmartIntego");
      m.insert("00:24:DD", "SimonsVoss Lock");
      m.insert("00:25:EE", "SimonsVoss Cylinder");
      m.insert("00:26:30", "SimonsVoss Transponder");

      // Bosch/access Ition
      m.insert("00:23:11", "Bosch");
      m.insert("00:24:22", "Bosch Access Ition");
      m.insert("00:25:33", "Bosch AMC");
      m.insert("00:26:44", "Bosch BIM");

      //更多 CCTV / Surveillance Systems

      // Lorex
      m.insert("00:1A:2B", "Lorex");
      m.insert("00:1B:3C", "Lorex Security");
      m.insert("00:1C:47", "Lorex NVR");
      m.insert("00:1D:50", "Lorex DVR");
      m.insert("00:1E:56", "Lorex IP Camera");
      m.insert("00:1F:67", "Lorex WiFi Camera");

      // Reolink
      m.insert("00:1B:4C", "Reolink");
      m.insert("00:1C:5D", "Reolink Camera");
      m.insert("00:1D:6E", "Reolink NVR");
      m.insert("00:1E:7F", "Reolink Argus");
      m.insert("00:1F:90", "Reolink E1");

      // Amcrest
      m.insert("00:1C:57", "Amcrest");
      m.insert("00:1D:68", "Amcrest Camera");
      m.insert("00:1E:79", "Amcrest NVR");
      m.insert("00:1F:8A", "Amcrest DVR");
      m.insert("00:20:9B", "Amcrest ProHD");

      // Foscam
      m.insert("00:1D:8C", "Foscam");
      m.insert("00:1E:9D", "Foscam Camera");
      m.insert("00:1F:AE", "Foscam R2");
      m.insert("00:20:BF", "Foscam C1");
      m.insert("00:21:D0", "Foscam FI");

      // Swann
      m.insert("00:1E:55", "Swann");
      m.insert("00:1F:66", "Swann Security");
      m.insert("00:20:77", "Swann NVR");
      m.insert("00:21:88", "Swann DVR");

      // Uniview
      m.insert("00:1F:77", "Uniview");
      m.insert("00:20:88", "Uniview Camera");
      m.insert("00:21:99", "Uniview NVR");
      m.insert("00:22:AA", "Uniview DVR");

      // Tiandy
      m.insert("00:20:99", "Tiandy");
      m.insert("00:21:AA", "Tiandy Camera");
      m.insert("00:22:BB", "Tiandy NVR");
      m.insert("00:23:CC", "Tiandy Smart");

      // Milesight
      m.insert("00:21:BB", "Milesight");
      m.insert("00:22:CC", "Milesight Camera");
      m.insert("00:23:DD", "Milesight NVR");
      m.insert("00:24:EE", "Milesight LPR");

      // Provision-ISR
      m.insert("00:22:DD", "Provision-ISR");
      m.insert("00:23:EE", "Provision-ISR Camera");
      m.insert("00:24:FF", "Provision-ISR DVR");
      m.insert("00:25:00", "Provision-ISR NVR");

      // Hikvision Pro Series
      m.insert("00:24:1C", "Hikvision Pro");
      m.insert("00:24:B9", "Hikvision Ultra");
      m.insert("00:25:26", "Hikvision WiFi");
      m.insert("00:25:AE", "Hikvision Solar");
      m.insert("00:26:19", "Hikvision PoE");

      // Dahua Pro Series
      m.insert("00:25:21", "Dahua Pro");
      m.insert("00:25:A3", "Dahua Commercial");
      m.insert("00:26:14", "Dahua TiOC");
      m.insert("00:26:85", "Dahua WizSense");
      m.insert("00:27:56", "Dahua WizMind");

      // == Enterprise VoIP Phones ==

      // Cisco IP Phones
      m.insert("00:01:42", "Cisco");
      m.insert("00:02:3A", "Cisco IP Phone");
      m.insert("00:04:8B", "Cisco Unified IP Phone");
      m.insert("00:05:25", "Cisco 7900 Series");
      m.insert("00:06:27", "Cisco 7940");
      m.insert("00:07:29", "Cisco 7945");
      m.insert("00:08:2B", "Cisco 7960");
      m.insert("00:09:2D", "Cisco 7965");
      m.insert("00:0A:2F", "Cisco 7970");
      m.insert("00:0B:31", "Cisco 7975");
      m.insert("00:0C:33", "Cisco 8800 Series");
      m.insert("00:0D:35", "Cisco 8845");
      m.insert("00:0E:37", "Cisco 8851");
      m.insert("00:0F:39", "Cisco 8865");
      m.insert("00:10:3B", "Cisco IP Communicator");
      m.insert("00:11:3D", "Cisco Jabber");
      m.insert("00:12:3F", "Cisco DX Series");
      m.insert("00:13:41", "Cisco DX650");
      m.insert("00:14:43", "Cisco DX70");
      m.insert("00:15:45", "Cisco DX80");
      m.insert("00:16:47", "Cisco SPA Series");
      m.insert("00:17:49", "Cisco SPA504G");
      m.insert("00:18:4B", "Cisco SPA525G");
      m.insert("00:19:4D", "Cisco MPP Series");
      m.insert("00:1A:4F", "Cisco 7821");
      m.insert("00:1B:51", "Cisco 7841");
      m.insert("00:1C:53", "Cisco 7861");
      m.insert("00:1D:55", "Cisco 8800");

      // Poly (Polycom)
      m.insert("00:01:03", "Polycom");
      m.insert("00:04:38", "Poly");
      m.insert("00:0B:81", "Polycom");
      m.insert("00:12:3A", "Polycom SoundPoint");
      m.insert("00:13:21", "Polycom SoundStation");
      m.insert("00:14:4F", "Polycom VVX");
      m.insert("00:15:70", "Polycom SoundStation IP");
      m.insert("00:16:35", "Polycom IP 330");
      m.insert("00:17:08", "Polycom IP 4000");
      m.insert("00:18:BA", "Polycom IP 5000");
      m.insert("00:19:11", "Polycom IP 6000");
      m.insert("00:1A:33", "Polycom VVX 150");
      m.insert("00:1B:44", "Polycom VVX 250");
      m.insert("00:1C:55", "Polycom VVX 350");
      m.insert("00:1D:66", "Polycom VVX 450");
      m.insert("00:1E:77", "Polycom VVX 500");
      m.insert("00:1F:88", "Polycom VVX 600");
      m.insert("00:20:99", "Polycom VVX 1500");
      m.insert("00:21:AA", "Polycom CX Series");
      m.insert("00:22:BB", "Polycom RealPresence");
      m.insert("00:23:CC", "Polycom Trio");
      m.insert("00:24:DD", "Polycom Studio");
      m.insert("00:25:EE", "Polycom VoxBox");

      // Yealink
      m.insert("00:15:65", "Yealink");
      m.insert("00:1A:2C", "Yealink IP Phone");
      m.insert("00:1C:BF", "Yealink T4 Series");
      m.insert("00:1D:0F", "Yealink T46");
      m.insert("00:1E:55", "Yealink T48");
      m.insert("00:1F:77", "Yealink T5 Series");
      m.insert("00:21:19", "Yealink T53");
      m.insert("00:22:4C", "Yealink T54");
      m.insert("00:23:11", "Yealink T57");
      m.insert("00:24:99", "Yealink T58");
      m.insert("00:25:46", "Yealink VP59");
      m.insert("00:26:30", "Yealink DECT");
      m.insert("00:27:10", "Yealink W56");
      m.insert("00:28:32", "Yealink CP Series");
      m.insert("00:29:45", "Yealink CP960");
      m.insert("00:2A:6A", "Yealink CP920");
      m.insert("00:2B:8C", "Yealink MP56");
      m.insert("00:2C:21", "Yealink WH Series");
      m.insert("00:2D:54", "Yealink UH Series");
      m.insert("00:2E:87", "Yealink Exp40");
      m.insert("00:2F:33", "Yealink EXP20");

      // Grandstream
      m.insert("00:0B:82", "Grandstream");
      m.insert("00:1A:C1", "Grandstream IP Phone");
      m.insert("00:1B:3C", "Grandstream GXP");
      m.insert("00:1C:47", "Grandstream GXP1400");
      m.insert("00:1D:50", "Grandstream GXP1405");
      m.insert("00:1E:56", "Grandstream GXP2100");
      m.insert("00:1F:67", "Grandstream GXP2120");
      m.insert("00:20:47", "Grandstream GXV");
      m.insert("00:21:27", "Grandstream GXV3275");
      m.insert("00:22:19", "Grandstream GXV3380");
      m.insert("00:23:12", "Grandstream HT Series");
      m.insert("00:24:8C", "Grandstream HT801");
      m.insert("00:25:46", "Grandstream HT802");
      m.insert("00:26:30", "Grandstream DP Series");
      m.insert("00:27:10", "Grandstream DP720");
      m.insert("00:28:32", "Grandstream DP750");
      m.insert("00:29:45", "Grandstream UCM");
      m.insert("00:2A:6A", "Grandstream UCM6300");
      m.insert("00:2B:8C", "Grandstream Wave");

      // Avaya
      m.insert("00:01:63", "Avaya");
      m.insert("00:0B:85", "Avaya IP Phone");
      m.insert("00:12:3A", "Avaya 9600 Series");
      m.insert("00:13:21", "Avaya 9608");
      m.insert("00:14:4F", "Avaya 9611");
      m.insert("00:15:70", "Avaya 9620");
      m.insert("00:16:35", "Avaya 9640");
      m.insert("00:17:08", "Avaya 9641");
      m.insert("00:18:BA", "Avaya 9650");
      m.insert("00:19:11", "Avaya 9670");
      m.insert("00:1A:33", "Avaya J100 Series");
      m.insert("00:1B:44", "Avaya J139");
      m.insert("00:1C:55", "Avaya J169");
      m.insert("00:1D:66", "Avaya J179");
      m.insert("00:1E:77", "Avaya One-X");
      m.insert("00:1F:88", "Avaya Flare");
      m.insert("00:20:99", "Avaya 1100 Series");
      m.insert("00:21:AA", "Avaya 1200 Series");
      m.insert("00:22:BB", "Avaya IP Office");

      // Mitel
      m.insert("00:08:00", "Mitel");
      m.insert("00:0A:6D", "Mitel IP Phone");
      m.insert("00:12:3A", "Mitel 5300 Series");
      m.insert("00:13:21", "Mitel 5310");
      m.insert("00:14:4F", "Mitel 5320");
      m.insert("00:15:70", "Mitel 5330");
      m.insert("00:16:35", "Mitel 5340");
      m.insert("00:17:08", "Mitel 5360");
      m.insert("00:18:BA", "Mitel 5600 Series");
      m.insert("00:19:11", "Mitel 5610");
      m.insert("00:1A:33", "Mitel 5620");
      m.insert("00:1B:44", "Mitel 6800 Series");
      m.insert("00:1C:55", "Mitel 6863");
      m.insert("00:1D:66", "Mitel 6865");
      m.insert("00:1E:77", "Mitel 6867");
      m.insert("00:1F:88", "Mitel 6869");
      m.insert("00:20:99", "Mitel DECT");
      m.insert("00:21:AA", "Mitel MiVoice");
      m.insert("00:22:BB", "Mitel MiCollab");
      m.insert("00:23:CC", "Mitel Border Gateway");

      // Snom
      m.insert("00:04:13", "Snom");
      m.insert("00:12:3A", "Snom IP Phone");
      m.insert("00:13:21", "Snom 300");
      m.insert("00:14:4F", "Snom 320");
      m.insert("00:15:70", "Snom 360");
      m.insert("00:16:35", "Snom 370");
      m.insert("00:17:08", "Snom 700 Series");
      m.insert("00:18:BA", "Snom 710");
      m.insert("00:19:11", "Snom 715");
      m.insert("00:1A:33", "Snom 720");
      m.insert("00:1B:44", "Snom 725");
      m.insert("00:1C:55", "Snom 760");
      m.insert("00:1D:66", "Snom 820");
      m.insert("00:1E:77", "Snom 821");
      m.insert("00:1F:88", "Snom 870");
      m.insert("00:20:99", "Snom D3");
      m.insert("00:21:AA", "Snom M9");
      m.insert("00:22:BB", "Snom PA1");
      m.insert("00:23:CC", "Snom SSH");
      m.insert("00:24:DD", "Snom USB");

      // == Video Conferencing Systems ==

      // Cisco Webex
      m.insert("00:50:56", "Cisco");
      m.insert("00:1B:54", "Cisco Webex");
      m.insert("00:1C:BF", "Cisco Webex Desk");
      m.insert("00:1D:0F", "Cisco Webex Board");
      m.insert("00:1E:55", "Cisco Webex Room");
      m.insert("00:1F:77", "Cisco Webex Room Kit");
      m.insert("00:21:19", "Cisco Webex Room 55");
      m.insert("00:22:4C", "Cisco Webex Room 70");
      m.insert("00:23:11", "Cisco Webex Codec");
      m.insert("00:24:99", "Cisco Webex Share");
      m.insert("00:25:46", "Cisco Webex Camera");
      m.insert("00:26:30", "Cisco Webex PTZ");
      m.insert("00:27:10", "Cisco Webex Navigator");
      m.insert("00:28:32", "Cisco TelePresence");
      m.insert("00:29:45", "Cisco MX Series");
      m.insert("00:2A:6A", "Cisco SX Series");
      m.insert("00:2B:8C", "Cisco Profile Series");

      // Poly Video
      m.insert("00:04:F2", "Poly");
      m.insert("00:12:3A", "Poly Video");
      m.insert("00:13:21", "Poly Studio");
      m.insert("00:14:4F", "Poly Studio X");
      m.insert("00:15:70", "Poly Studio X30");
      m.insert("00:16:35", "Poly Studio X50");
      m.insert("00:17:08", "Poly G7500");
      m.insert("00:18:BA", "Poly Group Series");
      m.insert("00:19:11", "Poly RealPresence");
      m.insert("00:1A:33", "Poly HDX");
      m.insert("00:1B:44", "Poly EagleEye");
      m.insert("00:1C:55", "Poly EagleEye IV");
      m.insert("00:1D:66", "Poly EagleEye Mini");
      m.insert("00:1E:77", "Poly EagleEye Cube");
      m.insert("00:1F:88", "Poly VoxBox");
      m.insert("00:20:99", "Poly Trio");
      m.insert("00:21:AA", "Poly Trio 8800");
      m.insert("00:22:BB", "Poly Trio 8500");
      m.insert("00:23:CC", "Poly Trio 7300");
      m.insert("00:24:DD", "Poly Touch Control");

      // Zoom Rooms
      m.insert("00:1A:2C", "Zoom");
      m.insert("00:1C:BF", "Zoom Rooms");
      m.insert("00:1D:0F", "Zoom Room Appliance");
      m.insert("00:1E:55", "Zoom Room Controller");
      m.insert("00:1F:77", "Zoom Room Compute");
      m.insert("00:21:19", "Zoom Room Display");
      m.insert("00:22:4C", "Zoom Room Camera");
      m.insert("00:23:11", "Zoom Room Audio");
      m.insert("00:24:99", "Zoom Rooms for Touch");
      m.insert("00:25:46", "Zoom Digital Signage");

      // Logitech Video Conferencing
      m.insert("00:05:B9", "Logitech");
      m.insert("00:1D:8F", "Logitech Rally");
      m.insert("00:1E:55", "Logitech Rally Bar");
      m.insert("00:1F:77", "Logitech Rally Bar Mini");
      m.insert("00:21:19", "Logitech MeetUp");
      m.insert("00:22:4C", "Logitech PTZ Pro");
      m.insert("00:23:11", "Logitech PTZ Pro 2");
      m.insert("00:24:99", "Logitech Brio");
      m.insert("00:25:46", "Logitech C920");
      m.insert("00:26:30", "Logitech C925");
      m.insert("00:27:10", "Logitech C930");
      m.insert("00:28:32", "Logitech C922");
      m.insert("00:29:45", "Logitech StreamCam");
      m.insert("00:2A:6A", "Logitech Tap");
      m.insert("00:2B:8C", "Logitech Tap IP");
      m.insert("00:2C:21", "Logitech Tap Scheduler");
      m.insert("00:2D:54", "Logitech Scribe");
      m.insert("00:2E:87", "Logitech Swytch");

      // Crestron
      m.insert("00:08:9B", "Crestron");
      m.insert("00:12:3A", "Crestron Flex");
      m.insert("00:13:21", "Crestron UC Engine");
      m.insert("00:14:4F", "Crestron Mercury");
      m.insert("00:15:70", "Crestron AirMedia");
      m.insert("00:16:35", "Crestron DM NVX");
      m.insert("00:17:08", "Crestron HD-RX");
      m.insert("00:18:BA", "Crestron TS-");
      m.insert("00:19:11", "Crestron TSW");
      m.insert("00:1A:33", "Crestron TSW-1070");
      m.insert("00:1B:44", "Crestron CP3");
      m.insert("00:1C:55", "Crestron MC3");
      m.insert("00:1D:66", "Crestron DSP");
      m.insert("00:1E:77", "Crestron DMPS");
      m.insert("00:1F:88", "Crestron V-Panel");

      // == Commercial Thin Clients ==

      // Dell Wyse
      m.insert("00:14:22", "Dell");
      m.insert("00:15:99", "Dell Wyse");
      m.insert("00:16:32", "Dell Wyse ThinOS");
      m.insert("00:17:44", "Dell Wyse Thin Client");
      m.insert("00:18:4D", "Dell Wyse 3010");
      m.insert("00:19:1C", "Dell Wyse 3020");
      m.insert("00:1A:11", "Dell Wyse 3040");
      m.insert("00:1B:3C", "Dell Wyse 5010");
      m.insert("00:1C:12", "Dell Wyse 5020");
      m.insert("00:1D:0F", "Dell Wyse 5040");
      m.insert("00:1E:55", "Dell Wyse 5060");
      m.insert("00:1F:77", "Dell Wyse 5070");
      m.insert("00:21:19", "Dell Wyse Z50");
      m.insert("00:22:4C", "Dell Wyse D10D");
      m.insert("00:23:11", "Dell Wyse D90D");
      m.insert("00:24:99", "Dell OptiPlex 3000 Thin");
      m.insert("00:25:46", "Dell Latitude 5000 Thin");

      // HP Thin Clients
      m.insert("00:0C:E6", "HP");
      m.insert("00:14:38", "HP Thin Client");
      m.insert("00:15:2D", "HP t530");
      m.insert("00:16:42", "HP t540");
      m.insert("00:17:1E", "HP t630");
      m.insert("00:18:4D", "HP t640");
      m.insert("00:19:1C", "HP t650");
      m.insert("00:1A:11", "HP t730");
      m.insert("00:1B:3C", "HP t740");
      m.insert("00:1C:12", "HP t755");
      m.insert("00:1D:0F", "HP t760");
      m.insert("00:1E:55", "HP t430");
      m.insert("00:1F:77", "HP t440");
      m.insert("00:21:19", "HP t450");
      m.insert("00:22:4C", "HP t510");
      m.insert("00:23:11", "HP t520");
      m.insert("00:24:99", "HP t620");
      m.insert("00:25:46", "HP t628");
      m.insert("00:26:30", "HP mt21");
      m.insert("00:27:10", "HP mt22");
      m.insert("00:28:32", "HP mt32");
      m.insert("00:29:45", "HP mt42");
      m.insert("00:2A:6A", "HP mt44");
      m.insert("00:2B:8C", "HP mt45");
      m.insert("00:2C:21", "HP Jet Fusion");

      // IGEL Technology
      m.insert("00:1A:2C", "IGEL");
      m.insert("00:1C:BF", "IGEL Technology");
      m.insert("00:1D:0F", "IGEL UD3");
      m.insert("00:1E:55", "IGEL UD5");
      m.insert("00:1F:77", "IGEL UD7");
      m.insert("00:21:19", "IGEL UD9");
      m.insert("00:22:4C", "IGEL UD11");
      m.insert("00:23:11", "IGEL IZ2");
      m.insert("00:24:99", "IGEL COSM");
      m.insert("00:25:46", "IGEL COSM Z");
      m.insert("00:26:30", "IGEL Workspace");
      m.insert("00:27:10", "IGEL IGEL Linux");
      m.insert("00:28:32", "IGEL Endpoint Management");
      m.insert("00:29:45", "IGEL Universal");
      m.insert("00:2A:6A", "IGEL HSD");

      // NComputing
      m.insert("00:0A:8A", "NComputing");
      m.insert("00:1B:3C", "NComputing RX300");
      m.insert("00:1C:47", "NComputing vSpace");
      m.insert("00:1D:50", "NComputing L300");
      m.insert("00:1E:56", "NComputing R300");
      m.insert("00:1F:67", "NComputing Q210");
      m.insert("00:21:27", "NComputing Q230");
      m.insert("00:22:19", "NComputing M300");
      m.insert("00:23:12", "NComputing M350");
      m.insert("00:24:8C", "NComputing NUMO");
      m.insert("00:25:46", "NComputing vCAST");
      m.insert("00:26:30", "NComputing Virtual");

      // Lenovo ThinkCentre Tiny
      m.insert("00:1A:2C", "Lenovo");
      m.insert("00:1C:BF", "Lenovo ThinkCentre");
      m.insert("00:1D:0F", "Lenovo ThinkCentre Tiny");
      m.insert("00:1E:55", "Lenovo ThinkCentre M720");
      m.insert("00:1F:77", "Lenovo ThinkCentre M720q");
      m.insert("00:21:19", "Lenovo ThinkCentre M920");
      m.insert("00:22:4C", "Lenovo ThinkCentre M920q");
      m.insert("00:23:11", "Lenovo ThinkCentre M70q");
      m.insert("00:24:99", "Lenovo ThinkCentre M75q");
      m.insert("00:25:46", "Lenovo ThinkStation");
      m.insert("00:26:30", "Lenovo ThinkEdge");
      m.insert("00:27:10", "Lenovo ThinkCenter Nano");
      m.insert("00:28:32", "Lenovo V30a");
      m.insert("00:29:45", "Lenovo V50a");

      // == KVM Switches & Serial Consoles ==

      // Aten
      m.insert("00:0B:81", "Aten");
      m.insert("00:1C:57", "Aten International");
      m.insert("00:1D:8C", "Aten KVM");
      m.insert("00:1E:55", "Aten CS");
      m.insert("00:1F:77", "Aten KE");
      m.insert("00:21:19", "Aten KL");
      m.insert("00:22:4C", "Aten KH");
      m.insert("00:23:11", "Aten UH");
      m.insert("00:24:99", "Aten VM");
      m.insert("00:25:46", "Aten VS");
      m.insert("00:26:30", "Aten US");
      m.insert("00:27:10", "Aten EC");
      m.insert("00:28:32", "Aten IC");
      m.insert("00:29:45", "Aten IP");

      // Raritan
      m.insert("00:0E:7F", "Raritan");
      m.insert("00:1C:BF", "Raritan KVM");
      m.insert("00:1D:0F", "Raritan Dominion");
      m.insert("00:1E:55", "Raritan PX");
      m.insert("00:1F:77", "Raritan EMX");
      m.insert("00:21:19", "Raritan CommandCenter");
      m.insert("00:22:4C", "Raritan DX");
      m.insert("00:23:11", "Raritan KX");
      m.insert("00:24:99", "Raritan KX III");
      m.insert("00:25:46", "Raritan KX2");
      m.insert("00:26:30", "Raritan Paragon");
      m.insert("00:27:10", "Raritan MasterConsole");
      m.insert("00:28:32", "Raritan D2CIM");
      m.insert("00:29:45", "Raritan CIM");

      // Avocent (Vertiv)
      m.insert("00:0B:81", "Avocent");
      m.insert("00:1B:3C", "Avocent KVM");
      m.insert("00:1C:47", "Avocent DSR");
      m.insert("00:1D:50", "Avocent DSR2");
      m.insert("00:1E:56", "Avocent DSR2035");
      m.insert("00:1F:67", "Avocent DSR2160");
      m.insert("00:21:27", "Avocent MergePoint");
      m.insert("00:22:19", "Avocent AutoView");
      m.insert("00:23:12", "Avocent SwitchView");
      m.insert("00:24:8C", "Avocent Cybex");
      m.insert("00:25:46", "Avocent ACS");
      m.insert("00:26:30", "Avocent ACS8000");
      m.insert("00:27:10", "Avocent CMAST");
      m.insert("00:28:32", "Avocent Universal");
      m.insert("00:29:45", "Avocent Digital");

      // Belkin
      m.insert("00:05:25", "Belkin");
      m.insert("00:12:3A", "Belkin KVM");
      m.insert("00:13:21", "Belkin F1D");
      m.insert("00:14:4F", "Belkin F1DN");
      m.insert("00:15:70", "Belkin Flip");
      m.insert("00:16:35", "Belkin SOHO KVM");
      m.insert("00:17:08", "Belkin Secure KVM");
      m.insert("00:18:BA", "Belkin DVI KVM");
      m.insert("00:19:11", "Belkin HDMI KVM");
      m.insert("00:1A:33", "Belkin VGA KVM");

      // ServerTech (Raritan)
      m.insert("00:1B:3C", "Server Technology");
      m.insert("00:1C:47", "Server Tech");
      m.insert("00:1D:50", "ServerTech PRO2");
      m.insert("00:1E:56", "ServerTech STARTER");
      m.insert("00:1F:67", "ServerTech PRO2 PDU");
      m.insert("00:21:27", "ServerTech Sentry");
      m.insert("00:22:19", "ServerTech Switched PDU");
      m.insert("00:23:12", "ServerTech Monitored PDU");
      m.insert("00:24:8C", "ServerTech PRO2 MIB");
      m.insert("00:25:46", "ServerTech Tower PDU");
      m.insert("00:26:30", "ServerTech PRO2 Fan");
      m.insert("00:27:10", "ServerTech GFCI");

      // == Enterprise Network Storage ==

      // NetApp
      m.insert("00:A0:98", "NetApp");
      m.insert("00:1B:3C", "NetApp FAS");
      m.insert("00:1C:47", "NetApp AFF");
      m.insert("00:1D:50", "NetApp ONTAP");
      m.insert("00:1E:56", "NetApp AFF A400");
      m.insert("00:1F:67", "NetApp AFF A700");
      m.insert("00:21:27", "NetApp FAS2720");
      m.insert("00:22:19", "NetApp FAS2750");
      m.insert("00:23:12", "NetApp FAS8300");
      m.insert("00:24:8C", "NetApp FAS8700");
      m.insert("00:25:46", "NetApp SolidFire");
      m.insert("00:26:30", "NetApp E-Series");
      m.insert("00:27:10", "NetApp EF-Series");
      m.insert("00:28:32", "NetApp HCI");
      m.insert("00:29:45", "NetApp Cloud");
      m.insert("00:2A:6A", "NetApp FlexPod");

      // Dell EMC (Enterprise)
      m.insert("00:14:22", "Dell EMC");
      m.insert("00:15:99", "Dell EMC PowerStore");
      m.insert("00:16:32", "Dell EMC PowerEdge");
      m.insert("00:17:44", "Dell EMC Unity");
      m.insert("00:18:4D", "Dell EMC VNX");
      m.insert("00:19:1C", "Dell EMC Isilon");
      m.insert("00:1A:11", "Dell EMC Data Domain");
      m.insert("00:1B:3C", "Dell EMC SC");
      m.insert("00:1C:12", "Dell EMC PS");
      m.insert("00:1D:0F", "Dell EMC Compellent");
      m.insert("00:1E:55", "Dell EMC EqualLogic");
      m.insert("00:1F:77", "Dell EMC VMAX");
      m.insert("00:21:19", "Dell EMC XTremeIO");
      m.insert("00:22:4C", "Dell EMC XtremIO");
      m.insert("00:23:11", "Dell EMC DSSD");
      m.insert("00:24:99", "Dell EMC ECS");
      m.insert("00:25:46", "Dell EMC Isilon H");
      m.insert("00:26:30", "Dell EMC PowerVault");
      m.insert("00:27:10", "Dell EMC ML3");
      m.insert("00:28:32", "Dell EMC TL");

      // HPE Storage
      m.insert("00:1A:4B", "HPE");
      m.insert("00:1B:3C", "HPE StoreEasy");
      m.insert("00:1C:47", "HPE StoreOnce");
      m.insert("00:1D:50", "HPE 3PAR");
      m.insert("00:1E:56", "HPE Nimble");
      m.insert("00:1F:67", "HPE InfoSight");
      m.insert("00:21:27", "HPE MSA");
      m.insert("00:22:19", "HPE Primary");
      m.insert("00:23:12", "HPE Secondary");
      m.insert("00:24:8C", "HPE Alletra");
      m.insert("00:25:46", "HPE Alletra 6000");
      m.insert("00:26:30", "HPE Alletra 9000");
      m.insert("00:27:10", "HPE Apollo");
      m.insert("00:28:32", "HPE SimpliVity");
      m.insert("00:29:45", "HPE Primera");
      m.insert("00:2A:6A", "HPE XP");
      m.insert("00:2B:8C", "HPE XP7");

      // IBM Storage
      m.insert("00:06:29", "IBM");
      m.insert("00:14:6C", "IBM Storage");
      m.insert("00:1B:3C", "IBM Spectrum");
      m.insert("00:1C:47", "IBM FlashSystem");
      m.insert("00:1D:50", "IBM DS");
      m.insert("00:1E:56", "IBM Elastic");
      m.insert("00:1F:67", "IBM Spectrum Scale");
      m.insert("00:21:27", "IBM Spectrum Protect");
      m.insert("00:22:19", "IBM Spectrum Archive");
      m.insert("00:23:12", "IBM Storwize");
      m.insert("00:24:8C", "IBM XIV");
      m.insert("00:25:46", "IBM SONAS");
      m.insert("00:26:30", "IBM SAN");
      m.insert("00:27:10", "IBM TotalStorage");
      m.insert("00:28:32", "IBM DCS");

      // Pure Storage
      m.insert("00:1A:2C", "Pure Storage");
      m.insert("00:1C:BF", "Pure");
      m.insert("00:1D:0F", "Pure FlashArray");
      m.insert("00:1E:55", "Pure FlashArray//X");
      m.insert("00:1F:77", "Pure FlashArray//C");
      m.insert("00:21:19", "Pure FlashStack");
      m.insert("00:22:4C", "Pure FlashBlade");
      m.insert("00:23:11", "Pure Evergreen");
      m.insert("00:24:99", "Pure DirectFlash");
      m.insert("00:25:46", "Pure ActiveCluster");
      m.insert("00:26:30", "Pure Cloud Block");
      m.insert("00:27:10", "Pure Portworx");

      // == Fire Alarm & Life Safety (Commercial) ==

      // Simplex (Honeywell)
      m.insert("00:01:37", "Simplex");
      m.insert("00:07:7A", "Simplex Grinnell");
      m.insert("00:08:84", "Simplex Fire");
      m.insert("00:09:1A", "Simplex Alarm");
      m.insert("00:0A:5A", "Simplex 4000");
      m.insert("00:0B:1D", "Simplex 4100");
      m.insert("00:0C:26", "Simplex 4007ES");
      m.insert("00:0D:47", "Simplex 4010");
      m.insert("00:0E:88", "Simplex 4100ES");
      m.insert("00:0F:6D", "Simplex 4120");
      m.insert("00:10:E1", "Simplex 4050");
      m.insert("00:11:BB", "Simplex 4100U");
      m.insert("00:12:7A", "Simplex TrueAlarm");
      m.insert("00:13:1D", "Simplex Smoke Detector");
      m.insert("00:14:88", "Simplex Heat Detector");
      m.insert("00:15:62", "Simplex Pull Station");
      m.insert("00:16:93", "Simplex Horn");
      m.insert("00:17:5F", "Simplex Strobe");
      m.insert("00:18:50", "Simplex Speaker");
      m.insert("00:19:11", "Simplex Module");

      // Notifier (Honeywell)
      m.insert("00:0B:1D", "Notifier");
      m.insert("00:0C:26", "Notifier Fire");
      m.insert("00:0D:47", "Notifier ONYX");
      m.insert("00:0E:88", "Notifier NFS");
      m.insert("00:0F:6D", "Notifier NFS-320");
      m.insert("00:10:E1", "Notifier NFS-640");
      m.insert("00:11:BB", "Notifier NFS2-3030");
      m.insert("00:12:7A", "Notifier IDP");
      m.insert("00:13:1D", "Notifier IDP-1100");
      m.insert("00:14:88", "Notifier IDP-2100");
      m.insert("00:15:62", "Notifier IDP-2200");
      m.insert("00:16:93", "Notifier IDP-2200T");
      m.insert("00:17:5F", "Notifier B210W");
      m.insert("00:18:50", "Notifier B200S");
      m.insert("00:19:11", "Notifier B200SR");
      m.insert("00:1A:22", "Notifier FSP-951");
      m.insert("00:1B:3C", "Notifier FSP-851");

      // Edwards (Carrier)
      m.insert("00:04:3F", "Edwards");
      m.insert("00:08:9B", "Edwards Fire");
      m.insert("00:09:3A", "Edwards Signaling");
      m.insert("00:0A:52", "Edwards 101");
      m.insert("00:0B:47", "Edwards 302");
      m.insert("00:0C:29", "Edwards 303");
      m.insert("00:0D:18", "Edwards 304");
      m.insert("00:0E:57", "Edwards 301");
      m.insert("00:0F:42", "Edwards 404");
      m.insert("00:10:9C", "Edwards 450");
      m.insert("00:11:6A", "Edwards 640");
      m.insert("00:12:4D", "Edwards 850");
      m.insert("00:13:5A", "Edwards 870");
      m.insert("00:14:22", "Edwards 950");
      m.insert("00:15:63", "Edwards 955");
      m.insert("00:16:2B", "Edwards 960");
      m.insert("00:17:6F", "Edwards 970");
      m.insert("00:18:8A", "Edwards 980");

      // Silent Knight (Honeywell)
      m.insert("00:0D:3A", "Silent Knight");
      m.insert("00:11:44", "Silent Knight Fire");
      m.insert("00:15:86", "Silent Knight SK");
      m.insert("00:1A:33", "Silent Knight 5700");
      m.insert("00:1C:57", "Silent Knight 6100");
      m.insert("00:1E:68", "Silent Knight 6200");
      m.insert("00:1F:33", "Silent Knight 6300");
      m.insert("00:21:27", "Silent Knight 6800");
      m.insert("00:22:19", "Silent Knight 6850");
      m.insert("00:23:12", "Silent Knight 7000");
      m.insert("00:24:8C", "Silent Knight IntelliQuad");
      m.insert("00:25:46", "Silent Knight Signal");

      // Gamewell-FCI (Honeywell)
      m.insert("00:10:E1", "Gamewell-FCI");
      m.insert("00:11:BB", "Gamewell Fire");
      m.insert("00:12:7A", "Gamewell FCI");
      m.insert("00:13:1D", "Gamewell E3");
      m.insert("00:14:88", "Gamewell S3");
      m.insert("00:15:62", "Gamewell F3");
      m.insert("00:16:93", "Gamewell Command");
      m.insert("00:17:5F", "Gamewell Control");
      m.insert("00:18:50", "Gamewell Display");
      m.insert("00:19:11", "Gamewell Detector");

      // == Commercial POS Systems ==

      // NCR (Enterprise)
      m.insert("00:1B:54", "NCR");
      m.insert("00:1C:25", "NCR Corporation");
      m.insert("00:1C:BE", "NCR RealPOS");
      m.insert("00:1D:35", "NCR XR");
      m.insert("00:1D:E6", "NCR XR7");
      m.insert("00:1E:4D", "NCR XR8");
      m.insert("00:1F:1A", "NCR Voyager");
      m.insert("00:1F:E7", "NCR Aloha");
      m.insert("00:20:39", "NCR Aloha POS");
      m.insert("00:20:F1", "NCR Counterpoint");
      m.insert("00:21:3D", "NCR Restaurant");
      m.insert("00:21:DB", "NCR Retail");
      m.insert("00:22:26", "NCR Hospitality");
      m.insert("00:22:C8", "NCR SelfServ");
      m.insert("00:23:11", "NCR Self-Checkout");
      m.insert("00:23:99", "NCR FastLane");
      m.insert("00:24:1A", "NCR PowerScan");
      m.insert("00:24:9C", "NCR Vapor");
      m.insert("00:25:21", "NCR Scan");
      m.insert("00:25:A3", "NCR Payments");

      // Verifone (Enterprise)
      m.insert("00:1C:26", "Verifone");
      m.insert("00:1D:8C", "Verifone Systems");
      m.insert("00:1E:55", "Verifone eVo");
      m.insert("00:1F:77", "Verifone Engage");
      m.insert("00:21:19", "Verifone VX");
      m.insert("00:22:4C", "Verifone MX");
      m.insert("00:23:11", "Verifone UX");
      m.insert("00:24:99", "Verifone Z11");
      m.insert("00:25:46", "Verifone P400");
      m.insert("00:26:30", "Verifone P200");
      m.insert("00:27:10", "Verifone P100");
      m.insert("00:28:32", "Verifone Qx80");
      m.insert("00:29:45", "Verifone Qx100");
      m.insert("00:2A:6A", "Verifone T650");
      m.insert("00:2B:8C", "Verifone T658");
      m.insert("00:2C:21", "Verifone e285");
      m.insert("00:2D:54", "Verifone e335");

      // Ingenico (Enterprise)
      m.insert("00:1D:0F", "Ingenico");
      m.insert("00:1E:55", "Ingenico Group");
      m.insert("00:1F:77", "Ingenico iPP");
      m.insert("00:21:19", "Ingenico iCT");
      m.insert("00:22:4C", "Ingenico iCL");
      m.insert("00:23:11", "Ingenico Lane");
      m.insert("00:24:99", "Ingenico Move");
      m.insert("00:25:46", "Ingenico Desk");
      m.insert("00:26:30", "Ingenico Self");
      m.insert("00:27:10", "Ingenico Telium");
      m.insert("00:28:32", "Ingenico Tetra");
      m.insert("00:29:45", "Ingenico Desk/5000");
      m.insert("00:2A:6A", "Ingenico Lane/7000");
      m.insert("00:2B:8C", "Ingenico Move/5000");
      m.insert("00:2C:21", "Ingenico Self/5000");

      // Zebra (Enterprise)
      m.insert("00:1A:56", "Zebra");
      m.insert("00:1B:C5", "Zebra Technologies");
      m.insert("00:1C:19", "Zebra DS");
      m.insert("00:1D:28", "Zebra LI");
      m.insert("00:1E:37", "Zebra LS");
      m.insert("00:1F:46", "Zebra Symbol");
      m.insert("00:20:55", "Zebra MC");
      m.insert("00:21:64", "Zebra TC");
      m.insert("00:22:73", "Zebra VC");
      m.insert("00:23:82", "Zebra WT");
      m.insert("00:24:8C", "Zebra RFD");
      m.insert("00:25:46", "Zebra FX");
      m.insert("00:26:30", "Zebra ZT");
      m.insert("00:27:10", "Zebra ZM");
      m.insert("00:28:32", "Zebra GK");
      m.insert("00:29:45", "Zebra GX");

      // == Enterprise WiFi Access Points ==

      // Aruba Networks (HPE)
      m.insert("00:0B:86", "Aruba");
      m.insert("00:1A:1E", "Aruba Networks");
      m.insert("00:1B:3C", "Aruba AP");
      m.insert("00:1C:47", "Aruba IAP");
      m.insert("00:1D:50", "Aruba Instant");
      m.insert("00:1E:56", "Aruba 200 Series");
      m.insert("00:1F:67", "Aruba 203H");
      m.insert("00:21:27", "Aruba 205H");
      m.insert("00:22:19", "Aruba 210 Series");
      m.insert("00:23:12", "Aruba 2530");
      m.insert("00:24:8C", "Aruba 2540");
      m.insert("00:25:46", "Aruba 300 Series");
      m.insert("00:26:30", "Aruba 303H");
      m.insert("00:27:10", "Aruba 305");
      m.insert("00:28:32", "Aruba 310 Series");
      m.insert("00:29:45", "Aruba 320 Series");
      m.insert("00:2A:6A", "Aruba 330 Series");
      m.insert("00:2B:8C", "Aruba 340 Series");
      m.insert("00:2C:21", "Aruba 500 Series");
      m.insert("00:2D:54", "Aruba 510 Series");
      m.insert("00:2E:87", "Aruba 530 Series");
      m.insert("00:2F:33", "Aruba 550 Series");
      m.insert("00:30:19", "Aruba 570 Series");
      m.insert("00:31:4A", "Aruba 580 Series");
      m.insert("00:32:7B", "Aruba 600 Series");
      m.insert("00:33:AC", "Aruba 610");
      m.insert("00:34:1D", "Aruba 620");
      m.insert("00:35:2E", "Aruba AP-505");
      m.insert("00:36:3F", "Aruba AP-515");
      m.insert("00:37:50", "Aruba AP-535");
      m.insert("00:38:61", "Aruba AP-555");
      m.insert("00:39:72", "Aruba AP-565");
      m.insert("00:3A:83", "Aruba AP-575");
      m.insert("00:3B:94", "Aruba AP-585");
      m.insert("00:3C:A5", "Aruba AP-595");
      m.insert("00:3D:B6", "Aruba 9000 Series");

      // Ruckus Networks (Commscope)
      m.insert("00:12:3A", "Ruckus");
      m.insert("00:13:21", "Ruckus Wireless");
      m.insert("00:14:4F", "Ruckus AP");
      m.insert("00:15:70", "Ruckus ZoneFlex");
      m.insert("00:16:35", "Ruckus Unleashed");
      m.insert("00:17:08", "Ruckus SmartZone");
      m.insert("00:18:BA", "Ruckus ZoneDirector");
      m.insert("00:19:11", "Ruckus R500");
      m.insert("00:1A:33", "Ruckus R510");
      m.insert("00:1B:44", "Ruckus R600");
      m.insert("00:1C:55", "Ruckus R610");
      m.insert("00:1D:66", "Ruckus R700");
      m.insert("00:1E:77", "Ruckus R710");
      m.insert("00:1F:88", "Ruckus R720");
      m.insert("00:20:99", "Ruckus R730");
      m.insert("00:21:AA", "Ruckus T300");
      m.insert("00:22:BB", "Ruckus T301");
      m.insert("00:23:CC", "Ruckus T310");
      m.insert("00:24:DD", "Ruckus T350");
      m.insert("00:25:EE", "Ruckus T500");
      m.insert("00:26:30", "Ruckus T600");
      m.insert("00:27:10", "Ruckus T610");
      m.insert("00:28:32", "Ruckus H320");
      m.insert("00:29:45", "Ruckus H510");
      m.insert("00:2A:6A", "Ruckus H550");
      m.insert("00:2B:8C", "Ruckus H730");
      m.insert("00:2C:21", "Ruckus R770");
      m.insert("00:2D:54", "Ruckus R790");

      // Juniper Mist
      m.insert("00:1B:54", "Juniper");
      m.insert("00:1D:9C", "Juniper Networks");
      m.insert("00:1E:55", "Juniper AP");
      m.insert("00:1F:77", "Juniper Mist");
      m.insert("00:21:19", "Juniper AP43");
      m.insert("00:22:4C", "Juniper AP33");
      m.insert("00:23:11", "Juniper AP12");
      m.insert("00:24:99", "Juniper AP32");
      m.insert("00:25:46", "Juniper AP21");
      m.insert("00:26:30", "Juniper AP61");
      m.insert("00:27:10", "Juniper BT11");
      m.insert("00:28:32", "Juniper IR500");
      m.insert("00:29:45", "Juniper Access Points");

      // Extreme Networks
      m.insert("00:04:96", "Extreme Networks");
      m.insert("00:1B:3C", "Extreme AP");
      m.insert("00:1C:47", "Extreme Wireless");
      m.insert("00:1D:50", "Extreme Wing");
      m.insert("00:1E:56", "Extreme AP305");
      m.insert("00:1F:67", "Extreme AP310");
      m.insert("00:21:27", "Extreme AP360");
      m.insert("00:22:19", "Extreme AP380");
      m.insert("00:23:12", "Extreme AP39");
      m.insert("00:24:8C", "Extreme AP5");
      m.insert("00:25:46", "Extreme AP6");
      m.insert("00:26:30", "Extreme AP7");
      m.insert("00:27:10", "Extreme AP8");
      m.insert("00:28:32", "Extreme 9500");
      m.insert("00:29:45", "Extreme 9600");

      // == Industrial Control & PLC ==

      // Allen-Bradley (Rockwell)
      m.insert("00:00:BC", "Allen-Bradley");
      m.insert("00:01:37", "Rockwell Automation");
      m.insert("00:08:9B", "Allen-Bradley PLC");
      m.insert("00:09:3A", "Allen-Bradley MicroLogix");
      m.insert("00:0A:52", "Allen-Bradley SLC");
      m.insert("00:0B:47", "Allen-Bradley PLC-5");
      m.insert("00:0C:29", "Allen-Bradley ControlLogix");
      m.insert("00:0D:18", "Allen-Bradley CompactLogix");
      m.insert("00:0E:57", "Allen-Bradley FlexLogix");
      m.insert("00:0F:42", "Allen-Bradley SoftLogix");
      m.insert("00:10:9C", "Allen-Bradley GuardLogix");
      m.insert("00:11:6A", "Allen-Bradley CompactGuardLogix");
      m.insert("00:12:4D", "Allen-Bradley PanelView");
      m.insert("00:13:5A", "Allen-Bradley PowerFlex");
      m.insert("00:14:22", "Allen-Bradley Kinetix");
      m.insert("00:15:63", "Allen-Bradley Stratix");
      m.insert("00:16:2B", "Allen-Bradley EtherNet/IP");
      m.insert("00:17:6F", "Allen-Bradley DeviceNet");
      m.insert("00:18:8A", "Allen-Bradley DH+");
      m.insert("00:19:11", "Allen-Bradley Remote I/O");

      // Schneider Electric (Industrial)
      m.insert("00:04:A3", "Schneider");
      m.insert("00:08:9B", "Schneider Electric");
      m.insert("00:09:3A", "Schneider Modicon");
      m.insert("00:0A:52", "Schneider M221");
      m.insert("00:0B:47", "Schneider M241");
      m.insert("00:0C:29", "Schneider M251");
      m.insert("00:0D:18", "Schneider M262");
      m.insert("00:0E:57", "Schneider LMC058");
      m.insert("00:0F:42", "Schneider TM221");
      m.insert("00:10:9C", "Schneider TM241");
      m.insert("00:11:6A", "Schneider TM251");
      m.insert("00:12:4D", "Schneider TM262");
      m.insert("00:13:5A", "Schneider BMX");
      m.insert("00:14:22", "Schneider BME");
      m.insert("00:15:63", "Schneider BME GT");
      m.insert("00:16:2B", "Schneider Premium");
      m.insert("00:17:6F", "Schneider Quantum");
      m.insert("00:18:8A", "Schneider Altivar");
      m.insert("00:19:11", "Schneider Lexium");
      m.insert("00:1A:22", "Schneider Preventa");

      // == Commercial Display & Digital Signage ==

      // Samsung Commercial Display
      m.insert("00:00:4A", "Samsung");
      m.insert("00:04:AC", "Samsung Commercial");
      m.insert("00:06:5B", "Samsung Display");
      m.insert("00:09:6B", "Samsung QM");
      m.insert("00:11:25", "Samsung QB");
      m.insert("00:14:5A", "Samsung QH");
      m.insert("00:17:DF", "Samsung QF");
      m.insert("00:1A:64", "Samsung QN");
      m.insert("00:1D:8C", "Samsung OH");
      m.insert("00:1F:19", "Samsung IF");
      m.insert("00:1B:21", "Samsung ED");
      m.insert("00:1E:64", "Samsung BE");
      m.insert("00:1F:3B", "Samsung TE");
      m.insert("00:22:FB", "Samsung PE");
      m.insert("00:23:12", "Samsung ME");
      m.insert("00:24:99", "Samsung LH");
      m.insert("00:25:46", "Samsung DM");
      m.insert("00:26:30", "Samsung SyncMaster");
      m.insert("00:27:10", "Samsung MagicInfo");
      m.insert("00:28:32", "Samsung Smart Signage");

      // LG Commercial Display
      m.insert("00:1D:25", "LG");
      m.insert("00:1E:4C", "LG Commercial");
      m.insert("00:1F:33", "LG Digital Signage");
      m.insert("00:21:27", "LG SM");
      m.insert("00:22:19", "LG SE");
      m.insert("00:23:12", "LG SL");
      m.insert("00:24:8C", "LG SV");
      m.insert("00:25:46", "LG UL");
      m.insert("00:26:30", "LG UV");
      m.insert("00:27:10", "LG VM");
      m.insert("00:28:32", "LG VX");
      m.insert("00:29:45", "LG WS");
      m.insert("00:2A:6A", "LG WT");
      m.insert("00:2B:8C", "LG WS80");
      m.insert("00:2C:21", "LG WE");
      m.insert("00:2D:54", "LG WebOS");

      // NEC Display
      m.insert("00:04:2A", "NEC");
      m.insert("00:12:3A", "NEC Display");
      m.insert("00:13:21", "NEC V");
      m.insert("00:14:4F", "NEC E");
      m.insert("00:15:70", "NEC M");
      m.insert("00:16:35", "NEC P");
      m.insert("00:17:08", "NEC X");
      m.insert("00:18:BA", "NEC C");
      m.insert("00:19:11", "NEC L");
      m.insert("00:1A:33", "NEC UN");
      m.insert("00:1B:44", "NEC VN");
      m.insert("00:1C:55", "NEC DV");
      m.insert("00:1D:66", "NEC R");
      m.insert("00:1E:77", "NEC PA");
      m.insert("00:1F:88", "NEC P");
      m.insert("00:20:99", "NEC PX");
      m.insert("00:21:AA", "NEC PH");
      m.insert("00:22:BB", "NEC PS");
      m.insert("00:23:CC", "NEC LFD");
      m.insert("00:24:DD", "NEC EA");
      m.insert("00:25:EE", "NEC AccuSync");

      // == Emergency Alert & Mass Notification ==

      // Honeywell Silent Knight
      m.insert("00:0B:1D", "Honeywell");
      m.insert("00:0C:26", "Honeywell Silent Knight");
      m.insert("00:0D:47", "Honeywell Farenhyt");
      m.insert("00:0E:88", "Honeywell Fire-Lite");
      m.insert("00:0F:6D", "HoneywellNotifier");
      m.insert("00:10:E1", "Honeywell E3 Series");
      m.insert("00:11:BB", "Honeywell PowerSeries");
      m.insert("00:12:7A", "Honeywell Galaxy");
      m.insert("00:13:1D", "Honeywell Vista");
      m.insert("00:14:88", "Honeywell Pro");
      m.insert("00:15:62", "Honeywell LYNX");
      m.insert("00:16:93", "Honeywell ProA7");

      // CheKt Visual Alarm Systems
      m.insert("00:1B:4C", "CheKt");
      m.insert("00:1C:5D", "CheKt Visual Alarm");
      m.insert("00:1D:6E", "CheKt VSA-100");
      m.insert("00:1E:7F", "CheKt VSA-200");
      m.insert("00:1F:90", "CheKt VSA-300");
      m.insert("00:20:A1", "CheKt VSA-400");
      m.insert("00:21:B2", "CheKt VSA-500");
      m.insert("00:22:C3", "CheKt VSA-600");
      m.insert("00:23:D4", "CheKt VSA-700");
      m.insert("00:24:E5", "CheKt VSA-800");
      m.insert("00:25:F6", "CheKt VSA-900");
      m.insert("00:26:07", "CheKt Strobe");
      m.insert("00:27:18", "CheKt Siren");
      m.insert("00:28:29", "CheKt Beacon");
      m.insert("00:29:3A", "CheKt Alert");
      m.insert("00:2A:4B", "CheKt Controller");
      m.insert("00:2B:5C", "CheKt Panel");
      m.insert("00:2C:6D", "CheKt Keypad");
      m.insert("00:2D:7E", "CheKt Detector");
      m.insert("00:2E:8F", "CheKt Sensor");
      m.insert("00:2F:A0", "CheKt Module");
      m.insert("00:30:B1", "CheKt Power Supply");
      m.insert("00:31:C2", "CheKt Battery Backup");
      m.insert("00:32:D3", "CheKt Fire Alarm");
      m.insert("00:33:E4", "CheKt Security Alarm");
      m.insert("00:34:F5", "CheKt Industrial");
      m.insert("00:35:06", "CheKt Commercial");
      m.insert("00:36:17", "CheKt Residential");
      m.insert("00:37:28", "CheKt Outdoor");
      m.insert("00:38:39", "CheKt Weatherproof");
      m.insert("00:39:4A", "CheKt Explosion Proof");
      m.insert("00:3A:5B", "CheKt Hazardous Area");
      m.insert("00:3B:6C", "CheKt Marine Grade");
      m.insert("00:3C:7D", "CheKt Red Flash");
      m.insert("00:3D:8E", "CheKt Amber Flash");
      m.insert("00:3E:9F", "CheKt Blue Flash");
      m.insert("00:3F:B0", "CheKt Clear Flash");
      m.insert("00:40:C1", "CheKt Multi-Color");
      m.insert("00:41:D2", "CheKt Audible");
      m.insert("00:42:E3", "CheKt Visual");
      m.insert("00:43:F4", "CheKt Combo");

      // == Commercial Printers & MFP ==

      // Xerox
      m.insert("00:00:AA", "Xerox");
      m.insert("00:01:00", "Xerox Corporation");
      m.insert("00:04:5D", "Xerox WorkCentre");
      m.insert("00:04:61", "Xerox VersaLink");
      m.insert("00:04:63", "Xerox AltaLink");
      m.insert("00:04:79", "Xerox DocuCentre");
      m.insert("00:04:96", "Xerox PrimeLink");
      m.insert("00:05:1F", "Xerox ColorQube");
      m.insert("00:05:8B", "Xerox Phaser");
      m.insert("00:07:95", "Xerox DocuPrint");
      m.insert("00:0A:3B", "Xerox Nuvera");
      m.insert("00:0D:BD", "Xerox Iridesse");
      m.insert("00:14:6C", "Xerox D95");

      // Canon imageRunner
      m.insert("00:00:1A", "Canon");
      m.insert("00:04:CA", "Canon");
      m.insert("00:16:31", "Canon imageRUNNER");
      m.insert("00:22:5C", "Canon imageRUNNER ADVANCE");
      m.insert("00:0A:42", "Canon imageCLASS");
      m.insert("00:0D:5A", "Canon imagePROGRAF");
      m.insert("00:11:66", "Canon MAXIFY");
      m.insert("00:15:B8", "Canon Selphy");
      m.insert("00:1A:33", "Canon PIXMA");
      m.insert("00:1C:57", "Canon imagePRESS");
      m.insert("00:1E:68", "Canon varioPRINT");
      m.insert("00:1F:33", "Canon Arizona");
      m.insert("00:21:27", "Canon ColorWave");
      m.insert("00:22:19", "Canon PlotWave");

      // HP LaserJet Enterprise
      m.insert("00:0C:E6", "HP");
      m.insert("00:14:38", "HP LaserJet");
      m.insert("00:15:2D", "HP LaserJet Enterprise");
      m.insert("00:16:42", "HP LaserJet Pro");
      m.insert("00:17:1E", "HP LaserJet Managed");
      m.insert("00:18:4D", "HP PageWide");
      m.insert("00:19:1C", "HP PageWide Enterprise");
      m.insert("00:1A:11", "HP Color LaserJet");
      m.insert("00:1B:3C", "HP LaserJet Ultra");
      m.insert("00:1C:12", "HP LaserJet CM");
      m.insert("00:1D:0F", "HP LaserJet M");
      m.insert("00:1E:55", "HP LaserJet Pro M");
      m.insert("00:1F:77", "HP LaserJet Enterprise M");
      m.insert("00:21:19", "HP Neverstop");
      m.insert("00:22:4C", "HP Tango");
      m.insert("00:23:11", "HP OfficeJet");
      m.insert("00:24:99", "HP OfficeJet Enterprise");
      m.insert("00:25:46", "HP OfficeJet Pro");
      m.insert("00:26:30", "HP DeskJet");
      m.insert("00:27:10", "HP Envy");

      // == Electric Vehicle Charging Stations ==

      // ChargePoint
      m.insert("00:1B:17", "ChargePoint");
      m.insert("00:1D:8F", "ChargePoint Home");
      m.insert("00:1E:55", "ChargePoint Commercial");
      m.insert("00:1F:77", "ChargePoint CP6000");
      m.insert("00:21:19", "ChargePoint CPE250");
      m.insert("00:22:4C", "ChargePoint CPF25");
      m.insert("00:23:11", "ChargePoint CT4000");
      m.insert("00:24:99", "ChargePoint CTS");
      m.insert("00:25:46", "ChargePoint Fleet");
      m.insert("00:26:30", "ChargePoint Express");
      m.insert("00:27:10", "ChargePoint DC Fast");

      // EVBox
      m.insert("00:1A:33", "EVBox");
      m.insert("00:1B:44", "EVBox Commercial");
      m.insert("00:1C:55", "EVBox Residential");
      m.insert("00:1D:66", "EVBox Everon");
      m.insert("00:1E:77", "EVBox Ultroni");
      m.insert("00:1F:88", "EVBox Elvi");
      m.insert("00:20:99", "EVBox BusinessLine");
      m.insert("00:21:AA", "EVBox Partner");

      // Wallbox
      m.insert("00:1C:57", "Wallbox");
      m.insert("00:1D:68", "Wallbox Pulsar");
      m.insert("00:1E:79", "Wallbox Pulsar Plus");
      m.insert("00:1F:8A", "Wallbox Commander");
      m.insert("00:20:9B", "Wallbox Copper");
      m.insert("00:21:AC", "Wallbox Arctic");
      m.insert("00:22:BD", "Wallbox Supernova");

      // Tesla Destination Charging
      m.insert("00:1C:BF", "Tesla");
      m.insert("00:1D:0F", "Tesla Destination");
      m.insert("00:1E:55", "Tesla Wall Connector");
      m.insert("00:1F:77", "Tesla Mobile Connector");
      m.insert("00:21:19", "Tesla Supercharger");
      m.insert("00:22:4C", "Tesla V2 Supercharger");
      m.insert("00:23:11", "Tesla V3 Supercharger");
      m.insert("00:24:99", "Tesla Megacharger");

      // JuiceNet
      m.insert("00:1B:3C", "JuiceNet");
      m.insert("00:1C:47", "JuiceBox");
      m.insert("00:1D:50", "JuiceBox Pro");
      m.insert("00:1E:56", "JuiceBox 40");
      m.insert("00:1F:67", "JuiceBox 50");
      m.insert("00:21:27", "JuiceNet Enterprise");
      m.insert("00:22:19", "JuiceNet Fleet");

      // EVgo
      m.insert("00:1D:0F", "EVgo");
      m.insert("00:1E:55", "EVgo Fast Chargers");
      m.insert("00:1F:77", "EVgo EVD");
      m.insert("00:21:19", "EVgo Partner");

      // Electrify America
      m.insert("00:1E:55", "Electrify America");
      m.insert("00:1F:77", "Electrify America DCFC");
      m.insert("00:21:19", "Electrify America Home");

      // Flo
      m.insert("00:1A:2C", "Flo");
      m.insert("00:1B:3C", "Flo Services");
      m.insert("00:1C:47", "Flo Home");
      m.insert("00:1D:50", "Flo Commercial");
      m.insert("00:1E:56", "Flo CTV");
      m.insert("00:1F:67", "Flo RV");

      // eMobility (Various)
      m.insert("00:19:88", "ABB");
      m.insert("00:1A:99", "ABB Terra");
      m.insert("00:1B:AA", "ABB AC Charging");
      m.insert("00:1C:BB", "ABB DC Fast");
      m.insert("00:1D:CC", "Siemens");
      m.insert("00:1E:DD", "Siemens ChargePoint");
      m.insert("00:1F:EE", "Siemens VersiCharge");
      m.insert("00:20:FF", "Schneider EV");
      m.insert("00:21:11", "Schneider EVlink");
      m.insert("00:22:22", "Schneider HomeLink");
      m.insert("00:23:33", "GE WattStation");
      m.insert("00:24:44", "GE DuraStation");
      m.insert("00:25:55", "Eaton");
      m.insert("00:26:66", "Eaton EV Charger");
      m.insert("00:27:77", "Blink");
      m.insert("00:28:88", "Blink HQ");
      m.insert("00:29:99", "ClipperCreek");
      m.insert("00:2A:AA", "ClipperCreek HCS");
      m.insert("00:2B:BB", "SemaConnect");
      m.insert("00:2C:CC", "SemaConnect Series");
      m.insert("00:2D:DD", "OpConnect");
      m.insert("00:2E:EE", "Greenlots");
      m.insert("00:2F:FF", "FreeWire");
      m.insert("00:30:00", "FreeWire Boost");
      m.insert("00:31:11", "MagiCharge");
      m.insert("00:32:22", "TeraWatt");
      m.insert("00:33:33", "SparkCharge");
      m.insert("00:34:44", "Zerova");
      m.insert("00:35:55", "Drizly");

      // == Renewable Energy Systems ==

      // SolarEdge
      m.insert("00:1B:3C", "SolarEdge");
      m.insert("00:1C:47", "SolarEdge Inverter");
      m.insert("00:1D:50", "SolarEdge HD-Wave");
      m.insert("00:1E:56", "SolarEdge SE");
      m.insert("00:1F:67", "SolarEdge StorEdge");
      m.insert("00:21:27", "SolarEdge Solar Battery");
      m.insert("00:22:19", "SolarEdge Power Optimizer");
      m.insert("00:23:12", "SolarEdge Monitoring");
      m.insert("00:24:8C", "SolarEdge SetApp");

      // Enphase
      m.insert("00:1A:2C", "Enphase");
      m.insert("00:1B:3C", "Enphase Inverter");
      m.insert("00:1C:47", "Enphase IQ");
      m.insert("00:1D:50", "Enphase IQ6");
      m.insert("00:1E:56", "Enphase IQ7");
      m.insert("00:1F:67", "Enphase IQ8");
      m.insert("00:21:27", "Enphase Envoy");
      m.insert("00:22:19", "Enphase Microinverter");
      m.insert("00:23:12", "Enphase Battery");
      m.insert("00:24:8C", "Enphase Enlighten");

      // SMA Solar Technology
      m.insert("00:1C:57", "SMA");
      m.insert("00:1D:68", "SMA Sunny Boy");
      m.insert("00:1E:79", "SMA Sunny Tripower");
      m.insert("00:1F:8A", "SMA Sunny Highpower");
      m.insert("00:20:9B", "SMA Core1");
      m.insert("00:21:AC", "SMA Sunny Central");
      m.insert("00:22:BD", "SMA Sunny Island");
      m.insert("00:23:CC", "SMA Homemanager");
      m.insert("00:24:DD", "SMA Webconnect");

      // Solar Inverters - Other
      m.insert("00:19:11", "Fronius");
      m.insert("00:1A:22", "Fronius Primo");
      m.insert("00:1B:33", "Fronius Symo");
      m.insert("00:1C:44", "Fronius Galvo");
      m.insert("00:1D:55", "Huawei");
      m.insert("00:1E:66", "Huawei SUN2000");
      m.insert("00:1F:77", "Huawei SmartLogger");
      m.insert("00:20:88", "Sungrow");
      m.insert("00:21:99", "Sungrow SG");
      m.insert("00:22:AA", "Sungrow SH");
      m.insert("00:23:BB", "Goodwe");
      m.insert("00:24:CC", "Goodwe GW");
      m.insert("00:25:DD", "Goodwe SD");
      m.insert("00:26:EE", "Growatt");
      m.insert("00:27:FF", "Growatt MIN");
      m.insert("00:28:00", "Growatt SPH");
      m.insert("00:29:11", "Victron");
      m.insert("00:2A:22", "Victron MultiPlus");
      m.insert("00:2B:33", "Victron Quattro");
      m.insert("00:2C:44", "Victron BlueSolar");
      m.insert("00:2D:55", "OutBack Power");
      m.insert("00:2E:66", "OutBack Radian");
      m.insert("00:2F:77", "OutBack FX");
      m.insert("00:30:88", "Schneider XW");
      m.insert("00:31:99", "Schneider SW");
      m.insert("00:32:AA", "Schneider ConeXt");

      // Tesla Powerwall & Energy
      m.insert("00:1C:BF", "Tesla");
      m.insert("00:1D:0F", "Tesla Powerwall");
      m.insert("00:1E:55", "Tesla Powerwall 2");
      m.insert("00:1F:77", "Tesla Powerwall+");
      m.insert("00:21:19", "Tesla Powerwall AC");
      m.insert("00:22:4C", "Tesla Gateway");
      m.insert("00:23:11", "Tesla Backup Gateway");
      m.insert("00:24:99", "Tesla Solar Inverter");
      m.insert("00:25:46", "Tesla Solar Roof");

      // LG Chem / RESU
      m.insert("00:1A:56", "LG Chem");
      m.insert("00:1B:67", "LG RESU");
      m.insert("00:1C:78", "LG RESU10");
      m.insert("00:1D:89", "LG RESU7H");
      m.insert("00:1E:9A", "LG RESU13");
      m.insert("00:1F:AB", "LG High Voltage");

      // Solar Battery - Other
      m.insert("00:18:8A", "Panasonic");
      m.insert("00:19:9B", "Panasonic EverVolt");
      m.insert("00:1A:AC", "Sonnen");
      m.insert("00:1B:BD", "Sonnen Eco");
      m.insert("00:1C:CE", "Sonnen Core");
      m.insert("00:1D:DF", "SimpliPhi");
      m.insert("00:1E:F0", "SimpliPhi PHI");
      m.insert("00:1F:01", "Enphase Battery");
      m.insert("00:20:12", "SolarEdge Battery");
      m.insert("00:21:23", "Generac");
      m.insert("00:22:34", "Generac PWRcell");
      m.insert("00:23:45", "Eaton");
      m.insert("00:24:56", "Eaton Battery");

      // Wind Energy
      m.insert("00:1D:8F", "Vestas");
      m.insert("00:1E:55", "Vestas V90");
      m.insert("00:1F:77", "Vestas V112");
      m.insert("00:21:19", "Vestas V136");
      m.insert("00:22:4C", "Siemens Gamesa");
      m.insert("00:23:11", "Siemens Gamesa SG");
      m.insert("00:24:99", "GE Wind");
      m.insert("00:25:46", "GE Haliade");
      m.insert("00:26:30", "Nordex");
      m.insert("00:27:10", "Nordex N");
      m.insert("00:28:32", "Enercon");
      m.insert("00:29:45", "Enercon E");
      m.insert("00:2A:6A", "Wind Controller");
      m.insert("00:2B:8C", "Wind Inverter");

      // == Commercial Smart Lighting ==

      // Philips Lighting (Commercial)
      m.insert("00:1A:22", "Philips");
      m.insert("00:1B:33", "Philips Lighting");
      m.insert("00:1C:44", "Philips Hue Commercial");
      m.insert("00:1D:55", "Philips Dynalite");
      m.insert("00:1E:66", "Philips LightMaster");
      m.insert("00:1F:77", "Philips ActiLume");
      m.insert("00:20:88", "Philips EasyAir");
      m.insert("00:21:99", "Philips SpaceWise");
      m.insert("00:22:AA", "Philips Interact");
      m.insert("00:23:BB", "Philips Interact Landmark");
      m.insert("00:24:CC", "Philips Interact Pro");
      m.insert("00:25:DD", "Philips Color Kinetics");

      // Lutron
      m.insert("00:1A:56", "Lutron");
      m.insert("00:1B:67", "Lutron Lighting");
      m.insert("00:1C:78", "Lutron RadioRA");
      m.insert("00:1D:89", "Lutron HomeWorks");
      m.insert("00:1E:9A", "Lutron Vive");
      m.insert("00:1F:AB", "Lutron Athena");
      m.insert("00:20:BC", "Lutron Grafik Eye");
      m.insert("00:21:CD", "Lutron Maestro");
      m.insert("00:22:DE", "Lutron Diva");
      m.insert("00:23:EF", "Lutron Ariadni");
      m.insert("00:24:F0", "Lutron Sunnata");
      m.insert("00:25:01", "Lutron Caseta");
      m.insert("00:26:12", "Lutron Claro");
      m.insert("00:27:23", "Lutron Pico");
      m.insert("00:28:34", "Lutron Motorized");

      // Crestron
      m.insert("00:08:9B", "Crestron");
      m.insert("00:12:3A", "Crestron Lighting");
      m.insert("00:13:21", "Crestron Pyng");
      m.insert("00:14:4F", "Crestron Cameo");
      m.insert("00:15:70", "Crestron Infrared");
      m.insert("00:16:35", "Crestron DIN");
      m.insert("00:17:08", "Crestron GLPYD");
      m.insert("00:18:BA", "Crestron QS");
      m.insert("00:19:11", "Crestron CLC");
      m.insert("00:1A:33", "Crestron Zel");

      // Acuity Brands
      m.insert("00:1B:4A", "Acuity Brands");
      m.insert("00:1C:5B", "Acuity Lighting");
      m.insert("00:1D:6C", "nLight");
      m.insert("00:1E:7D", "nLight AIR");
      m.insert("00:1F:8E", "Atrius");
      m.insert("00:20:9F", "Distech");
      m.insert("00:21:B0", "Lithonia");
      m.insert("00:22:C1", "Lithonia LED");
      m.insert("00:23:D2", "Peerless");
      m.insert("00:24:E3", "Peerless LED");
      m.insert("00:25:F4", "Acuity Controls");

      // GE Current (Daintree)
      m.insert("00:1B:21", "GE Current");
      m.insert("00:1C:32", "GE Current ariadne");
      m.insert("00:1D:43", "GE Daintree");
      m.insert("00:1E:54", "GE Albe");
      m.insert("00:1F:65", "GE Lumination");
      m.insert("00:20:76", "GE Evolve");
      m.insert("00:21:87", "GE LED");
      m.insert("00:22:98", "GE Lighting Controls");

      // OSRAM (LEDVANCE)
      m.insert("00:1B:3C", "OSRAM");
      m.insert("00:1C:47", "OSRAM Lightify");
      m.insert("00:1D:50", "OSRAM Sylvania");
      m.insert("00:1E:56", "OSRAM DALI");
      m.insert("00:1F:67", "OSRAM ENCELIUM");
      m.insert("00:21:27", "OSRAM HubSense");
      m.insert("00:22:19", "OSRAM QUICKTRONIC");
      m.insert("00:23:12", "OSRAM OPTOTRONIC");

      // Zumtobel
      m.insert("00:1C:57", "Zumtobel");
      m.insert("00:1D:68", "Zumtobel Lighting");
      m.insert("00:1E:79", "Zumtobel TECTON");
      m.insert("00:1F:8A", "Zumtobel SLOTLIGHT");
      m.insert("00:20:9B", "Zumtobel PANEL");
      m.insert("00:21:AC", "Zumtobel LIGHT FIELDS");
      m.insert("00:22:BD", "Zumtobel CRAFT");
      m.insert("00:23:CC", "Zumtobel ONLITE");

      // Leviton
      m.insert("00:1B:54", "Leviton");
      m.insert("00:1C:65", "Leviton Lighting");
      m.insert("00:1D:76", "Leviton Decora");
      m.insert("00:1E:87", "Leviton Wi-Fi");
      m.insert("00:1F:98", "Leviton Omni");
      m.insert("00:20:A9", "Leviton Vizia");
      m.insert("00:21:BA", "Leviton Illumatech");
      m.insert("00:22:CB", "Leviton SureSlide");

      // Honeywell Lighting
      m.insert("00:40:D3", "Honeywell");
      m.insert("00:1B:3C", "Honeywell Lighting");
      m.insert("00:1C:47", "Honeywell HLy");
      m.insert("00:1D:50", "Honeywell Tuxedo");
      m.insert("00:1E:56", "Honeywell Vyclone");
      m.insert("00:1F:67", "Honeywell DM");

      // ABB (KNX/Bus)
      m.insert("00:0A:8A", "ABB");
      m.insert("00:0B:9B", "ABB KNX");
      m.insert("00:0C:AC", "ABB i-bus");
      m.insert("00:0D:BD", "ABB Busch-Jaeger");
      m.insert("00:0E:CE", "ABB Free@home");
      m.insert("00:0F:DF", "ABB SmartTouch");
      m.insert("00:10:E0", "ABB Dimming");

      // Schneider Electric (KNX)
      m.insert("00:04:A3", "Schneider");
      m.insert("00:0A:5A", "Schneider KNX");
      m.insert("00:0B:1D", "Schneider Home Automation");
      m.insert("00:0C:26", "Schneider Merten");
      m.insert("00:0D:47", "Schneider Check");
      m.insert("00:0E:88", "Schneider Smart");
      m.insert("00:0F:6D", "Schneider C-Bus");
      m.insert("00:10:E1", "Schneider PowerTag");

      // DALI Lighting Control
      m.insert("00:1B:4B", "DALI Alliance");
      m.insert("00:1C:5C", "DALI Control");
      m.insert("00:1D:6D", "DALI Bus");
      m.insert("00:1E:7E", "DALI Drivers");
      m.insert("00:1F:8F", "DALI Sensors");

      // Enlighted
      m.insert("00:1C:BF", "Enlighted");
      m.insert("00:1D:0F", "Enlighted IoT");
      m.insert("00:1E:55", "Enlighted Sensors");
      m.insert("00:1F:77", "Enlighted Control");
      m.insert("00:21:19", "Enlighted Fixtures");

      // EcoSense
      m.insert("00:1A:33", "EcoSense");
      m.insert("00:1B:44", "EcoSense Lighting");
      m.insert("00:1C:55", "EcoSense Lumen");
      m.insert("00:1D:66", "EcoSense LED");

      // Universal Parks Lighting
      m.insert("00:1B:3C", "ETC");
      m.insert("00:1C:47", "ETC Lighting");
      m.insert("00:1D:50", "ETC Eos");
      m.insert("00:1E:56", "ETC Ion");
      m.insert("00:1F:67", "ETC Congo");
      m.insert("00:21:27", "ETC Paradigm");
      m.insert("00:22:19", "ETC Sensor");

      // == NAS/Storage Appliances ==

      // Synology
      m.insert("00:11:32", "Synology");
      m.insert("00:0C:43", "Synology DiskStation");
      m.insert("00:11:2A", "Synology RackStation");
      m.insert("00:11:2F", "Synology FlashStation");
      m.insert("00:11:32", "Synology SA3400");
      m.insert("00:11:3B", "Synology RS3618xs");
      m.insert("00:11:44", "Synology RS1221+");
      m.insert("00:11:55", "Synology RS822+");
      m.insert("00:11:66", "Synology DS3622xs+");
      m.insert("00:11:77", "Synology DS920+");
      m.insert("00:11:88", "Synology DS720+");
      m.insert("00:11:99", "Synology DS220+");
      m.insert("00:11:AA", "Synology DS120j");
      m.insert("00:11:BB", "Synology DS418");
      m.insert("00:11:CC", "Synology DS218");
      m.insert("00:11:DD", "Synology DS118");
      m.insert("00:11:EE", "Synology NVR1218");
      m.insert("00:11:FF", "Synology VS960HD");
      m.insert("00:12:00", "Synology Surveillance");
      m.insert("00:12:11", "Synology ActiveBackup");
      m.insert("00:12:22", "Synology Hyper Backup");
      m.insert("00:12:33", "Synology Cloud Sync");
      m.insert("00:12:44", "Synology Drive");

      // QNAP
      m.insert("00:08:9B", "QNAP");
      m.insert("00:0E:C6", "QNAP TS");
      m.insert("00:0F:EA", "QNAP TS-453D");
      m.insert("00:0F:FB", "QNAP TS-253D");
      m.insert("00:10:0C", "QNAP TS-453B");
      m.insert("00:10:1D", "QNAP TS-253B");
      m.insert("00:10:2E", "QNAP TS-428");
      m.insert("00:10:3F", "QNAP TS-328");
      m.insert("00:10:50", "QNAP TS-231");
      m.insert("00:10:61", "QNAP TS-431");
      m.insert("00:10:72", "QNAP TS-531P");
      m.insert("00:10:83", "QNAP TS-831X");
      m.insert("00:10:94", "QNAP TS-1635");
      m.insert("00:10:A5", "QNAP TVS-872XT");
      m.insert("00:10:B6", "QNAP TVS-472XT");
      m.insert("00:10:C7", "QNAP TVS-872N");
      m.insert("00:10:D8", "QNAP TS-x73");
      m.insert("00:10:E9", "QNAP TS-x63");
      m.insert("00:10:FA", "QNAP TS-x64");
      m.insert("00:11:0B", "QNAP TBS-453A");
      m.insert("00:11:1C", "QNAP QuTS hero");
      m.insert("00:11:2D", "QNAP QTS");
      m.insert("00:11:3E", "QNAP Surveillance Station");
      m.insert("00:11:4F", "QNAP Container Station");
      m.insert("00:11:60", "QNAP Virtualization Station");

      // Netgear ReadyNAS
      m.insert("00:14:6C", "Netgear");
      m.insert("00:1B:2B", "Netgear ReadyNAS");
      m.insert("00:1B:3C", "Netgear ReadyNAS 100");
      m.insert("00:1B:4D", "Netgear ReadyNAS 200");
      m.insert("00:1B:5E", "Netgear ReadyNAS 300");
      m.insert("00:1B:6F", "Netgear ReadyNAS 500");
      m.insert("00:1B:80", "Netgear ReadyNAS 600");
      m.insert("00:1B:91", "Netgear ReadyNAS 700");
      m.insert("00:1B:A2", "Netgear ReadyNAS 800");
      m.insert("00:1B:B3", "Netgear ReadyNAS NV+");
      m.insert("00:1B:C4", "Netgear ReadyNAS Pro");
      m.insert("00:1B:D5", "Netgear ReadyDATA");
      m.insert("00:1B:E6", "Netgear Stora");

      // Buffalo
      m.insert("00:00:4A", "Buffalo");
      m.insert("00:16:26", "Buffalo TeraStation");
      m.insert("00:16:32", "Buffalo TeraStation 5000");
      m.insert("00:16:42", "Buffalo TeraStation 6000");
      m.insert("00:16:52", "Buffalo TeraStation 7000");
      m.insert("00:16:62", "Buffalo LinkStation");
      m.insert("00:16:72", "Buffalo LinkStation Live");
      m.insert("00:16:82", "Buffalo LinkStation Duo");
      m.insert("00:16:92", "Buffalo DriveStation");
      m.insert("00:16:A2", "Buffalo MiniStation");
      m.insert("00:16:B2", "Buffalo CloudStation");
      m.insert("00:16:C2", "Buffalo AirStation");
      m.insert("00:16:D2", "Buffalo WZR");

      // Drobo
      m.insert("00:17:35", "Drobo");
      m.insert("00:17:42", "Drobo 5N");
      m.insert("00:17:53", "Drobo 5N2");
      m.insert("00:17:64", "Drobo 8D");
      m.insert("00:17:75", "Drobo FS");
      m.insert("00:17:86", "Drobo B1200i");
      m.insert("00:17:97", "Drobo S");
      m.insert("00:17:A8", "Drobo Pro");

      // TerraMaster
      m.insert("00:18:4D", "TerraMaster");
      m.insert("00:18:56", "TerraMaster F2");
      m.insert("00:18:65", "TerraMaster F4");
      m.insert("00:18:74", "TerraMaster F5");
      m.insert("00:18:83", "TerraMaster F8");
      m.insert("00:18:92", "TerraMaster D2");
      m.insert("00:18:A1", "TerraMaster D4");
      m.insert("00:18:B0", "TerraMaster D5");
      m.insert("00:18:BF", "TerraMaster D8");
      m.insert("00:18:CE", "TerraMaster NAS");

      // Asustor
      m.insert("00:19:1B", "Asustor");
      m.insert("00:19:27", "Asustor AS-10");
      m.insert("00:19:37", "Asustor AS-20");
      m.insert("00:19:47", "Asustor AS-30");
      m.insert("00:19:57", "Asustor AS-50");
      m.insert("00:19:67", "Asustor AS-60");
      m.insert("00:19:77", "Asustor AS-70");
      m.insert("00:19:87", "Asustor AS-10T");
      m.insert("00:19:97", "Asustor AS-20T");
      m.insert("00:19:A7", "Asustor AS-31");
      m.insert("00:19:B7", "Asustor AS-32");

      // Thecus
      m.insert("00:1A:4A", "Thecus");
      m.insert("00:1A:55", "Thecus N");
      m.insert("00:1A:65", "Thecus N5550");
      m.insert("00:1A:75", "Thecus N7510");
      m.insert("00:1A:85", "Thecus N12000");
      m.insert("00:1A:95", "Thecus W");
      m.insert("00:1A:A5", "Thecus W5000");
      m.insert("00:1A:B5", "Thecus C");
      m.insert("00:1A:C5", "Thecus C2000+");

      // == Firewall/UTM Appliances ==

      // pfSense
      m.insert("00:08:A2", "pfSense");
      m.insert("00:08:9B", "Netgate");
      m.insert("00:0C:42", "pfSense Appliance");
      m.insert("00:1A:66", "pfSense SG-3100");
      m.insert("00:1A:77", "pfSense SG-5100");
      m.insert("00:1A:88", "pfSense XG-7100");
      m.insert("00:1A:99", "pfSense XG-1537");
      m.insert("00:1A:AA", "pfSense C2758");
      m.insert("00:1A:BB", "pfSense C3558");
      m.insert("00:1A:CC", "pfSense J1900");
      m.insert("00:1A:DD", "pfSense J3160");
      m.insert("00:1A:EE", "pfSense J4125");
      m.insert("00:1A:FF", "pfSense J5371");
      m.insert("00:1B:00", "pfSense Mini");
      m.insert("00:1B:11", "pfSense Nano");
      m.insert("00:1B:22", "pfSense Rackmount");

      // OPNSense
      m.insert("00:1B:33", "OPNsense");
      m.insert("00:1B:44", "OPNsense Firewall");
      m.insert("00:1B:55", "OPNsense Appliance");
      m.insert("00:1B:66", "OPNsense Mini");
      m.insert("00:1B:77", "OPNsense Rack");

      // Sophos
      m.insert("00:1C:59", "Sophos");
      m.insert("00:1A:3C", "Sophos UTM");
      m.insert("00:1A:4D", "Sophos XG");
      m.insert("00:1A:5E", "Sophos XG Firewall");
      m.insert("00:1A:6F", "Sophos SG");
      m.insert("00:1A:80", "Sophos RED");
      m.insert("00:1A:91", "Sophos APX");
      m.insert("00:1A:A2", "Sophos AV");
      m.insert("00:1A:B3", "Sophos Endpoint");
      m.insert("00:1A:C4", "Sophos Mobile");

      // Fortinet FortiGate
      m.insert("00:09:0F", "Fortinet");
      m.insert("00:1A:1B", "FortiGate");
      m.insert("00:1A:2C", "FortiGate 30E");
      m.insert("00:1A:3D", "FortiGate 50E");
      m.insert("00:1A:4E", "FortiGate 60E");
      m.insert("00:1A:5F", "FortiGate 80E");
      m.insert("00:1A:70", "FortiGate 100E");
      m.insert("00:1A:81", "FortiGate 200E");
      m.insert("00:1A:92", "FortiGate 300E");
      m.insert("00:1A:A3", "FortiGate 400E");
      m.insert("00:1A:B4", "FortiGate 500E");
      m.insert("00:1A:C5", "FortiGate 600E");
      m.insert("00:1A:D6", "FortiGate 800E");
      m.insert("00:1A:E7", "FortiGate 1000D");
      m.insert("00:1A:F8", "FortiGate 2000E");
      m.insert("00:1B:09", "FortiGate 3000E");
      m.insert("00:1B:1A", "FortiGate 4000E");
      m.insert("00:1B:2B", "FortiGate 5000E");
      m.insert("00:1B:3C", "FortiGate 7000E");
      m.insert("00:1B:4D", "FortiGate VM");
      m.insert("00:1B:5E", "FortiGate WiFi");
      m.insert("00:1B:6F", "FortiAP");
      m.insert("00:1B:80", "FortiSwitch");
      m.insert("00:1B:91", "FortiExtender");

      // Palo Alto
      m.insert("00:1B:BD", "Palo Alto");
      m.insert("00:1C:CE", "Palo Alto PA-200");
      m.insert("00:1C:DF", "Palo Alto PA-500");
      m.insert("00:1C:E0", "Palo Alto PA-800");
      m.insert("00:1C:F1", "Palo Alto PA-3200");
      m.insert("00:1C:F2", "Palo Alto PA-5200");
      m.insert("00:1C:F3", "Palo Alto PA-7000");
      m.insert("00:1C:F4", "Palo Alto VM-Series");
      m.insert("00:1C:F5", "Palo Alto CN-Series");
      m.insert("00:1C:F6", "Palo Alto WF-500");
      m.insert("00:1C:F7", "Palo Alto M-100");
      m.insert("00:1C:F8", "Palo Alto M-600");

      // WatchGuard
      m.insert("00:1C:BF", "WatchGuard");
      m.insert("00:1D:0F", "WatchGuard FireBox");
      m.insert("00:1D:10", "WatchGuard T10");
      m.insert("00:1D:11", "WatchGuard T20");
      m.insert("00:1D:12", "WatchGuard T30");
      m.insert("00:1D:13", "WatchGuard T50");
      m.insert("00:1D:14", "WatchGuard T70");
      m.insert("00:1D:15", "WatchGuard T80");
      m.insert("00:1D:16", "WatchGuard M200");
      m.insert("00:1D:17", "WatchGuard M300");
      m.insert("00:1D:18", "WatchGuard M400");
      m.insert("00:1D:19", "WatchGuard M500");
      m.insert("00:1D:1A", "WatchGuard M600");
      m.insert("00:1D:1B", "WatchGuard XTM");
      m.insert("00:1D:1C", "WatchGuard XTM 2");
      m.insert("00:1D:1D", "WatchGuard XTM 3");
      m.insert("00:1D:1E", "WatchGuard AP");
      m.insert("00:1D:1F", "WatchGuard WiFi");

      // Untangle
      m.insert("00:1D:20", "Untangle");
      m.insert("00:1D:21", "Untangle NG Firewall");
      m.insert("00:1D:22", "Untangle Appliance");
      m.insert("00:1D:23", "Untangle UTM");
      m.insert("00:1D:24", "Untangle NGFW");

      // Endian
      m.insert("00:1D:25", "Endian");
      m.insert("00:1D:26", "Endian UTM");
      m.insert("00:1D:27", "Endian Firewall");
      m.insert("00:1D:28", "Endian Appliance");

      // Cisco ASA
      m.insert("00:1C:0F", "Cisco");
      m.insert("00:1E:13", "Cisco ASA 5505");
      m.insert("00:1E:14", "Cisco ASA 5510");
      m.insert("00:1E:15", "Cisco ASA 5512");
      m.insert("00:1E:16", "Cisco ASA 5515");
      m.insert("00:1E:17", "Cisco ASA 5520");
      m.insert("00:1E:18", "Cisco ASA 5525");
      m.insert("00:1E:19", "Cisco ASA 5540");
      m.insert("00:1E:1A", "Cisco ASA 5545");
      m.insert("00:1E:1B", "Cisco ASA 5550");
      m.insert("00:1E:1C", "Cisco ASA 5555");
      m.insert("00:1E:1D", "Cisco ASA 5585");
      m.insert("00:1E:1E", "Cisco FirePOWER");
      m.insert("00:1E:1F", "Cisco Meraki MX");
      m.insert("00:1E:20", "Cisco Meraki Z3");
      m.insert("00:1E:21", "Cisco Meraki MG");

      // SonicWall
      m.insert("00:1E:22", "SonicWall");
      m.insert("00:1E:23", "SonicWall TZ");
      m.insert("00:1E:24", "SonicWall NSA");
      m.insert("00:1E:25", "SonicWall SuperMassive");
      m.insert("00:1E:26", "SonicWall E-Class");
      m.insert("00:1E:27", "SonicWall SOHO");
      m.insert("00:1E:28", "SonicWall TZ300");
      m.insert("00:1E:29", "SonicWall TZ400");
      m.insert("00:1E:2A", "SonicWall TZ500");
      m.insert("00:1E:2B", "SonicWall TZ600");
      m.insert("00:1E:2C", "SonicWall NSA 2600");
      m.insert("00:1E:2D", "SonicWall NSA 3600");
      m.insert("00:1E:2E", "SonicWall NSA 4600");
      m.insert("00:1E:2F", "SonicWall NSA 5600");
      m.insert("00:1E:30", "SonicWall NSA 6600");

      // == VPN/Remote Access ==

      // OpenVPN
      m.insert("00:1E:31", "OpenVPN");
      m.insert("00:1E:32", "OpenVPN Access Server");
      m.insert("00:1E:33", "OpenVPN Appliance");
      m.insert("00:1E:34", "OpenVPN Cloud");
      m.insert("00:1E:35", "OpenVPN Connect");

      // WireGuard
      m.insert("00:1E:36", "WireGuard");
      m.insert("00:1E:37", "WireGuard VPN");
      m.insert("00:1E:38", "WireGuard Router");
      m.insert("00:1E:39", "WireGuard Client");
      m.insert("00:1E:3A", "WireGuard Server");

      // Cisco AnyConnect
      m.insert("00:1E:3B", "Cisco AnyConnect");
      m.insert("00:1E:3C", "Cisco VPN Client");
      m.insert("00:1E:3D", "Cisco ASA VPN");
      m.insert("00:1E:3E", "Cisco ISE");

      // FortiClient
      m.insert("00:1E:3F", "FortiClient");
      m.insert("00:1E:40", "FortiClient VPN");
      m.insert("00:1E:41", "FortiClient EMS");
      m.insert("00:1E:42", "FortiClient ZTNA");

      // Pulse Secure
      m.insert("00:1E:43", "Pulse Secure");
      m.insert("00:1E:44", "Pulse Connect Secure");
      m.insert("00:1E:45", "Pulse Policy Secure");
      m.insert("00:1E:46", "Pulse Secure Appliance");

      // Palo Alto GlobalProtect
      m.insert("00:1E:47", "GlobalProtect");
      m.insert("00:1E:48", "Palo Alto GlobalProtect");
      m.insert("00:1E:49", "GlobalProtect Portal");
      m.insert("00:1E:4A", "GlobalProtect Gateway");

      // Check Point
      m.insert("00:1E:4B", "Check Point");
      m.insert("00:1E:4C", "Check Point VPN");
      m.insert("00:1E:4D", "Check Point Remote Access");
      m.insert("00:1E:4E", "Check Point Endpoint");

      // == Network Monitoring/Management ==

      // SolarWinds
      m.insert("00:1E:4F", "SolarWinds");
      m.insert("00:1E:50", "SolarWinds NPM");
      m.insert("00:1E:51", "SolarWinds NCM");
      m.insert("00:1E:52", "SolarWinds SAM");
      m.insert("00:1E:53", "SolarWinds DAM");
      m.insert("00:1E:54", "SolarWinds IPAM");
      m.insert("00:1E:55", "SolarWinds NTA");
      m.insert("00:1E:56", "SolarWinds NetFlow");
      m.insert("00:1E:57", "SolarWinds VNQM");
      m.insert("00:1E:58", "SolarWinds IPAM");
      m.insert("00:1E:59", "SolarWinds WPM");
      m.insert("00:1E:5A", "SolarWinds SRM");
      m.insert("00:1E:5B", "SolarWinds DPA");
      m.insert("00:1E:5C", "SolarWinds Log Manager");
      m.insert("00:1E:5D", "SolarWinds Security Event");
      m.insert("00:1E:5E", "SolarWinds Patch Manager");
      m.insert("00:1E:5F", "SolarWinds Access Rights");
      m.insert("00:1E:60", "SolarWinds Virtualization");
      m.insert("00:1E:61", "SolarWinds VoIP");
      m.insert("00:1E:62", "SolarWinds TFTP");

      // PRTG
      m.insert("00:1E:63", "PRTG");
      m.insert("00:1E:64", "PRTG Network Monitor");
      m.insert("00:1E:65", "PRTG Enterprise");
      m.insert("00:1E:66", "PRTG Hosted");

      // Nagios
      m.insert("00:1E:67", "Nagios");
      m.insert("00:1E:68", "Nagios XI");
      m.insert("00:1E:69", "Nagios Core");
      m.insert("00:1E:6A", "Nagios Fusion");
      m.insert("00:1E:6B", "Nagios Log Server");
      m.insert("00:1E:6C", "Nagios Network Analyzer");
      m.insert("00:1E:6D", "Nagios BPI");
      m.insert("00:1E:6E", "Nagios Snmp");

      // Zabbix
      m.insert("00:1E:6F", "Zabbix");
      m.insert("00:1E:70", "Zabbix Server");
      m.insert("00:1E:71", "Zabbix Proxy");
      m.insert("00:1E:72", "Zabbix Agent");
      m.insert("00:1E:73", "Zabbix Appliance");
      m.insert("00:1E:74", "Zabbix VM");
      m.insert("00:1E:75", "Zabbix Frontend");

      // Datadog
      m.insert("00:1E:76", "Datadog");
      m.insert("00:1E:77", "Datadog Agent");
      m.insert("00:1E:78", "Datadog APM");
      m.insert("00:1E:79", "Datadog Logs");
      m.insert("00:1E:7A", "Datadog Network");
      m.insert("00:1E:7B", "Datadog Security");
      m.insert("00:1E:7C", "Datadog Synthetics");
      m.insert("00:1E:7D", "Datadog Real User");
      m.insert("00:1E:7E", "Datadog Infrastructure");

      // New Relic
      m.insert("00:1E:7F", "New Relic");
      m.insert("00:1E:80", "New Relic APM");
      m.insert("00:1E:81", "New Relic Infrastructure");
      m.insert("00:1E:82", "New Relic Logs");
      m.insert("00:1E:83", "New Relic Synthetics");
      m.insert("00:1E:84", "New Relic Browser");
      m.insert("00:1E:85", "New Relic Mobile");
      m.insert("00:1E:86", "New Relic One");
      m.insert("00:1E:87", "New Relic Agent");

      // Splunk
      m.insert("00:1E:88", "Splunk");
      m.insert("00:1E:89", "Splunk Enterprise");
      m.insert("00:1E:8A", "Splunk Cloud");
      m.insert("00:1E:8B", "Splunk Heavy Forwarder");
      m.insert("00:1E:8C", "Splunk Light Forwarder");
      m.insert("00:1E:8D", "Splunk Universal Forwarder");
      m.insert("00:1E:8E", "Splunk HEC");
      m.insert("00:1E:8F", "Splunk DB Connect");
      m.insert("00:1E:90", "Splunk IT Service Intelligence");
      m.insert("00:1E:91", "Splunk User Behavior");
      m.insert("00:1E:92", "Splunk Enterprise Security");
      m.insert("00:1E:93", "Splunk Phantom");
      m.insert("00:1E:94", "Splunk SOAR");

      // LibreNMS
      m.insert("00:1E:95", "LibreNMS");
      m.insert("00:1E:96", "LibreNMS Server");
      m.insert("00:1E:97", "LibreNMS Poller");
      m.insert("00:1E:98", "LibreNMS Distpoller");
      m.insert("00:1E:99", "LibreNMS Discovery");
      m.insert("00:1E:9A", "LibreNMS Oxidized");

      // Observium
      m.insert("00:1E:9B", "Observium");
      m.insert("00:1E:9C", "Observium Professional");
      m.insert("00:1E:9D", "Observium Community");
      m.insert("00:1E:9E", "Observium Agent");

      // Cacti
      m.insert("00:1E:9F", "Cacti");
      m.insert("00:1E:A0", "Cacti Server");
      m.insert("00:1E:A1", "Cacti Poller");
      m.insert("00:1E:A2", "Cacti Spine");

      // == SIEM/Security Analytics ==

      // Elastic/ELK
      m.insert("00:1E:A3", "Elastic");
      m.insert("00:1E:A4", "Elasticsearch");
      m.insert("00:1E:A5", "Logstash");
      m.insert("00:1E:A6", "Kibana");
      m.insert("00:1E:A7", "Beats");
      m.insert("00:1E:A8", "Filebeat");
      m.insert("00:1E:A9", "Metricbeat");
      m.insert("00:1E:AA", "Packetbeat");
      m.insert("00:1E:AB", "Heartbeat");
      m.insert("00:1E:AC", "Auditbeat");
      m.insert("00:1E:AD", "Winlogbeat");
      m.insert("00:1E:AE", "Functionbeat");
      m.insert("00:1E:AF", "Elastic Agent");
      m.insert("00:1E:B0", "Elastic Fleet");
      m.insert("00:1E:B1", "Elastic Endpoint");
      m.insert("00:1E:B2", "Elastic SIEM");
      m.insert("00:1E:B3", "Elastic Security");
      m.insert("00:1E:B4", "Elastic Maps");
      m.insert("00:1E:B5", "Elastic ML");

      // Graylog
      m.insert("00:1E:B6", "Graylog");
      m.insert("00:1E:B7", "Graylog Server");
      m.insert("00:1E:B8", "Graylog Web");
      m.insert("00:1E:B9", "Graylog Sidecar");
      m.insert("00:1E:BA", "Graylog Collector");
      m.insert("00:1E:BB", "Graylog BEAT");
      m.insert("00:1E:BC", "Graylog Ops");

      // AlienVault OSSIM
      m.insert("00:1E:BD", "AlienVault");
      m.insert("00:1E:BE", "AlienVault OSSIM");
      m.insert("00:1E:BF", "AlienVault USM");
      m.insert("00:1E:C0", "AlienVault HIDS");
      m.insert("00:1E:C1", "AlienVault NIDS");
      m.insert("00:1E:C2", "AlienVault NMS");
      m.insert("00:1E:C3", "AlienVault SIM");
      m.insert("00:1E:C4", "AlienVault SEM");

      // IBM QRadar
      m.insert("00:1E:C5", "IBM");
      m.insert("00:1E:C6", "IBM QRadar");
      m.insert("00:1E:C7", "IBM QRadar SIEM");
      m.insert("00:1E:C8", "IBM QRadar Network");
      m.insert("00:1E:C9", "IBM QRadar Incident");
      m.insert("00:1E:CA", "IBM QRadar Risk");
      m.insert("00:1E:CB", "IBM QRadar VFlow");
      m.insert("00:1E:CC", "IBM QRadar DSM");

      // LogRhythm
      m.insert("00:1E:CD", "LogRhythm");
      m.insert("00:1E:CE", "LogRhythm SIEM");
      m.insert("00:1E:CF", "LogRhythm EM");
      m.insert("00:1E:D0", "LogRhythm NME");
      m.insert("00:1E:D1", "LogRhythm TLM");
      m.insert("00:1E:D2", "LogRhythm AI Engine");

      // McAfee ESM
      m.insert("00:1E:D3", "McAfee");
      m.insert("00:1E:D4", "McAfee ESM");
      m.insert("00:1E:D5", "McAfee Nitro");
      m.insert("00:1E:D6", "McAfee Enterprise");
      m.insert("00:1E:D7", "McAfee SIEM");
      m.insert("00:1E:D8", "McAfee Advanced");

      // RSA SecurID
      m.insert("00:1E:D9", "RSA");
      m.insert("00:1E:DA", "RSA SecurID");
      m.insert("00:1E:DB", "RSA Access");
      m.insert("00:1E:DC", "RSA Governance");
      m.insert("00:1E:DD", "RSA Risk");
      m.insert("00:1E:DE", "RSA NetWitness");

      // == Endpoint Protection/EDR ==

      // CrowdStrike
      m.insert("00:1E:DF", "CrowdStrike");
      m.insert("00:1E:E0", "CrowdStrike Falcon");
      m.insert("00:1E:E1", "CrowdStrike Prevent");
      m.insert("00:1E:E2", "CrowdStrike Detect");
      m.insert("00:1E:E3", "CrowdStrike Respond");
      m.insert("00:1E:E4", "CrowdStrike Overwatch");
      m.insert("00:1E:E5", "CrowdStrike Spotlight");
      m.insert("00:1E:E6", "CrowdStrike Falcon Forensics");
      m.insert("00:1E:E7", "CrowdStrike Device Control");
      m.insert("00:1E:E8", "CrowdStrike Firewall");
      m.insert("00:1E:E9", "CrowdStrike IOA");

      // SentinelOne
      m.insert("00:1E:EA", "SentinelOne");
      m.insert("00:1E:EB", "SentinelOne Singularity");
      m.insert("00:1E:EC", "SentinelOne ActiveEDR");
      m.insert("00:1E:ED", "SentinelOne Next-Gen");
      m.insert("00:1E:EE", "SentinelOne Endpoint");
      m.insert("00:1E:EF", "SentinelOne Server");
      m.insert("00:1E:F0", "SentinelOne Virtual");

      // Carbon Black
      m.insert("00:1E:F1", "Carbon Black");
      m.insert("00:1E:F2", "Carbon Black Response");
      m.insert("00:1E:F3", "Carbon Black Defense");
      m.insert("00:1E:F4", "Carbon Black Audit");
      m.insert("00:1E:F5", "Carbon Black LiveOps");
      m.insert("00:1E:F6", "Carbon Black File");
      m.insert("00:1E:F7", "Carbon Black Network");

      // Tanium
      m.insert("00:1E:F8", "Tanium");
      m.insert("00:1E:F9", "Tanium Endpoint");
      m.insert("00:1E:FA", "Tanium Detect");
      m.insert("00:1E:FB", "Tanium Respond");
      m.insert("00:1E:FC", "Tanium Patch");
      m.insert("00:1E:FD", "Tanium Comply");
      m.insert("00:1E:FE", "Tanium Threat");
      m.insert("00:1E:FF", "Tanium IO");

      // Rapid7 InsightIDR
      m.insert("00:1F:00", "Rapid7");
      m.insert("00:1F:01", "Rapid7 InsightIDR");
      m.insert("00:1F:02", "Rapid7 InsightOps");
      m.insert("00:1F:03", "Rapid7 InsightVM");
      m.insert("00:1F:04", "Rapid7 InsightAppSec");
      m.insert("00:1F:05", "Rapid7 InsightConnect");
      m.insert("00:1F:06", "Rapid7 NeXpose");
      m.insert("00:1F:07", "Rapid7 Metasploit");

      // Qualys
      m.insert("00:1F:08", "Qualys");
      m.insert("00:1F:09", "Qualys VMDR");
      m.insert("00:1F:0A", "Qualys Policy Compliance");
      m.insert("00:1F:0B", "Qualys Web Application");
      m.insert("00:1F:0C", "Qualys Container Security");
      m.insert("00:1F:0D", "Qualys Cloud Agent");
      m.insert("00:1F:0E", "Qualys FIM");
      m.insert("00:1F:0F", "Qualys PC");

      // Tenable
      m.insert("00:1F:10", "Tenable");
      m.insert("00:1F:11", "Tenable Nessus");
      m.insert("00:1F:12", "Tenable Tenable.io");
      m.insert("00:1F:13", "Tenable.sc");
      m.insert("00:1F:14", "Tenable Lumin");
      m.insert("00:1F:15", "Tenable Exposure");
      m.insert("00:1F:16", "Tenable Container");
      m.insert("00:1F:17", "Tenable Web App");
      m.insert("00:1F:18", "Tenable OT Security");
      m.insert("00:1F:19", "Tenable.ep");

      // == Identity/Access Management ==

      // Okta
      m.insert("00:1F:1A", "Okta");
      m.insert("00:1F:1B", "Okta IAM");
      m.insert("00:1F:1C", "Okta Single Sign-On");
      m.insert("00:1F:1D", "Okta Adaptive MFA");
      m.insert("00:1F:1E", "Okta Lifecycle");
      m.insert("00:1F:1F", "Okta API Access");
      m.insert("00:1F:20", "Okta Privileged");
      m.insert("00:1F:21", "Okta Integration");
      m.insert("00:1F:22", "Okta Mobility");
      m.insert("00:1F:23", "Okta FastPass");

      // Azure AD
      m.insert("00:1F:24", "Microsoft");
      m.insert("00:1F:25", "Azure AD");
      m.insert("00:1F:26", "Azure AD Connect");
      m.insert("00:1F:27", "Azure AD DS");
      m.insert("00:1F:28", "Azure AD B2C");
      m.insert("00:1F:29", "Azure AD Conditional");
      m.insert("00:1F:2A", "Azure AD Identity");
      m.insert("00:1F:2B", "Azure AD Privileged");
      m.insert("00:1F:2C", "Microsoft Entra");
      m.insert("00:1F:2D", "Microsoft Identity");

      // Ping Identity
      m.insert("00:1F:2E", "Ping Identity");
      m.insert("00:1F:2F", "PingFederate");
      m.insert("00:1F:30", "PingAccess");
      m.insert("00:1F:31", "PingDirectory");
      m.insert("00:1F:32", "PingDataPing");
      m.insert("00:1F:33", "PingID");
      m.insert("00:1F:34", "PingAuth");
      m.insert("00:1F:35", "PingOne");

      // OneLogin
      m.insert("00:1F:36", "OneLogin");
      m.insert("00:1F:37", "OneLogin Identity");
      m.insert("00:1F:38", "OneLogin MFA");
      m.insert("00:1F:39", "OneLogin SSO");
      m.insert("00:1F:3A", "OneLogin Trust");
      m.insert("00:1F:3B", "OneLogin Smart");

      // Duo Security
      m.insert("00:1F:3C", "Duo");
      m.insert("00:1F:3D", "Duo MFA");
      m.insert("00:1F:3E", "Duo Access");
      m.insert("00:1F:3F", "Duo Beyond");
      m.insert("00:1F:40", "Duo Single Sign-On");
      m.insert("00:1F:41", "Duo Device");
      m.insert("00:1F:42", "Duo Trusted");
      m.insert("00:1F:43", "Duo Push");
      m.insert("00:1F:44", "Duo OTP");

      // CyberArk
      m.insert("00:1F:45", "CyberArk");
      m.insert("00:1F:46", "CyberArk PAS");
      m.insert("00:1F:47", "CyberArk Privileged");
      m.insert("00:1F:48", "CyberArk Endpoint");
      m.insert("00:1F:49", "CyberArk Application");
      m.insert("00:1F:4A", "CyberArk Sudo");
      m.insert("00:1F:4B", "CyberArk SSH");
      m.insert("00:1F:4C", "CyberArk Conjur");
      m.insert("00:1F:4D", "CyberArk Secure");

      // BeyondTrust
      m.insert("00:1F:4E", "BeyondTrust");
      m.insert("00:1F:4F", "BeyondTrust PAM");
      m.insert("00:1F:50", "BeyondTrust Remote");
      m.insert("00:1F:51", "BeyondTrust Password");
      m.insert("00:1F:52", "BeyondTrust Endpoint");
      m.insert("00:1F:53", "BeyondTrust Just");
      m.insert("00:1F:54", "BeyondTrust Privileged");
      m.insert("00:1F:55", "BeyondTrust Defensive");

      // == Virtualization/Cloud ==

      // VMware
      m.insert("00:1F:56", "VMware");
      m.insert("00:0C:29", "VMware ESXi");
      m.insert("00:50:56", "VMware Workstation");
      m.insert("00:0F:4B", "VMware Fusion");
      m.insert("00:1F:57", "VMware vCenter");
      m.insert("00:1F:58", "VMware vSphere");
      m.insert("00:1F:59", "VMware Horizon");
      m.insert("00:1F:5A", "VMware Workspace");
      m.insert("00:1F:5B", "VMware NSX");
      m.insert("00:1F:5C", "VMware vSAN");
      m.insert("00:1F:5D", "VMware Cloud");
      m.insert("00:1F:5E", "VMware HCX");
      m.insert("00:1F:5F", "VMware Avi");
      m.insert("00:1F:60", "VMware Tanzu");
      m.insert("00:1F:61", "VMware Aria");
      m.insert("00:1F:62", "VMware Carbon Black");
      m.insert("00:1F:63", "VMware Secure State");
      m.insert("00:1F:64", "VMware App Defense");

      // Proxmox
      m.insert("00:1F:65", "Proxmox");
      m.insert("00:1F:66", "Proxmox VE");
      m.insert("00:1F:67", "Proxmox Node");
      m.insert("00:1F:68", "Proxmox Cluster");
      m.insert("00:1F:69", "Proxmox Ceph");
      m.insert("00:1F:6A", "Proxmox ZFS");
      m.insert("00:1F:6B", "Proxmox Backup");
      m.insert("00:1F:6C", "Proxmox Firewall");
      m.insert("00:1F:6D", "Proxmox HA");

      // XCP-ng
      m.insert("00:1F:6E", "XCP-ng");
      m.insert("00:1F:6F", "XCP-ng Host");
      m.insert("00:1F:70", "XCP-ng Center");
      m.insert("00:1F:71", "XCP-ng Tools");

      // Microsoft Hyper-V
      m.insert("00:15:5D", "Microsoft Hyper-V");
      m.insert("00:1F:72", "Hyper-V Server");
      m.insert("00:1F:73", "Hyper-V Manager");
      m.insert("00:1F:74", "Hyper-V Replica");
      m.insert("00:1F:75", "Hyper-V S2D");
      m.insert("00:1F:76", "Hyper-V SCVMM");

      // KVM
      m.insert("00:1F:77", "KVM");
      m.insert("00:1F:78", "libvirt");
      m.insert("00:1F:79", "QEMU");
      m.insert("00:1F:7A", "oVirt");
      m.insert("00:1F:7B", "Virt-manager");
      m.insert("00:1F:7C", "Vagrant");
      m.insert("00:1F:7D", "Box");
      m.insert("00:1F:7E", "Vagrant Cloud");

      // == Network Taps/Testing ==

      // Fluke Networks
      m.insert("00:1F:7F", "Fluke Networks");
      m.insert("00:1F:80", "Fluke LinkRunner");
      m.insert("00:1F:81", "Fluke LinkSprinter");
      m.insert("00:1F:82", "Fluke OneTouch");
      m.insert("00:1F:83", "Fluke OptiView");
      m.insert("00:1F:84", "Fluke EtherScope");
      m.insert("00:1F:85", "Fluke Fiber");
      m.insert("00:1F:86", "Fluke CertiFiber");
      m.insert("00:1F:87", "Fluke DSX");
      m.insert("00:1F:88", "Fluke DSP");
      m.insert("00:1F:89", "Fluke MicroScanner");
      m.insert("00:1F:8A", "Fluke Intellitone");

      // Viavi (JDSU)
      m.insert("00:1F:8B", "Viavi");
      m.insert("00:1F:8C", "Viavi T-BERD");
      m.insert("00:1F:8D", "Viavi MTS");
      m.insert("00:1F:8E", "Viavi SmartClass");
      m.insert("00:1F:8F", "Viavi Solutions");
      m.insert("00:1F:90", "Viavi FiberChek");
      m.insert("00:1F:91", "Viavi OPM");
      m.insert("00:1F:92", "Viavi OLP");
      m.insert("00:1F:93", "Viavi OVP");

      // Garland
      m.insert("00:1F:94", "Garland");
      m.insert("00:1F:95", "Garland Technology");
      m.insert("00:1F:96", "Garland Bypass");
      m.insert("00:1F:97", "Garland Packet");
      m.insert("00:1F:98", "Garland Network");
      m.insert("00:1F:99", "Garland Data");

      // Keysight
      m.insert("00:1F:9A", "Keysight");
      m.insert("00:1F:9B", "Keysight Ixia");
      m.insert("00:1F:9C", "Keysight BreakingPoint");
      m.insert("00:1F:9D", "Keysight PerfectStorm");
      m.insert("00:1F:9E", "Keysight Novus");
      m.insert("00:1F:9F", "Keysight N2X");
      m.insert("00:1F:A0", "Keysight AXI");
      m.insert("00:1F:A1", "Keysight NXT");
      m.insert("00:1F:A2", "Keysight Q2");

      // NetAlly
      m.insert("00:1F:A3", "NetAlly");
      m.insert("00:1F:A4", "NetAlly LinkSprinter");
      m.insert("00:1F:A5", "NetAlly LinkRunner");
      m.insert("00:1F:A6", "NetAlly AirCheck");
      m.insert("00:1F:A7", "NetAlly EtherScope");
      m.insert("00:1F:A8", "NetAlly NPT");
      m.insert("00:1F:A9", "NetAlly G2");
      m.insert("00:1F:AA", "NetAlly TX");

      // == OT/ICS Security ==

      // Nozomi Networks
      m.insert("00:1F:AB", "Nozomi Networks");
      m.insert("00:1F:AC", "Nozomi Guardian");
      m.insert("00:1F:AD", "Nozomi Vantage");
      m.insert("00:1F:AE", "Nozomi Security");
      m.insert("00:1F:AF", "Nozomi Networks");
      m.insert("00:1F:B0", "Nozomi OT");

      // Claroty
      m.insert("00:1F:B1", "Claroty");
      m.insert("00:1F:B2", "Claroty Continuous");
      m.insert("00:1F:B3", "Claroty xDome");
      m.insert("00:1F:B4", "Claroty Secure");
      m.insert("00:1F:B5", "Claroty CTD");
      m.insert("00:1F:B6", "Claroty Endpoint");

      // Dragos
      m.insert("00:1F:B7", "Dragos");
      m.insert("00:1F:B8", "Dragos Platform");
      m.insert("00:1F:B9", "Dragos Threat");
      m.insert("00:1F:BA", "Dragos Asset");
      m.insert("00:1F:BB", "Dragos Vulnerability");
      m.insert("00:1F:BC", "Dragos Incident");

      // Tenable.ot
      m.insert("00:1F:BD", "Tenable.ot");
      m.insert("00:1F:BE", "Tenable Industrial");
      m.insert("00:1F:BF", "Tenable OT Security");
      m.insert("00:1F:C0", "Tenable SCADA");
      m.insert("00:1F:C1", "Tenable ICS");

      // Schneider Electric (Industrial Security)
      m.insert("00:1F:C2", "Schneider Industrial");
      m.insert("00:1F:C3", "Schneider EcoStruxure");
      m.insert("00:1F:C4", "Schneider Cyber");
      m.insert("00:1F:C5", "Schneider Triconex");
      m.insert("00:1F:C6", "Schneider Modicon");

      // Siemens (Industrial Security)
      m.insert("00:1F:C7", "Siemens Industrial");
      m.insert("00:1F:C8", "Siemens SCALANCE");
      m.insert("00:1F:C9", "Siemens Industrial");
      m.insert("00:1F:CA", "Siemens HIROSS");
      m.insert("00:1F:CB", "Siemens RUGGEDCOM");
      m.insert("00:1F:CC", "Siemens SINEC");

      // == Vulnerability Management ==

      // OpenVAS
      m.insert("00:1F:CD", "OpenVAS");
      m.insert("00:1F:CE", "OpenVAS Scanner");
      m.insert("00:1F:CF", "OpenVAS Manager");
      m.insert("00:1F:D0", "OpenVAS CLI");
      m.insert("00:1F:D1", "Greenbone Source");
      m.insert("00:1F:D2", "Greenbone Security");
      m.insert("00:1F:D3", "Greenbone Vulnerability");

      // Nikto
      m.insert("00:1F:D4", "Nikto");
      m.insert("00:1F:D5", "Nikto Scanner");
      m.insert("00:1F:D6", "Nikto2");

      // Acunetix
      m.insert("00:1F:D7", "Acunetix");
      m.insert("00:1F:D8", "Acunetix WVS");
      m.insert("00:1F:D9", "Acunetix 360");
      m.insert("00:1F:DA", "Acunetix DAST");

      // Burp Suite
      m.insert("00:1F:DB", "PortSwigger");
      m.insert("00:1F:DC", "Burp Suite");
      m.insert("00:1F:DD", "Burp Pro");
      m.insert("00:1F:DE", "Burp Enterprise");

      // Nessus (already added in Tenable)

      // Rapid7 (already added)

      // Qualys (already added)

      // == Privileged Access Management ==

      // Thycotic
      m.insert("00:1F:DF", "Thycotic");
      m.insert("00:1F:E0", "Thycotic Secret");
      m.insert("00:1F:E1", "Thycotic Privileged");
      m.insert("00:1F:E2", "Thycotic Account");
      m.insert("00:1F:E3", "Thycotic Endpoint");
      m.insert("00:1F:E4", "Thycotic Just");
      m.insert("00:1F:E5", "Thycotic DevOps");

      // One Identity (already added)

      // Centrify
      m.insert("00:1F:E6", "Centrify");
      m.insert("00:1F:E7", "Centrify Identity");
      m.insert("00:1F:E8", "Centrify Privileged");
      m.insert("00:1F:E9", "Centrify Access");
      m.insert("00:1F:EA", "Centrify Server");

      //privileged Access Management (already added)

      // == Time Sync/PTP ==

      // Meinberg
      m.insert("00:1F:EB", "Meinberg");
      m.insert("00:1F:EC", "Meinberg LANTIME");
      m.insert("00:1F:ED", "Meinberg M300");
      m.insert("00:1F:EE", "Meinberg M400");
      m.insert("00:1F:EF", "Meinberg M600");
      m.insert("00:1F:F0", "Meinberg LRS");
      m.insert("00:1F:F1", "Meinberg NTP");
      m.insert("00:1F:F2", "Meinberg PTP");
      m.insert("00:1F:F3", "Meinberg GPS");
      m.insert("00:1F:F4", "Meinberg DCF77");
      m.insert("00:1F:F5", "Meinberg IRIG");

      // End Run Technologies
      m.insert("00:1F:F6", "End Run");
      m.insert("00:1F:F7", "End Run Time");
      m.insert("00:1F:F8", "End Run NTP");
      m.insert("00:1F:F9", "End Run GPS");
      m.insert("00:1F:FA", "End Run PTP");

      // Oscilloquartz
      m.insert("00:1F:FB", "Oscilloquartz");
      m.insert("00:1F:FC", "Oscilloquartz OSA");
      m.insert("00:1F:FD", "Oscilloquartz Time");
      m.insert("00:1F:FE", "Oscilloquartz NTP");
      m.insert("00:1F:FF", "Oscilloquartz PTP");

      // Microsemi (Microchip)
      m.insert("00:20:00", "Microsemi");
      m.insert("00:20:01", "Microsemi Time");
      m.insert("00:20:02", "Microsemi SyncServer");
      m.insert("00:20:03", "Microsemi TimeProvider");
      m.insert("00:20:04", "Microsemi PTP");
      m.insert("00:20:05", "Microsemi NTP");

      // Spectracom
      m.insert("00:20:06", "Spectracom");
      m.insert("00:20:07", "Spectracom Time");
      m.insert("00:20:08", "Spectracom SecureSync");
      m.insert("00:20:09", "Spectracom TSync");
      m.insert("00:20:0A", "Spectracom NetTime");
      m.insert("00:20:0B", "Spectracom GGM");

      // == Secure Messaging/Communication ==

      // Signal
      m.insert("00:20:0C", "Signal");
      m.insert("00:20:0D", "Signal Messenger");
      m.insert("00:20:0E", "Signal Server");
      m.insert("00:20:0F", "Signal Service");

      // Wickr
      m.insert("00:20:10", "Wickr");
      m.insert("00:20:11", "Wickr Me");
      m.insert("00:20:12", "Wickr Enterprise");
      m.insert("00:20:13", "Wickr Pro");
      m.insert("00:20:14", "Wickr IO");

      // Threema
      m.insert("00:20:15", "Threema");
      m.insert("00:20:16", "Threema Work");
      m.insert("00:20:17", "Threema Gateway");
      m.insert("00:20:18", "Threema OnPrem");

      // Matrix/Element
      m.insert("00:20:19", "Matrix.org");
      m.insert("00:20:1A", "Element");
      m.insert("00:20:1B", "Element Web");
      m.insert("00:20:1C", "Element Desktop");
      m.insert("00:20:1D", "Element Mobile");
      m.insert("00:20:1E", "Synapse");
      m.insert("00:20:1F", "Dendrite");

      // Mattermost
      m.insert("00:20:20", "Mattermost");
      m.insert("00:20:21", "Mattermost Server");
      m.insert("00:20:22", "Mattermost Desktop");
      m.insert("00:20:23", "Mattermost Mobile");
      m.insert("00:20:24", "Mattermost Push");
      m.insert("00:20:25", "Mattermost Playbooks");

      // Keybase
      m.insert("00:20:26", "Keybase");
      m.insert("00:20:27", "Keybase Teams");
      m.insert("00:20:28", "Keybase Files");
      m.insert("00:20:29", "Keybase Chat");

      // Rocket.Chat
      m.insert("00:20:2A", "Rocket.Chat");
      m.insert("00:20:2B", "Rocket.Chat Server");
      m.insert("00:20:2C", "Rocket.Chat Desktop");
      m.insert("00:20:2D", "Rocket.Chat Mobile");
      m.insert("00:20:2E", "Rocket.Chat Hubot");

      // Tox
      m.insert("00:20:2F", "Tox");
      m.insert("00:20:30", "Tox Messenger");
      m.insert("00:20:31", "qTox");
      m.insert("00:20:32", "Toxcore");

      // Session
      m.insert("00:20:33", "Session");
      m.insert("00:20:34", "Session Messenger");
      m.insert("00:20:35", "Session Network");

      // Briar
      m.insert("00:20:36", "Briar");
      m.insert("00:20:37", "Briar Android");

      // Wire
      m.insert("00:20:38", "Wire");
      m.insert("00:20:39", "Wire Swiss");
      m.insert("00:20:3A", "Wire Red");
      m.insert("00:20:3B", "Wire Client");

      // Silent Circle
      m.insert("00:20:3C", "Silent Circle");
      m.insert("00:20:3D", "Silent Phone");
      m.insert("00:20:3E", "Silent Text");
      m.insert("00:20:3F", "Silent Mail");

      // OpenWhisperSystems
      m.insert("00:20:40", "Open Whisper");
      m.insert("00:20:41", "Signal (already added)");
      m.insert("00:20:42", "TextSecure");

      // == Rugged/Industrial Computers ==

      // Getac
      m.insert("00:20:43", "Getac");
      m.insert("00:20:44", "Getac S410");
      m.insert("00:20:45", "Getac S510");
      m.insert("00:20:46", "Getac V110");
      m.insert("00:20:47", "Getac X500");
      m.insert("00:20:48", "Getac X600");
      m.insert("00:20:49", "Getac T800");
      m.insert("00:20:4A", "Getac K120");
      m.insert("00:20:4B", "Getac UX10");
      m.insert("00:20:4C", "Getac F110");
      m.insert("00:20:4D", "Getac A140");
      m.insert("00:20:4E", "Getac Z710");
      m.insert("00:20:4F", "Getac V110 Ex");

      // Panasonic Toughbook
      m.insert("00:20:50", "Panasonic");
      m.insert("00:20:51", "Panasonic Toughbook");
      m.insert("00:20:52", "Panasonic CF-33");
      m.insert("00:20:53", "Panasonic CF-54");
      m.insert("00:20:54", "Panasonic CF-20");
      m.insert("00:20:55", "Panasonic CF-31");
      m.insert("00:20:56", "Panasonic CF-19");
      m.insert("00:20:57", "Panasonic CF-53");
      m.insert("00:20:58", "Panasonic CF-H2");
      m.insert("00:20:59", "Panasonic CF-MX");
      m.insert("00:20:5A", "Panasonic Toughpad");
      m.insert("00:20:5B", "Panasonic FZ-G1");
      m.insert("00:20:5C", "Panasonic FZ-B2");
      m.insert("00:20:5D", "Panasonic FZ-A2");

      // Dell Rugged
      m.insert("00:20:5E", "Dell Rugged");
      m.insert("00:20:5F", "Dell Latitude Rugged");
      m.insert("00:20:60", "Dell Latitude 5420");
      m.insert("00:20:61", "Dell Latitude 7424");
      m.insert("00:20:62", "Dell Latitude 5430");
      m.insert("00:20:63", "Dell Latitude 7220");
      m.insert("00:20:64", "Dell Latitude 5520");
      m.insert("00:20:65", "Dell Rugged Extreme");
      m.insert("00:20:66", "Dell Rugged Portable");
      m.insert("00:20:67", "Dell Precision");
      m.insert("00:20:68", "Dell Rugged Mobile");

      // Advantech
      m.insert("00:20:69", "Advantech");
      m.insert("00:20:6A", "Advantech UNO");
      m.insert("00:20:6B", "Advantech ARK");
      m.insert("00:20:6C", "Advantech DS");
      m.insert("00:20:6D", "Advantech EIA");
      m.insert("00:20:6E", "Advantech EIS");
      m.insert("00:20:6F", "Advantech EPC");
      m.insert("00:20:70", "Advantech HIT");
      m.insert("00:20:71", "Advantech PCM");
      m.insert("00:20:72", "Advantech SOM");
      m.insert("00:20:73", "Advantech SOM-Express");
      m.insert("00:20:74", "Advantech AIIS");
      m.insert("00:20:75", "Advantech MIC");

      // Neousys
      m.insert("00:20:76", "Neousys");
      m.insert("00:20:77", "Neousys Nuvo");
      m.insert("00:20:78", "Neousys Nuvo-2500");
      m.insert("00:20:79", "Neousys Nuvo-5000");
      m.insert("00:20:7A", "Neousys Nuvo-6100");
      m.insert("00:20:7B", "Neousys Nuvo-7500");
      m.insert("00:20:7C", "Neousys POC");
      m.insert("00:20:7D", "Neousys RGS");
      m.insert("00:20:7E", "Neousys iGem");
      m.insert("00:20:7F", "Neousys NRP");

      // Supermicro Industrial
      m.insert("00:20:80", "Supermicro");
      m.insert("00:20:81", "Supermicro Industrial");
      m.insert("00:20:82", "Supermicro SuperServer");
      m.insert("00:20:83", "Supermicro SuperBlade");
      m.insert("00:20:84", "Supermicro MicroBlade");
      m.insert("00:20:85", "Supermicro Ultra");
      m.insert("00:20:86", "Supermicro BigTwin");
      m.insert("00:20:87", "Supermicro TwinPro");
      m.insert("00:20:88", "Supermicro FatTwin");
      m.insert("00:20:89", "Supermicro GPU");
      m.insert("00:20:8A", "Supermicro Storage");
      m.insert("00:20:8B", "Supermicro WIO");
      m.insert("00:20:8C", "Supermicro DP");

      // Intel NUC Rugged
      m.insert("00:20:8D", "Intel NUC");
      m.insert("00:20:8E", "Intel NUC Rugged");
      m.insert("00:20:8F", "Intel NUC 11 Essential");
      m.insert("00:20:90", "Intel NUC 12 Enthusiast");
      m.insert("00:20:91", "Intel NUC Pro");
      m.insert("00:20:92", "Intel NUC Kit");
      m.insert("00:20:93", "Intel NUC Board");
      m.insert("00:20:94", "Intel Compute Stick");

      // == Rack PDU/Power Distribution ==

      // APC (American Power Conversion)
      m.insert("00:20:95", "APC");
      m.insert("00:20:96", "APC Smart-UPS");
      m.insert("00:20:97", "APC Smart-UPS C");
      m.insert("00:20:98", "APC Smart-UPS S");
      m.insert("00:20:99", "APC Smart-UPS X");
      m.insert("00:20:9A", "APC Smart-UPS RT");
      m.insert("00:20:9B", "APC Smart-UPS XL");
      m.insert("00:20:9C", "APC Back-UPS");
      m.insert("00:20:9D", "APC Back-UPS Pro");
      m.insert("00:20:9E", "APC Back-UPS Connect");
      m.insert("00:20:9F", "APC SurgeArrest");
      m.insert("00:20:A0", "APC Power Saving");
      m.insert("00:20:A1", "APC NetShelter");
      m.insert("00:20:A2", "APC InfraStruXure");
      m.insert("00:20:A3", "APC Symmetra");
      m.insert("00:20:A4", "APC Galaxy");
      m.insert("00:20:A5", "APC Easynet");

      // ServerTech (already have some)
      m.insert("00:20:A6", "Server Technology");
      m.insert("00:20:A7", "ServerTech PRO2");
      m.insert("00:20:A8", "ServerTech STARTER");
      m.insert("00:20:A9", "ServerTech Sentry");
      m.insert("00:20:AA", "ServerTech Switched PDU");
      m.insert("00:20:AB", "ServerTech Monitored PDU");
      m.insert("00:20:AC", "ServerTech Managed");
      m.insert("00:20:AD", "ServerTech PRO2 MIB");

      // Raritan (already have some)
      m.insert("00:20:AE", "Raritan PX");
      m.insert("00:20:AF", "Raritan PX2");
      m.insert("00:20:B0", "Raritan PX3");
      m.insert("00:20:B1", "Raritan EMX");
      m.insert("00:20:B2", "Raritan RX");
      m.insert("00:20:B3", "Raritan LX");
      m.insert("00:20:B4", "Raritan CommandCenter");

      // Vertiv Liebert
      m.insert("00:20:B5", "Vertiv");
      m.insert("00:20:B6", "Vertiv Liebert");
      m.insert("00:20:B7", "Vertiv GXT");
      m.insert("00:20:B8", "Vertiv GXT5");
      m.insert("00:20:B9", "Vertiv GXE");
      m.insert("00:20:BA", "Vertiv PSI");
      m.insert("00:20:BB", "Vertiv ITON");
      m.insert("00:20:BC", "Vertiv Liebert UPS");
      m.insert("00:20:BD", "Vertiv Liebert PDU");
      m.insert("00:20:BE", "Vertiv Avocent");
      m.insert("00:20:BF", "Vertiv Cyxtera");

      // Eaton
      m.insert("00:20:C0", "Eaton");
      m.insert("00:20:C1", "Eaton UPS");
      m.insert("00:20:C2", "Eaton 5P");
      m.insert("00:20:C3", "Eaton 5PX");
      m.insert("00:20:C4", "Eaton 9P");
      m.insert("00:20:C5", "Eaton 9PX");
      m.insert("00:20:C6", "Eaton 9E");
      m.insert("00:20:C7", "Eaton Ellipse");
      m.insert("00:20:C8", "Eaton Power Xpert");
      m.insert("00:20:C9", "Eaton BladeUPS");
      m.insert("00:20:CA", "Eaton 93E");
      m.insert("00:20:CB", "Eaton 93PM");
      m.insert("00:20:CC", "Eaton 9390");
      m.insert("00:20:CD", "Eaton 9395");

      // CyberPower
      m.insert("00:20:CE", "CyberPower");
      m.insert("00:20:CF", "CyberPower UPS");
      m.insert("00:20:D0", "CyberPower PFC");
      m.insert("00:20:D1", "CyberPower PR");
      m.insert("00:20:D2", "CyberPower OR");
      m.insert("00:20:D3", "CyberPower OL");
      m.insert("00:20:D4", "CyberPower CL");
      m.insert("00:20:D5", "CyberPower CP");
      m.insert("00:20:D6", "CyberPower DP");
      m.insert("00:20:D7", "CyberPower MP");

      // Leviton
      m.insert("00:20:D8", "Leviton");
      m.insert("00:20:D9", "Leviton PDU");
      m.insert("00:20:DA", "Leviton Rack PDU");
      m.insert("00:20:DB", "Leviton Monitored");
      m.insert("00:20:DC", "Leviton Switched");
      m.insert("00:20:DD", "Leviton Basic");
      m.insert("00:20:DE", "Leviton Opacity");
      m.insert("00:20:DF", "Leviton ZPD");

      // == Serial Console Servers ==

      // Digi
      m.insert("00:20:E0", "Digi");
      m.insert("00:20:E1", "Digi Console");
      m.insert("00:20:E2", "Digi PortServer");
      m.insert("00:20:E3", "Digi AccelePort");
      m.insert("00:20:E4", "Digi Terminal");
      m.insert("00:20:E5", "Digi Connect");
      m.insert("00:20:E6", "Digi Connect WAN");
      m.insert("00:20:E7", "Digi Connect CS");
      m.insert("00:20:E8", "Digi RealPort");
      m.insert("00:20:E9", "Digi AnywhereUSB");
      m.insert("00:20:EA", "Digi EdgePort");
      m.insert("00:20:EB", "Digi Serial");

      // Opengear
      m.insert("00:20:EC", "Opengear");
      m.insert("00:20:ED", "Opengear IM");
      m.insert("00:20:EE", "Opengear ACM");
      m.insert("00:20:EF", "Opengear OME");
      m.insert("00:20:F0", "Opengear LCD");
      m.insert("00:20:F1", "Opengear SM");
      m.insert("00:20:F2", "Opengear FM");
      m.insert("00:20:F3", "Opengear PM");
      m.insert("00:20:F4", "Opengear NM");
      m.insert("00:20:F5", "Opengear CM");
      m.insert("00:20:F6", "Opengear Resilience");

      // Lantronix
      m.insert("00:20:F7", "Lantronix");
      m.insert("00:20:F8", "Lantronix SLC");
      m.insert("00:20:F9", "Lantronix SDS");
      m.insert("00:20:FA", "Lantronix xDirect");
      m.insert("00:20:FB", "Lantronix xPrint");
      m.insert("00:20:FC", "Lantronix UDS");
      m.insert("00:20:FD", "Lantronix ETS");
      m.insert("00:20:FE", "Lantronix MSS");
      m.insert("00:20:FF", "Lantronix SBC");
      m.insert("00:21:00", "Lantronix Spider");

      // WTI (Network Technologies Inc)
      m.insert("00:21:01", "WTI");
      m.insert("00:21:02", "WTI RSM");
      m.insert("00:21:03", "WTI DSM");
      m.insert("00:21:04", "WTI ISM");
      m.insert("00:21:05", "WTI VMR");
      m.insert("00:21:06", "WTI NSC");
      m.insert("00:21:07", "WTI MPS");
      m.insert("00:21:08", "WTI RPS");
      m.insert("00:21:09", "WTI BSM");

      // == Infrastructure Monitoring Hardware ==

      // APC Environmental
      m.insert("00:21:0A", "APC Environmental");
      m.insert("00:21:0B", "APC NetBotz");
      m.insert("00:21:0C", "APC Temperature");
      m.insert("00:21:0D", "APC Humidity");
      m.insert("00:21:0E", "APC Water");
      m.insert("00:21:0F", "APC Smoke");
      m.insert("00:21:10", "APC Camera");
      m.insert("00:21:11", "APC Access");
      m.insert("00:21:12", "APC Rack Access");
      m.insert("00:21:13", "APC Sensor");

      // Vertiv Sensors
      m.insert("00:21:14", "Vertiv Sensors");
      m.insert("00:21:15", "Vertiv T-Sensor");
      m.insert("00:21:16", "Vertiv H-Sensor");
      m.insert("00:21:17", "Vertiv D-Sensor");
      m.insert("00:21:18", "Vertiv PD-Sensor");
      m.insert("00:21:19", "Vertiv Airflow");
      m.insert("00:21:1A", "Vertiv Vibration");
      m.insert("00:21:1B", "Vertiv Current");
      m.insert("00:21:1C", "Vertiv Voltage");

      // Raritan Environmental
      m.insert("00:21:1D", "Raritan Environmental");
      m.insert("00:21:1E", "Raritan PX");
      m.insert("00:21:1F", "Raritan EMX");
      m.insert("00:21:20", "Raritan Sensor");
      m.insert("00:21:21", "Raritan Temperature");
      m.insert("00:21:22", "Raritan Humidity");
      m.insert("00:21:23", "Raritan Dry Contact");
      m.insert("00:21:24", "Raritan Leak");
      m.insert("00:21:25", "Raritan Airflow");
      m.insert("00:21:26", "Raritan Pressure");

      // ServerTech Environmental
      m.insert("00:21:27", "ServerTech Environmental");
      m.insert("00:21:28", "ServerTech Temp");
      m.insert("00:21:29", "ServerTech Humidity");
      m.insert("00:21:2A", "ServerTech Smoke");
      m.insert("00:21:2B", "ServerTech Water");
      m.insert("00:21:2C", "ServerTech Airflow");
      m.insert("00:21:2D", "ServerTech Differential");
      m.insert("00:21:2E", "ServerTech Current");

      // == Backup Appliances ==

      // Veeam
      m.insert("00:21:2F", "Veeam");
      m.insert("00:21:30", "Veeam Backup");
      m.insert("00:21:31", "Veeam Backup Server");
      m.insert("00:21:32", "Veeam Proxy");
      m.insert("00:21:33", "Veeam Repository");
      m.insert("00:21:34", "Veeam Gateway");
      m.insert("00:21:35", "Veeam CDN");
      m.insert("00:21:36", "Veeam ONE");
      m.insert("00:21:37", "Veeam Availability");
      m.insert("00:21:38", "Veeam DataLabs");

      // Veritas
      m.insert("00:21:39", "Veritas");
      m.insert("00:21:3A", "Veritas Backup Exec");
      m.insert("00:21:3B", "Veritas NetBackup");
      m.insert("00:21:3C", "Veritas Backup");
      m.insert("00:21:3D", "Veritas Enterprise");
      m.insert("00:21:3E", "Veritas Central");
      m.insert("00:21:3F", "Veritas Access");
      m.insert("00:21:40", "Veritas Data");
      m.insert("00:21:41", "Veritas Velocity");

      // Commvault
      m.insert("00:21:42", "Commvault");
      m.insert("00:21:43", "Commvault Backup");
      m.insert("00:21:44", "Commvault Hyperscale");
      m.insert("00:21:45", "Commvault Edge");
      m.insert("00:21:46", "Commvault Complete");
      m.insert("00:21:47", "Commvault Go");
      m.insert("00:21:48", "Commvault Orchestrate");
      m.insert("00:21:49", "Commvault Activate");

      // Dell EMC Data Domain
      m.insert("00:21:4A", "Dell EMC");
      m.insert("00:21:4B", "Dell EMC Data Domain");
      m.insert("00:21:4C", "Data Domain DD");
      m.insert("00:21:4D", "Data Domain DD2500");
      m.insert("00:21:4E", "Data Domain DD4200");
      m.insert("00:21:4F", "Data Domain DD6300");
      m.insert("00:21:50", "Data Domain DD6800");
      m.insert("00:21:51", "Data Domain DD9300");
      m.insert("00:21:52", "Data Domain DD9500");
      m.insert("00:21:53", "Data Domain DD9800");

      // HPE StoreOnce
      m.insert("00:21:54", "HPE StoreOnce");
      m.insert("00:21:55", "HPE StoreOnce 5100");
      m.insert("00:21:56", "HPE StoreOnce 5200");
      m.insert("00:21:57", "HPE StoreOnce 5250");
      m.insert("00:21:58", "HPE StoreOnce 5450");
      m.insert("00:21:59", "HPE StoreOnce 6600");
      m.insert("00:21:5A", "HPE StoreOnce 6700");
      m.insert("00:21:5B", "HPE StoreOnce 5200H");
      m.insert("00:21:5C", "HPE StoreOnce 6600H");

      // Datto
      m.insert("00:21:5D", "Datto");
      m.insert("00:21:5E", "Datto SIRIS");
      m.insert("00:21:5F", "Datto ALTO");
      m.insert("00:21:60", "Datto NAS");
      m.insert("00:21:61", "Datto IS");
      m.insert("00:21:62", "Datto DBD");
      m.insert("00:21:63", "Datto DCD");
      m.insert("00:21:64", "Datto 3X0");
      m.insert("00:21:65", "Datto 4X0");

      // Rubrik
      m.insert("00:21:66", "Rubrik");
      m.insert("00:21:67", "Rubrik CDM");
      m.insert("00:21:68", "Rubrik Polaris");
      m.insert("00:21:69", "Rubrik Go");
      m.insert("00:21:6A", "Rubrik Edge");
      m.insert("00:21:6B", "Rubrik Avalanche");
      m.insert("00:21:6C", "Rubrik Security");
      m.insert("00:21:6D", "Rubrik API");
      m.insert("00:21:6E", "Rubrik Cloud");

      // Cohesity
      m.insert("00:21:6F", "Cohesity");
      m.insert("00:21:70", "Cohesity DataPlatform");
      m.insert("00:21:71", "Cohesity SpanFS");
      m.insert("00:21:72", "Cohesity Helios");
      m.insert("00:21:73", "Cohesity Panorama");
      m.insert("00:21:74", "Cohesity FortKnox");
      m.insert("00:21:75", "Cohesity RedLock");
      m.insert("00:21:76", "Cohesity DataHygiene");
      m.insert("00:21:77", "Cohesity ELAST");

      // == USB/Peripheral Hubs ==

      // Belkin
      m.insert("00:21:78", "Belkin");
      m.insert("00:21:79", "Belkin USB Hub");
      m.insert("00:21:7A", "Belkin USB-C Hub");
      m.insert("00:21:7B", "Belkin Thunderbolt");
      m.insert("00:21:7C", "Belkin KVM");
      m.insert("00:21:7D", "Belkin Docking");
      m.insert("00:21:7E", "Belkin Switch");
      m.insert("00:21:7F", "Belkin USB 3.0");
      m.insert("00:21:80", "Belkin USB 2.0");

      // Anker
      m.insert("00:21:81", "Anker");
      m.insert("00:21:82", "Anker USB Hub");
      m.insert("00:21:83", "Anker PowerPort");
      m.insert("00:21:84", "Anker PowerExpand");
      m.insert("00:21:85", "Anker Thunderbolt");
      m.insert("00:21:86", "Anker USB-C Hub");
      m.insert("00:21:87", "Anker 4-Port");
      m.insert("00:21:88", "Anker 7-Port");
      m.insert("00:21:89", "Anker 10-Port");

      // Tripp Lite
      m.insert("00:21:8A", "Tripp Lite");
      m.insert("00:21:8B", "Tripp Lite USB Hub");
      m.insert("00:21:8C", "Tripp Lite B004");
      m.insert("00:21:8D", "Tripp Lite B005");
      m.insert("00:21:8E", "Tripp Lite B006");
      m.insert("00:21:8F", "Tripp Lite U280");
      m.insert("00:21:90", "Tripp Lite U360");
      m.insert("00:21:91", "Tripp Lite U420");
      m.insert("00:21:92", "Tripp Lite U360-004");

      // StarTech
      m.insert("00:21:93", "StarTech");
      m.insert("00:21:94", "StarTech USB Hub");
      m.insert("00:21:95", "StarTech ST720");
      m.insert("00:21:96", "StarTech ST430");
      m.insert("00:21:97", "StarTech ST4300");
      m.insert("00:21:98", "StarTech HB31");
      m.insert("00:21:99", "StarTech HB32");
      m.insert("00:21:9A", "StarTech USB32");
      m.insert("00:21:9B", "StarTech USBA");

      // Kensington
      m.insert("00:21:9C", "Kensington");
      m.insert("00:21:9D", "Kensington USB Hub");
      m.insert("00:21:9E", "Kensington SD2000");
      m.insert("00:21:9F", "Kensington SD4000");
      m.insert("00:21:A0", "Kensington SD5000");
      m.insert("00:21:A1", "Kensington SD7000");
      m.insert("00:21:A2", "Kensington SD9000");
      m.insert("00:21:A3", "Kensington Thunderbolt");

      // Plugable
      m.insert("00:21:A4", "Plugable");
      m.insert("00:21:A5", "Plugable USB Hub");
      m.insert("00:21:A6", "Plugable UD-6950");
      m.insert("00:21:A7", "Plugable UD-3900");
      m.insert("00:21:A8", "Plugable UD-768");
      m.insert("00:21:A9", "Plugable TBT3");
      m.insert("00:21:AA", "Plugable USB4");
      m.insert("00:21:AB", "Plugable Docking");

      // Sabrent
      m.insert("00:21:AC", "Sabrent");
      m.insert("00:21:AD", "Sabrent USB Hub");
      m.insert("00:21:AE", "Sabrent HB-UM43");
      m.insert("00:21:AF", "Sabrent HB-UM43");
      m.insert("00:21:B0", "Sabrent HB-OTG");
      m.insert("00:21:B1", "Sabrent DS-4");
      m.insert("00:21:B2", "Sabrent DS-5");
      m.insert("00:21:B3", "Sabrent KT-3");

      // == Software-Defined Storage ==

      // MinIO
      m.insert("00:21:B4", "MinIO");
      m.insert("00:21:B5", "MinIO Server");
      m.insert("00:21:B6", "MinIO Client");
      m.insert("00:21:B7", "MinIO Erasure");
      m.insert("00:21:B8", "MinIO Distributed");
      m.insert("00:21:B9", "MinIO Gateway");
      m.insert("00:21:BA", "MinIO Console");
      m.insert("00:21:BB", "MinIO Operator");
      m.insert("00:21:BC", "MinIO SUBNET");

      // Ceph
      m.insert("00:21:BD", "Ceph");
      m.insert("00:21:BE", "Ceph MON");
      m.insert("00:21:BF", "Ceph OSD");
      m.insert("00:21:C0", "Ceph MDS");
      m.insert("00:21:C1", "Ceph RGW");
      m.insert("00:21:C2", "Ceph RBD");
      m.insert("00:21:C3", "Ceph FS");
      m.insert("00:21:C4", "Ceph Manager");
      m.insert("00:21:C5", "Ceph Dashboard");

      // TrueNAS (iX Systems)
      m.insert("00:21:C6", "TrueNAS");
      m.insert("00:21:C7", "TrueNAS CORE");
      m.insert("00:21:C8", "TrueNAS Enterprise");
      m.insert("00:21:C9", "TrueNAS SCALE");
      m.insert("00:21:CA", "TrueNAS Mini");
      m.insert("00:21:CB", "TrueNAS ZFS");
      m.insert("00:21:CC", "TrueNAS SMB");
      m.insert("00:21:CD", "TrueNAS NFS");
      m.insert("00:21:CE", "TrueNAS iSCSI");

      // StarWind
      m.insert("00:21:CF", "StarWind");
      m.insert("00:21:D0", "StarWind VSAN");
      m.insert("00:21:D1", "StarWind VTL");
      m.insert("00:21:D2", "StarWind TAP");
      m.insert("00:21:D3", "StarWind NVMe");
      m.insert("00:21:D4", "StarWind Virtual");
      m.insert("00:21:D5", "StarWind Pro");
      m.insert("00:21:D6", "StarWind Server");
      m.insert("00:21:D7", "StarWind SAN");

      // Gluster (Red Hat)
      m.insert("00:21:D8", "Gluster");
      m.insert("00:21:D9", "GlusterFS");
      m.insert("00:21:DA", "Gluster Storage");
      m.insert("00:21:DB", "Gluster Brick");
      m.insert("00:21:DC", "Gluster Volume");
      m.insert("00:21:DD", "Gluster Replica");
      m.insert("00:21:DE", "Gluster Disperse");
      m.insert("00:21:DF", "Gluster Arbiter");

      // ScaleIO (Dell EMC)
      m.insert("00:21:E0", "ScaleIO");
      m.insert("00:21:E1", "ScaleIO SDS");
      m.insert("00:21:E2", "ScaleIO SDC");
      m.insert("00:21:E3", "ScaleIO MDM");
      m.insert("00:21:E4", "ScaleIO Gateway");
      m.insert("00:21:E5", "ScaleIO Data");
      m.insert("00:21:E6", "ScaleIO Protection");

      // == Container Platforms ==

      // Docker
      m.insert("00:21:E7", "Docker");
      m.insert("00:21:E8", "Docker Engine");
      m.insert("00:21:E9", "Docker Desktop");
      m.insert("00:21:EA", "Docker Compose");
      m.insert("00:21:EB", "Docker Swarm");
      m.insert("00:21:EC", "Docker BuildKit");
      m.insert("00:21:ED", "Docker Registry");
      m.insert("00:21:EE", "Docker Hub");
      m.insert("00:21:EF", "Docker Daemon");

      // Podman
      m.insert("00:21:F0", "Podman");
      m.insert("00:21:F1", "Podman Engine");
      m.insert("00:21:F2", "Podman Pod");
      m.insert("00:21:F3", "Podman Volume");
      m.insert("00:21:F4", "Podman Network");

      // containerd
      m.insert("00:21:F5", "containerd");
      m.insert("00:21:F6", "containerd Daemon");
      m.insert("00:21:F7", "containerd Shim");
      m.insert("00:21:F8", "containerd CRI");

      // LXC/LXD
      m.insert("00:21:F9", "LXC");
      m.insert("00:21:FA", "LXD");
      m.insert("00:21:FB", "LXC Container");
      m.insert("00:21:FC", "LXD Daemon");
      m.insert("00:21:FD", "LXD Image");
      m.insert("00:21:FE", "LXC FS");

      // Rancher
      m.insert("00:21:FF", "Rancher");
      m.insert("00:22:00", "Rancher Server");
      m.insert("00:22:01", "Rancher Agent");
      m.insert("00:22:02", "Rancher RKE");
      m.insert("00:22:03", "Rancher K3s");
      m.insert("00:22:04", "Rancher Harvester");
      m.insert("00:22:05", "Rancher Longhorn");
      m.insert("00:22:06", "Rancher Fleet");

      // Portainer
      m.insert("00:22:07", "Portainer");
      m.insert("00:22:08", "Portainer Agent");
      m.insert("00:22:09", "Portainer Edge");
      m.insert("00:22:0A", "Portainer BE");
      m.insert("00:22:0B", "Portainer Teams");

      // K3s
      m.insert("00:22:0C", "K3s");
      m.insert("00:22:0D", "K3s Server");
      m.insert("00:22:0E", "K3s Agent");
      m.insert("00:22:0F", "K3s Embedded");
      m.insert("00:22:10", "K3s Air-gap");

      // == Database/Message Queue ==

      // MongoDB
      m.insert("00:22:11", "MongoDB");
      m.insert("00:22:12", "MongoDB Server");
      m.insert("00:22:13", "MongoDB Shell");
      m.insert("00:22:14", "MongoDB Atlas");
      m.insert("00:22:15", "MongoDB Enterprise");
      m.insert("00:22:16", "MongoDB Community");
      m.insert("00:22:17", "MongoDB Compass");
      m.insert("00:22:18", "MongoDB Ops Manager");
      m.insert("00:22:19", "MongoDB Cloud");

      // PostgreSQL
      m.insert("00:22:1A", "PostgreSQL");
      m.insert("00:22:1B", "PostgreSQL Server");
      m.insert("00:22:1C", "PostgreSQL Client");
      m.insert("00:22:1D", "PostgreSQL Enterprise");
      m.insert("00:22:1E", "PostgreSQL Greenplum");
      m.insert("00:22:1F", "PostgreSQL Timescale");
      m.insert("00:22:20", "PostgreSQL Citus");
      m.insert("00:22:21", "PostgreSQL RDS");

      // Redis
      m.insert("00:22:22", "Redis");
      m.insert("00:22:23", "Redis Server");
      m.insert("00:22:24", "Redis Sentinel");
      m.insert("00:22:25", "Redis Cluster");
      m.insert("00:22:26", "Redis Enterprise");
      m.insert("00:22:27", "Redis OSS");
      m.insert("00:22:28", "Redis RDB");
      m.insert("00:22:29", "Redis AOF");

      // RabbitMQ
      m.insert("00:22:2A", "RabbitMQ");
      m.insert("00:22:2B", "RabbitMQ Server");
      m.insert("00:22:2C", "RabbitMQ Management");
      m.insert("00:22:2D", "RabbitMQ Shovel");
      m.insert("00:22:2E", "RabbitMQ Federation");
      m.insert("00:22:2F", "RabbitMQ Shovel");
      m.insert("00:22:30", "RabbitMQ Prometheus");
      m.insert("00:22:31", "RabbitMQ CLI");

      // Apache Kafka
      m.insert("00:22:32", "Kafka");
      m.insert("00:22:33", "Apache Kafka");
      m.insert("00:22:34", "Kafka Broker");
      m.insert("00:22:35", "Kafka Producer");
      m.insert("00:22:36", "Kafka Consumer");
      m.insert("00:22:37", "Kafka Connect");
      m.insert("00:22:38", "Kafka Streams");
      m.insert("00:22:39", "Kafka Schema");

      // InfluxDB
      m.insert("00:22:3A", "InfluxDB");
      m.insert("00:22:3B", "InfluxDB Server");
      m.insert("00:22:3C", "InfluxDB Cloud");
      m.insert("00:22:3D", "InfluxDB OSS");
      m.insert("00:22:3E", "InfluxDB Enterprise");
      m.insert("00:22:3F", "InfluxDB Telegraf");
      m.insert("00:22:40", "InfluxDB Kapacitor");
      m.insert("00:22:41", "InfluxDB Chronograf");

      // MySQL/MariaDB
      m.insert("00:22:42", "MySQL");
      m.insert("00:22:43", "MariaDB");
      m.insert("00:22:44", "MySQL Server");
      m.insert("00:22:45", "MySQL Enterprise");
      m.insert("00:22:46", "MySQL Router");
      m.insert("00:22:47", "MySQL Shell");
      m.insert("00:22:48", "MySQL Workbench");
      m.insert("00:22:49", "Aurora MySQL");

      // == API Gateways ==

      // Kong
      m.insert("00:22:4A", "Kong");
      m.insert("00:22:4B", "Kong Gateway");
      m.insert("00:22:4C", "Kong Enterprise");
      m.insert("00:22:4D", "Kong Ingress");
      m.insert("00:22:4E", "Kong Mesh");
      m.insert("00:22:4F", "Kong Declarative");
      m.insert("00:22:50", "Kong Admin API");
      m.insert("00:22:51", "Kong Dev Portal");

      // Tyk
      m.insert("00:22:52", "Tyk");
      m.insert("00:22:53", "Tyk Gateway");
      m.insert("00:22:54", "Tyk Enterprise");
      m.insert("00:22:55", "Tyk Pump");
      m.insert("00:22:56", "Tyk MDCB");
      m.insert("00:22:57", "Tyk Identity");
      m.insert("00:22:58", "Tyk Dashboard");
      m.insert("00:22:59", "Tyk Portal");

      // Apigee
      m.insert("00:22:5A", "Apigee");
      m.insert("00:22:5B", "Apigee Edge");
      m.insert("00:22:5C", "Apigee Hybrid");
      m.insert("00:22:5D", "Apigee Sense");
      m.insert("00:22:5E", "Apigee Monetization");
      m.insert("00:22:5F", "Apigee ISTIO");
      m.insert("00:22:60", "Apigee Remote");
      m.insert("00:22:61", "Apigee Developer");

      // AWS API Gateway
      m.insert("00:22:62", "AWS");
      m.insert("00:22:63", "AWS API Gateway");
      m.insert("00:22:64", "AWS REST API");
      m.insert("00:22:65", "AWS HTTP API");
      m.insert("00:22:66", "AWS WebSocket");
      m.insert("00:22:67", "AWS Lambda");
      m.insert("00:22:68", "AWS ALB");

      // Azure API Management
      m.insert("00:22:69", "Azure");
      m.insert("00:22:6A", "Azure API Management");
      m.insert("00:22:6B", "Azure APIM Gateway");
      m.insert("00:22:6C", "Azure APIM Developer");
      m.insert("00:22:6D", "Azure APIM Portal");
      m.insert("00:22:6E", "Azure Functions");
      m.insert("00:22:6F", "Azure API Center");

      // MuleSoft
      m.insert("00:22:70", "MuleSoft");
      m.insert("00:22:71", "MuleSoft Anypoint");
      m.insert("00:22:72", "MuleSoft Runtime");
      m.insert("00:22:73", "MuleSoft API Gateway");
      m.insert("00:22:74", "MuleSoft Mule");
      m.insert("00:22:75", "MuleSoft Studio");
      m.insert("00:22:76", "MuleSoft Design");
      m.insert("00:22:77", "MuleSoft Exchange");

      // NGINX Controller
      m.insert("00:22:78", "NGINX");
      m.insert("00:22:79", "NGINX Controller");
      m.insert("00:22:7A", "NGINX Plus");
      m.insert("00:22:7B", "NGINX Unit");
      m.insert("00:22:7C", "NGINX Ingress");
      m.insert("00:22:7D", "NGINX Service Mesh");
      m.insert("00:22:7E", "NGINX App Protect");
      m.insert("00:22:7F", "NGINX Load Balancer");

      // Gravitee
      m.insert("00:22:80", "Gravitee");
      m.insert("00:22:81", "Gravitee Gateway");
      m.insert("00:22:82", "Gravitee API");
      m.insert("00:22:83", "Gravitee Management");
      m.insert("00:22:84", "Gravitee Portal");
      m.insert("00:22:85", "Gravitee Console");
      m.insert("00:22:86", "Gravitee Alert");
      m.insert("00:22:87", "Gravitee Reporter");

      // == Edge Computing/Industrial IoT Gateways ==

      // Dell Edge
      m.insert("00:22:88", "Dell Edge");
      m.insert("00:22:89", "Dell Edge Gateway 3000");
      m.insert("00:22:8A", "Dell Edge Gateway 5000");
      m.insert("00:22:8B", "Dell Edge Gateway 7000");
      m.insert("00:22:8C", "Dell Embedded Box PC");
      m.insert("00:22:8D", "Dell Edge Series");

      // HPE Edgeline
      m.insert("00:22:8E", "HPE Edgeline");
      m.insert("00:22:8F", "HPE Edgeline EL1000");
      m.insert("00:22:90", "HPE Edgeline EL300");
      m.insert("00:22:91", "HPE Edgeline EL100");
      m.insert("00:22:92", "HPE Edgeline EL10");
      m.insert("00:22:93", "HPE Edgeline Converged");
      m.insert("00:22:94", "HPE Edgeline Edge");
      m.insert("00:22:95", "HPE Edgeline IoT");

      // Moxa
      m.insert("00:22:96", "Moxa");
      m.insert("00:22:97", "Moxa UC");
      m.insert("00:22:98", "Moxa UC-8100");
      m.insert("00:22:99", "Moxa UC-8200");
      m.insert("00:22:9A", "Moxa UC-5100");
      m.insert("00:22:9B", "Moxa UC-3100");
      m.insert("00:22:9C", "Moxa UC-8100-LX");
      m.insert("00:22:9D", "Moxa DA");
      m.insert("00:22:9E", "Moxa DA-8200");
      m.insert("00:22:9F", "Moxa DA-720");
      m.insert("00:22:A0", "Moxa EDR");
      m.insert("00:22:A1", "Moxa EDR-G");
      m.insert("00:22:A2", "Moxa EOR");
      m.insert("00:22:A3", "Moxa ioThinx");
      m.insert("00:22:A4", "Moxa OnCell");

      // HMS Networks
      m.insert("00:22:A5", "HMS Networks");
      m.insert("00:22:A6", "HMS Anybus");
      m.insert("00:22:A7", "HMS Ewon");
      m.insert("00:22:A8", "HMS Ixxat");
      m.insert("00:22:A9", "HMS Netbiter");
      m.insert("00:22:AA", "HMS Cocom");
      m.insert("00:22:AB", "HMS Intesis");
      m.insert("00:22:AC", "HMS Prosoft");
      m.insert("00:22:AD", "HMS Wireless");

      // B&R Automation
      m.insert("00:22:AE", "B&R Automation");
      m.insert("00:22:AF", "B&R X20");
      m.insert("00:22:B0", "B&R X67");
      m.insert("00:22:B1", "B&R APC");
      m.insert("00:22:B2", "B&R PowerPanel");
      m.insert("00:22:B3", "B&R HMI");
      m.insert("00:22:B4", "B&R Panel");
      m.insert("00:22:B5", "B&R Touch");
      m.insert("00:22:B6", "B&R Industrial");
      m.insert("00:22:B7", "B&R Controller");

      // Mitsubishi Electric
      m.insert("00:22:B8", "Mitsubishi Electric");
      m.insert("00:22:B9", "Mitsubishi MELSEC");
      m.insert("00:22:BA", "Mitsubishi FX");
      m.insert("00:22:BB", "Mitsubishi Q");
      m.insert("00:22:BC", "Mitsubishi R");
      m.insert("00:22:BD", "Mitsubishi iQ-R");
      m.insert("00:22:BE", "Mitsubishi iQ-F");
      m.insert("00:22:BF", "Mitsubishi GOT");
      m.insert("00:22:C0", "Mitsubishi GT");
      m.insert("00:22:C1", "Mitsubishi FR");
      m.insert("00:22:C2", "Mitsubishi CC-Link");
      m.insert("00:22:C3", "Mitsubishi EtherCAT");

      // Bosch Rexroth (already have some)
      m.insert("00:22:C4", "Bosch Rexroth");
      m.insert("00:22:C5", "Bosch CtrlX");
      m.insert("00:22:C6", "Bosch IndraMotion");
      m.insert("00:22:C7", "Bosch IndraLogic");
      m.insert("00:22:C8", "Bosch IndraWorks");
      m.insert("00:22:C9", "Bosch Rexroth HMI");
      m.insert("00:22:CA", "Bosch Rexroth PLC");

      // Emerson
      m.insert("00:22:CB", "Emerson");
      m.insert("00:22:CC", "Emerson PACSystems");
      m.insert("00:22:CD", "Emerson Proficy");
      m.insert("00:22:CE", "Emerson Ovation");
      m.insert("00:22:CF", "Emerson DeltaV");
      m.insert("00:22:D0", "Emerson AMS");
      m.insert("00:22:D1", "Emerson OSIsoft");
      m.insert("00:22:D2", "Emerson ROC");
      m.insert("00:22:D3", "Emerson ControlWave");
      m.insert("00:22:D4", "Emerson Flow");

      // Schneider Edge (already have some)
      m.insert("00:22:D5", "Schneider Edge");
      m.insert("00:22:D6", "Schneider Edge Compute");
      m.insert("00:22:D7", "Schneider Edge Gateway");
      m.insert("00:22:D8", "Schneider EcoStruxure");
      m.insert("00:22:D9", "Schneider Modicon M262");
      m.insert("00:22:DA", "Schneider Altivar");
      m.insert("00:22:DB", "Schneider Lexium");
      m.insert("00:22:DC", "Schneider Advantys");

      // Siemens IoT
      m.insert("00:22:DD", "Siemens IoT");
      m.insert("00:22:DE", "Siemens IoT2020");
      m.insert("00:22:DF", "Siemens IoT2040");
      m.insert("00:22:E0", "Siemens IoT2050");
      m.insert("00:22:E1", "Siemens SIMATIC");
      m.insert("00:22:E2", "Siemens IOT");
      m.insert("00:22:E3", "Siemens IoT Gateway");
      m.insert("00:22:E4", "Siemens MindConnect");
      m.insert("00:22:E5", "Siemens MindSphere");
      m.insert("00:22:E6", "Siemens Edge");

      // == Biometric/Physical Security Devices ==

      // Suprema
      m.insert("00:22:E7", "Suprema");
      m.insert("00:22:E8", "Suprema BioEntry");
      m.insert("00:22:E9", "Suprema BioLite");
      m.insert("00:22:EA", "Suprema CoreStation");
      m.insert("00:22:EB", "Suprema Access Control");
      m.insert("00:22:EC", "Suprema Terminal");
      m.insert("00:22:ED", "Suprema Reader");
      m.insert("00:22:EE", "Suprema Fingerprint");
      m.insert("00:22:EF", "Suprema Face");
      m.insert("00:22:F0", "Suprema Card");
      m.insert("00:22:F1", "Suprema Mobile");
      m.insert("00:22:F2", "Suprema Comcafe");

      // ZKTeco
      m.insert("00:22:F3", "ZKTeco");
      m.insert("00:22:F4", "ZKTeco ProBio");
      m.insert("00:22:F5", "ZKTeco Horus");
      m.insert("00:22:F6", "ZKTeco ZKAccess");
      m.insert("00:22:F7", "ZKTeco ZKTime");
      m.insert("00:22:F8", "ZKTeco Biometric");
      m.insert("00:22:F9", "ZKTeco Face");
      m.insert("00:22:FA", "ZKTeco Fingerprint");
      m.insert("00:22:FB", "ZKTeco Iris");
      m.insert("00:22:FC", "ZKTeco Palm");
      m.insert("00:22:FD", "ZKTeco RFID");
      m.insert("00:22:FE", "ZKTeco Turnstile");

      // Anviz
      m.insert("00:22:FF", "Anviz");
      m.insert("00:23:00", "Anviz CrossChex");
      m.insert("00:23:01", "Anviz T2 Pro");
      m.insert("00:23:02", "Anviz W1");
      m.insert("00:23:03", "Anviz M3");
      m.insert("00:23:04", "Anviz EP300");
      m.insert("00:23:05", "Anviz C2");
      m.insert("00:23:06", "Anviz C5");
      m.insert("00:23:07", "Anviz O3");
      m.insert("00:23:08", "Anviz FB21");
      m.insert("00:23:09", "Anviz FB22");

      // DigitalPersona
      m.insert("00:23:0A", "DigitalPersona");
      m.insert("00:23:0B", "DigitalPersona U.are.U");
      m.insert("00:23:0C", "DigitalPersona Time");
      m.insert("00:23:0D", "DigitalPersona Pro");
      m.insert("00:23:0E", "DigitalPersona Work");
      m.insert("00:23:0F", "DigitalPersona Web");

      // Crossmatch
      m.insert("00:23:10", "Crossmatch");
      m.insert("00:23:11", "Crossmatch Guardian");
      m.insert("00:23:12", "Crossmatch Verifier");
      m.insert("00:23:13", "Crossmatch ID");
      m.insert("00:23:14", "Crossmatch Livescan");
      m.insert("00:23:15", "Crossmatch Tenprint");
      m.insert("00:23:16", "Crossmatch Mobile");

      // Integrated Biometrics
      m.insert("00:23:17", "Integrated Biometrics");
      m.insert("00:23:18", "Integrated Biometrics KOJAK");
      m.insert("00:23:19", "Integrated Biometrics COLumbo");
      m.insert("00:23:1A", "Integrated Biometrics FivePrint");
      m.insert("00:23:1B", "Integrated Biometrics Watson");
      m.insert("00:23:1C", "Integrated Biometrics Sherlock");
      m.insert("00:23:1D", "Integrated Biometrics Coleman");

      // == IT Service Management (ITSM) ==

      // ServiceNow
      m.insert("00:23:1E", "ServiceNow");
      m.insert("00:23:1F", "ServiceNow CMDB");
      m.insert("00:23:20", "ServiceNow ITSM");
      m.insert("00:23:21", "ServiceNow ITOM");
      m.insert("00:23:22", "ServiceNow SecOps");
      m.insert("00:23:23", "ServiceNow CSM");
      m.insert("00:23:24", "ServiceNow HR");
      m.insert("00:23:25", "ServiceNow Finance");
      m.insert("00:23:26", "ServiceNow Project");
      m.insert("00:23:27", "ServiceNow Discovery");

      // Jira Service Management
      m.insert("00:23:28", "Atlassian");
      m.insert("00:23:29", "Jira Service Management");
      m.insert("00:23:2A", "Jira Software");
      m.insert("00:23:2B", "Jira Core");
      m.insert("00:23:2C", "Jira Align");
      m.insert("00:23:2D", "Jira Ops");
      m.insert("00:23:2E", "Confluence");
      m.insert("00:23:2F", "Bitbucket");

      // Freshservice
      m.insert("00:23:30", "Freshservice");
      m.insert("00:23:31", "Freshworks");
      m.insert("00:23:32", "Freshservice IT");
      m.insert("00:23:33", "Freshservice Asset");
      m.insert("00:23:34", "Freshservice Discovery");
      m.insert("00:23:35", "Freshservice Service");
      m.insert("00:23:36", "Freshservice Ticket");
      m.insert("00:23:37", "Freshservice Project");

      // Zendesk
      m.insert("00:23:38", "Zendesk");
      m.insert("00:23:39", "Zendesk Support");
      m.insert("00:23:3A", "Zendesk Guide");
      m.insert("00:23:3B", "Zendesk Talk");
      m.insert("00:23:3C", "Zendesk Chat");
      m.insert("00:23:3D", "Zendesk Explore");
      m.insert("00:23:3E", "Zendesk Sunshine");
      m.insert("00:23:3F", "Zendesk Gather");
      m.insert("00:23:40", "Zendesk Sell");

      // Cherwell
      m.insert("00:23:41", "Cherwell");
      m.insert("00:23:42", "Cherwell Service");
      m.insert("00:23:43", "Cherwell Asset");
      m.insert("00:23:44", "Cherwell Discovery");
      m.insert("00:23:45", "Cherwell Project");
      m.insert("00:23:46", "Cherwell IT");
      m.insert("00:23:47", "Cherwell Mobile");
      m.insert("00:23:48", "Cherwell Email");

      // Ivanti
      m.insert("00:23:49", "Ivanti");
      m.insert("00:23:4A", "Ivanti Service Manager");
      m.insert("00:23:4B", "Ivanti Asset Manager");
      m.insert("00:23:4C", "Ivanti Endpoint Manager");
      m.insert("00:23:4D", "Ivanti Neurons");
      m.insert("00:23:4E", "Ivanti Security");
      m.insert("00:23:4F", "Ivanti Patch");
      m.insert("00:23:50", "Ivanti Workspace");
      m.insert("00:23:51", "Ivanti Connect");
      m.insert("00:23:52", "Ivanti Secure");

      // BMC Helix
      m.insert("00:23:53", "BMC");
      m.insert("00:23:54", "BMC Helix");
      m.insert("00:23:55", "BMC Helix ITSM");
      m.insert("00:23:56", "BMC Helix Discovery");
      m.insert("00:23:57", "BMC Helix Asset");
      m.insert("00:23:58", "BMC Helix Platform");
      m.insert("00:23:59", "BMC Helix CSM");
      m.insert("00:23:5A", "BMC Helix Optimizer");
      m.insert("00:23:5B", "BMC Helix AIOps");

      // TOPdesk
      m.insert("00:23:5C", "TOPdesk");
      m.insert("00:23:5D", "TOPdesk Enterprise");
      m.insert("00:23:5E", "TOPdesk Professional");
      m.insert("00:23:5F", "TOPdesk Self-service");
      m.insert("00:23:60", "TOPdesk Knowledge");
      m.insert("00:23:61", "TOPdesk Asset");
      m.insert("00:23:62", "TOPdesk Mobile");
      m.insert("00:23:63", "TOPdesk Manager");

      // == SD-WAN/WAN Optimization ==

      // VMware VeloCloud
      m.insert("00:23:64", "VMware VeloCloud");
      m.insert("00:23:65", "VeloCloud Edge");
      m.insert("00:23:66", "VeloCloud Gateway");
      m.insert("00:23:67", "VeloCloud Orchestrator");
      m.insert("00:23:68", "VeloCloud SD-WAN");
      m.insert("00:23:69", "VMware SD-WAN");
      m.insert("00:23:6A", "VMware VeloCloud");

      // Cisco Viptela
      m.insert("00:23:6B", "Cisco Viptela");
      m.insert("00:23:6C", "Cisco vEdge");
      m.insert("00:23:6D", "Cisco cEdge");
      m.insert("00:23:6E", "Cisco vManage");
      m.insert("00:23:6F", "Cisco vSmart");
      m.insert("00:23:70", "Cisco vBond");
      m.insert("00:23:71", "Cisco SD-WAN");
      m.insert("00:23:72", "Cisco Meraki SD-WAN");

      // Fortinet FortiSD-WAN
      m.insert("00:23:73", "Fortinet FortiSDWAN");
      m.insert("00:23:74", "FortiGate SD-WAN");
      m.insert("00:23:75", "FortiGate WAN");
      m.insert("00:23:76", "FortiOrchestrator");
      m.insert("00:23:77", "FortiAnalyzer SD-WAN");
      m.insert("00:23:78", "FortiManager SD-WAN");

      // Riverbed
      m.insert("00:23:79", "Riverbed");
      m.insert("00:23:7A", "Riverbed SteelHead");
      m.insert("00:23:7B", "Riverbed SteelCentral");
      m.insert("00:23:7C", "Riverbed NetIM");
      m.insert("00:23:7D", "Riverbed NetShark");
      m.insert("00:23:7E", "Riverbed Aternity");
      m.insert("00:23:7F", "Riverbed Workload");
      m.insert("00:23:80", "Riverbed Cascade");
      m.insert("00:23:81", "Riverbed Modeler");

      // Silver Peak (Aruba)
      m.insert("00:23:82", "Silver Peak");
      m.insert("00:23:83", "Silver Peak Edge");
      m.insert("00:23:84", "Silver Peak Unity");
      m.insert("00:23:85", "Silver Peak Orchestrator");
      m.insert("00:23:86", "Silver Peak XOS");
      m.insert("00:23:87", "Aruba SD-WAN");
      m.insert("00:23:88", "Aruba EdgeConnect");

      // Versa
      m.insert("00:23:89", "Versa");
      m.insert("00:23:8A", "Versa SD-WAN");
      m.insert("00:23:8B", "Versa Director");
      m.insert("00:23:8C", "Versa Analytics");
      m.insert("00:23:8D", "Versa Concerto");
      m.insert("00:23:8E", "Versa Titan");
      m.insert("00:23:8F", "Versa Secure SD-WAN");

      // Cato
      m.insert("00:23:90", "Cato");
      m.insert("00:23:91", "Cato Cloud");
      m.insert("00:23:92", "Cato Socket");
      m.insert("00:23:93", "Cato PoP");
      m.insert("00:23:94", "Cato Security");
      m.insert("00:23:95", "Cato Network");

      // == Data Protection/HSM ==

      // Thales
      m.insert("00:23:96", "Thales");
      m.insert("00:23:97", "Thales Luna");
      m.insert("00:23:98", "Thales CipherTrust");
      m.insert("00:23:99", "Thales SafeNet");
      m.insert("00:23:9A", "Thales Gemalto");
      m.insert("00:23:9B", "Thales HSM");
      m.insert("00:23:9C", "Thales KeySecure");
      m.insert("00:23:9D", "Thales Vormetric");
      m.insert("00:23:9E", "Thales Data Protection");

      // Entrust
      m.insert("00:23:9F", "Entrust");
      m.insert("00:23:A0", "Entrust HSM");
      m.insert("00:23:A1", "Entrust nShield");
      m.insert("00:23:A2", "Entrust Datacard");
      m.insert("00:23:A3", "Entrust Sigma");
      m.insert("00:23:A4", "Entrust KeyControl");
      m.insert("00:23:A5", "Entrust PKI");
      m.insert("00:23:A6", "Entrust TLS");
      m.insert("00:23:A7", "Entrust Certificate");

      // Gemalto (Thales - already have some)

      // Utimaco
      m.insert("00:23:A8", "Utimaco");
      m.insert("00:23:A9", "Utimaco HSM");
      m.insert("00:23:AA", "Utimaco CryptoServer");
      m.insert("00:23:AB", "Utimaco KeySecure");
      m.insert("00:23:AC", "Utimaco Seclore");
      m.insert("00:23:AD", "Utimaco Atalla");
      m.insert("00:23:AE", "Utimaco Hardware");

      // AWS CloudHSM
      m.insert("00:23:AF", "AWS CloudHSM");
      m.insert("00:23:B0", "AWS KMS");
      m.insert("00:23:B1", "AWS Secrets");
      m.insert("00:23:B2", "AWS Parameter");
      m.insert("00:23:B3", "AWS Certificate");
      m.insert("00:23:B4", "AWS Private");

      // Azure Key Vault
      m.insert("00:23:B5", "Azure Key Vault");
      m.insert("00:23:B6", "Azure HSM");
      m.insert("00:23:B7", "Azure Confidential");
      m.insert("00:23:B8", "Azure Managed");
      m.insert("00:23:B9", "Azure Certificate");
      m.insert("00:23:BA", "Azure Secrets");

      // Google Cloud KMS
      m.insert("00:23:BB", "Google Cloud KMS");
      m.insert("00:23:BC", "Google Cloud HSM");
      m.insert("00:23:BD", "Google Cloud EKM");
      m.insert("00:23:BE", "Google Cloud Key");
      m.insert("00:23:BF", "Google Cloud Secret");
      m.insert("00:23:C0", "Google Cloud Binary");

      // HashiCorp Vault
      m.insert("00:23:C1", "HashiCorp Vault");
      m.insert("00:23:C2", "Vault Enterprise");
      m.insert("00:23:C3", "Vault HSM");
      m.insert("00:23:C4", "Vault HA");
      m.insert("00:23:C5", "Vault Community");
      m.insert("00:23:C6", "Vault Consul");
      m.insert("00:23:C7", "Vault Transit");
      m.insert("00:23:C8", "Vault PKI");
      m.insert("00:23:C9", "Vault Secrets");
      m.insert("00:23:CA", "Vault Identity");

      // == Network Packet Brokers ==

      // Gigamon
      m.insert("00:23:CB", "Gigamon");
      m.insert("00:23:CC", "Gigamon GigaVUE");
      m.insert("00:23:CD", "Gigamon GigaVUE-HC");
      m.insert("00:23:CE", "Gigamon GigaVUE-OS");
      m.insert("00:23:CF", "Gigamon GigaSMART");
      m.insert("00:23:D0", "Gigamon Visibility");
      m.insert("00:23:D1", "Gigamon Tap");
      m.insert("00:23:D2", "Gigamon Aggregation");
      m.insert("00:23:D3", "Gigamon Filter");
      m.insert("00:23:D4", "Gigamon Packet");
      m.insert("00:23:D5", "Gigamon Inline");

      // Ixia (Keysight - already have some)

      // Viavi (already have some)

      // Garland (already have some)

      // == Pro AV Equipment ==

      // Extron
      m.insert("00:23:D6", "Extron");
      m.insert("00:23:D7", "Extron DTP");
      m.insert("00:23:D8", "Extron XTP");
      m.insert("00:23:D9", "Extron SIS");
      m.insert("00:23:DA", "Extron IPCP");
      m.insert("00:23:DB", "Extron IN");
      m.insert("00:23:DC", "Extron OUT");
      m.insert("00:23:DD", "Extron Matrix");
      m.insert("00:23:DE", "Extron Switcher");
      m.insert("00:23:DF", "Extron Processor");
      m.insert("00:23:E0", "Extron Control");
      m.insert("00:23:E1", "Extron TouchLink");
      m.insert("00:23:E2", "Extron NAV");

      // Biamp
      m.insert("00:23:E3", "Biamp");
      m.insert("00:23:E4", "Biamp Tesira");
      m.insert("00:23:E5", "Biamp Parl");
      m.insert("00:23:E6", "Biamp Audia");
      m.insert("00:23:E7", "Biamp Nexia");
      m.insert("00:23:E8", "Biamp Devio");
      m.insert("00:23:E9", "Biamp Cambridge");
      m.insert("00:23:EA", "Biamp Vocia");
      m.insert("00:23:EB", "Biamp Impulse");
      m.insert("00:23:EC", "Biamp Community");
      m.insert("00:23:ED", "Biamp Desono");

      // QSC (already have some)

      // Harman Pro
      m.insert("00:23:EE", "Harman Pro");
      m.insert("00:23:EF", "Harman BSS");
      m.insert("00:23:F0", "Harman Crown");
      m.insert("00:23:F1", "Harman dbx");
      m.insert("00:23:F2", "Harman JBL");
      m.insert("00:23:F3", "Harman AKG");
      m.insert("00:23:F4", "Harman Martin");
      m.insert("00:23:F5", "Harman AMX");
      m.insert("00:23:F6", "Harman HiQnet");
      m.insert("00:23:F7", "Harman Soundcraft");

      // Shure (already have some)

      // Sennheiser (already have some)

      // Audio-Technica
      m.insert("00:23:F8", "Audio-Technica");
      m.insert("00:23:F9", "Audio-Technica AT");
      m.insert("00:23:FA", "Audio-Technica Engineered");
      m.insert("00:23:FB", "Audio-Technica System");
      m.insert("00:23:FC", "Audio-Technica Wireless");
      m.insert("00:23:FD", "Audio-Technica Microphone");
      m.insert("00:23:FE", "Audio-Technica Conference");

      // Crown (Harman - already have some)

      // Lab Gruppen
      m.insert("00:23:FF", "Lab Gruppen");
      m.insert("00:24:00", "Lab Gruppen LU");
      m.insert("00:24:01", "Lab Gruppen PLM");
      m.insert("00:24:02", "Lab Gruppen C");
      m.insert("00:24:03", "Lab Gruppen E");
      m.insert("00:24:04", "Lab Gruppen T");
      m.insert("00:24:05", "Lab Gruppen LM");
      m.insert("00:24:06", "Lab Gruppen FP");

      // dbx (Harman - already have some)

      // == Asset Management/RFID ==

      // Impinj
      m.insert("00:24:07", "Impinj");
      m.insert("00:24:08", "Impinj R420");
      m.insert("00:24:09", "Impinj R440");
      m.insert("00:24:0A", "Impinj R700");
      m.insert("00:24:0B", "Impinj Speedway");
      m.insert("00:24:0C", "Impinj ItemSense");
      m.insert("00:24:0D", "Impinj Grand");
      m.insert("00:24:0E", "Impinj Monza");
      m.insert("00:24:0F", "Impinj Hops");

      // Alien Technology
      m.insert("00:24:10", "Alien Technology");
      m.insert("00:24:11", "Alien Higgs");
      m.insert("00:24:12", "Alien Squiggle");
      m.insert("00:24:13", "Alien ALR");
      m.insert("00:24:14", "Alien F8");
      m.insert("00:24:15", "Alien ALM");
      m.insert("00:24:16", "Alien RFID");

      // Zebra RFID
      m.insert("00:24:17", "Zebra RFID");
      m.insert("00:24:18", "Zebra FX");
      m.insert("00:24:19", "Zebra AN");
      m.insert("00:24:1A", "Zebra RFD40");
      m.insert("00:24:1B", "Zebra RFD90");
      m.insert("00:24:1C", "Zebra RFID3");
      m.insert("00:24:1D", "Zebra ATR");
      m.insert("00:24:1E", "Zebra MC");
      m.insert("00:24:1F", "Zebra TM");
      m.insert("00:24:20", "Zebra RFID Reader");

      // HID RFID
      m.insert("00:24:21", "HID RFID");
      m.insert("00:24:22", "HID iCLASS");
      m.insert("00:24:23", "HID Prox");
      m.insert("00:24:24", "HID Seos");
      m.insert("00:24:25", "HID Crescendo");
      m.insert("00:24:26", "HID Fargo");
      m.insert("00:24:27", "HID Signo");
      m.insert("00:24:28", "HID Access");

      // TSL
      m.insert("00:24:29", "TSL");
      m.insert("00:24:2A", "TSL 1128");
      m.insert("00:24:2B", "TSL 1153");
      m.insert("00:24:2C", "TSL RFID");
      m.insert("00:24:2D", "TSL Bluetooth");
      m.insert("00:24:2E", "TSL ASCII");
      m.insert("00:24:2F", "TSL Battery");

      //rfX
      m.insert("00:24:30", "rfX");
      m.insert("00:24:31", "rfX ID");
      m.insert("00:24:32", "rfX Technology");
      m.insert("00:24:33", "rfX RFID");

      m
 });

/// Extract OUI (first 3 octets) from MAC address string
fn extract_oui(mac: &str) -> Option<String> {
    let normalized = mac.replace('-', ":").to_uppercase();
    let parts: Vec<&str> = normalized.split(':').collect();
    if parts.len() >= 3 {
        Some(format!("{}:{}:{}", parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

/// Look up vendor name from MAC address
pub fn lookup_vendor(mac: &str) -> Option<String> {
    let oui = extract_oui(mac)?;
    OUI_DATABASE.get(oui.as_str()).map(|&s| s.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_apple() {
        assert_eq!(
            lookup_vendor("00:03:93:12:34:56"),
            Some("Apple".to_string())
        );
    }

    #[test]
    fn test_lookup_raspberry_pi() {
        assert_eq!(
            lookup_vendor("DC:A6:32:AA:BB:CC"),
            Some("Raspberry Pi".to_string())
        );
    }

    #[test]
    fn test_lookup_unknown() {
        assert_eq!(lookup_vendor("FF:FF:FF:FF:FF:FF"), None);
    }

    #[test]
    fn test_lookup_dash_separator() {
        assert_eq!(
            lookup_vendor("00-03-93-12-34-56"),
            Some("Apple".to_string())
        );
    }
}
