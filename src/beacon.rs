use crate::FormatP;
use std::ffi::{c_char, c_int, c_uint, c_void};
// Beacon Fn types
pub type BeaconOutputFn = unsafe extern "C" fn(c_int, *const c_char, c_int);
pub type BeaconPrintfFn = unsafe extern "C" fn(c_int, *const c_char, ...);

#[cfg(feature = "format")]
pub type BeaconFormatAllocFn = unsafe extern "C" fn(*mut FormatP, c_int);
#[cfg(feature = "format")]
pub type BeaconFormatFreeFn = unsafe extern "C" fn(*mut FormatP);
// Windows-specific types
#[cfg(feature = "process_injection")]
type HANDLE = *mut c_void;
#[cfg(feature = "process_injection")]
type BOOL = c_int;
#[cfg(feature = "process_injection")]
type DWORD = c_uint;

#[cfg(feature = "process_injection")]
#[repr(C)]
pub struct PROCESS_INFORMATION {
    h_process: HANDLE,
    h_thread: HANDLE,
    process_id: DWORD,
    thread_id: DWORD,
}

#[cfg(feature = "process_injection")]
pub struct BeaconInjectionFunctions {
    pub get_spawn_to: BeaconGetSpawnToFn,
    pub inject_process: BeaconInjectProcessFn,
    pub inject_temporary_process: BeaconInjectTemporaryProcessFn,
    pub cleanup_process: BeaconCleanupProcessFn,
}

// Function types for process injection
#[cfg(feature = "process_injection")]
pub type BeaconGetSpawnToFn = unsafe extern "C" fn(BOOL, *mut c_char, c_int);
#[cfg(feature = "process_injection")]
pub type BeaconInjectProcessFn =
    unsafe extern "C" fn(HANDLE, c_int, *mut c_char, c_int, c_int, *mut c_char, c_int);
#[cfg(feature = "process_injection")]
pub type BeaconInjectTemporaryProcessFn =
    unsafe extern "C" fn(*mut PROCESS_INFORMATION, *mut c_char, c_int, c_int, *mut c_char, c_int);
#[cfg(feature = "process_injection")]
pub type BeaconCleanupProcessFn = unsafe extern "C" fn(*mut PROCESS_INFORMATION);

// Beacon data parsing function types
#[cfg(feature = "data")]
pub type BeaconDataParseFn = unsafe extern "C" fn(*mut DataP, *mut c_char, c_int);
#[cfg(feature = "data")]
pub type BeaconDataIntFn = unsafe extern "C" fn(*mut DataP) -> c_int;
#[cfg(feature = "data")]
pub type BeaconDataShortFn = unsafe extern "C" fn(*mut DataP) -> i16;
#[cfg(feature = "data")]
pub type BeaconDataLengthFn = unsafe extern "C" fn(*mut DataP) -> c_int;
#[cfg(feature = "data")]
pub type BeaconDataExtractFn = unsafe extern "C" fn(*mut DataP, *mut c_int) -> *mut c_char;

#[cfg(feature = "data")]
// Add these to your Beacon struct
pub struct BeaconDataFunctions {
    pub parse: BeaconDataParseFn,
    pub get_int: BeaconDataIntFn,
    pub get_short: BeaconDataShortFn,
    pub get_length: BeaconDataLengthFn,
    pub extract: BeaconDataExtractFn,
}

pub struct Beacon {
    output: BeaconOutputFn,
    pub printf: BeaconPrintfFn,
    #[cfg(feature = "process_injection")]
    pub injection: BeaconInjectionFunctions,
}

#[repr(C)]
pub enum BeaconOutputType {
    Standard = 0x0,
    Oem = 0x1e,
    Utf8 = 0x20,
    Error = 0x0d,
}

#[repr(C)]
pub struct DataP {
    pub original: *mut c_char,
    pub buffer: *mut c_char,
    pub length: c_int,
    pub size: c_int,
}

impl Beacon {
    pub fn new(
        output: BeaconOutputFn,
        printf: BeaconPrintfFn,
        #[cfg(feature = "process_injection")] get_spawn_to: BeaconGetSpawnToFn,
        #[cfg(feature = "process_injection")] inject_process: BeaconInjectProcessFn,
        #[cfg(feature = "process_injection")]
        inject_temporary_process: BeaconInjectTemporaryProcessFn,
        #[cfg(feature = "process_injection")] cleanup_process: BeaconCleanupProcessFn,
        // Arguments from Beacon
        args: *mut c_char,
        alen: c_int,
    ) -> Self {
        let mut beacon = Self {
            output,
            printf,
            #[cfg(feature = "process_injection")]
            injection: BeaconInjectionFunctions {
                get_spawn_to,
                inject_process,
                inject_temporary_process,
                cleanup_process,
            },
        };
        beacon
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
    pub fn printf(&mut self, mut msg: &str, arg: *const c_char) {
        unsafe {
            (self.printf)(0, msg.as_ptr() as *const c_char, arg);
        }
    }


    pub fn print(&mut self, mut msg: &str) {
        unsafe {
            (self.printf)(0, msg.as_ptr() as *const c_char);
        }
    }
}

