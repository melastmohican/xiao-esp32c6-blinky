//! Grove Light Sensor Example
//!
//! This example demonstrates using a Grove Light Sensor to measure ambient
//! light levels and automatically control a Grove LED module. Both are
//! connected to the Seeed Studio Grove Base for XIAO
//! (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove Light Sensor connected to A0 connector on Grove Base
//! - Grove LED module connected to D7 connector on Grove Base
//! - Light sensor signal wire (yellow) is connected to GPIO0 (A0 on XIAO ESP32-C6)
//! - LED signal wire (yellow) is connected to GPIO17 (D7 on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - A0 (Light Sensor) => GPIO0 (ADC1_CH0)
//! - D7 (LED) => GPIO17
//!
//! Behavior: The LED turns on when the light level drops below a threshold (dark),
//! and turns off when there is sufficient light. Light values are printed to serial.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcCalLine, AdcConfig, Attenuation},
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();

// Light threshold for turning on the LED (lower values = darker)
// With calibration enabled, this value is in millivolts (mV).
// Observed values: Dark ~100mV, Normal ~800mV, Bright ~1800mV
const LIGHT_THRESHOLD: u16 = 500;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Configure ADC1 for reading the light sensor
    let mut adc1_config = AdcConfig::new();

    // Enable GPIO0 (A0) as an analog input pin with 11dB attenuation and calibration
    // (allows reading the full 0-3.3V range with better accuracy)
    let mut adc_pin = adc1_config.enable_pin_with_cal::<_, AdcCalLine<esp_hal::peripherals::ADC1>>(
        peripherals.GPIO0,
        Attenuation::_11dB,
    );

    // Create ADC instance
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    // Set GPIO17 (D7) as an output for Grove LED, starting LOW (off)
    let mut led = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());

    let delay = Delay::new();

    // Give the system time to initialize before printing
    delay.delay_millis(100);

    esp_println::println!("Grove Light Sensor Example");
    esp_println::println!(
        "Light threshold: {}mV (LED turns on below this value)",
        LIGHT_THRESHOLD
    );
    esp_println::println!("Starting light sensor readings (calibrated mV)...");
    esp_println::println!();

    loop {
        // Read the analog value from light sensor (in millivolts)
        // Higher values = more light, Lower values = darker
        let light_value: u16 = match nb::block!(adc1.read_oneshot(&mut adc_pin)) {
            Ok(val) => val,
            Err(_) => {
                esp_println::println!("Error reading ADC!");
                delay.delay_millis(500);
                continue;
            }
        };

        // Calculate approximate light percentage (0-100%)
        // Adjusted max reference to 2500mV based on observed values (Bright ~1800mV)
        let light_percent = (light_value as u32 * 100) / 2500;

        // Determine light level description (thresholds adjusted for observed mV)
        let light_level = if light_value < 200 {
            "Very Dark"
        } else if light_value < 600 {
            "Dark"
        } else if light_value < 1200 {
            "Dim"
        } else if light_value < 2000 {
            "Moderate"
        } else {
            "Bright"
        };

        // Control LED based on light threshold
        // Turn LED on when it's dark (below threshold)
        if light_value < LIGHT_THRESHOLD {
            led.set_high();
            esp_println::println!(
                "Light: {} ({}%) - {} | LED: ON",
                light_value,
                light_percent,
                light_level
            );
        } else {
            led.set_low();
            esp_println::println!(
                "Light: {} ({}%) - {} | LED: OFF",
                light_value,
                light_percent,
                light_level
            );
        }

        // Wait before next reading
        delay.delay_millis(500);
    }
}
