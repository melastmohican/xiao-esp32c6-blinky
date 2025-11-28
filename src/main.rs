//! Blinks an LED
//!
//! The following wiring is assumed:
//! - LED => GPIO15
//!   https://github.com/espressif/arduino-esp32/blob/d47771f2cc649c3cd52a3f6eb3d9b97c82005ffb/variants/XIAO_ESP32C6/pins_arduino.h#L13

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

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO15, Level::High, OutputConfig::default());
    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(250);
        led.toggle();
        // or using  duration
        delay.delay(Duration::from_secs(1));
    }
}
