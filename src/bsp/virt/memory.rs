// SPDX-License-Identifier: MIT

//! Memory Management for the Virt Device

use core::{cell::UnsafeCell, ops::RangeInclusive};

//------------------------------------------------------------------------------
//- Symbols
//------------------------------------------------------------------------------

/// Memory Map of this board
pub(super) mod map {
    pub const UART_MMIO: *mut u8 = 0x1000_0000 as *mut u8;
}

// Symbols from the linker script.
extern "Rust" {
    static __bss_start: UnsafeCell<u64>;
    static __bss_end: UnsafeCell<u64>;
}

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

/// Returns the range of the .bss section
pub fn bss_range() -> RangeInclusive<*mut u64> {
    // Safety: The values are provided by the Linker Script
    unsafe {
        let start = __bss_start.get();
        let end = __bss_end.get();
        assert!(end > start);

        RangeInclusive::new(start, end.sub(1))
    }
}
