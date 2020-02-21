//! An I2C detect example using rust-wasm-hal
//!
// Copyright 2020 Ryan Kurte

#![no_std]
#![no_main]
#![feature(lang_items, start)]

#[macro_use]
extern crate log;

extern crate libc;
extern crate panic_abort;

extern crate embedded_hal;
use embedded_hal::prelude::*;

extern crate wasm_hal;
use wasm_hal::prelude::*;

#[no_mangle]
pub extern fn main(argc: u32, argv: u32) -> i32 {
    // Initialise logging support
    WasmLogger::init();

    // Take device instance
    let mut device = Device::take().unwrap();

    // Fetch arguments
    let mut args = device.args(argc, argv);
    if args.count() != 5 {
        info!("wasm i2cdetect");
        info!("usage: i2cdetect BUS FREQ SDA SCL");
        return 1;
    }

    // Fetch arguments
    let bus = match args.get_u32(1) {
        Some(v) => v,
        None => return 2,
    };
    let freq = match args.get_u32(2) {
        Some(v) => v,
        None => return 3,
    };
    let sda = match args.get_u32(3) {
        Some(v) => v,
        None => return 4,
    };
    let scl = match args.get_u32(4) {
        Some(v) => v,
        None => return 5,
    };

    // Connect to I2C device
    let mut i2c = match device.i2c_init(bus, freq, sda, scl) {
        Ok(v) => v,
        Err(_e) => return 6,
    };

    // For each possible address
    for i in 0..128 {
        let mut d = [0u8; 1];

        // Attempt a read
        let _ = i2c.read(i, &mut d);

        // Print the result
        info!("Address: {} response: {}", i, d[0]);
    }

    // Shutdown the I2C device
    i2c.deinit();

    return 0;
}
