//! ESP32 WASM Runtime definitions
//! 
//! This module defines the API available within the WASM runtime
//! Note that these methods are subject to change while we're discovering what works
//! 
// Copyright 2020 Ryan Kurte

/// WASM HAL API
///
/// This defines the WASM functions that're expected to support the wasm-hal
///  - Values should be passed via pointers, < 0 return codes for errors
///  - Peripherals are referred to by `port`, a numeric identifier correlating with the peripheral ID
///    (eg. port=1 -> USART1)
///  - Safety should be imposed by the underlying runtime as multiple wasm tasks could attempt to access
///    the same peripheral
#[link(wasm_import_module = "env")]
extern {
    /// Delay for the specified number of milliseconds
    pub fn delay_ms(m: u32);

    /// Delay for the specified number of microseconds
    pub fn delay_us(u: u32);

    /// Fetch the number of ticks since the task started
    pub fn get_ticks(m: &u32);

    /// Write out to system logs
    pub fn log_write(v: *const u8, l: u32);

    /// Fetch arguments from the OS
    /// Note that this may be depricated if a better approach is realised
    pub fn arg_get(index: u32, buff: *const u8, buff_len: *const u8) -> i32;


    /// Initialise the provided GPIO pin in input or output mode
    pub fn gpio_init(port: u32, pin: u32, output: u32) -> i32;

    /// Deinitialise the specified GPIO pin
    pub fn gpio_deinit(port: u32, pin: u32) -> i32;

    /// Write to a GPIO pin
    pub fn gpio_write(port: u32, pin: u32, value: u32) -> i32;

    // Read from a GPIO pin
    pub fn gpio_read(port: u32, pin: u32, value: *const u32) -> i32;


    /// Initialise the specified I2C port with the provided frequency and pins
    pub fn i2c_init(port: u32, freq: u32, sda: u32, scl: u32) -> i32;

    /// Deinitialise the specified I2C port
    pub fn i2c_deinit(port: u32) -> i32;

    /// Write to an I2C device on the specified port
    pub fn i2c_write(port: u32, address: u32, data_out: *const u8, data_out_len: u32) -> i32;

    /// Read from an I2C device on the specified port
    pub fn i2c_read(port: u32, address: u32, data_in: *const u8, data_in_len: u32) -> i32;

    /// Write to then read from an I2C device on the specified port
    pub fn i2c_write_read(port: u32, address: u32, data_out: *const u8, data_out_len: u32, data_in: *const u8, data_in_len: u32) -> i32;

    // TODO: work out how interface with transactional I2C API
    // see: https://github.com/rust-embedded/embedded-hal/pull/178


    /// Initialise the specified SPI peripheral with the provided frequency and pins (manually managing CS)
    pub fn spi_init_no_cs(port: u32, freq: u32, mosi: u32, miso: u32, sck: u32) -> i32;

    /// Initialise the specified SPI peripheral with the provided frequency and pins (driver managing CS)
    pub fn spi_init_with_cs(port: u32, freq: u32, mosi: u32, miso: u32, sck: u32, cs: u32) -> i32;

    /// Deinitialise the specified SPI peripheral
    pub fn spi_deinit(port: u32) -> i32;

    /// Write to an SPI device on the specified peripheral
    pub fn spi_write(port: u32, data_out: *const u8, data_out_len: u32) -> i32;

    /// Write to and read from an SPI device on the specified peripheral
    pub fn spi_transfer(port: u32, data_out: *const u8, data_in: *const u8, data_len: u32) -> i32;
    
    // TODO: work out how to interface with transactional SPI API
    // see: https://github.com/rust-embedded/embedded-hal/pull/178

}

