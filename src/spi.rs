//! esp32-wasm SPI implementation
//! 
// Copyright 2020 Ryan Kurte


use embedded_hal::blocking::spi::*;

use crate::error::WasmError;

use crate::runtime;

/// SPI wrapper object
/// This contains the underlying peripheral ID
pub struct WasmSpi(pub(crate) u32);

impl WasmSpi {
    /// Initialise the underlying SPI device
    pub fn init(&self, freq: u32, mosi: u32, miso: u32, sck: u32, cs: Option<u32>) -> Result<(), WasmError> {
        let res = unsafe {
            match cs {
                Some(cs) => runtime::spi_init_with_cs(self.0, freq, mosi, miso, sck, cs),
                None => runtime::spi_init_no_cs(self.0, freq, mosi, miso, sck),
            }
        };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }

        Ok(())
    }

    /// De-initialise the underlying SPI device
    pub fn deinit(&mut self) {
        unsafe {
            runtime::spi_deinit(self.0);
        }
    }
}

impl Write<u8> for WasmSpi {
    type Error = WasmError;

    fn write(&mut self, data_out: &[u8]) -> Result<(), Self::Error> {
        let d = data_out.as_ptr();
        let l = data_out.len() as u32;

        let res = unsafe { runtime::spi_write(self.0, d, l) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }

        Ok(())
    }
    
}

impl Transfer<u8> for WasmSpi {
    type Error = WasmError;

    fn transfer<'w>(&mut self, data: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {

        let d_out = data.as_ptr();
        let l_out = data.len() as u32;

        let d_in = data.as_ptr();

        let res = unsafe { runtime::spi_transfer(self.0, d_out, d_in, l_out) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }

        Ok(data)
    }


}