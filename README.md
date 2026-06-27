# Ravenos

An experimental 64-bit RISC-V operating system kernel written in Rust.

Ravenos is a bare-metal S-mode kernel targeting the RISC-V architecture, built from scratch as a learning project and reference implementation. The goal is to progressively implement the core subsystems of a real operating system (boot, console I/O, batch execution, memory management) on QEMU-emulated RISC-V hardware, without relying on any existing OS or runtime.

## Milestones

- [x] Bare-metal environment setup (`no_std`)
- [ ] Bootstrapping via SBI / OpenSBI
- [ ] Basic UART/Console text output (`println!` macro)
- [ ] Batch application loading and execution
- [ ] Memory Management & Paging

## Quick Start

Once prerequisites are installed (see below), clone and run:

```bash
git clone https://github.com/princemuel/ravenos.git
cd ravenos
cargo run
```

This will build the kernel and launch it inside a QEMU RISC-V 64 virtual machine.

## Prerequisites

### Rust Toolchain

This project requires Rust installed via `rustup`.

**Linux / macOS / WSL:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

> Add `source "$HOME/.cargo/env"` to your `.bashrc` or `.zshrc` to make this permanent.

**Windows:**

Download and run `rustup-init.exe` from [https://rustup.rs](https://rustup.rs), then open a new terminal and continue from the next step.

> **Note:** Windows support is untested. WSL2 is the recommended path on Windows.

Verify installation:

```bash
rustc --version
cargo --version
```

### RISC-V 64 Compilation Target

```bash
rustup target add riscv64gc-unknown-none-elf
```

This target triplet means:

| Component   | Meaning                                                                |
| ----------- | ---------------------------------------------------------------------- |
| `riscv64gc` | RISC-V 64-bit with G (IMAFD) and C (compressed) instruction extensions |
| `unknown`   | No specific CPU vendor                                                 |
| `none`      | No operating system                                                    |
| `elf`       | ELF output, no standard runtime                                        |

> [!TIP]
> **G = IMAFD:** integer base (I) + multiply (M) + atomics (A) + single-precision float (F) + double-precision float (D). `riscv64gc` is shorthand for `riscv64imafdc`.

### QEMU Emulator

Ravenos runs on a QEMU-emulated RISC-V 64 virtual machine and requires `qemu-system-riscv64` >= 7.0.

**Arch Linux:**

```bash
sudo pacman -S qemu-system-riscv
```

**Ubuntu / Debian:**

```bash
sudo apt update -y
sudo apt install qemu-system-misc
```

**macOS (Homebrew):**

```bash
brew install qemu
```

**Verify:**

```bash
qemu-system-riscv64 --version
```

> **Other Linux distros:** Install the QEMU RISC-V package equivalent for your distro (e.g. `qemu-system-riscv` on Fedora/DNF, `qemu` on Gentoo/Portage). The package name varies but the binary is always `qemu-system-riscv64`.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
