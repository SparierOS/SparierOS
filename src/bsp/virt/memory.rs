// SPDX-License-Identifier: MIT

//! Memory Management for the Virt Device

use core::{cell::UnsafeCell, ops::RangeInclusive};

// Symbols from the linker script.
extern "Rust" {
    static __bss_start: UnsafeCell<u64>;
    static __bss_end: UnsafeCell<u64>;
}

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
