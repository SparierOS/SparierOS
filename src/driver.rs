// SPDX-License-Identifier: MIT

use crate::bsp;

/// Driver Interfaces
pub mod interface {
    /// Driver functions.
    pub trait DeviceDriver {
        /// Return a string for identifying the driver
        fn name(&self) -> &'static str;

        /// Return if the driver was sucessfully initalized
        fn initialized(&self) -> bool;

        /// Initialize device
        ///
        /// # Safety
        /// - Can not be guaranteed, depends on device driver
        unsafe fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    /// Driver management functions.
    ///
    /// Supplied by the `BSP`
    pub trait DriverManager {
        /// Return a slice of  all available drivers.
        fn list_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)];

        /// Initializes all drivers of a BSP
        ///
        /// # Safety
        /// - Can not be guaranteed, depends on device drivers
        unsafe fn initalize_all_drivers(&self) {
            for device in self.list_drivers().iter() {
                if let Err(msg) = device.init() {
                    panic!("Could not initialize driver: {}: {}", device.name(), msg);
                }
            }
        }
    }
}

//------------------------------------------------------------------------------
//- Functions
//------------------------------------------------------------------------------

/// Initializes all available drivers
///
/// # Safety
/// - Can not be guaranteed, depends on device driver
pub unsafe fn init() {
    use crate::driver::interface::DriverManager;

    bsp::driver::driver_manager().initalize_all_drivers();
}
