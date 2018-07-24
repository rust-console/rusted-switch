#![feature(lang_items, const_fn, panic_implementation)]
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#![crate_type = "staticlib"]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub unsafe extern fn rust_main() {
  printf("\x1b[16;16HPress PLUS to exit.".as_ptr() as *const i8);

  while appletMainLoop() {
    hidScanInput();

    let kDown = HidControllerKeys(hidKeysDown(HidControllerID::CONTROLLER_P1_AUTO) as u32);

    if kDown == HidControllerKeys::KEY_PLUS {
      break;
    }

    printf("This key is pressed: %d\n".as_ptr() as *const i8, kDown);

    gfxFlushBuffers();
    gfxSwapBuffers();
    gfxWaitForVsync();
  }
}

pub mod lang_items;
