# Building
target := "riscv64gc-unknown-none-elf"
mode := "release"
kernel_elf := "target" / target / mode / "os"
kernel_bin := kernel_elf + ".bin"
disasm_tmp := "target" / target / mode / "asm"

mode_arg := if mode == "release" { "--release" } else { "" }

# BOARD
board := "qemu"
sbi := env("SBI", "rustsbi")
bootloader := "bootloader/" + sbi + "-" + board + ".bin"

# KERNEL ENTRY
kernel_entry_pa := "0x80200000"

# Binutils
objdump := "rust-objdump --arch-name=riscv64"
objcopy := "rust-objcopy --binary-architecture=riscv64"

# Disassembly
disasm := env("DISASM", "-x")

qemu_name := "qemu-system-riscv64"
qemu_args := "-machine virt -nographic -bios " + bootloader + " -device loader,file=" + kernel_bin + ",addr=" + kernel_entry_pa

default: build

build: env kernel-bin

env:
    (rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add {{ target }}
    (command -v rust-objcopy >/dev/null && command -v rust-objdump >/dev/null) || cargo install cargo-binutils
    rustup component add rust-src
    rustup component add llvm-tools-preview

kernel-bin: kernel
    {{ objcopy }} {{ kernel_elf }} --strip-all -O binary {{ kernel_bin }}

kernel:
    cd user && make build
    @echo Platform: {{ board }}
    cp src/linker-{{ board }}.ld src/linker.ld
    cargo build {{ mode_arg }}
    rm src/linker.ld

clean:
    cargo clean

disasm: kernel
    {{ objdump }} {{ disasm }} {{ kernel_elf }} | less

disasm-vim: kernel
    {{ objdump }} {{ disasm }} {{ kernel_elf }} > {{ disasm_tmp }}
    nvim {{ disasm_tmp }}
    rm {{ disasm_tmp }}

qemu-version-check:
    sh scripts/qemu-version-check.sh {{ qemu_name }}

run: qemu-version-check build
    qemu-system-riscv64 {{ qemu_args }}

# Launches QEMU (halted, waiting for gdb) and a gdb client, each in its own Konsole window.
debug: qemu-version-check build
    konsole --new-tab -e bash -c "qemu-system-riscv64 {{ qemu_args }} -s -S; exec bash" &
    sleep 1
    konsole --new-tab -e bash -c "riscv64-unknown-elf-gdb -ex 'file {{ kernel_elf }}' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'; exec bash" &

gdbserver: qemu-version-check build
    qemu-system-riscv64 {{ qemu_args }} -s -S

gdbclient: build
    riscv64-unknown-elf-gdb -ex 'file {{ kernel_elf }}' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'
