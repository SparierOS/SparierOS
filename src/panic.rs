// SPDX-License-Identifier: MIT

//! The custom Panic Handler of the OS

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: print _info to console for easier debugging
    // Safety: this code does not interfere with anything else
    unsafe { asm!("1: wfi \n j 1b") };
    unreachable!()
}
