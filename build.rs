use dotenvy::dotenv;
use std::env;

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Get WiFi credentials from environment
    let ssid = env::var("WIFI_SSID").expect("WIFI_SSID must be set in .env");
    let password = env::var("WIFI_PASSWORD").expect("WIFI_PASSWORD must be set in .env");

    // Generate code for NVS initialization
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rustc-env=WIFI_SSID={}", &ssid);
    println!("cargo:rustc-env=WIFI_PASSWORD={}", &password);

    // Required for ESP-IDF
    embuild::espidf::sysenv::output();
}
