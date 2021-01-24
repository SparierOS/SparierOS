// SPDX-License-Identifier: MIT

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
    }
}
