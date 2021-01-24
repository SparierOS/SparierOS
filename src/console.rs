// SPDX-License-Identifier: MIT

//! System console

use crate::bsp;
use core::fmt;

/// Console interfaces
pub mod interface {
    /// This trait is required for a device or other struct to be used as the OS console
    pub trait All {
        fn write_fmt(&self, args: core::fmt::Arguments<'_>);
    }
}

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    use crate::console::interface::All;

    bsp::console::console().write_fmt(args);
}

/// Copy of the print macro from std
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

/// Copy of the println macro from std
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::console::_print(format_args_nl!($($arg)*));
    })
}
