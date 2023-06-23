use std::time::Duration;

use embedded_svc::{http::client::Client, io::Write, utils::io};
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use log::*;
pub struct HttpClientApp {
    client: Client<EspHttpConnection>,
}

#[allow(unused)]
#[derive(Debug)]
pub enum HttpClientMethods {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpClientApp {
    pub fn new() -> HttpClientApp {
        let mut config = Configuration::default();
        config.timeout = Some(Duration::from_secs(5));
        let client = Client::wrap(EspHttpConnection::new(&config).unwrap());
        HttpClientApp { client }
    }
    fn create_header<'a>(
        &self,
        headers: Option<Vec<(&'a str, &'a str)>>,
    ) -> Vec<(&'a str, &'a str)> {
        let mut headers_content = vec![("accept", "*"), ("content-type", "*")];
        if let Some(headers) = headers {
            headers_content.extend(headers);
        }
        headers_content
    }

    pub fn request(
        &mut self,
        method: HttpClientMethods,
        url: &str,
        headers: Option<Vec<(&str, &str)>>,
        payload: Option<&[u8]>,
        wait_response: bool,
    ) -> anyhow::Result<()> {
        let headers = self.create_header(headers);

        let mut request = match method {
            HttpClientMethods::GET => self.client.get(url)?,
            HttpClientMethods::POST => self.client.post(url, &headers)?,
            HttpClientMethods::PUT => self.client.put(url, &headers)?,
            HttpClientMethods::DELETE => self.client.delete(url)?,
        };
        match payload {
            Some(payload) => {
                request.write_all(payload)?;
            }
            None => {}
        }

        request.flush()?;
        info!("-> {:?} {}", method, url);
        match request.submit() {
            Ok(mut response) => {
                let status = response.status();
                if wait_response {
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
                }
            }
            Err(err) => error!("ERROR REQUEST - {:?}", err),
        }

        Ok(())
    }
}
