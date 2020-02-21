//! esp32-wasm GPIO implementation
//! 
// Copyright 2020 Ryan Kurte

use embedded_hal::digital::v2::*;

use crate::error::WasmError;
use crate::runtime;

// GPIO wrapper object
pub struct WasmGpio(pub(crate) u32, pub(crate) u32);

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GpioMode {
    Input = 0,
    Output = 1,
}

impl WasmGpio {
    /// Initialise a GPIO pin
    pub fn init(&self, mode: GpioMode) -> Result<(), WasmError> {
        
        let res = unsafe { runtime::gpio_init(self.0, self.1, mode as u32) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }
        
        Ok(())
    }

    pub fn deinit(&mut self) {
        unsafe {
            runtime::gpio_deinit(self.0, self.1);
        }
    }
}

impl OutputPin for WasmGpio {
    type Error = WasmError;

    fn set_high(&mut self) -> Result<(), Self::Error> {

        let res = unsafe { runtime::gpio_write(self.0, self.1, 1) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }
        
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {

        let res = unsafe { runtime::gpio_write(self.0, self.1, 0) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }
        
        Ok(())
    }
}

impl InputPin for WasmGpio {
    type Error = WasmError;

    fn is_high(&self) -> Result<bool, Self::Error> {
        let mut v: u32 = 0;
        let p = &mut v;

        let res = unsafe { runtime::gpio_read(self.0, self.1, p) };

        if res < 0 {
            return Err(WasmError::Runtime(res))
        }
        
        Ok(v != 0)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        let v = self.is_high()?;

        Ok(v == false)
    }
}