//! Grove RGB LED Stick (15-WS2813 Mini) Example using esp-hal-smartled
//!
//! ⚠️ POWER WARNING ⚠️
//! The Grove RGB LED Stick with 15 LEDs requires more power than the 3.3V Grove Base
//! connectors can reliably provide. You MUST connect the stick directly to the
//! 5V pin on the XIAO ESP32-C6 headers for stable operation.
//!
//! Wiring Diagram:
//!
//! Grove RGB LED Stick  <===>   XIAO ESP32-C6 (Direct Headers)
//! -------------------          ------------------------------
//! VCC (Red Wire)       ===>    5V Pin
//! GND (Black Wire)     ===>    GND Pin
//! DIN (Yellow Wire)    ===>    D7 (GPIO 17)
//! NC  (White Wire)     ===>    Not Connected
//!
//! Note:
//! - This example uses the `esp-hal-smartled` crate for robust RMT timing.
//! - The color order is standard GRB for WS2813.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, main, rmt::Rmt, time::Rate};
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite, RGB8,
};

esp_bootloader_esp_idf::esp_app_desc!();

const NUM_LEDS: usize = 15;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    esp_println::println!("Grove RGB LED Stick (15-WS2813) SmartLED Example");

    // Initialize RMT
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("RMT init failed");

    // Configure RMT buffer
    let mut rmt_buffer = smart_led_buffer!(NUM_LEDS);

    // Initialize SmartLedsAdapter
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO17, &mut rmt_buffer);

    esp_println::println!("LED strip initialized!");

    // Clear
    let mut data = [RGB8::default(); NUM_LEDS];
    led.write(data.iter().cloned()).unwrap();
    delay.delay_millis(100);

    let mut hue: u8 = 0;

    loop {
        // Rainbow effect
        for _ in 0..255 {
            for (i, led_color) in data.iter_mut().enumerate() {
                // Hue offset for rainbow
                let h = hue.wrapping_add((i * 10) as u8);
                *led_color = hsv2rgb(Hsv {
                    hue: h,
                    sat: 255,
                    val: 100, // Brightness 100/255
                });
            }

            led.write(data.iter().cloned()).unwrap();

            hue = hue.wrapping_add(1);
            delay.delay_millis(20);
        }

        esp_println::println!("Looping rainbow...");
    }
}
