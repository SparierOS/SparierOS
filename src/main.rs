// SPDX-License-Identifier: MIT

//! The main kernel binary

#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

// the entry point is in `mod cpu`

mod bsp;
mod cpu;
mod memory;
mod panic;
mod runtime;

/// Kernel inizialisation code.
///
/// # Safety
///
/// - Only one hart must be active
unsafe fn kernel_init() -> ! {
    asm!("1: wfi \n j 1b");
    unreachable!()
}
