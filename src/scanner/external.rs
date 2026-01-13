use std::net::IpAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ShodanResponse {
    ports: Vec<u16>,
}

pub async fn get_public_ip() -> Option<IpAddr> {
    let host = "api.ipify.org";
    let port = 80;
    let addr = format!("{}:{}", host, port);

    if let Ok(mut stream) = TcpStream::connect(addr).await {
        let request = format!(
            "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: SCANNR/1.0\r\n\r\n",
            host
        );

        if stream.write_all(request.as_bytes()).await.is_ok() {
            let mut response = String::new();
            if stream.read_to_string(&mut response).await.is_ok() {
                if let Some(body) = response.split("\r\n\r\n").nth(1) {
                    let ip_str = body.trim();
                    if let Ok(ip) = ip_str.parse::<IpAddr>() {
                        return Some(ip);
                    }
                }
            }
        }
    }
    None
}

pub async fn get_open_ports_from_shodan(ip: &IpAddr) -> Vec<u16> {
    let url = format!("https://internetdb.shodan.io/{}", ip);
    let output_result;

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        
        output_result = Command::new("powershell")
            .args(["-Command", &format!("(Invoke-WebRequest -Uri '{}' -UseBasicParsing).Content", url)])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
    }

    #[cfg(not(target_os = "windows"))]
    {
        output_result = Command::new("curl")
            .args(["-s", &url])
            .output();
    }

    if let Ok(output) = output_result {
        if output.status.success() {
            if let Ok(json_str) = String::from_utf8(output.stdout) {
                if let Ok(response) = serde_json::from_str::<ShodanResponse>(&json_str) {
                    return response.ports;
                }
            }
        }
    }

    Vec::new()
}
