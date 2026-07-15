use core::panic::PanicInfo;

use crate::sbi::shutdown;

#[panic_handler]
pub(crate) fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(loc) = info.location() {
        println!("Panicked at {}:{} {}", loc.file(), loc.line(), info.message());
    } else {
        println!("Panicked: {}", info.message());
    }

    shutdown(true)
}
