extern crate bindgen;

use std::env;
use std::path::PathBuf;

pub fn main() {
    let devkitpro_path = env::var("DEVKITPRO").unwrap();

    println!("cargo:rustc-link-lib=static=nx");
    println!("cargo:rustc-link-search=native={}/libnx/lib", devkitpro_path);
    
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .trust_clang_mangling(false)
        .use_core()
        .ctypes_prefix("lang_items")

        .header("wrapper.h")

        .clang_arg(format!("-I{}/libnx/include", devkitpro_path))
        .clang_arg(format!("-I{}/devkitA64/aarch64-none-elf/include", devkitpro_path))

        .rustified_enum("HidNpadStyleTag")
        .rustified_enum("HidNpadIdType")
        .rustified_enum("HidNpadButton")

        .generate_inline_functions(true)

        .blocklist_type("u8")
        .blocklist_type("u16")
        .blocklist_type("u32")
        .blocklist_type("u64")
        
        .derive_default(true)

        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
