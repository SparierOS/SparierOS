// SPDX-License-Identifier: MIT

//! Synchronization capabilities for the kernel

use core::cell::UnsafeCell;

/// Synchronization interfaces
pub mod interface {
    /// Gurantees exclusive acces to the wrapped data
    pub trait Mutex {
        type Data;

        /// lock the data and temporarly access it
        fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    }
}

// TODO: replace with actual lock, this works only as long we have no paralellism
/// Pseudo Lock, pretending to give exclusive access
pub struct NoLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

impl<T> NoLock<T> {
    /// Create a wrapper for data
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

// implement Send & Sync traits so the lock can be shared
unsafe impl<T> Send for NoLock<T> where T: ?Sized {}
unsafe impl<T> Sync for NoLock<T> where T: ?Sized {}

// implements the Mutex trait but does not ensure exclusivity
impl<T> interface::Mutex for NoLock<T> {
    type Data = T;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        // TODO: ensure exclusitivity here and rename Lock afterwards
        // Safety: only safe with no paralellism
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}
