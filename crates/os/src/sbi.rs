pub(crate) fn console_write_byte(byte: u8) { let _ = sbi_rt::console_write_byte(byte); }

#[expect(dead_code, reason = "will use this later")]
pub(crate) fn console_putchar(ch: char) {
    let mut buf = [0u8; 4]; // max UTF-8 encoding length for any `char`
    for byte in ch.encode_utf8(&mut buf).bytes() {
        console_write_byte(byte);
    }
}

#[expect(clippy::panic, reason = "unreachable used due to never return type")]
pub(crate) fn shutdown(failed: bool) -> ! {
    use sbi_rt::{NoReason, Shutdown, SystemFailure, system_reset};

    let ret = if failed {
        system_reset(Shutdown, SystemFailure)
    } else {
        system_reset(Shutdown, NoReason)
    };

    panic!("system_reset returned unexpectedly: {ret:?}");
}
