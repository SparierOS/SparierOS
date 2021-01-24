// SPDX-License-Identifier: MIT

//! Driver Manager for Virt Device

use crate::driver;

/// Singleton for Driver Management
static VIRT_DRIVER_MANAGER: VirtDriverManager = VirtDriverManager {
    // Remember to update driver list when adding new drivers
    device_drivers: [&super::UART],
};

/// Wraps driver list & driver management functions
struct VirtDriverManager {
    device_drivers: [&'static (dyn DeviceDriver + Sync); 1],
}

use driver::interface::DeviceDriver;

impl driver::interface::DriverManager for VirtDriverManager {
    fn list_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)] {
        &self.device_drivers[..]
    }
}

/// Get driver manager for this board
pub fn driver_manager() -> &'static impl driver::interface::DriverManager {
    &VIRT_DRIVER_MANAGER
}
