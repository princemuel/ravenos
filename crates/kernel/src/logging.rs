use crate::console;

#[inline]
pub fn init() {
    console::init(&Console);
    console::set_log_level(option_env!("LOG"));
}

struct Console;
impl console::Stdout for Console {
    fn putc(&self, val: u8) { let _ = sbi_rt::console_write_byte(val); }
}
