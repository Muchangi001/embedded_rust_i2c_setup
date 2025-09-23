# I2C LCD Display Driver for ESP32

A robust no_std Rust driver for HD44780-based LCD displays with PCF8574 I2C backpack, designed for ESP32 microcontrollers using the `esp-hal` framework.

## Features

- **Automatic I2C Address Detection** - Scans common addresses (0x27, 0x3F, 0x20, 0x38)
- **Robust Initialization** - Multiple retry attempts with proper timing delays
- **4-bit Mode Operation** - Efficient communication with HD44780 controllers
- **Backlight Control** - Built-in backlight testing and control
- **Error Handling** - Comprehensive error reporting with LED feedback
- **Built-in Testing** - Self-test patterns to verify display functionality

## Hardware Requirements

- ESP32 microcontroller
- 16x2 LCD display with PCF8574 I2C backpack
- Connections:
  - SDA: GPIO21
  - SCL: GPIO22
  - Status LED: GPIO2

## Usage

### Basic Setup

```rust
let i2c = I2c::new(peripherals.I2C0, I2cConfig::default().with_frequency(Rate::from_khz(10)))
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22);

let mut lcd = LcdI2c::new(i2c).expect("LCD initialization failed");
```

### Display Operations

```rust
// Clear display
lcd.clear()?;

// Set cursor position (col, row)
lcd.set_cursor(0, 0)?;

// Print text
lcd.print("Hello, World!")?;

// Print at specific position
lcd.print_at(0, 1, "Line 2")?;

// Run self-test
lcd.test_display()?;
```

## Error Indicators

The driver provides visual feedback through GPIO2 LED:

- **2 quick blinks**: Startup sequence
- **1 long blink**: LCD found successfully
- **3 quick blinks**: Display test passed
- **5 quick blinks**: Display test failed
- **SOS pattern**: LCD not detected or initialization failed

## Technical Details

### I2C Configuration
- **Speed**: 10kHz (slow for maximum compatibility)
- **Addresses scanned**: 0x27, 0x3F, 0x20, 0x38

### LCD Initialization Sequence
1. Extended power-up delay (200ms)
2. Backlight test
3. Software reset sequence
4. 8-bit to 4-bit mode transition
5. Display configuration

### Communication Protocol
- 4-bit mode operation
- Proper timing delays between commands
- Backlight control integrated
- RS pin control for data/command selection

## Project Structure

```
src/
├── main.rs                 # Main application with LCD driver
└── (no additional modules)
```

## Dependencies

- `esp-hal` - ESP32 hardware abstraction layer
- `embedded-hal` - Hardware abstraction traits

## Building

```bash
cargo build --release
```

## License

This project is designed for embedded systems use with the ESP32 (wroom32) platform.