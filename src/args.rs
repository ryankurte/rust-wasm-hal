//! esp32-wasm args implementation
//!
//! This provides an adaptor to query for argc/argv style aruments
//! from the runtime.
//! 
// Copyright 2020 Ryan Kurte


use crate::runtime;

pub const ARG_MAX_LEN: usize = 32;

// Args object abstract over runtime args_get function
pub struct WasmArgs{
    count: usize,
    buff: [u8; ARG_MAX_LEN],
}

impl WasmArgs {
    /// Create a new argument adaptor with the provided number of args
    // TODO: should we query for the arg count as well..?
    pub fn new(count: usize) -> Self {
        Self{count, buff: [0u8; ARG_MAX_LEN]}
    }

    /// Fetch number of args
    pub fn count(&self) -> usize {
        self.count as usize
    }

    /// Fetch an argument by index
    pub fn get(&mut self, index: usize) -> Option<&str> {
        if index >= self.count {
            return None
        }

        let buff_p = self.buff.as_ptr();
        let buff_len = [self.buff.len() as u8];

        unsafe { 
            runtime::arg_get(index as u32, buff_p, buff_len.as_ptr())
        };

        if buff_len[0] == 0 {
            return None;
        }

        let s = unsafe {
            core::str::from_utf8_unchecked(&self.buff[..buff_len[0] as usize])
        };

        Some(s)
    }
}
