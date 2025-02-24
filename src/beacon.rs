use crate::FormatP;
use core::ffi::{c_char, c_int};

// Beacon Fn types
pub type BeaconOutputFn = unsafe extern "C" fn(c_int, *const c_char, c_int);
pub type BeaconFormatAllocFn = unsafe extern "C" fn(*mut FormatP, c_int);
pub type BeaconFormatFreeFn = unsafe extern "C" fn(*mut FormatP);
pub type BeaconPrintfFn = unsafe extern "C" fn(c_int, *const c_char, ...);

pub struct Beacon {
    pub(crate) output: BeaconOutputFn,
    pub(crate) format_alloc: BeaconFormatAllocFn,
    pub(crate) format_free: BeaconFormatFreeFn,
    pub(crate) printf: BeaconPrintfFn,
}

#[repr(C)]
pub enum BeaconOutputType {
    Standard = 0x0,
    Oem = 0x1e,
    Utf8 = 0x20,
    Error = 0x0d,
}

impl Beacon {
    pub fn alloc(&self, buffer: &mut FormatP, size: c_int) {
        unsafe {
            (self.format_alloc)(buffer, size);
        }
    }

    pub fn output(&self, out_type: BeaconOutputType, msg: &str) {
        unsafe {
            (self.output)(
                out_type as c_int,
                msg.as_ptr() as *const c_char,
                msg.len() as c_int,
            );
        }
    }

    pub fn free(&self, buffer: &mut FormatP) {
        unsafe {
            (self.format_free)(buffer);
        }
    }

    pub fn printf(&self, msg: &str) {
        unsafe { (self.printf)(0, msg.as_ptr() as *const c_char) }
    }
}
