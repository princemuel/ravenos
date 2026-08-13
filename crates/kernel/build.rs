use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is always set by cargo"),
    );
    let linker_script = manifest_dir.join("src/linker.ld");

    println!("cargo:rustc-link-arg=-T{}", linker_script.display());

    // Re-run this build script if the linker script changes, so edits
    // to memory layout are picked up without a `cargo clean`.
    println!("cargo:rerun-if-changed={}", linker_script.display());
}
