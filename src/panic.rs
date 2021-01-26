// SPDX-License-Identifier: MIT

//! The custom Panic Handler of the OS

use crate::bsp;
use core::panic::PanicInfo;

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Safety: this is a last attempt at printing information before
    //         halting execution, safety is not critical here
    unsafe {
        use crate::console::interface::FullConsole;

        bsp::console::panic_console().write_fmt(format_args_nl!("{}", info));
    }

    // Safety: this code does not interfere with anything else
    unsafe { asm!("1: wfi \n j 1b") };
    unreachable!()
}
