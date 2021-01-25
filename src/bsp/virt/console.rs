// SPDX-License-Identifier: MIT

//! Console support for the Virt device

use super::memory;
use crate::{bsp::device_driver, console::interface::FullConsole};

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

/// Create a new emergency output driver so the panic function may print some debug information
///
/// Safety:
/// - must only be used by the panic function when crashing the kernel
pub unsafe fn panic_console() -> impl FullConsole {
    use crate::driver::interface::DeviceDriver;

    let uart = device_driver::UartDevice::new(memory::map::UART_MMIO);
    uart.init().unwrap();
    uart
}

/// Return a reference to the console
pub fn console() -> &'static impl FullConsole {
    &super::UART
}
