use crate::FormatP;
use core::ffi::{c_char, c_int, c_uint, c_void};

// Beacon Fn types
pub type BeaconOutputFn = unsafe extern "C" fn(c_int, *const c_char, c_int);
pub type BeaconFormatAllocFn = unsafe extern "C" fn(*mut FormatP, c_int);
pub type BeaconFormatFreeFn = unsafe extern "C" fn(*mut FormatP);
pub type BeaconPrintfFn = unsafe extern "C" fn(c_int, *const c_char, ...);

// Windows-specific types
type HANDLE = *mut c_void;
type BOOL = c_int;
type DWORD = c_uint;

#[repr(C)]
pub struct PROCESS_INFORMATION {
    h_process: HANDLE,
    h_thread: HANDLE,
    process_id: DWORD,
    thread_id: DWORD,
}

pub struct BeaconInjectionFunctions {
    pub get_spawn_to: BeaconGetSpawnToFn,
    pub inject_process: BeaconInjectProcessFn,
    pub inject_temporary_process: BeaconInjectTemporaryProcessFn,
    pub cleanup_process: BeaconCleanupProcessFn,
}

// Function types for process injection
pub type BeaconGetSpawnToFn = unsafe extern "C" fn(BOOL, *mut c_char, c_int);
pub type BeaconInjectProcessFn =
    unsafe extern "C" fn(HANDLE, c_int, *mut c_char, c_int, c_int, *mut c_char, c_int);
pub type BeaconInjectTemporaryProcessFn =
    unsafe extern "C" fn(*mut PROCESS_INFORMATION, *mut c_char, c_int, c_int, *mut c_char, c_int);
pub type BeaconCleanupProcessFn = unsafe extern "C" fn(*mut PROCESS_INFORMATION);

pub struct Beacon {
    output: BeaconOutputFn,
    format_alloc: BeaconFormatAllocFn,
    format_free: BeaconFormatFreeFn,
    printf: BeaconPrintfFn,
    buffer: FormatP,
    pub injection: BeaconInjectionFunctions,
}

#[repr(C)]
pub enum BeaconOutputType {
    Standard = 0x0,
    Oem = 0x1e,
    Utf8 = 0x20,
    Error = 0x0d,
}

impl Beacon {
    pub fn new(
        output: BeaconOutputFn,
        format_alloc: BeaconFormatAllocFn,
        format_free: BeaconFormatFreeFn,
        printf: BeaconPrintfFn,
        get_spawn_to: BeaconGetSpawnToFn,
        inject_process: BeaconInjectProcessFn,
        inject_temporary_process: BeaconInjectTemporaryProcessFn,
        cleanup_process: BeaconCleanupProcessFn,
    ) -> Self {
        let mut beacon = Self {
            output,
            format_alloc,
            format_free,
            printf,
            buffer: FormatP {
                original: core::ptr::null_mut(),
                buffer: core::ptr::null_mut(),
                length: 0,
                size: 0,
            },
            injection: BeaconInjectionFunctions {
                get_spawn_to,
                inject_process,
                inject_temporary_process,
                cleanup_process,
            },
        };

        // Initialize the buffer
        unsafe {
            (beacon.format_alloc)(&mut beacon.buffer, 16 * 1024);
        }

        beacon
    }
    pub fn alloc(&mut self, size: c_int) {
        unsafe {
            (self.format_alloc)(&mut self.buffer, size);
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

    pub fn free(&mut self) {
        unsafe {
            (self.format_free)(&mut self.buffer);
        }
    }

    pub fn printf(&mut self, msg: &str) {
        unsafe {
            (self.printf)(0, msg.as_ptr() as *const c_char);
        }
    }
}

impl Drop for Beacon {
    fn drop(&mut self) {
        self.free();
    }
}
