#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(core_intrinsics)]

extern crate panic_abort;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const fn null<T>() -> *mut T { 0 as *mut T }

#[no_mangle]
pub extern "C" fn main() -> ! {
  unsafe {
    consoleInit(null());

    let mut k_held_old = HidControllerKeys(0);
    
    printf("\x1b[1;1HPress PLUS to exit.\n".as_ptr() as *const i8);
    printf("\x1b[2;1HOr any other key to see its value.\n".as_ptr() as *const i8);

    while appletMainLoop() {
      hidScanInput();
      let k_held = HidControllerKeys(hidKeysHeld(HidControllerID::CONTROLLER_P1_AUTO) as u32);

      if k_held == HidControllerKeys::KEY_PLUS {
        break;
      }

      if k_held != k_held_old {
        consoleClear();

        printf("\x1b[1;1HPress PLUS to exit.\n".as_ptr() as *const i8);
        printf("\x1b[2;1HOr any other key to see its value.\n".as_ptr() as *const i8);
        printf("\x1b[3;1HThis key is currently pressed: %d\n".as_ptr() as *const i8, k_held);
      }

      k_held_old = k_held;

      consoleUpdate(null());
    }

    consoleExit(null());
    panic!("Success. We shall however think on a way of exiting cleanly.");
  }
}

pub mod lang_items;
