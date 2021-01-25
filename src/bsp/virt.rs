// SPDX-License-Identifier: MIT

//! BSP Bundle for the Virt Device

pub mod console;
pub mod driver;
pub mod memory;

use super::device_driver;

//------------------------------------------------------------------------------
//- Symbols
//------------------------------------------------------------------------------

/// Safety: 0x1000_0000 is a known address of the virt UART
static UART: device_driver::UartDevice =
    unsafe { device_driver::UartDevice::new(memory::map::UART_MMIO) };
