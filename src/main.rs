// SPDX-License-Identifier: MIT

//! The main kernel binary

#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![no_main]
#![no_std]

// the entry point is in `mod cpu`

mod bsp;
mod console;
mod cpu;
mod driver;
mod memory;
mod panic;
mod runtime;
mod synchronization;

/// Kernel initialization code.
///
/// # Safety
///
/// - Only one hart must be active
unsafe fn kernel_init() -> ! {
    driver::init();

    kernel_main()
}

/// Main Function, executed after initializing basic runtime and drivers
fn kernel_main() -> ! {
    println!("[+] Kernel started");

    println!("[+] Driver initialization:");
    use driver::interface::DriverManager;
    for device in bsp::driver::driver_manager().list_drivers().iter() {
        if device.initialized() {
            println!("    [+] {} : successfull", device.name());
        } else {
            println!("    [+] {} : failed", device.name());
        }
    }

    println!("Hello World!");

    // Safety: this code does not interfere with anything else
    unsafe { asm!("1: wfi \n j 1b") };
    unreachable!()
}
