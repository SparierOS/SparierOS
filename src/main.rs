// SPDX-License-Identifier: MIT

//! The main kernel binary

#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

// the entry point is in `mod cpu`

mod bsp;
mod cpu;
mod panic;
