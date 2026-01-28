//! Grove Air Pressure Sensor (BMP280) Example
//!
//! This example demonstrates using the `bme280` crate to read barometric pressure,
//! temperature, and calculate altitude from a BMP280 sensor.
//! Connected to the Seeed Studio Grove Base for XIAO.
//!
//! Driver Note:
//! We use the `bme280` crate because it provides a modern driver compatible with
//! `embedded-hal` 1.0 (required by `esp-hal` 1.0). The BMP280 and BME280 share
//! the same register map for temperature and pressure, making this driver
//! perfectly compatible with both.
//!
//! Wiring:
//! - Grove BMP280 Sensor connected to I2C connector
//! - SDA => GPIO22
//! - SCL => GPIO23
//!
//! Behavior: Reads pressure (hPa), temperature (°C), and altitude (m) every second.

#![no_std]
#![no_main]

use bme280::i2c::BME280;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();

/// Standard sea level pressure in hPa
const SEA_LEVEL_PRESSURE_HPA: f32 = 1013.25;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    // Give the system time to initialize
    delay.delay_millis(100);

    esp_println::println!("Grove Air Pressure Sensor (BMP280) Example");
    esp_println::println!("Initializing I2C...");

    // Configure I2C with GPIO22 (SDA) and GPIO23 (SCL)
    let mut i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("Failed to create I2C")
        .with_sda(peripherals.GPIO22)
        .with_scl(peripherals.GPIO23);

    esp_println::println!("Initializing BMP280 sensor...");

    // Try primary address (0x76) first, borrowing the I2C bus
    let mut bmp280 = BME280::new_primary(&mut i2c);

    if let Err(e) = bmp280.init(&mut delay) {
        esp_println::println!("Failed at 0x76: {:?}", e);
        esp_println::println!("Trying secondary address 0x77...");

        // Re-assigning bmp280 lets us re-borrow i2c
        bmp280 = BME280::new_secondary(&mut i2c);

        if let Err(e) = bmp280.init(&mut delay) {
            esp_println::println!("Failed at 0x77: {:?}", e);
            esp_println::println!("Could not initialize sensor at either address.");
            loop {
                delay.delay_millis(1000);
            }
        }
    }

    esp_println::println!("Sensor initialized successfully!");
    esp_println::println!();
    esp_println::println!("Starting air pressure readings...");
    esp_println::println!(
        "(Altitude is relative to sea level pressure: {:.2} hPa)",
        SEA_LEVEL_PRESSURE_HPA
    );
    esp_println::println!("=====================================================");

    loop {
        match bmp280.measure(&mut delay) {
            Ok(measurements) => {
                let temperature = measurements.temperature;
                let pressure = measurements.pressure / 100.0; // Convert Pa to hPa

                // Calculate altitude
                // Barometric formula: altitude = 44330 * (1 - (p/p0)^0.1903)
                let altitude =
                    44330.0 * (1.0 - libm::powf(pressure / SEA_LEVEL_PRESSURE_HPA, 0.1903));

                // Convert temperature to Fahrenheit
                let temp_f = temperature * 9.0 / 5.0 + 32.0;

                // Determine weather indication based on pressure
                let weather = if pressure < 1000.0 {
                    "Stormy"
                } else if pressure < 1013.0 {
                    "Rainy/Cloudy"
                } else if pressure < 1020.0 {
                    "Changeable"
                } else if pressure < 1030.0 {
                    "Fair"
                } else {
                    "Very Dry/Sunny"
                };

                esp_println::println!("Temperature: {:.1}C ({:.1}F)", temperature, temp_f);
                esp_println::println!("Pressure:    {:.2} hPa - {}", pressure, weather);
                esp_println::println!(
                    "Altitude:    {:.1} m ({:.1} ft)",
                    altitude,
                    altitude * 3.28084
                );
                esp_println::println!("-----------------------------------------------------");
            }
            Err(e) => {
                esp_println::println!("Error reading sensor: {:?}", e);
            }
        }

        // Wait 1 second before next reading
        delay.delay_millis(1000);
    }
}
