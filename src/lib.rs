#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(core_intrinsics)]

use core::intrinsics::abort;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const fn null<T>() -> *mut T { 0 as *mut T }

#[no_mangle]
pub extern "C" fn main() -> ! {
  unsafe {
    consoleInit(null());
    gfxInitDefault();

    printf("\x1b[16;16HPress PLUS to exit.".as_ptr() as *const i8);

    while appletMainLoop() {
      hidScanInput();

      let k_down = HidControllerKeys(hidKeysDown(HidControllerID::CONTROLLER_P1_AUTO) as u32);

      if k_down == HidControllerKeys::KEY_PLUS {
        break;
      }

      printf("This key is pressed: %d\n".as_ptr() as *const i8, k_down);

      gfxFlushBuffers();
      gfxSwapBuffers();
      gfxWaitForVsync();
    }

    gfxExit();
    abort();
  }
}

pub mod lang_items;
