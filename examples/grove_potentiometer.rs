//! Grove Rotary Potentiometer Example
//!
//! This example demonstrates using a Grove Rotary Potentiometer to control
//! the blink rate of a Grove LED module. Both are connected to the
//! Seeed Studio Grove Base for XIAO
//! (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove Rotary Potentiometer connected to A0 connector on Grove Base
//! - Grove LED module connected to D7 connector on Grove Base
//! - Potentiometer signal wire (yellow) is connected to GPIO0 (A0 on XIAO ESP32-C6)
//! - LED signal wire (yellow) is connected to GPIO17 (D7 on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - A0 (Potentiometer) => GPIO0 (ADC1_CH0)
//! - D7 (LED) => GPIO17
//!
//! Behavior: Rotating the potentiometer changes the LED blink delay from 50ms to 1000ms

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Configure ADC1 for reading the potentiometer
    let mut adc1_config = AdcConfig::new();

    // Enable GPIO0 (A0) as an analog input pin with 11dB attenuation
    // (allows reading the full 0-3.3V range)
    let mut adc_pin = adc1_config.enable_pin(peripherals.GPIO0, Attenuation::_11dB);

    // Create ADC instance
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    // Set GPIO17 (D7) as an output for Grove LED, starting LOW (off)
    let mut led = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());

    let delay = Delay::new();

    // Give the system time to initialize before printing
    delay.delay_millis(100);

    esp_println::println!("Grove Potentiometer ADC Example");
    esp_println::println!("Rotate the potentiometer to change the LED blink rate");
    esp_println::println!("Starting ADC readings...");

    loop {
        // Read the analog value from potentiometer (0-4095 for 12-bit ADC)
        let adc_value: u16 = match nb::block!(adc1.read_oneshot(&mut adc_pin)) {
            Ok(val) => val,
            Err(_) => {
                esp_println::println!("Error reading ADC!");
                continue;
            }
        };

        // Map ADC value (0-4095) to delay range (50-1000 ms)
        // Formula: delay_ms = 50 + (adc_value * 950 / 4095)
        let delay_ms = 50 + ((adc_value as u32 * 950) / 4095);

        // Print the ADC value and calculated delay
        esp_println::println!("ADC Value: {} | Delay: {}ms", adc_value, delay_ms);

        // Toggle LED
        led.toggle();
        delay.delay_millis(delay_ms);
    }
}
