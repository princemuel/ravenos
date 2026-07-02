use core::fmt::{self, Write};

use crate::sbi;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            sbi::console_write_byte(byte);
        }
        Ok(())
    }
}

pub(crate) fn print(args: fmt::Arguments<'_>) { let _ret = Stdout.write_fmt(args).ok(); }

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
