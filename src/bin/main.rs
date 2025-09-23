#![no_std]
#![no_main]
#![allow(dead_code)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{I2c, Config as I2cConfig};
use embedded_hal::i2c::I2c as I2cTrait;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

const LCD_ADDRESSES: [u8; 4] = [0x27, 0x3F, 0x20, 0x38];

const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_CURSORSHIFT: u8 = 0x10;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_SETCGRAMADDR: u8 = 0x40;
const LCD_SETDDRAMADDR: u8 = 0x80;

const LCD_ENTRYRIGHT: u8 = 0x00;
const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

const LCD_DISPLAYON: u8 = 0x04;
const LCD_DISPLAYOFF: u8 = 0x00;
const LCD_CURSORON: u8 = 0x02;
const LCD_CURSOROFF: u8 = 0x00;
const LCD_BLINKON: u8 = 0x01;
const LCD_BLINKOFF: u8 = 0x00;

const LCD_8BITMODE: u8 = 0x10;
const LCD_4BITMODE: u8 = 0x00;
const LCD_2LINE: u8 = 0x08;
const LCD_1LINE: u8 = 0x00;
const LCD_5X10DOTS: u8 = 0x04;
const LCD_5X8DOTS: u8 = 0x00;

const RS: u8 = 0x01;
const RW: u8 = 0x02;
const EN: u8 = 0x04;
const BACKLIGHT: u8 = 0x08;

struct LcdI2c<I2C> {
    i2c: I2C,
    address: u8,
    backlight_state: u8,
}

impl<I2C> LcdI2c<I2C>
where
    I2C: I2cTrait,
{
    fn new(mut i2c: I2C) -> Result<Self, &'static str> {
        let mut found_address = None;
        
        for &addr in &LCD_ADDRESSES {
            if i2c.write(addr, &[0x00]).is_ok() {
                found_address = Some(addr);
                break;
            }
        }
        
        let address = found_address.ok_or("No LCD found")?;
        
        let mut lcd = LcdI2c {
            i2c,
            address,
            backlight_state: BACKLIGHT,
        };
        
        for attempt in 0..3 {
            if lcd.init().is_ok() {
                return Ok(lcd);
            }
            lcd.delay_ms(100 * (attempt + 1) as u64);
        }
        
        Err("LCD init failed after 3 attempts")
    }
    
    fn init(&mut self) -> Result<(), &'static str> {
        self.delay_ms(200);
        
        self.backlight_test()?;
        
        for _ in 0..3 {
            self.write_raw(0x00)?;
            self.delay_ms(10);
            
            self.write_raw(0x30 | BACKLIGHT)?;
            self.pulse_enable_raw(0x30 | BACKLIGHT)?;
            self.delay_ms(50);
            
            self.write_raw(0x30 | BACKLIGHT)?;
            self.pulse_enable_raw(0x30 | BACKLIGHT)?;
            self.delay_ms(10);
            
            self.write_raw(0x30 | BACKLIGHT)?;
            self.pulse_enable_raw(0x30 | BACKLIGHT)?;
            self.delay_ms(5);
            
            self.write_raw(0x20 | BACKLIGHT)?;
            self.pulse_enable_raw(0x20 | BACKLIGHT)?;
            self.delay_ms(5);
            
            if self.send_command(LCD_FUNCTIONSET | LCD_4BITMODE | LCD_2LINE | LCD_5X8DOTS).is_ok() {
                break;
            }
            self.delay_ms(100);
        }
        
        self.send_command(LCD_DISPLAYCONTROL | LCD_DISPLAYOFF)?;
        self.send_command(LCD_CLEARDISPLAY)?;
        self.delay_ms(10);
        self.send_command(LCD_ENTRYMODESET | LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT)?;
        self.send_command(LCD_DISPLAYCONTROL | LCD_DISPLAYON | LCD_CURSOROFF | LCD_BLINKOFF)?;
        
        Ok(())
    }
    
    fn backlight_test(&mut self) -> Result<(), &'static str> {
        self.write_raw(0x00)?;
        self.delay_ms(100);
        self.write_raw(BACKLIGHT)?;
        self.delay_ms(100);
        Ok(())
    }
    
    fn write_raw(&mut self, data: u8) -> Result<(), &'static str> {
        self.i2c.write(self.address, &[data]).map_err(|_| "I2C write failed")
    }
    
    fn pulse_enable_raw(&mut self, data: u8) -> Result<(), &'static str> {
        self.write_raw(data | EN)?;
        self.delay_us(2);
        self.write_raw(data & !EN)?;
        self.delay_us(100);
        Ok(())
    }
    
    fn send_command(&mut self, cmd: u8) -> Result<(), &'static str> {
        let high = (cmd & 0xF0) | self.backlight_state;
        self.write_raw(high)?;
        self.pulse_enable_raw(high)?;
        
        let low = ((cmd << 4) & 0xF0) | self.backlight_state;
        self.write_raw(low)?;
        self.pulse_enable_raw(low)?;
        
        match cmd {
            LCD_CLEARDISPLAY | LCD_RETURNHOME => self.delay_ms(5),
            _ => self.delay_us(200),
        }
        Ok(())
    }
    
    fn send_data(&mut self, data: u8) -> Result<(), &'static str> {
        let high = (data & 0xF0) | self.backlight_state | RS;
        self.write_raw(high)?;
        self.pulse_enable_raw(high)?;
        
        let low = ((data << 4) & 0xF0) | self.backlight_state | RS;
        self.write_raw(low)?;
        self.pulse_enable_raw(low)?;
        
        self.delay_us(200);
        Ok(())
    }
    
    fn clear(&mut self) -> Result<(), &'static str> {
        self.send_command(LCD_CLEARDISPLAY)?;
        self.delay_ms(5);
        Ok(())
    }
    
    fn set_cursor(&mut self, col: u8, row: u8) -> Result<(), &'static str> {
        let row_offsets = [0x00, 0x40];
        if row < 2 && col < 16 {
            let pos = col + row_offsets[row as usize];
            self.send_command(LCD_SETDDRAMADDR | pos)?;
        }
        Ok(())
    }
    
    fn print(&mut self, text: &str) -> Result<(), &'static str> {
        for byte in text.bytes() {
            self.send_data(byte)?;
        }
        Ok(())
    }
    
    fn print_at(&mut self, col: u8, row: u8, text: &str) -> Result<(), &'static str> {
        self.set_cursor(col, row)?;
        self.print(text)
    }
    
    fn get_address(&self) -> u8 {
        self.address
    }
    
    fn test_display(&mut self) -> Result<(), &'static str> {
        self.clear()?;
        
        self.set_cursor(0, 0)?;
        for _ in 0..16 {
            self.send_data(b'A')?;
        }
        
        self.set_cursor(0, 1)?;
        for _ in 0..16 {
            self.send_data(b'B')?;
        }
        
        self.delay_ms(2000);
        
        self.clear()?;
        self.print_at(0, 0, "0123456789ABCDEF")?;
        self.print_at(0, 1, "Test Pattern OK!")?;
        
        Ok(())
    }
    
    fn delay_ms(&self, ms: u64) {
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(ms) {}
    }
    
    fn delay_us(&self, us: u64) {
        let start = Instant::now();
        while start.elapsed() < Duration::from_micros(us) {}
    }
}

