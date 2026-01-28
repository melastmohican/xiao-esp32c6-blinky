//! Grove 4-Digit Display (TM1637) Example
//!
//! Based on: https://github.com/JadKHaddad/tm1637/blob/master/examples/esp32/4-digits/src/bin/main.rs
//!
//! ⚠️ POWER WARNING ⚠️
//! The Seeed Studio Grove 4-Digit Display (TM1637) requires 5V to operate correctly.
//! The Grove Base for XIAO only provides 3.3V on its connectors, which is insufficient.
//! You MUST connect the display directly to the 5V pin on the XIAO ESP32-C6 headers.
//!
//! Wiring Diagram:
//!
//! Grove 4-Digit Display  <===>   XIAO ESP32-C6 (Direct Headers)
//! ---------------------          ------------------------------
//! VCC (Red Wire)         ===>    5V Pin (Top right, usually)
//! GND (Black Wire)       ===>    GND Pin
//! CLK (Yellow Wire)      ===>    D2 (GPIO 2)
//! DIO (White Wire)       ===>    D1 (GPIO 1)
//!
//! Note:
//! - D2 (GPIO 2) is used for CLK.
//! - D1 (GPIO 1) is used for DIO.
//! - Internal pull-ups are enabled on DIO to assist with signal integrity.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{DriveMode, Level, Output, OutputConfig, Pull},
    main,
};
use tm1637_embedded_hal::{
    mappings::{DigitBits, LoCharBits, UpCharBits},
    options::{ScrollDirection, ScrollStyle},
    Brightness, TM1637Builder,
};

esp_bootloader_esp_idf::esp_app_desc!();

const DELAY_MS: u32 = 2000;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure GPIO2 (D2) as CLK output
    let clk = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    // Configure GPIO1 (D1) as DIO output (Open Drain + Internal Pull-up)
    let config = OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain)
        .with_pull(Pull::Up);
    let dio = Output::new(peripherals.GPIO1, Level::High, config);

    esp_println::println!("Grove 4-Digit Display (TM1637) Example");

    // Create a TM1637 instance with 4 digits.
    let mut tm = TM1637Builder::new(clk, dio, delay)
        // Set the brightness to level 3.
        .brightness(Brightness::L3)
        // Set the delay between each bit to 100us.
        .delay_us(100)
        .build_blocking::<4>();

    // Initialize the display.
    tm.init().unwrap();

    // Display the number `1234`
    let bytes = [0b00000110, 0b01011011, 0b01001111, 0b01100110];
    tm.display_slice(0, &bytes).unwrap();

    // We need a delay mechanism since 'delay' was consumed by the builder.
    // esp-hal 1.0 Delay::new() creates a new instance.
    let loop_delay = Delay::new();

    loop_delay.delay_millis(DELAY_MS);

    // Use the `mappings` module to display the sequence `AC2b`.
    let bytes = [
        UpCharBits::UpA as u8,
        UpCharBits::UpC as u8,
        DigitBits::Two as u8,
        LoCharBits::LoB as u8,
    ];
    tm.display_slice(0, &bytes).unwrap();

    loop_delay.delay_millis(DELAY_MS);

    // Display the string `Err` at the first position.
    tm.options().position(0).str("Err ").display().unwrap();

    loop_delay.delay_millis(DELAY_MS);

    // Calculated temperature value.
    let temperature = 25;
    tm.options().u8_2(temperature).str(" c").display().unwrap();

    loop_delay.delay_millis(DELAY_MS);

    // Clear the display.
    tm.clear().unwrap();

    // Count from -99 to 99.
    for i in -99..100 {
        tm.options().r_i16_4(i).display().unwrap();
        loop_delay.delay_millis(10);
    }

    loop_delay.delay_millis(DELAY_MS);
    tm.clear().unwrap();

    // Clock demo 23:58 to 23:59
    for minute in 58..=59 {
        for second in 0..15 {
            // Shortened second loop for demo speed
            let colon = second % 2 == 0;
            tm.options()
                .clock()
                .hour(23)
                .minute(minute)
                .finish()
                .set_dot(1, colon)
                .display()
                .unwrap();

            loop_delay.delay_millis(100);
        }
    }

    loop_delay.delay_millis(DELAY_MS);

    // Scrolling text
    tm.options()
        .str("HELLO ruSt 123 ")
        .scroll()
        .style(ScrollStyle::Circular)
        .direction(ScrollDirection::LeftToRight)
        .delay_ms(200)
        .finish()
        .run(); // This runs indefinitely? No, run() usually runs once or loops?
                // In the example, it says "animate them to make them fit".
                // Let's assume it returns after one cycle or needs to be in a loop?
                // The example calls .run().

    loop_delay.delay_millis(DELAY_MS);

    // Flip display
    tm.options().str("FLIP").flip().display().unwrap();
    loop_delay.delay_millis(DELAY_MS);

    // Animate flipped
    tm.options()
        .str("FLIP ")
        .str("FLOP")
        .flip()
        .scroll()
        .style(ScrollStyle::Linear)
        .direction(ScrollDirection::LeftToRight)
        .delay_ms(300)
        .finish()
        .run();

    loop_delay.delay_millis(DELAY_MS);

    // Brightness ramp
    tm.options().str("8888").display().unwrap();
    let levels = [
        Brightness::L0,
        Brightness::L1,
        Brightness::L2,
        Brightness::L3,
        Brightness::L4,
        Brightness::L5,
        Brightness::L6,
        Brightness::L7,
    ];
    for level in levels {
        tm.set_brightness(level).unwrap();
        loop_delay.delay_millis(200);
    }

    tm.clear().unwrap();
    tm.options().str("done").display().unwrap();

    loop {
        loop_delay.delay_millis(1000);
    }
}
