// SPDX-License-Identifier: MIT

//! Console support for the Virt device

use crate::{bsp::device_driver, console};

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

/// Create a new emergency output driver so the panic function may print some debug information
///
/// Safety:
/// - must only be used by the panic function when crashing the kernel
pub unsafe fn panic_console() -> impl console::interface::All {
    use crate::driver::interface::DeviceDriver;

    let uart = device_driver::UartDevice::new(0x1000_0000 as *mut u8);
    uart.init().unwrap();
    uart
}

/// Return a reference to the console
pub fn console() -> &'static impl console::interface::All {
    &super::UART
}
