#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
// use esp_hal::uart::{Uart, Config, DataBits, StopBits, Parity};
use esp_hal::gpio::{Level, Output, OutputConfig};
//use esp_hal::i2c::master::{I2c, Config};
use esp_hal::spi::master::{Spi, Config};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config); // peripherals singleton

    // Configure built in led as output (GPIO fuctionality)
    //let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());

    // Configure UART2 for HC-06 communication
    // let uart_config = Config::default()
    //     .with_baudrate(9600)
    //     .with_data_bits(DataBits::_8)
    //     .with_parity(Parity::None)
    //     .with_stop_bits(StopBits::_1);

    // let mut uart2 = Uart::new(peripherals.UART2, uart_config)
    //     .expect("Failed to initialize UART2")
    //     .with_tx(peripherals.GPIO17)
    //     .with_rx(peripherals.GPIO16);

    // Wait for HC-06 to initialize
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_secs(2) {}

    // Configure HC-06 module name
    //uart2.write(b"AT+NAMEIEEE_UoN").expect("Failed to send name command");
    //uart2.flush().expect("Failed to flash AT command");
    
    // Some delay for the AT command to be processed
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(1000) {}

    // Configure I2C
    // let mut i2c_driver = I2c::new(peripherals.I2C0, Config::default())
    // .expect("Failed to initialize I2C driver")
    // .with_sda(peripherals.GPIO21)
    // .with_scl(peripherals.GPIO22);

    // Configure SPI
    let mut spi_driver = Spi::new(peripherals.SPI2, Config::default())
    .expect("Failed to initialize SPI driver")
    .with_sck(peripherals.GPIO6)
    .with_mosi(peripherals.GPIO8)
    .with_miso(peripherals.GPIO7);

    loop {
        // uart2.write(b"Hello Muchangi\r\n").expect("Failed to send message");
        // led.toggle();

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_secs(2) {}
    }
}