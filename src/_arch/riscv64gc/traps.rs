// SPDX-License-Identifier: MIT

//! Architectural processor code.

// include the assembly file
// NOTE: as of 23.01.2021 rust-analyzer wrongly markes this line as an error
global_asm!(include_str!("traps.S"));

extern "C" {
    pub fn setup_trap_vector(kernel_registers: *mut usize);

    pub fn enter_into_usermode(registers: *mut usize, pc: usize) -> !;

    pub fn syscall(function: usize, ...) -> usize;

    pub fn return_from_trap(registers: *mut usize, pc: usize) -> !;
}

