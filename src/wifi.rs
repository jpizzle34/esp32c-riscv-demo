use anyhow::Result;
use embassy_time::{Duration, Timer};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi};
use heapless::String;

const WIFI_NAMESPACE: &str = "wifi";
const SSID_KEY: &str = "ssid";
const PASS_KEY: &str = "pass";

// These are populated at build time from .env
const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

fn store_wifi_credentials(nvs: &mut EspNvs<NvsDefault>) -> Result<()> {
    // Remove any quotes if present
    let ssid = WIFI_SSID.trim_matches('"');
    let pass = WIFI_PASSWORD.trim_matches('"');

    log::info!("Storing SSID (length: {}): {}", ssid.len(), ssid);
    log::info!("Storing password length: {}", pass.len());

    nvs.set_str(SSID_KEY, ssid)?;
    nvs.set_str(PASS_KEY, pass)?;

    log::info!("Stored credentials in NVS");
    Ok(())
}

fn load_wifi_credentials(nvs: &mut EspNvs<NvsDefault>) -> Result<(String<32>, String<64>)> {
    let mut ssid: String<32> = String::new();
    let mut password: String<64> = String::new();

    let mut ssid_buf = [0u8; 32];
    let mut pass_buf = [0u8; 64];

    match (
        nvs.get_str(SSID_KEY, &mut ssid_buf),
        nvs.get_str(PASS_KEY, &mut pass_buf),
    ) {
        (Ok(Some(stored_ssid)), Ok(Some(stored_pass))) => {
            // Convert stored values to proper strings
            let stored_ssid = stored_ssid.trim_matches(|c: char| c == '\0' || c == '"');
            let stored_pass = stored_pass.trim_matches(|c: char| c == '\0' || c == '"');

            log::info!("Retrieved ssid: {}", stored_ssid);

            ssid.push_str(stored_ssid).map_err(|_| {
                anyhow::anyhow!("SSID too long (length: {}, max: 32)", stored_ssid.len())
            })?;
            password.push_str(stored_pass).map_err(|_| {
                anyhow::anyhow!("Password too long (length: {}, max: 64)", stored_pass.len())
            })?;
            log::info!("Loaded credentials from NVS");
        }
        _ => {
            // Store default credentials if not found
            store_wifi_credentials(nvs)?;

            // Remove any quotes if present
            let default_ssid = WIFI_SSID.trim_matches('"');
            let default_pass = WIFI_PASSWORD.trim_matches('"');

            log::info!("Using default SSID: {}", default_ssid);
            log::info!("Using default password length: {}", default_pass.len());

            ssid.push_str(default_ssid).map_err(|_| {
                anyhow::anyhow!(
                    "Default SSID too long (length: {}, max: 32)",
                    default_ssid.len()
                )
            })?;
            password.push_str(default_pass).map_err(|_| {
                anyhow::anyhow!(
                    "Default password too long (length: {}, max: 64)",
                    default_pass.len()
                )
            })?;
            log::info!("Using default credentials");
        }
    }

    Ok((ssid, password))
}

pub async fn connect(
    modem: Modem,
    sysloop: EspSystemEventLoop,
    nvs_partition: EspDefaultNvsPartition,
) -> Result<EspWifi<'static>> {
    let mut nvs = EspNvs::new(nvs_partition, WIFI_NAMESPACE, true)?;
    let (ssid, password) = load_wifi_credentials(&mut nvs)?;

    let mut wifi = EspWifi::new(modem, sysloop, None)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid,
        password,
        auth_method: AuthMethod::None,
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.connect()?;
    // wifi.is_up()?;

    while !wifi.is_up()? {
        log::info!("Waiting for connection...");
        Timer::after(Duration::from_millis(1000)).await;
    }

    log::info!("WiFi connected!");
    Ok(wifi)
}
