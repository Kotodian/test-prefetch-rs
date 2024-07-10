use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    ptr::NonNull,
};

const ALIGNMENT: usize = 64;

pub struct Buffer {
    ptr: NonNull<u8>,
    layout: Layout,
    len: usize,
}

impl Buffer {
    pub fn new(len: usize) -> Self {
        if len == 0 {
            return Self {
                ptr: std::ptr::NonNull::dangling(),
                layout: Layout::from_size_align(64, 64).unwrap(),
                len,
            };
        }

        let padded_len = len.checked_next_multiple_of(ALIGNMENT).unwrap();
        let layout = Layout::from_size_align(padded_len, ALIGNMENT).unwrap();

        let ptr = unsafe { alloc_zeroed(layout) };
        let ptr = NonNull::new(ptr).unwrap();

        Self { ptr, layout, len }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    /// Get a mutable pointer to the underlying memory.
    ///
    /// The underlying memory is aligned to [crate::ALIGNMENT] bytes and padded to [crate::ALIGNMENT] bytes.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Get a slice to the underlying memory
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len) }
    }

    /// Get a mutable slice to the underlying memory
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }

    /// Length of the buffer. Keep in mind that the underlying memory is padded to [crate::ALIGNMENT] bytes
    /// so might be bigger than the returned value.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns if length of buffer is zero
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Create a Buffer from given slice.
    pub fn from_slice(src: &[u8]) -> Self {
        // Has to be mut because we write to it with buf.ptr
        #[allow(unused_mut)]
        let mut buf = Self::new(src.len());

        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), buf.as_mut_ptr(), buf.len);
        }

        buf
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            // Don't dealloc if we have 0 len, because we didn't alloc at the start.
            if self.len > 0 {
                dealloc(self.as_mut_ptr(), self.layout);
            }
        }
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe {
            // Has to be mut because we write to it with other.ptr
            #[allow(unused_mut)]
            let mut other = Self::new(self.len);

            std::ptr::copy_nonoverlapping(self.as_ptr(), other.as_mut_ptr(), self.len);
            other
        }
    }
}
