#[lang = "eh_personality"] pub extern fn eh_personality() {}

use core::panic::PanicInfo;
#[panic_implementation] #[no_mangle] pub fn panic(_info : &PanicInfo) -> ! { loop{} }

pub enum c_void {}
pub type c_char = i8;
pub type c_int = i32;
pub type c_long = i64;
pub type c_longlong = i64;
pub type c_schar = i8;
pub type c_short = i16;
pub type c_uchar = u8;
pub type c_uint = u32;
pub type c_ulong = u64;
pub type c_ulonglong = u64;
pub type c_ushort = u16;
