//! Grove 3-Axis Accelerometer (LIS3DHTR) Example
//!
//! This example demonstrates using a Grove 3-Axis Digital Accelerometer (LIS3DHTR)
//! to read acceleration data on X, Y, and Z axes. Connected to the Seeed Studio
//! Grove Base for XIAO (https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
//!
//! The following wiring is assumed:
//! - Grove LIS3DHTR Sensor connected to I2C connector on Grove Base
//! - SDA wire is connected to GPIO22 (SDA on XIAO ESP32-C6)
//! - SCL wire is connected to GPIO23 (SCL on XIAO ESP32-C6)
//!   https://github.com/espressif/arduino-esp32/blob/master/variants/XIAO_ESP32C6/pins_arduino.h
//!
//! Pin mapping:
//! - SDA => GPIO22
//! - SCL => GPIO23
//!
//! Behavior: Reads X, Y, Z acceleration in g-force and prints to serial.
//! Also detects orientation (which way is "up") and motion/shake events.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();

/// LIS3DHTR I2C address (can be 0x18 or 0x19 depending on SA0 pin)
const LIS3DHTR_ADDR: u8 = 0x19;

/// LIS3DHTR Registers
const REG_WHO_AM_I: u8 = 0x0F;
const REG_CTRL_REG1: u8 = 0x20;
const REG_CTRL_REG4: u8 = 0x23;
const REG_OUT_X_L: u8 = 0x28;

/// LIS3DHTR WHO_AM_I value
const LIS3DHTR_CHIP_ID: u8 = 0x33;

/// Acceleration range options
#[derive(Clone, Copy)]
#[allow(dead_code)]
enum AccelRange {
    Range2G = 0b00,  // ±2g
    Range4G = 0b01,  // ±4g
    Range8G = 0b10,  // ±8g
    Range16G = 0b11, // ±16g
}

impl AccelRange {
    fn sensitivity(self) -> f32 {
        // Sensitivity in mg/digit (high-resolution mode, 12-bit)
        match self {
            AccelRange::Range2G => 1.0,
            AccelRange::Range4G => 2.0,
            AccelRange::Range8G => 4.0,
            AccelRange::Range16G => 12.0,
        }
    }
}

/// Output data rate options
#[derive(Clone, Copy)]
#[allow(dead_code)]
enum DataRate {
    PowerDown = 0b0000,
    Hz1 = 0b0001,
    Hz10 = 0b0010,
    Hz25 = 0b0011,
    Hz50 = 0b0100,
    Hz100 = 0b0101,
    Hz200 = 0b0110,
    Hz400 = 0b0111,
}

/// LIS3DHTR sensor driver
struct Lis3dhtr<'a> {
    i2c: I2c<'a, esp_hal::Blocking>,
    range: AccelRange,
}

impl<'a> Lis3dhtr<'a> {
    /// Create a new LIS3DHTR driver instance
    fn new(mut i2c: I2c<'a, esp_hal::Blocking>, delay: &Delay) -> Result<Self, &'static str> {
        // Check chip ID
        let chip_id = Self::read_register(&mut i2c, REG_WHO_AM_I)?;
        if chip_id != LIS3DHTR_CHIP_ID {
            esp_println::println!(
                "Unexpected chip ID: 0x{:02X} (expected 0x{:02X})",
                chip_id,
                LIS3DHTR_CHIP_ID
            );
            return Err("Invalid chip ID - not a LIS3DHTR");
        }

        let range = AccelRange::Range2G;

        // Configure CTRL_REG1:
        // - ODR = 100Hz (0b0101)
        // - Low power mode disabled
        // - All axes enabled (X, Y, Z)
        Self::write_register(&mut i2c, REG_CTRL_REG1, 0b01010111)?;

        // Configure CTRL_REG4:
        // - Block data update enabled
        // - Little endian
        // - Full scale = ±2g
        // - High resolution mode enabled
        Self::write_register(&mut i2c, REG_CTRL_REG4, 0b10001000)?;

        delay.delay_millis(20);

        Ok(Self { i2c, range })
    }

    /// Read a single register
    fn read_register(i2c: &mut I2c<'a, esp_hal::Blocking>, reg: u8) -> Result<u8, &'static str> {
        let mut data = [0u8; 1];
        if i2c.write_read(LIS3DHTR_ADDR, &[reg], &mut data).is_err() {
            return Err("Failed to read register");
        }
        Ok(data[0])
    }

    /// Write a single register
    fn write_register(
        i2c: &mut I2c<'a, esp_hal::Blocking>,
        reg: u8,
        value: u8,
    ) -> Result<(), &'static str> {
        if i2c.write(LIS3DHTR_ADDR, &[reg, value]).is_err() {
            return Err("Failed to write register");
        }
        Ok(())
    }

    /// Set the measurement range
    #[allow(dead_code)]
    fn set_range(&mut self, range: AccelRange) -> Result<(), &'static str> {
        // Read current CTRL_REG4
        let mut ctrl4 = Self::read_register(&mut self.i2c, REG_CTRL_REG4)?;

        // Clear FS bits and set new range
        ctrl4 = (ctrl4 & 0b11001111) | ((range as u8) << 4);

        Self::write_register(&mut self.i2c, REG_CTRL_REG4, ctrl4)?;
        self.range = range;

        Ok(())
    }

