// SPDX-License-Identifier: MIT

//! Driver Manager for Virt Device

use crate::driver::interface::{DeviceDriver, DriverManager};

//------------------------------------------------------------------------------
//- Symbols
//------------------------------------------------------------------------------

/// Singleton for Driver Management
static VIRT_DRIVER_MANAGER: VirtDriverManager = VirtDriverManager {
    // Remember to update driver list when adding new drivers
    device_drivers: [&super::UART],
};

//------------------------------------------------------------------------------
//- Structs
//------------------------------------------------------------------------------

/// Wraps driver list & driver management functions
struct VirtDriverManager {
    device_drivers: [&'static (dyn DeviceDriver + Sync); 1],
}

//------------------------------------------------------------------------------
//- Struct Implementations
//------------------------------------------------------------------------------

impl DriverManager for VirtDriverManager {
    fn list_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)] {
        &self.device_drivers[..]
    }
}

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

/// Get driver manager for this board
pub fn driver_manager() -> &'static impl DriverManager {
    &VIRT_DRIVER_MANAGER
}
