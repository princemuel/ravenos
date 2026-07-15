use crate::sbi;

struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            sbi::console_write_byte(byte);
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub(crate) fn print(args: core::fmt::Arguments<'_>) {
    use core::fmt::Write as _;
    #[expect(clippy::expect_used, reason = "will refactor later")]
    Stdout.write_fmt(args).expect("failed to write to stdout");
}
