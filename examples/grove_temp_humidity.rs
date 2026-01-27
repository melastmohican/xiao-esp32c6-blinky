//! Grove Temperature and Humidity Sensor (AHT20) Example
//!
//! This example demonstrates using a Grove AHT20 Temperature and Humidity Sensor
//! to read ambient temperature and relative humidity. Connected to the Seeed Studio
//! Grove Base for XIAO (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove AHT20 Sensor connected to I2C connector on Grove Base
//! - SDA wire is connected to GPIO22 (SDA on XIAO ESP32-C6)
//! - SCL wire is connected to GPIO23 (SCL on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - SDA => GPIO22
//! - SCL => GPIO23
//!
//! Behavior: Reads temperature (°C/°F) and humidity (%) every 2 seconds and prints to serial.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();

/// AHT20 I2C address
const AHT20_ADDR: u8 = 0x38;

/// AHT20 Commands
const CMD_INITIALIZE: [u8; 3] = [0xBE, 0x08, 0x00];
const CMD_TRIGGER_MEASUREMENT: [u8; 3] = [0xAC, 0x33, 0x00];
const CMD_SOFT_RESET: u8 = 0xBA;

/// Status bit masks
const STATUS_BUSY: u8 = 0x80;
const STATUS_CALIBRATED: u8 = 0x08;

/// AHT20 sensor driver
struct Aht20<'a> {
    i2c: I2c<'a, esp_hal::Blocking>,
    delay: Delay,
}

impl<'a> Aht20<'a> {
    /// Create a new AHT20 driver instance
    fn new(mut i2c: I2c<'a, esp_hal::Blocking>, delay: Delay) -> Result<Self, &'static str> {
        // Wait for sensor to power up
        delay.delay_millis(40);

        // Check if sensor needs initialization
        let mut status = [0u8; 1];
        if i2c.read(AHT20_ADDR, &mut status).is_err() {
            return Err("Failed to read sensor status");
        }

        // If not calibrated, initialize the sensor
        if (status[0] & STATUS_CALIBRATED) == 0 {
            if i2c.write(AHT20_ADDR, &CMD_INITIALIZE).is_err() {
                return Err("Failed to initialize sensor");
            }
            delay.delay_millis(10);
        }

        Ok(Self { i2c, delay })
    }

    /// Perform a soft reset
    #[allow(dead_code)]
    fn reset(&mut self) -> Result<(), &'static str> {
        if self.i2c.write(AHT20_ADDR, &[CMD_SOFT_RESET]).is_err() {
            return Err("Failed to reset sensor");
        }
        self.delay.delay_millis(20);
        Ok(())
    }

    /// Read temperature and humidity
    fn measure(&mut self) -> Result<(f32, f32), &'static str> {
        // Trigger measurement
        if self
            .i2c
            .write(AHT20_ADDR, &CMD_TRIGGER_MEASUREMENT)
            .is_err()
        {
            return Err("Failed to trigger measurement");
        }

        // Wait for measurement to complete (typical 80ms)
        self.delay.delay_millis(80);

        // Read 7 bytes: status + 5 data bytes + CRC
        let mut data = [0u8; 7];

        // Poll until not busy (with timeout)
        for _ in 0..10 {
            if self.i2c.read(AHT20_ADDR, &mut data).is_err() {
                return Err("Failed to read measurement");
            }

            if (data[0] & STATUS_BUSY) == 0 {
                break;
            }
            self.delay.delay_millis(10);
        }

        if (data[0] & STATUS_BUSY) != 0 {
            return Err("Sensor busy timeout");
        }

        // Parse humidity (20-bit value in data[1], data[2], data[3] upper 4 bits)
        let humidity_raw =
            ((data[1] as u32) << 12) | ((data[2] as u32) << 4) | ((data[3] as u32) >> 4);
        let humidity = (humidity_raw as f32 / 1048576.0) * 100.0;

        // Parse temperature (20-bit value in data[3] lower 4 bits, data[4], data[5])
        let temp_raw =
            (((data[3] & 0x0F) as u32) << 16) | ((data[4] as u32) << 8) | (data[5] as u32);
        let temperature = ((temp_raw as f32 / 1048576.0) * 200.0) - 50.0;

        Ok((temperature, humidity))
    }
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Give the system time to initialize
    delay.delay_millis(100);

    esp_println::println!("Grove Temperature & Humidity Sensor (AHT20) Example");
    esp_println::println!("Initializing I2C...");

    // Configure I2C with GPIO22 (SDA) and GPIO23 (SCL)
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("Failed to create I2C")
        .with_sda(peripherals.GPIO22)
        .with_scl(peripherals.GPIO23);

    esp_println::println!("Initializing AHT20 sensor...");

    // Initialize the AHT20 sensor
    let mut sensor = match Aht20::new(i2c, delay) {
        Ok(s) => s,
        Err(e) => {
            esp_println::println!("Failed to initialize AHT20: {}", e);
            loop {
                delay.delay_millis(1000);
            }
        }
    };

    esp_println::println!("Sensor initialized successfully!");
    esp_println::println!();
    esp_println::println!("Starting temperature and humidity readings...");
    esp_println::println!("=========================================");

    loop {
        // Read temperature and humidity from the sensor
        match sensor.measure() {
            Ok((temp_c, humidity)) => {
                // Convert to Fahrenheit
                let temp_f = temp_c * 9.0 / 5.0 + 32.0;

                // Determine comfort level based on humidity
                let comfort = if humidity < 30.0 {
                    "Too Dry"
                } else if humidity < 40.0 {
                    "Dry"
                } else if humidity <= 60.0 {
                    "Comfortable"
                } else if humidity <= 70.0 {
                    "Humid"
                } else {
                    "Too Humid"
                };

                // Determine temperature description
                let temp_desc = if temp_c < 10.0 {
                    "Cold"
                } else if temp_c < 18.0 {
                    "Cool"
                } else if temp_c <= 24.0 {
                    "Comfortable"
                } else if temp_c <= 30.0 {
                    "Warm"
                } else {
                    "Hot"
                };

                esp_println::println!(
                    "Temperature: {:.1}C ({:.1}F) - {}",
                    temp_c,
                    temp_f,
                    temp_desc
                );
                esp_println::println!("Humidity:    {:.1}% RH - {}", humidity, comfort);
                esp_println::println!("-----------------------------------------");
            }
            Err(e) => {
                esp_println::println!("Error reading sensor: {}", e);
            }
        }

        // Wait 2 seconds before next reading
        delay.delay_millis(2000);
    }
}
