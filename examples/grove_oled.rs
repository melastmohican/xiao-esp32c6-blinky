//! Grove OLED Display Example
//!
//! This example demonstrates using a Grove OLED Display (0.96" 128x64, SSD1306)
//! with the embedded-graphics library. Connected to the Seeed Studio Grove Base
//! for XIAO (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove OLED Display connected to I2C connector on Grove Base
//! - SDA wire is connected to GPIO22 (D4 on XIAO ESP32-C6)
//! - SCL wire is connected to GPIO23 (D5 on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - SDA => GPIO22
//! - SCL => GPIO23
//!
//! Behavior: Displays text, shapes, and a simple animation on the OLED.

#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, ascii::FONT_9X18_BOLD, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle, Triangle},
    text::{Alignment, Text},
};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    main,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Give the system time to initialize
    delay.delay_millis(100);

    esp_println::println!("Grove OLED Display Example");
    esp_println::println!("Initializing I2C...");

    // Configure I2C with GPIO22 (SDA) and GPIO23 (SCL)
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("Failed to create I2C")
        .with_sda(peripherals.GPIO22)
        .with_scl(peripherals.GPIO23);

    esp_println::println!("Initializing OLED display...");

    // Create the I2C interface for the display
    let interface = I2CDisplayInterface::new(i2c);

    // Initialize the display (128x64 pixels, I2C address 0x3C is default for Grove OLED)
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    if let Err(e) = display.init() {
        esp_println::println!("Failed to initialize display: {:?}", e);
        loop {
            delay.delay_millis(1000);
        }
    }

    esp_println::println!("Display initialized successfully!");

    // Text styles
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let title_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();

    // Animation frame counter
    let mut frame: u32 = 0;

    loop {
        // Clear the display buffer
        display.clear_buffer();

        match (frame / 100) % 4 {
            0 => {
                // Screen 1: Welcome text
                Text::with_alignment(
                    "XIAO ESP32-C6",
                    Point::new(64, 20),
                    title_style,
                    Alignment::Center,
                )
                .draw(&mut display)
                .unwrap();

                Text::with_alignment(
                    "Grove OLED Demo",
                    Point::new(64, 38),
                    text_style,
                    Alignment::Center,
                )
                .draw(&mut display)
                .unwrap();

                Text::with_alignment(
                    "embedded-graphics",
                    Point::new(64, 52),
                    text_style,
                    Alignment::Center,
                )
                .draw(&mut display)
                .unwrap();
            }
            1 => {
                // Screen 2: Draw shapes
                Text::new("Shapes Demo", Point::new(25, 10), text_style)
                    .draw(&mut display)
                    .unwrap();

                // Rectangle
                Rectangle::new(Point::new(5, 20), Size::new(30, 25))
                    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                    .draw(&mut display)
                    .unwrap();

                // Filled circle
                Circle::new(Point::new(45, 20), 25)
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(&mut display)
                    .unwrap();

                // Triangle
                Triangle::new(Point::new(95, 45), Point::new(80, 20), Point::new(110, 20))
                    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                    .draw(&mut display)
                    .unwrap();

                // Lines
                Line::new(Point::new(5, 55), Point::new(123, 55))
                    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                    .draw(&mut display)
                    .unwrap();
            }
            2 => {
                // Screen 3: Animated circle
                Text::new("Animation", Point::new(35, 10), text_style)
                    .draw(&mut display)
                    .unwrap();

                // Bouncing circle animation
                let x = 20 + ((frame % 100) as i32 * 88 / 100);
                let y = 35 + (((frame % 50) as i32 - 25).abs() * 20 / 25);

                Circle::new(Point::new(x - 8, y - 8), 16)
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(&mut display)
                    .unwrap();

                // Draw floor line
                Line::new(Point::new(10, 58), Point::new(118, 58))
                    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                    .draw(&mut display)
                    .unwrap();
            }
            _ => {
                // Screen 4: Frame counter
                Text::new("Statistics", Point::new(30, 10), text_style)
                    .draw(&mut display)
                    .unwrap();

                // Display frame counter (using static buffer for no_std)
                Text::new("Frame:", Point::new(10, 30), text_style)
                    .draw(&mut display)
                    .unwrap();

                // Simple digit display for frame count
                let display_frame = frame / 10; // Slow down the counter display
                let thousands = (display_frame / 1000) % 10;
                let hundreds = (display_frame / 100) % 10;
                let tens = (display_frame / 10) % 10;
                let ones = display_frame % 10;

                // Create digit string manually (no_std compatible)
                let digits: [u8; 5] = [
                    b'0' + thousands as u8,
                    b'0' + hundreds as u8,
                    b'0' + tens as u8,
                    b'0' + ones as u8,
                    0,
                ];

                // Safe conversion since we only use ASCII digits
                let digit_str = unsafe { core::str::from_utf8_unchecked(&digits[..4]) };

                Text::new(digit_str, Point::new(60, 30), title_style)
                    .draw(&mut display)
                    .unwrap();

                // Progress bar
                Rectangle::new(Point::new(10, 45), Size::new(108, 12))
                    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                    .draw(&mut display)
                    .unwrap();

                let progress_width = ((frame % 100) as u32 * 104) / 100;
                Rectangle::new(Point::new(12, 47), Size::new(progress_width, 8))
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(&mut display)
                    .unwrap();
            }
        }

        // Flush the buffer to the display
        if let Err(e) = display.flush() {
            esp_println::println!("Display flush error: {:?}", e);
        }

        frame = frame.wrapping_add(1);

        // Small delay for animation timing (~30 FPS)
        delay.delay_millis(33);
    }
}
