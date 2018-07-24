extern crate bindgen;

use std::env;
use std::path::PathBuf;

pub fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .trust_clang_mangling(false)
        .use_core()
        .ctypes_prefix("lang_items")

        .header("wrapper.h")

        .clang_arg("-I/opt/devkitpro/libnx/include")
        .clang_arg("-I/opt/devkitpro/devkitA64/aarch64-none-elf/include")
        
        .clang_arg("-I/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/8.1.0/include")
        // .whitelist_function("consoleInit")
        // .whitelist_function("hidKeysDown")
        .bitfield_enum("HidMouseButton")
        .bitfield_enum("HidKeyboardModifier")
        .rustified_enum("HidKeyboardScancode")
        .bitfield_enum("HidControllerType")
        .rustified_enum("HidControllerLayoutType")
        .bitfield_enum("HidControllerColorDescription")
        .bitfield_enum("HidControllerKeys")
        .rustified_enum("HidControllerJoystick")
        .bitfield_enum("HidControllerConnectionState")
        .rustified_enum("HidControllerID")

        .blacklist_type("u8")
        .blacklist_type("u16")
        .blacklist_type("u32")
        .blacklist_type("u64")
        
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native=/opt/devkitpro/libnx/lib/");
    println!("cargo:rustc-link-lib=static=nx");
}
