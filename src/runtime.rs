// SPDX-License-Identifier: MIT

//! Runtime for Rust and the kernel

use crate::{bsp, memory};

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

/// Sets up a minimal runtime for the kernel
///
/// # Safety
///
/// - Only one hart must be active
#[no_mangle]
pub unsafe fn runtime_init() -> ! {
    clear_bss();

    crate::kernel_init()
}

/// Clears the .bss section, by filling it with 0s
///
/// # Safety
///
/// - Must be called before any use of the bss section
#[inline(always)]
unsafe fn clear_bss() {
    memory::zero_volatile(bsp::memory::bss_range());
}
