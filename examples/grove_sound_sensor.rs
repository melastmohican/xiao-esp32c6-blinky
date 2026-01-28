//! Grove Sound Sensor Example
//!
//! This example demonstrates using a Grove Sound Sensor to detect ambient
//! sound levels and trigger a Grove LED module when sound exceeds a threshold.
//! Both are connected to the Seeed Studio Grove Base for XIAO
//! (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove Sound Sensor connected to A0 connector on Grove Base
//! - Grove LED module connected to D7 connector on Grove Base
//! - Sound sensor signal wire (yellow) is connected to GPIO0 (A0 on XIAO ESP32-C6)
//! - LED signal wire (yellow) is connected to GPIO17 (D7 on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - A0 (Sound Sensor) => GPIO0 (ADC1_CH0)
//! - D7 (LED) => GPIO17
//!
//! Behavior: The LED lights up when the sound level exceeds a threshold (e.g., clap detection).
//! Sound values are continuously printed to serial.

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

// Sensitivity threshold (diff from baseline in mV)
// Adjusted to 1000mV to filter out large noise spikes
const SOUND_SENSITIVITY: u16 = 1000;

// How long to keep the LED on after detecting loud sound (in ms)
const LED_ON_DURATION_MS: u32 = 200;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Configure ADC1 for reading the sound sensor
    let mut adc1_config = AdcConfig::new();

    // Enable GPIO0 (A0) as an analog input pin with 11dB attenuation and calibration
    let mut adc_pin = adc1_config.enable_pin_with_cal::<_, AdcCalLine<esp_hal::peripherals::ADC1>>(
        peripherals.GPIO0,
        Attenuation::_11dB,
    );

    // Create ADC instance
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    // Set GPIO17 (D7) as an output for Grove LED, starting LOW (off)
    let mut led = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());

    let delay = Delay::new();

    esp_println::println!("Grove Sound Sensor Example");
    esp_println::println!("Calibrating baseline (silence) level...");

    // Calibrate baseline
    let mut baseline_acc: u32 = 0;
    let samples = 50;
    for _ in 0..samples {
        let val: u16 = nb::block!(adc1.read_oneshot(&mut adc_pin)).unwrap_or(0);
        baseline_acc += val as u32;
        delay.delay_millis(10);
    }
    let baseline = (baseline_acc / samples) as u16;

    esp_println::println!("Baseline calibrated: {} mV", baseline);
    esp_println::println!("Sensitivity: {} mV", SOUND_SENSITIVITY);
    esp_println::println!("Make some noise!");
    esp_println::println!();

    let mut sample_count: u32 = 0;

    loop {
        // Read the analog value (in mV)
        let sound_value: u16 = match nb::block!(adc1.read_oneshot(&mut adc_pin)) {
            Ok(val) => val,
            Err(_) => {
                esp_println::println!("Error reading ADC!");
                continue;
            }
        };

        // Calculate absolute difference from baseline
        let diff = sound_value.abs_diff(baseline);

        sample_count += 1;

        // Check if sound variance exceeds sensitivity threshold
        if diff > SOUND_SENSITIVITY {
            led.set_high();
            esp_println::println!(
                ">>> SOUND DETECTED! Level: {} mV (Diff: {}) <<<",
                sound_value,
                diff
            );
            delay.delay_millis(LED_ON_DURATION_MS);
            led.set_low();
            sample_count = 0;
        } else if sample_count.is_multiple_of(50) {
            esp_println::println!("Silence... (Level: {} mV)", sound_value);
        }

        // Fast sampling
        delay.delay_millis(5);
    }
}
