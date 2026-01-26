//! Grove LED Example
//!
//! This example demonstrates using a Grove LED connected to the
//! Seeed Studio Grove Base for XIAO (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove LED module connected to D7 connector on Grove Base
//! - Signal wire (yellow) is connected to GPIO17 (D7 on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - D7 => GPIO17
//!
//! The LED will blink with a pattern: 250ms ON, 1s OFF

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::OutputConfig,
    gpio::{Level, Output},
    main,
    time::Duration,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set GPIO17 (D7) as an output for Grove LED, starting LOW
    let mut led = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());
    let delay = Delay::new();

    loop {
        // Turn LED on
        led.set_high();
        delay.delay_millis(250);

        // Turn LED off
        led.set_low();
        delay.delay(Duration::from_secs(1));
    }
}
