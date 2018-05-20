<h1 align="center">
  <img src="https://user-images.githubusercontent.com/735858/40282489-8b1954b6-5c46-11e8-8a37-edc3f6f3ffeb.png" height="300"/>
  <br/>
  Rusted Switch
</h1>

## How
This project uses [rust-bindgen](https://github.com/rust-lang-nursery/rust-bindgen) to create bindings for [libnx](https://github.com/switchbrew/libnx).

For some reason, I couldn't use Makefile to compile Rust, so that's why this `makew` file exists. Also, direct compilation from Rust to elf didn't find `_start` function to run the code, that's why there's a C -> Rust (-> C) architecture.

## Credits
- [rpi3-rust-template](https://github.com/cs140e/rpi3-rust-template) on building for aarch64-none-elf.
