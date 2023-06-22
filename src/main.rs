use anyhow;
mod modules;
use embedded_svc::{http::client::Client as HttpClient, io::Write, utils::io};
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

use crate::modules::wifi_conn::wifi::{connect_wifi, wifi};


const URL: &'static str = "https://eou4ul6qklamb5m.m.pipedream.net";

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("STARTING....");
    let mut wifi = wifi()?;
    connect_wifi(&mut wifi)?;
    let mut client = create_client()?;

    post_request(&mut client)?;
    Ok(())
}

fn create_client() -> anyhow::Result<HttpClient<EspHttpConnection>> {
    info!("CREATE CLIENT...");
    return Ok(HttpClient::wrap(EspHttpConnection::new(
        &Default::default(),
    )?));
}

fn post_request(client: &mut HttpClient<EspHttpConnection>) -> anyhow::Result<()> {
    // Prepare payload
    let payload = b"Hello world! from esp do pai";

    // Prepare headers and URL
    let content_length_header = format!("{}", payload.len());
    let headers = [
        // ("accept", "text/plain"),
        ("content-type", "text/plain"),
        // ("connection", "close"),
        // ("content-length", &*content_length_header),
    ];

    // Send request
    let mut request = client.post(&URL, &headers)?;
    request.write_all(payload)?;
    request.flush()?;
    info!("-> POST {}", URL);
    let mut response = request.submit()?;

    // Process response
    let status = response.status();
    info!("<- {}", status);
    let (_headers, mut body) = response.split();
    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;
    info!("Read {} bytes", bytes_read);
    match std::str::from_utf8(&buf[0..bytes_read]) {
        Ok(body_string) => info!(
            "Response body (truncated to {} bytes): {:?}",
            buf.len(),
            body_string
        ),
        Err(e) => error!("Error decoding response body: {}", e),
    };

    // Drain the remaining response bytes
    while body.read(&mut buf)? > 0 {}

    Ok(())
}
