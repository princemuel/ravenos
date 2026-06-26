use core::panic::PanicInfo;

#[panic_handler]
pub(crate) const fn panic(_info: &PanicInfo<'_>) -> ! { loop {} }