fn blink_led(led: &mut Output, count: u8, on_ms: u64, off_ms: u64) {
    for _ in 0..count {
        led.set_high();
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(on_ms) {}
        led.set_low();
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(off_ms) {}
    }
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    
    blink_led(&mut led, 2, 200, 200);
    
    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default().with_frequency(Rate::from_khz(10)))
        .expect("I2C init failed")
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);
    
    let mut lcd = match LcdI2c::new(i2c) {
        Ok(mut l) => {
            blink_led(&mut led, 1, 500, 0);
            
            if l.test_display().is_ok() {
                blink_led(&mut led, 3, 100, 100);
            } else {
                blink_led(&mut led, 5, 100, 100);
            }
            l
        },
        Err(_) => {
            loop {
                blink_led(&mut led, 3, 100, 100);
                blink_led(&mut led, 3, 300, 100);
                blink_led(&mut led, 3, 100, 100);
                let start = Instant::now();
                while start.elapsed() < Duration::from_secs(2) {}
            }
        }
    };
    
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(3) {}
    
    let mut counter = 0u32;
    
    loop {
        if let Err(_) = lcd.clear() {
            blink_led(&mut led, 2, 50, 50);
        }
        
        if let Err(_) = lcd.print_at(0, 0, "I2C configured!") {
            blink_led(&mut led, 2, 50, 50);
        }
        
        let ones = (counter % 10) as u8 + b'0';
        let tens = ((counter / 10) % 10) as u8 + b'0';
        let hundreds = ((counter / 100) % 10) as u8 + b'0';
        
        if let Err(_) = lcd.print_at(0, 1, "Count: ") {
            blink_led(&mut led, 2, 50, 50);
        }
        if let Err(_) = lcd.send_data(hundreds) {
            blink_led(&mut led, 2, 50, 50);
        }
        if let Err(_) = lcd.send_data(tens) {
            blink_led(&mut led, 2, 50, 50);
        }
        if let Err(_) = lcd.send_data(ones) {
            blink_led(&mut led, 2, 50, 50);
        }
        
        counter += 1;
        if counter > 999 { counter = 0; }
        
        led.set_high();
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(50) {}
        led.set_low();
        
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(1) {}
    }
}