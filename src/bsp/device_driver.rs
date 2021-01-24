// SPDX-License-Identifier: MIT

//! Reexports drivers

#[cfg(any(feature = "bsp_virt"))]
mod uart16550;

#[cfg(any(feature = "bsp_virt"))]
pub use uart16550::*;
