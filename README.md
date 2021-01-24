# SparierOS

SparierOS is an operating system developed for future RISC-V targets. 
The project is not intended to be used by anyone at the moment.

## Enviroment Setup

These are the steps to set up the recommended development enviorment, for this project:

1. **RiscV target**:
    Get the components for compiling to RiscV, ex. under Ubuntu run\
    `rustup target add riscv64gc-unknown-none-elf`
1. **qemu**:
    Install qemu for RiscV emulation, ex. under Ubuntu run\
    `apt install qemu-system-misc`
1. **cargo binutils**:
    Install the binutils for cargo, ex. under Ubuntu run\
    `cargo install cargo-binutils`
1. **llvm-tools**
    Install the llvm tools for compiling, ex. under Ubuntu run\
    `rustup component add llvm-tools-preview`

### Suggested VS Code Extensions
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
* [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
* [LinkerScript](https://marketplace.visualstudio.com/items?itemName=ZixuanWang.linkerscript)
* [RISC-V Support](https://marketplace.visualstudio.com/items?itemName=zhwu95.riscv)
* [Better Comments](https://marketplace.visualstudio.com/items?itemName=aaron-bond.better-comments)

## Project Structure

### Modules

The project is divided into different modules, each one responsible for a different key task or component of the OS.
Each module has its own `.rs` file in `src` folder, acting as a interface to the modules functionality, 
additional files belonging to the module may reside in a directory. 

> **Example:** The memory module resides in `src/memory.rs` and may have additional files in `src/memory/`

### Decoupling of Architecture

In order to decouple the OS as much as possible from the underlying architecture the `src/_arch/` module exists.
Each supported processor type has a subfolder in the `src/_arch/` directory, containing architecture specific code.
> **Example:** `src/_arch/riscv64gc/` for a RiscV 64-Bit CPU with extensions G & C

The architecture folders mirror the `src` folder. 
> **Example:** riscv64gc specific memory code goes into `src/_arch/riscv64gc/memory.rs` or a `src/_arch/riscv64gc/memory/` folder

Files in the architecture folder should usually **not** be accessed directly because they are conditionally reexported in the corresponding module files. 
> **Example:** `src/_arch/riscv64gc/memory.rs` will be exported in `src/memory.rs` if the project is compiled for a riscv64gc compatible target. 

Files in the `src/_arch/` folder/subfolders are imported conditionally with the `#[path = "_arch/**/*.rs"]` attribute.

### Board Specific Code

Board Support Packages are stored under the `src/bsp/` folder. 
Each supported board has an own subdirectory of `src/bsp/` which again mirrors the structure of the `/src/` folder. 
A subsection of the symbols provided in those files may be conditionally exported in the `src/bsp.rs` file.