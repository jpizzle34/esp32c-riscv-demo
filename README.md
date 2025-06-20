# ESP32-C RISC-V Demo

## Configuration Setup

1. Copy the example cargo config file:
   ```bash
   cp .cargo/config.toml.example .cargo/config.toml
   ```

2. Edit `.cargo/config.toml` and update the WiFi credentials:
   ```toml
   [env]
   WIFI_SSID = "your_wifi_ssid_here"
   WIFI_PASSWORD = "your_wifi_password_here"
   ```

Note: The `.cargo/config.toml` file is ignored by git to keep credentials secure.

## Environment Setup

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your WiFi credentials:
   ```
   WIFI_SSID=your_wifi_name_here
   WIFI_PASSWORD=your_wifi_password_here
   ```

Note: The `.env` file is ignored by git to keep credentials secure.

// ... rest of README ...