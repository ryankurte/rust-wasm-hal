


//! Rust WASM HAL
//! An embedded-hal implementation for WebAssembly, designed to support
//! The assembly of WASM compatible device crates and 

#![no_std]

extern crate embedded_hal;

pub mod args;
pub mod print;
pub mod delay;
pub mod i2c;
pub mod spi;
pub mod gpio;

pub mod runtime;
pub mod error;

pub mod prelude;

#[cfg(feature = "logger")]
pub mod logger;

use crate::error::WasmError;
use crate::delay::WasmDelay;
use crate::args::WasmArgs;
use crate::i2c::WasmI2c;
use crate::spi::WasmSpi;
use crate::gpio::{WasmGpio, GpioMode};

/// Abstract device object
pub struct Device;

/// Singleton device object
static mut DEVICE: Option<Device> = Some(Device);

impl Device {
    /// Take the device singleton
    pub fn take() -> Option<Self> {
        // TODO: mutable static is not ideal, but at this point we only
        // have one process within the WASM runtime so...
        unsafe { DEVICE.take() }
    }

    pub fn args(&mut self, argc: u32, _argv: u32) -> WasmArgs {
        WasmArgs::new(argc as usize)
    }

    /// Get the current millisecond tick count from the underlying os
    pub fn get_ticks_ms(&self) -> u32 {
        let mut v = 0;
        unsafe { runtime::get_ticks(&mut v) };
        v as u32
    }

    /// Initialise an I2C peripheral
    pub fn gpio_init(&mut self, port: u32, pin: u32, mode: GpioMode) -> Result<WasmGpio, WasmError> {
        let i = WasmGpio(port, pin);

        i.init(mode)?;

        Ok(i)
    }

    /// Initialise an I2C peripheral
    pub fn i2c_init(&mut self, index: u32, frequency: u32, sda_pin: u32, scl_pin: u32) -> Result<WasmI2c, WasmError> {
        let i = WasmI2c(index);

        i.init(frequency, sda_pin, scl_pin)?;

        Ok(i)
    }

    /// Initialise an SPI peripheral
    pub fn spi_init(&mut self, index: u32, frequency: u32, mosi_pin: u32, miso_pin: u32, sck_pin: u32, cs_pin: Option<u32>) -> Result<WasmSpi, WasmError> {
        let i = WasmSpi(index);

        i.init(frequency, mosi_pin, miso_pin, sck_pin, cs_pin)?;

        Ok(i)
    }
}

// Convenience implementation of delay
impl embedded_hal::blocking::delay::DelayMs<u32> for Device {
    fn delay_ms(&mut self, m: u32) {
        WasmDelay::delay_ms(&mut WasmDelay, m);
    }
}



