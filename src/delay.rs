//! esp32-wasm delay implementation
//! 
// Copyright 2020 Ryan Kurte

use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use crate::runtime;
// Delay object abstract over WASM delay function
pub struct WasmDelay;

impl DelayMs<u32> for WasmDelay {

    fn delay_ms(&mut self, m: u32) {
        unsafe { runtime::delay_ms( m ) };
    }
}

impl DelayUs<u32> for WasmDelay {

    fn delay_us(&mut self, u: u32) {
        unsafe { runtime::delay_us( u ) };
    }
}
