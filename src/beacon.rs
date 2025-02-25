use crate::FormatP;
use core::ffi::{c_char, c_int};

// Beacon Fn types
pub type BeaconOutputFn = unsafe extern "C" fn(c_int, *const c_char, c_int);
pub type BeaconFormatAllocFn = unsafe extern "C" fn(*mut FormatP, c_int);
pub type BeaconFormatFreeFn = unsafe extern "C" fn(*mut FormatP);
pub type BeaconPrintfFn = unsafe extern "C" fn(c_int, *const c_char, ...);

type HANDLE = *mut c_void;
type BOOL = c_int;
type DWORD = c_uint;
type LPVOID = *mut c_void;
type LPCVOID = *const c_void;
type SIZE_T = usize;
type LPHANDLE = *mut HANDLE;
type PDWORD = *mut DWORD;

// Memory-related functions
pub struct BeaconMemoryFunctions {
    pub virtual_alloc: unsafe extern "C" fn(LPVOID, SIZE_T, DWORD, DWORD) -> LPVOID,
    pub virtual_alloc_ex: unsafe extern "C" fn(HANDLE, LPVOID, SIZE_T, DWORD, DWORD) -> LPVOID,
    pub virtual_protect: unsafe extern "C" fn(LPVOID, SIZE_T, DWORD, PDWORD) -> BOOL,
    pub virtual_protect_ex: unsafe extern "C" fn(HANDLE, LPVOID, SIZE_T, DWORD, PDWORD) -> BOOL,
    pub virtual_free: unsafe extern "C" fn(LPVOID, SIZE_T, DWORD) -> BOOL,
    pub virtual_query:
        unsafe extern "C" fn(LPCVOID, *mut MEMORY_BASIC_INFORMATION, SIZE_T) -> SIZE_T,
    pub unmap_view_of_file: unsafe extern "C" fn(LPCVOID) -> BOOL,
}

// Process and thread functions
pub struct BeaconProcessFunctions {
    pub open_process: unsafe extern "C" fn(DWORD, BOOL, DWORD) -> HANDLE,
    pub open_thread: unsafe extern "C" fn(DWORD, BOOL, DWORD) -> HANDLE,
    pub close_handle: unsafe extern "C" fn(HANDLE) -> BOOL,
    pub get_thread_context: unsafe extern "C" fn(HANDLE, *mut CONTEXT) -> BOOL,
    pub set_thread_context: unsafe extern "C" fn(HANDLE, *const CONTEXT) -> BOOL,
    pub resume_thread: unsafe extern "C" fn(HANDLE) -> DWORD,
    pub duplicate_handle:
        unsafe extern "C" fn(HANDLE, HANDLE, HANDLE, LPHANDLE, DWORD, BOOL, DWORD) -> BOOL,
    pub read_process_memory:
        unsafe extern "C" fn(HANDLE, LPCVOID, LPVOID, SIZE_T, *mut SIZE_T) -> BOOL,
    pub write_process_memory:
        unsafe extern "C" fn(HANDLE, LPVOID, LPCVOID, SIZE_T, *mut SIZE_T) -> BOOL,
}

pub struct Beacon {
    output: BeaconOutputFn,
    format_alloc: BeaconFormatAllocFn,
    format_free: BeaconFormatFreeFn,
    printf: BeaconPrintfFn,
    buffer: FormatP,
    pub memory: BeaconMemoryFunctions,
    pub process: BeaconProcessFunctions,
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
