// SPDX-License-Identifier: MIT

//! Architectural processor code.

// include the assembly file
// NOTE: as of 23.01.2021 rust-analyzer wrongly markes this line as an error
global_asm!(include_str!("boot.S"));
