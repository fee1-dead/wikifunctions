use std::marker::PhantomData;
use std::mem::ManuallyDrop;

pub type Function = unsafe extern "C" fn(Bytes<'_>) -> OwnedBytes;

/// Type passed from the evaluator to the evaluated function as arguments.
#[repr(C)]
#[derive(Clone, Copy)] // copying is safe since this is a reference
pub struct Bytes<'a> {
    ptr: *const u8,
    len: usize,
    ph: PhantomData<&'a u8>,
}

impl<'a> Bytes<'a> {
    pub fn from_slice(x: &'a [u8]) -> Bytes<'a> {
        Bytes {
            ptr: x.as_ptr(),
            len: x.len(),
            ph: PhantomData,
        }
    }

    pub fn as_slice(&self) -> &'a [u8] {
        // SAFETY: `Bytes` can only be constructed from `&[u8]`s, and
        // the `PhantomData` makes sure that the pointer is valid.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

#[repr(C)]
pub struct OwnedBytes {
    ptr: *mut u8,
    len: usize,
}

impl OwnedBytes {
    pub fn from_vec(v: Vec<u8>) -> OwnedBytes {
        let mut bytes = ManuallyDrop::new(v.into_boxed_slice());
        let ptr = bytes.as_mut_ptr();
        let len = bytes.len();
        OwnedBytes { ptr, len }
    }

    pub fn into_vec(self) -> Vec<u8> {
        // SAFETY: `OwnedBytes` can only be constructed from `Vec<u8>`s.
        let v = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.len) };
        std::mem::forget(self);
        v        
    }
}

impl Drop for OwnedBytes {
    fn drop(&mut self) {
        // SAFETY: `OwnedBytes` can only be constructed from `Vec<u8>`s.
        drop(unsafe { Vec::from_raw_parts(self.ptr, self.len, self.len) })
    }
}
