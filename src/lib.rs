#![feature(lang_items)]
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#![crate_type = "staticlib"]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle] pub unsafe extern fn rust_appletMainLoop() -> bool { appletMainLoop() }
#[no_mangle] pub unsafe extern fn rust_gfxInitDefault() { gfxInitDefault() }
#[no_mangle] pub unsafe extern fn rust_hidScanInput() { hidScanInput() }
#[no_mangle] pub unsafe extern fn rust_gfxFlushBuffers() { gfxFlushBuffers() }
#[no_mangle] pub unsafe extern fn rust_gfxSwapBuffers() { gfxSwapBuffers() }
#[no_mangle] pub unsafe extern fn rust_gfxWaitForVsync() { gfxWaitForVsync() }
#[no_mangle] pub unsafe extern fn rust_gfxExit() { gfxExit() }

#[no_mangle]
pub unsafe extern fn like_a_main() {
  while appletMainLoop() {
    hidScanInput();

    let kDown = hidKeysDown(HidControllerID_CONTROLLER_P1_AUTO);

    if kDown & 10 > 0 { // KEY_PLUS
      break;
    }

    printf("Hello, world: %d\n".as_ptr() as *const i8, kDown);

    gfxFlushBuffers();
    gfxSwapBuffers();
    gfxWaitForVsync();
  }
}

pub mod lang_items;
