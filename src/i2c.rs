//! esp32-wasm I2C implementation
//! 
// Copyright 2020 Ryan Kurte

use embedded_hal::blocking::i2c::*;

use crate::error::WasmError;
use crate::runtime;

// I2C wrapper object
pub struct WasmI2c(pub(crate) u32);

impl WasmI2c {
    pub fn init(&self, freq: u32, sda: u32, scl: u32) -> Result<(), WasmError> {
        
        let res = unsafe { runtime::i2c_init(self.0, freq, sda, scl) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }
        
        Ok(())
    }

    pub fn deinit(&mut self) {
        unsafe {
            runtime::i2c_deinit(self.0);
        }
    }
}

impl Read for WasmI2c {
    type Error = WasmError;

    fn read(&mut self, address: u8, data_in: &mut [u8]) -> Result<(), Self::Error> {
        let d = data_in.as_ptr();
        let l = data_in.len() as u32;

        let res = unsafe { runtime::i2c_read(self.0, address as u32, d, l) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }
        
        Ok(())
    }
}

impl Write for WasmI2c {
    type Error = WasmError;

    fn write(&mut self, address: u8, data_out: &[u8]) -> Result<(), Self::Error> {
        let d = data_out.as_ptr();
        let l = data_out.len() as u32;

        let res = unsafe { runtime::i2c_write(self.0, address as u32, d, l) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }
        
        Ok(())
    }
    
}

impl WriteRead for WasmI2c {
    type Error = WasmError;

    fn write_read(&mut self, address: u8, data_out: &[u8], data_in: &mut [u8]) -> Result<(), Self::Error> {
        let d_out = data_out.as_ptr();
        let l_out = data_out.len() as u32;

        let d_in = data_in.as_ptr();
        let l_in = data_in.len() as u32;

        let res = unsafe { runtime::i2c_write_read(self.0, address as u32, d_out, l_out, d_in, l_in) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }
        
        Ok(())
    }


}