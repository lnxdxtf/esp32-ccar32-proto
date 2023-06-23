use anyhow;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi},
};
use log::*;

const SSID: &'static str = "";
const PASSWORD: &'static str = "";

pub struct WifiApp {
    pub wifi: BlockingWifi<EspWifi<'static>>,
}

impl WifiApp {
    pub fn new() -> WifiApp {
        info!("SETUP WIFI...");
        let peripherals = Peripherals::take().unwrap();
        let sys_loop = EspSystemEventLoop::take().unwrap();
        let nvs = EspDefaultNvsPartition::take().unwrap();
        let mut wifi = BlockingWifi::wrap(
            EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
            sys_loop,
        )
        .unwrap();
        WifiApp { wifi }
    }

    pub fn connect(&mut self) -> anyhow::Result<()> {
        info!("CONNECTING WIFI...");
        let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
            ssid: SSID.into(),
            password: PASSWORD.into(),
            bssid: None,
            auth_method: AuthMethod::WPA2Personal,
            channel: None,
        });

        self.wifi.set_configuration(&wifi_configuration)?;

        self.wifi.start()?;
        info!("Wifi started");

        info!("SCANNING WIFI...");
        match self.wifi.scan() {
            Ok(aps) => {
                for ap in aps {
                    info!("AP: {:?}", ap);
                }
            }
            Err(e) => error!("ERROR SCANNING WIFI - {:?}", e),
        }

        self.wifi.connect()?;
        info!("Wifi connected");

        self.wifi.wait_netif_up()?;
        info!("Wifi netif up");

        Ok(())
    }
}
