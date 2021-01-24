// SPDX-License-Identifier: MIT

//! The custom Panic Handler of the OS

use core::panic::PanicInfo;

use crate::{bsp, console};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Safety: this is a last attempt at printing information before
    //         halting execution, safety is not critical here
    unsafe {
        use console::interface::All;
        bsp::console::panic_console().write_fmt(format_args_nl!("{:?}", info));
    }

    // Safety: this code does not interfere with anything else
    unsafe { asm!("1: wfi \n j 1b") };
    unreachable!()
}
