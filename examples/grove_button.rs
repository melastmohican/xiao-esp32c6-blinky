//! Grove Button and LED Example
//!
//! This example demonstrates using a Grove Button to control a Grove LED.
//! Both are connected to the Seeed Studio Grove Base for XIAO
//! (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove Button module connected to D1 connector on Grove Base
//! - Grove LED module connected to D7 connector on Grove Base
//! - Button signal wire (yellow) is connected to GPIO1 (D1 on XIAO ESP32-C6)
//! - LED signal wire (yellow) is connected to GPIO17 (D7 on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - D1 (Button) => GPIO1
//! - D7 (LED) => GPIO17
//!
//! Behavior: LED turns ON when button is pressed, OFF when released

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set GPIO1 (D1) as an input for Grove Button with pull-down resistor
    // Grove buttons are active-high (pressed = HIGH, released = LOW)
    let button_config = InputConfig::default().with_pull(Pull::Down);
    let button = Input::new(peripherals.GPIO1, button_config);

    // Set GPIO17 (D7) as an output for Grove LED, starting LOW (off)
    let mut led = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());

    let delay = Delay::new();

    loop {
        // Read button state (is_high() returns true when button is pressed)
        if button.is_high() {
            // Button pressed - turn LED on
            led.set_high();
        } else {
            // Button released - turn LED off
            led.set_low();
        }

        // Small delay to debounce and reduce CPU usage
        delay.delay_millis(10);
    }
}
