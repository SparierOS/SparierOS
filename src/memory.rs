// SPDX-License-Identifier: MIT

//! Kernels Memory Management

use core::ops::RangeInclusive;

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

/// Fills a memory range with zeros
///
/// # Safety
///
/// - `range.start` and `range.end` must be valid.
/// - `range.start` and `range.end` must be `T` aligned.
pub unsafe fn zero_volatile<T>(range: RangeInclusive<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = *range.start();
    let end_inclusive = *range.end();

    while ptr <= end_inclusive {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}
