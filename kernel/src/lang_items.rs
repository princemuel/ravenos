use core::panic::PanicInfo;

use log::error;

use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(loc) = info.location() {
        error!("[kernel] Panicked at {}:{} {}", loc.file(), loc.line(), info.message());
    } else {
        error!("[kernel] Panicked: {}", info.message());
    }
    shutdown(true)
}
