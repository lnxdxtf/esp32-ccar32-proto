use anyhow;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
mod modules;
use crate::modules::{
    http::client::client::{HttpClientApp, HttpClientMethods},
    wifi_conn::wifi::WifiApp,
};
use log::*;

const URL: &'static str = "";

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("STARTING....");
    let mut wifi = WifiApp::new();
    wifi.connect()?;

    loop {
        let mut client = HttpClientApp::new();
        // let payload = Some(&b"ITS FROM ESP32"[..]);
        client.request(HttpClientMethods::GET, URL, None, None, false)?;
        // std::thread::sleep(std::time::Duration::from_secs(5));
    }

    Ok(())
}
