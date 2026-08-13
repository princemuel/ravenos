use log::{Level, LevelFilter, Log, Metadata, Record};
use spin::Once;

pub trait Stdout: Sync {
    fn putc(&self, val: u8);

    #[inline]
    fn puts(&self, val: &str) {
        for byte in val.bytes() {
            self.putc(byte);
        }
    }
}

static CONSOLE: Once<&'static dyn Stdout> = Once::new();

#[inline]
pub fn init(console: &'static dyn Stdout) {
    let _ = CONSOLE.call_once(|| console);
    log::set_logger(&Logger).expect("failed to initialize Logger");
}

#[inline]
pub fn set_log_level(val: Option<&str>) {
    log::set_max_level(match val {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Info,
    });
}

#[inline]
pub fn test_log() {
    use crate::println;

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
    ($($arg:tt)*) => ($crate::console::print(core::format_args!($($arg)*)));
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
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool { true }

    // fn enabled(&self, metadata: &Metadata<'_>) -> bool { metadata.level() <=
    // Level::Info
    // }

    fn log(&self, record: &Record<'_>) {
        let color = match record.level() {
            Level::Error => 31u8, // Red
            Level::Warn => 93,    // BrightYellow
            Level::Info => 34,    // Blue
            Level::Debug => 32,   // Green
            Level::Trace => 90,   // BrightBlack
        };

        println!("\u{1b}[{color}m[{:>5}] {}\u{1b}[0m", record.level(), record.args(),);
    }

    fn flush(&self) {}
}
