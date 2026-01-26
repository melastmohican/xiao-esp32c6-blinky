//! Grove Buzzer Example
//!
//! This example demonstrates using a Grove Buzzer module with PWM to create beep patterns.
//! The buzzer is connected to the Seeed Studio Grove Base for XIAO
//! (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove Buzzer module connected to A2 connector on Grove Base
//! - Signal wire (yellow) is connected to GPIO2 (A2 on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - A2 (Buzzer) => GPIO2
//!
//! Behavior: Creates different beep patterns using PWM (analogWrite)

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource, Ledc, LowSpeed,
    },
    main,
    time::Rate,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();

    // Initialize LEDC (LED PWM Controller) for PWM output (analogWrite)
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    // Configure timer0 with a pleasant frequency for the buzzer
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty8Bit, // 8-bit resolution (0-100%)
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_hz(2000), // 2kHz tone
        })
        .unwrap();

    // Configure PWM channel on GPIO2 (A2)
    let mut channel = ledc.channel(channel::Number::Channel0, peripherals.GPIO2);
    channel
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 0, // Start with 0% duty (off)
            drive_mode: esp_hal::gpio::DriveMode::PushPull,
        })
        .unwrap();

    // Give the system time to initialize before printing
    delay.delay_millis(100);

    esp_println::println!("Grove Buzzer Example");
    esp_println::println!("Playing different beep patterns with PWM (analogWrite)");

    loop {
        // Pattern 1: Three short beeps
        esp_println::println!("\nPattern 1: Three short beeps");
        for _ in 0..3 {
            channel.set_duty(50).unwrap();
            delay.delay_millis(200);
            channel.set_duty(0).unwrap();
            delay.delay_millis(200);
        }
        delay.delay_millis(1000);

        // Pattern 2: Long beep
        esp_println::println!("Pattern 2: Long beep");
        channel.set_duty(50).unwrap();
        delay.delay_millis(1000);
        channel.set_duty(0).unwrap();
        delay.delay_millis(1000);

        // Pattern 3: SOS pattern (... --- ...)
        esp_println::println!("Pattern 3: SOS pattern");
        // Three short
        for _ in 0..3 {
            channel.set_duty(50).unwrap();
            delay.delay_millis(200);
            channel.set_duty(0).unwrap();
            delay.delay_millis(200);
        }
        delay.delay_millis(200);
        // Three long
        for _ in 0..3 {
            channel.set_duty(50).unwrap();
            delay.delay_millis(600);
            channel.set_duty(0).unwrap();
            delay.delay_millis(200);
        }
        delay.delay_millis(200);
        // Three short
        for _ in 0..3 {
            channel.set_duty(50).unwrap();
            delay.delay_millis(200);
            channel.set_duty(0).unwrap();
            delay.delay_millis(200);
        }
        delay.delay_millis(2000);

        // Pattern 4: Rapid beeps
        esp_println::println!("Pattern 4: Rapid beeps");
        for _ in 0..10 {
            channel.set_duty(50).unwrap();
            delay.delay_millis(100);
            channel.set_duty(0).unwrap();
            delay.delay_millis(100);
        }
        delay.delay_millis(2000);
    }
}
