#[lang = "eh_personality"] pub extern fn eh_personality() {}

#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop{} }

pub type c_ushort = u16;
pub type c_uint = u32;
pub type c_int = i32;
pub type c_ulong = u64;
pub enum c_void {}
pub type c_char = i8;
