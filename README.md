# Rust WASM HAL

An [embedded-hal](https://github.com/rust-embedded/embedded-hal/) implementation for [WebAssembly (WASM)](https://webassembly.org/), designed to support the development of abstract WASM applications for arbitrary embedded platforms. This is intended to be extended with useful IoT interfaces to allow the construction of both drivers and full-features WASM applications.

Check out the [docs](https://docs.rs/wasm-hal) for usage or the [runtime](https://github.com/ryankurte/rust-wasm-hal/blob/master/src/runtime.rs) file for the underlying WASM interface. WASM applications are passed a set of arguments to allow external configuration so pins etc. can be selected for your platform.

This is split from work on [ESP32 WASM support](https://github.com/ryankurte/rust-esp32-wasm/) on the basis that this API can be common, allowing applications to be compiled and run on arbitrary WASM supporting platforms.


## Status / Features

WASM-HAL components:

- [ ] Core functions
  - [x] fetch arguments
- [ ] HAL functions
  - [x] delay
  - [x] print
  - [x] spi
  - [x] i2c
  - [x] gpio
  - [ ] uart / serial
- [ ] Extended functions
  - [ ] Device individualisation (get serial, p/n, set hostname, etc.)
  - [ ] Event publishing / subscription
  - [ ] Displays / rendering
  - [ ] WiFi connection / management
  - [ ] MQTT connection / management


## Getting Started


You'll need the `wasm-unknown-unknown` rust target installed, as well as `wasm-opt` from [binaryen](https://github.com/WebAssembly/binaryen) to strip debug symbols, check out the [example](https://github.com/ryankurte/rust-esp32-wasm/tree/master/example) to get started.

- Build with `cargo build --release`
- Optimize with: `wasm-opt -Oz -o test.wasm --strip-debug --strip-dwarf /media/tmp/wasm32-unknown-unknown/release/YOUR_BINARY.wasm`
- _Optional_ Check sizes with `twiggy top -n 21 test.wasm` and `twiggy dominators test.wasm`



## Notes

- You need to minimize the rustc stack size `"-C", "link-arg=-zstack-size=32768"` otherwise rustc defaults to using 1MB of stack and this won't run on devices without SPIRAM. The tradeoff here is that you may run out of stack space, so, ymmv.


## I have a problem and/or can I help?

Sure! Open an issue or a PR ^_^

