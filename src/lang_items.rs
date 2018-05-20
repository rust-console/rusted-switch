#[lang = "eh_personality"] pub extern fn eh_personality() {}

#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop{} }
