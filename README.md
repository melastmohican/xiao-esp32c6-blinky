# xiao-esp32c6-blinky

Rust examples for the [Seeed Studio XIAO-ESP32-C6 board](https://www.seeedstudio.com/Seeed-Studio-XIAO-ESP32C6-p-5884.html) with Grove modules.

## Project Setup

Project was generated using esp-template:

```bash
cargo generate -a https://github.com/esp-rs/esp-template
⚠️   Favorite `https://github.com/esp-rs/esp-template` not found in config, using it as a git repository: https://github.com/esp-rs/esp-template
🤷   Project Name: xiao-esp32c6-blinky
🔧   Destination: /Users/mordor/Src/rust/rust-embedded/xiao-esp32c6-blinky ...
🔧   project-name: xiao-esp32c6-blinky ...
🔧   Generating template ...
✔ 🤷   Which MCU to target? · esp32c6
✔ 🤷   Configure advanced template options? · false
🔧   Moving generated files into: `/Users/mordor/Src/rust/rust-embedded/xiao-esp32c6-blinky`...
🔧   Initializing a fresh Git repository
✨   Done! New project created ~/Src/rust/rust-embedded/xiao-esp32c6-blinky
```
To build run this command:

```bash
cargo build --release
```

To flash this project, run this command:

```bash
cargo run --release
    Compiling xiao-esp32c6-blinky v0.1.0 (/Users/mordor/Src/rust/rust-embedded/xiao-esp32c6-blinky)
    Finished `release` profile [optimized + debuginfo] target(s) in 58.42s
     Running `espflash flash --monitor target/riscv32imac-unknown-none-elf/release/xiao-esp32c6-blinky`
[2024-10-31T00:22:53Z INFO ] Detected 2 serial ports
[2024-10-31T00:22:53Z INFO ] Ports which match a known common dev board are highlighted
[2024-10-31T00:22:53Z INFO ] Please select a port
[2024-10-31T00:23:31Z INFO ] Serial port: '/dev/cu.usbmodem1417401'
[2024-10-31T00:23:31Z INFO ] Connecting...
[2024-10-31T00:23:31Z INFO ] Using flash stub
Chip type:         esp32c6 (revision v0.1)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi 6, BT 5
MAC address:       f0:f5:bd:2d:0c:88
App/part. size:    21,040/4,128,768 bytes, 0.51%
[00:00:00] [========================================]      13/13      0x0                                                                                                                               [00:00:00] [========================================]       1/1       0x8000                                                                                                                            [00:00:00] [========================================]      13/13      0x10000                                                                                                                           [2024-10-31T00:23:32Z INFO ] Flashing has completed!
Commands:
    CTRL+R    Reset chip
    CTRL+C    Exit

ESP-ROM:esp32c6-20220919
Build:Sep 19 2022
rst:0x15 (USB_UART_HPSYS),boot:0x16 (DOWNLOAD(USB/UART0/SDIO_REI_FEO))
Saved PC:0x4080054c
0x4080054c - esp32c6::gpio::RegisterBlock::pin
    at /Users/mordor/.cargo/registry/src/index.crates.io-6f17d22bba15001f/esp32c6-0.16.0/src/gpio.rs:195
SPIWP:0xee
mode:DIO, clock div:2
load:0x4086c410,len:0xd48
load:0x4086e610,len:0x2d68
load:0x40875720,len:0x1800
entry 0x4086c410
I (23) boot: ESP-IDF v5.1-beta1-378-gea5e0ff298-dirt 2nd stage bootloader
I (23) boot: compile time Jun  7 2023 08:02:08
I (24) boot: chip revision: v0.1
I (28) boot.esp32c6: SPI Speed      : 40MHz
I (33) boot.esp32c6: SPI Mode       : DIO
I (37) boot.esp32c6: SPI Flash Size : 4MB
I (42) boot: Enabling RNG early entropy source...
I (48) boot: Partition Table:
I (51) boot: ## Label            Usage          Type ST Offset   Length
I (58) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (66) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (73) boot:  2 factory          factory app      00 00 00010000 003f0000
I (81) boot: End of partition table
I (85) esp_image: segment 0: paddr=00010020 vaddr=42000020 size=048dch ( 18652) map
I (97) esp_image: segment 1: paddr=00014904 vaddr=40800000 size=00014h (    20) load
I (102) esp_image: segment 2: paddr=00014920 vaddr=42004920 size=01fa8h (  8104) map
I (112) esp_image: segment 3: paddr=000168d0 vaddr=40800014 size=008e4h (  2276) load
I (120) boot: Loaded app from partition at offset 0x10000
I (125) boot: Disabling RNG early entropy source...
```

You can check board info:

```bash
espflash board-info
[2024-10-31T00:29:09Z INFO ] Detected 2 serial ports
[2024-10-31T00:29:09Z INFO ] Ports which match a known common dev board are highlighted
[2024-10-31T00:29:09Z INFO ] Please select a port
[2024-10-31T00:29:12Z INFO ] Serial port: '/dev/cu.usbmodem1417401'
[2024-10-31T00:29:12Z INFO ] Connecting...
[2024-10-31T00:29:13Z INFO ] Using flash stub
Chip type:         esp32c6 (revision v0.1)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi 6, BT 5
MAC address:       f0:f5:bd:2d:0c:88
```

## Examples

This project includes examples for Grove modules connected via the [Grove Base for XIAO](https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html).

### Main Application

The main application blinks the built-in LED on GPIO15.

```bash
cargo run --release
```

### Grove LED Example

Demonstrates controlling a Grove LED module.

**Hardware:**
- Grove LED module connected to **D7** (GPIO17)

**Run:**
```bash
cargo run --example grove_led
```

The LED blinks with 250ms ON, 1000ms OFF pattern.

### Grove Button Example

Demonstrates using a Grove Button to control a Grove LED.

**Hardware:**
- Grove Button connected to **D1** (GPIO1)
- Grove LED connected to **D7** (GPIO17)

**Run:**
```bash
cargo run --example grove_button
```

The LED turns ON when button is pressed, OFF when released.

### Grove Rotary Potentiometer Example

Demonstrates using a Grove Rotary Potentiometer to control LED blink rate via ADC.

**Hardware:**
- Grove Rotary Potentiometer connected to **A0** (GPIO0/ADC1_CH0)
- Grove LED connected to **D7** (GPIO17)

**Run:**
```bash
cargo run --example grove_potentiometer
```

Rotate the potentiometer to adjust the LED blink rate from 50ms to 1000ms per toggle. ADC values are printed to the serial console.

### Grove Light Sensor Example

Demonstrates using a Grove Light Sensor to control a Grove LED based on ambient light.

**Hardware:**
- Grove Light Sensor connected to **A0** (GPIO0)
- Grove LED connected to **D7** (GPIO17)

**Run:**
```bash
cargo run --example grove_light_sensor
```

The LED turns ON when the sensor is covered (Dark) and OFF when exposed to light.

### Grove Sound Sensor Example

Demonstrates using a Grove Sound Sensor to detect loud noises (like clapping) and trigger a Grove LED.

**Hardware:**
- Grove Sound Sensor connected to **A0** (GPIO0)
- Grove LED connected to **D7** (GPIO17)

**Run:**
```bash
cargo run --example grove_sound_sensor
```

The sensor self-calibrates on startup. Clap or make noise to trigger the LED.

### Grove OLED Display Example

Demonstrates drawing text, shapes, and animations on a Grove OLED Display (SSD1306).

**Hardware:**
- Grove OLED Display connected to **I2C** connector
- **SDA** (D4) -> GPIO22
- **SCL** (D5) -> GPIO23

**Run:**
```bash
cargo run --example grove_oled
```

### Grove Air Pressure Sensor (BMP280) Example

Demonstrates reading temperature, pressure, and calculating altitude using a Grove BMP280 sensor.

**Hardware:**
- Grove BMP280 Sensor connected to **I2C** connector
- **SDA** (D4) -> GPIO22
- **SCL** (D5) -> GPIO23

**Run:**
```bash
cargo run --example grove_air_pressure
```

### Grove Temperature & Humidity Sensor (AHT20) Example

Demonstrates reading temperature and humidity from a Grove AHT20 sensor.

**Hardware:**
- Grove AHT20 Sensor connected to **I2C** connector
- **SDA** (D4) -> GPIO22
- **SCL** (D5) -> GPIO23

**Run:**
```bash
cargo run --example grove_temp_humidity
```

## Pin Mapping Reference

| Grove Connector | XIAO ESP32-C6 Pin | Function |
|-----------------|-------------------|----------|
| A0 | GPIO0 | ADC1_CH0 / Analog In |
| A1 | GPIO1 | ADC1_CH1 |
| A2 | GPIO2 | ADC1_CH2 / PWM |
| D1 | GPIO1 | Digital I/O |
| D7 | GPIO17 | Digital I/O |
| I2C (Internal) | GPIO22 (SDA), GPIO23 (SCL) | I2C Bus (D4/D5) |

## Hardware Links

- [XIAO ESP32-C6](https://www.seeedstudio.com/Seeed-Studio-XIAO-ESP32C6-p-5884.html)
- [Grove Base for XIAO](https://www.seeedstudio.com/Grove-Shield-for-Seeeduino-XIAO-p-4621.html)
- [Grove Beginner Kit for Arduino](https://www.seeedstudio.com/Grove-Beginner-Kit-for-Arduino-p-4549.html)
