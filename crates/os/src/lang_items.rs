use core::panic::PanicInfo;

use crate::sbi::shutdown;

#[panic_handler]
pub(crate) fn panic(info: &PanicInfo<'_>) -> ! {
    if let Some(location) = info.location() {
        println!("Panicked at {}:{} {}", location.file(), location.line(), info.message());
    } else {
        println!("Panicked: {}", info.message());
    }

    shutdown(true)
}
