// SPDX-License-Identifier: MIT

//! Driver for the Uart 16550

use crate::{
    console::interface::FullConsole,
    driver::interface::DeviceDriver,
    synchronization::{interface::Mutex, NoLock},
};
use core::{
    convert::TryInto,
    fmt::{Error, Write},
};

//------------------------------------------------------------------------------
//- Structs
//------------------------------------------------------------------------------

/// Thread-Safe Wrapper for UART
pub struct UartDevice {
    uart: NoLock<Uart>,
}

/// Abstraction of an UART controller
struct Uart {
    initialized: bool,
    base_address: *mut u8,
}

//------------------------------------------------------------------------------
//- Struct Implementations
//------------------------------------------------------------------------------

impl UartDevice {
    /// Create new UART driver
    ///
    /// # Safety
    /// - there must be a valid UART residing at the given base_adress
    pub const unsafe fn new(base_address: *mut u8) -> Self {
        UartDevice {
            uart: NoLock::new(Uart::new(base_address)),
        }
    }
}

// make UartDevice usable as a console
impl FullConsole for UartDevice {
    fn write_fmt(&self, args: core::fmt::Arguments<'_>) {
        self.uart.lock(|device| device.write_fmt(args)).unwrap();
    }
}

// make UartDevice usable as a device driver
impl DeviceDriver for UartDevice {
    fn name(&self) -> &'static str {
        "uart16550"
    }

    fn initialized(&self) -> bool {
        self.uart.lock(|device| device.initialized)
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        self.uart.lock(|device| device.init());

        Ok(())
    }
}

impl Uart {
    /// Create new UART driver
    ///
    /// # Safety
    /// - there must be a valid UART residing at the given base_adress
    const unsafe fn new(base_address: *mut u8) -> Self {
        Uart {
            initialized: false,
            base_address,
        }
    }

    /// Initialize the driver
    fn init(&mut self) {
        // Safety: Correctness of the base_address will be ensured at struct creation
        unsafe {
            // Set the word length to 8-bits by writing 1 into LCR[1:0]
            self.base_address.add(3).write_volatile((1 << 0) | (1 << 1));
            // Enable FIFO
            self.base_address.add(2).write_volatile(1 << 0);

            // TODO: calculate divisor (BAUD)
            let divisor = 600u16;
            let divisor_l: u8 = (divisor & 0xff).try_into().unwrap();
            let divisor_h: u8 = (divisor >> 8).try_into().unwrap();

            // Enable divisor latch
            let lcr = self.base_address.add(3).read_volatile();
            self.base_address.add(3).write_volatile(lcr | 1 << 7);
            // Write divisor
            self.base_address.add(0).write_volatile(divisor_l);
            self.base_address.add(1).write_volatile(divisor_h);
            // Close divisor latch
            self.base_address.add(3).write_volatile(lcr);
        }

        self.initialized = true;
    }

    /// Safety: Undefined behaviour on uninitialized UART
    unsafe fn put(&mut self, data: u8) {
        // directly write into MMIO
        self.base_address.add(0).write_volatile(data);
    }
}

// make UART writable
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        // We can't write to uninitialized drivers
        if !self.initialized {
            panic!("Trying to write to uninitialized driver");
        }

        for c in s.as_bytes() {
            // Safety: Uninitialized drivers are already handled
            unsafe {
                self.put(*c);
            }
        }

        Ok(())
    }
}
