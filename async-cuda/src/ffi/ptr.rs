/// Represents a device-local pointer. Pointers qualify as device-local if they refer to memory that
/// lives on the device, and not on the host.
///
/// # Safety
///
/// ## Null
///
/// Creating a null pointer is always unsafe, because any CUDA operations on null pointers can cause
/// undefined behavior.
///
/// Use the `unsafe` function `Ptr::null` to create a null pointer in cases where usage is safe.
pub struct DevicePtr {
    addr: *mut std::ffi::c_void,
}

impl DevicePtr {
    /// Create from device address.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address of pointer.
    #[inline]
    pub fn from_addr(addr: *mut std::ffi::c_void) -> Self {
        if !addr.is_null() {
            DevicePtr { addr }
        } else {
            panic!("unexpected null pointer");
        }
    }

    /// Create null pointer.
    ///
    /// # Safety
    ///
    /// This is unsafe because operating on a `null` pointer in CUDA code can cause crashes. In some
    /// cases it is allowed though, for example, a `null` pointer can designate the default stream
    /// in stream-related operations.
    #[inline]
    pub unsafe fn null() -> Self {
        DevicePtr {
            addr: std::ptr::null_mut(),
        }
    }

    /// Whether or not the device pointer is a null pointer.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.addr.is_null()
    }

    /// Get the readonly pointer value.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const std::ffi::c_void {
        self.addr as *const std::ffi::c_void
    }

    /// Get the mutable pointer value.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
        self.addr
    }

    /// Take the pointer from this wrapper and replace it with a null pointer.
    ///
    /// # Safety
    ///
    /// This operation is unsafe because it creates a null pointer.
    ///
    /// # Usage
    ///
    /// This function can be used inside [`Drop`] if it known that the pointer object will not be
    /// used for the remainder of the function scope, and the object is to be dropped.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use async_cuda::ffi::DevicePtr;
    /// pub struct Object {
    ///     internal: DevicePtr,
    /// }
    ///
    /// impl Drop for Object {
    ///     fn drop(&mut self) {
    ///         // SAFETY: This is safe because `self` and `self.internal`
    ///         // are not used beyond this unsafe block.
    ///         let ptr = unsafe {
    ///             self.internal.take();
    ///         };
    ///         // Propertly deallocate the pointer here and do *NOT* use
    ///         // use `self` for anything!
    ///     }
    /// }
    /// ```
    #[inline]
    pub unsafe fn take(&mut self) -> DevicePtr {
        DevicePtr {
            // sets `self.addr` to NULL, puts addr in new device ptr
            addr: std::mem::replace(&mut self.addr, std::ptr::null_mut()),
        }
    }
}

impl std::fmt::Display for DevicePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_holds_on() {
        let fake = 0xffffffff as *mut std::ffi::c_void;
        let ptr = DevicePtr::from_addr(fake);
        assert_eq!(ptr.as_ptr(), 0xffffffff as *const std::ffi::c_void);
    }

    #[test]
    #[should_panic]
    fn test_it_panics_when_null() {
        let _ = DevicePtr::from_addr(std::ptr::null_mut());
    }

    #[test]
    fn test_null() {
        let ptr = unsafe { DevicePtr::null() };
        assert!(ptr.is_null());
        assert_eq!(ptr.as_ptr(), std::ptr::null_mut());
    }

    #[test]
    fn test_take() {
        let fake = 0xffffffff as *mut std::ffi::c_void;
        let mut ptr = DevicePtr::from_addr(fake);
        assert_eq!(
            unsafe { ptr.take().as_ptr() },
            0xffffffff as *const std::ffi::c_void,
        );
    }
}
