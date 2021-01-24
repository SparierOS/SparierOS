// SPDX-License-Identifier: MIT

//! Conditional re-exporting of Board Support Packages

#[cfg(feature = "bsp_virt")]
mod virt;

#[cfg(feature = "bsp_virt")]
pub use virt::*;
