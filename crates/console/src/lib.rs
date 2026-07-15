#![no_std]

use log::{Level, LevelFilter, Log, Metadata, Record};
use spin::Once;

pub trait Console: Sync {
    fn putc(&self, val: u8);

    #[inline]
    fn puts(&self, val: &str) {
        for byte in val.bytes() {
            self.putc(byte);
        }
    }
}

static CONSOLE: Once<&'static dyn Console> = Once::new();

#[inline]
pub fn init_console(console: &'static dyn Console) {
    let _ = CONSOLE.call_once(|| console);
    log::set_logger(&Logger).expect("failed to initialize Logger");
}

#[inline]
pub fn set_log_level(val: Option<&str>) {
    log::set_max_level(
        val.and_then(|lvl| lvl.parse().ok())
            .unwrap_or(LevelFilter::Trace),
    );
}

#[inline]
pub fn test_log() {
    println!(
        r"
   ______                       __
  / ____/___  ____  _________  / /__
 / /   / __ \/ __ \/ ___/ __ \/ / _ \
/ /___/ /_/ / / / (__  ) /_/ / /  __/
\____/\____/_/ /_/____/\____/_/\___/
===================================="
    );
    log::trace!("LOG TEST >> Hello, world!");
    log::debug!("LOG TEST >> Hello, world!");
    log::info!("LOG TEST >> Hello, world!");
    log::warn!("LOG TEST >> Hello, world!");
    log::error!("LOG TEST >> Hello, world!");
    println!();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print(core::format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", core::format_args!($($arg)*)));
}

#[doc(hidden)]
#[inline]
pub fn print(args: core::fmt::Arguments<'_>) {
    use core::fmt::Write as _;
    #[expect(clippy::expect_used, reason = "will refactor later")]
    Logger.write_fmt(args).expect("failed to write to stdout");
}

struct Logger;

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        CONSOLE.get().expect("failed to initialize Console").puts(s);
        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool { metadata.level() <= Level::Info }

    fn log(&self, record: &Record<'_>) {
        let code = match record.level() {
            Level::Error => 31u8,
            Level::Warn => 93,
            Level::Info => 34,
            Level::Debug => 32,
            Level::Trace => 90,
        };

        println!("\x1b[{code}m[{:>5}] {}\x1b[0m", record.level(), record.args());
    }

    fn flush(&self) {}
}
