#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(core_intrinsics)]

extern crate panic_abort;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const fn null<T>() -> *mut T { 0 as *mut T }

#[no_mangle]
pub extern "C" fn main() {
  unsafe {
    consoleInit(null());

    // Configure our supported input layout: a single player with standard controller styles
    padConfigureInput(1, HidNpadStyleTag::HidNpadStyleSet_NpadStandard as u32);

    // Initialize the default gamepad (which reads handheld mode inputs as well as the first connected controller)
    let mut pad: PadState = Default::default();
    let mut padMask: u64 = 0;
    padMask |= (1 as u64) << HidNpadIdType::HidNpadIdType_No1 as u32;
    padMask |= (1 as u64) << HidNpadIdType::HidNpadIdType_Handheld as u32;
    padInitializeWithMask(&mut pad, padMask);

    let mut kHeldOld: u64 = 0;
    
    printf("\x1b[1;1HPress PLUS to exit.\n".as_ptr() as *const i8);
    printf("\x1b[2;1HOr any other key to see its value.\n".as_ptr() as *const i8);

    while appletMainLoop() {
      padUpdate(&mut pad);

      let kHeld = pad.buttons_cur;
      if (kHeld & (HidNpadButton::HidNpadButton_Plus as u64)) != 0 {
        break;
      }

      if kHeld != kHeldOld {
        consoleClear();

        printf("\x1b[1;1HPress PLUS to exit.\n".as_ptr() as *const i8);
        printf("\x1b[2;1HOr any other key to see its value.\n".as_ptr() as *const i8);
        printf("\x1b[3;1HThis key is currently pressed: %d\n".as_ptr() as *const i8, kHeld);
      }

      kHeldOld = kHeld;

      consoleUpdate(null());
    }

    consoleExit(null());
  }
}

pub mod lang_items;
