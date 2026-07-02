#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    clear_bss();

    println!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    unsafe extern "C" {
        static sbss: u8;
        static ebss: u8;
    }

    let start: *mut u8 = (&raw const sbss).cast_mut();

    let len = (&raw const ebss)
        .addr()
        .checked_sub((&raw const sbss).addr())
        .expect("ebss must not precede sbss - check the linker script's section ordering");

    // SAFETY: `sbss`/`ebss` are linker-provided symbols marking the bounds
    // of the .bss section (see linker.ld). At this point in early boot, no
    // Rust references exist into this range, and the CPU's stack pointer is set to
    // `.bss.stack`, which the linker script places outside [sbss, ebss). The
    // range is therefore valid for writes and unaliased.
    unsafe {
        core::ptr::write_bytes(start, 0, len);
    }
}
