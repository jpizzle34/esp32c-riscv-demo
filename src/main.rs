use embassy_time::{Duration, Timer};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;

mod wifi;

#[embassy_executor::task]
async fn blinky_task() {
    loop {
        log::info!("Blink! (every second)");
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) -> () {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    spawner.spawn(blinky_task()).unwrap();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs_partition = EspDefaultNvsPartition::take().unwrap();

    let wifi = wifi::connect(peripherals.modem, sysloop, nvs_partition)
        .await
        .unwrap();

    log::info!("Hello, world!");

    // Print connection status

    while wifi.is_connected().unwrap() {
        let ip_info = wifi.sta_netif().get_ip_info().unwrap();
        log::info!(
            "WiFi is still connected! Anyone there? IP Address: {}, Signal Strength: {}",
            ip_info.ip,
            wifi.get_rssi().unwrap()
        );
        Timer::after(Duration::from_millis(5000)).await;
    }
}
