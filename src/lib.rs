#![no_std]
#![feature(lang_items)]

#![crate_type = "staticlib"]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub unsafe extern fn rust_appletMainLoop() -> bool {
  appletMainLoop()
}

#[no_mangle] pub unsafe extern fn rust_gfxInitDefault() { gfxInitDefault() }
#[no_mangle] pub unsafe extern fn rust_hidScanInput() { hidScanInput() }
#[no_mangle] pub unsafe extern fn rust_gfxFlushBuffers() { gfxFlushBuffers() }
#[no_mangle] pub unsafe extern fn rust_gfxSwapBuffers() { gfxSwapBuffers() }
#[no_mangle] pub unsafe extern fn rust_gfxWaitForVsync() { gfxWaitForVsync() }
#[no_mangle] pub unsafe extern fn rust_gfxExit() { gfxExit() }

#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
  input * 2
}

pub mod lang_items;