    /// Read raw acceleration data for all axes
    fn read_raw(&mut self) -> Result<(i16, i16, i16), &'static str> {
        let mut data = [0u8; 6];
        // Set MSB of register address for auto-increment
        if self
            .i2c
            .write_read(LIS3DHTR_ADDR, &[REG_OUT_X_L | 0x80], &mut data)
            .is_err()
        {
            return Err("Failed to read acceleration data");
        }

        // Data is in little-endian format (LSB first)
        let x = i16::from_le_bytes([data[0], data[1]]);
        let y = i16::from_le_bytes([data[2], data[3]]);
        let z = i16::from_le_bytes([data[4], data[5]]);

        Ok((x, y, z))
    }

    /// Read acceleration in g-force for all axes
    pub fn read_acceleration(&mut self) -> Result<(f32, f32, f32), &'static str> {
        let (x_raw, y_raw, z_raw) = self.read_raw()?;

        // Convert to g using sensitivity
        // Raw values are 16-bit left-justified, so divide by 16 to get 12-bit value
        let sensitivity = self.range.sensitivity();
        let x_g = (x_raw as f32 / 16.0) * sensitivity / 1000.0;
        let y_g = (y_raw as f32 / 16.0) * sensitivity / 1000.0;
        let z_g = (z_raw as f32 / 16.0) * sensitivity / 1000.0;

        Ok((x_g, y_g, z_g))
    }

    /// Calculate total acceleration magnitude
    pub fn acceleration_magnitude(x: f32, y: f32, z: f32) -> f32 {
        libm::sqrtf(x * x + y * y + z * z)
    }

    /// Determine which axis is pointing up (detecting orientation)
    pub fn detect_orientation(x: f32, y: f32, z: f32) -> &'static str {
        let abs_x = libm::fabsf(x);
        let abs_y = libm::fabsf(y);
        let abs_z = libm::fabsf(z);

        if abs_z > abs_x && abs_z > abs_y {
            if z > 0.0 {
                "Z+ up (flat, face up)"
            } else {
                "Z- up (flat, face down)"
            }
        } else if abs_y > abs_x && abs_y > abs_z {
            if y > 0.0 {
                "Y+ up (standing, top up)"
            } else {
                "Y- up (standing, top down)"
            }
        } else if x > 0.0 {
            "X+ up (tilted right)"
        } else {
            "X- up (tilted left)"
        }
    }
}

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Give the system time to initialize
    delay.delay_millis(100);

    esp_println::println!("Grove 3-Axis Accelerometer (LIS3DHTR) Example");
    esp_println::println!("Initializing I2C...");

    // Configure I2C with GPIO6 (SDA) and GPIO7 (SCL)
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("Failed to create I2C")
        .with_sda(peripherals.GPIO22)
        .with_scl(peripherals.GPIO23);

    esp_println::println!("Initializing LIS3DHTR sensor...");

    // Initialize the LIS3DHTR sensor
    let mut sensor = match Lis3dhtr::new(i2c, &delay) {
        Ok(s) => s,
        Err(e) => {
            esp_println::println!("Failed to initialize LIS3DHTR: {}", e);
            loop {
                delay.delay_millis(1000);
            }
        }
    };

    esp_println::println!("Sensor initialized successfully!");
    esp_println::println!();
    esp_println::println!("Starting acceleration readings...");
    esp_println::println!("Move or tilt the sensor to see changes!");
    esp_println::println!("================================================");

    // Track previous magnitude for shake detection
    let mut prev_magnitude: f32 = 1.0;

    loop {
        match sensor.read_acceleration() {
            Ok((x, y, z)) => {
                let magnitude = Lis3dhtr::acceleration_magnitude(x, y, z);
                let orientation = Lis3dhtr::detect_orientation(x, y, z);

                // Detect shake/motion (significant change in magnitude)
                let delta = libm::fabsf(magnitude - prev_magnitude);
                let motion_status = if delta > 0.5 {
                    ">>> SHAKE DETECTED! <<<"
                } else if delta > 0.2 {
                    "Moving"
                } else {
                    "Still"
                };

                esp_println::println!("Acceleration (g):");
                esp_println::println!("  X: {:+.3}  Y: {:+.3}  Z: {:+.3}", x, y, z);
                esp_println::println!("  Magnitude: {:.3} g", magnitude);
                esp_println::println!("  Orientation: {}", orientation);
                esp_println::println!("  Status: {}", motion_status);
                esp_println::println!("------------------------------------------------");

                prev_magnitude = magnitude;
            }
            Err(e) => {
                esp_println::println!("Error reading sensor: {}", e);
            }
        }

        // Wait 100ms before next reading (10 Hz update rate for display)
        delay.delay_millis(100);
    }
}
