use crate::FormatP;
use core::ffi::{c_char, c_int, c_uint, c_void};

// Beacon Fn types
pub type BeaconOutputFn = unsafe extern "C" fn(c_int, *const c_char, c_int);
pub type BeaconFormatAllocFn = unsafe extern "C" fn(*mut FormatP, c_int);
pub type BeaconFormatFreeFn = unsafe extern "C" fn(*mut FormatP);
pub type BeaconPrintfFn = unsafe extern "C" fn(c_int, *const c_char, ...);

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
    format_alloc: BeaconFormatAllocFn,
    format_free: BeaconFormatFreeFn,
    pub printf: BeaconPrintfFn,
    buffer: FormatP,
    #[cfg(feature = "process_injection")]
    pub injection: BeaconInjectionFunctions,
    #[cfg(feature = "data")]
    pub data: BeaconDataFunctions,
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
        format_alloc: BeaconFormatAllocFn,
        format_free: BeaconFormatFreeFn,
        printf: BeaconPrintfFn,
        #[cfg(feature = "process_injection")] get_spawn_to: BeaconGetSpawnToFn,
        #[cfg(feature = "process_injection")] inject_process: BeaconInjectProcessFn,
        #[cfg(feature = "process_injection")]
        inject_temporary_process: BeaconInjectTemporaryProcessFn,
        #[cfg(feature = "process_injection")] cleanup_process: BeaconCleanupProcessFn,
        #[cfg(feature = "data")] data: BeaconDataFunctions,
        // Arguments from Beacon
        args: *mut c_char,
        alen: c_int,
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
            #[cfg(feature = "process_injection")]
            injection: BeaconInjectionFunctions {
                get_spawn_to,
                inject_process,
                inject_temporary_process,
                cleanup_process,
            },
            #[cfg(feature = "data")]
            data,
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
    #[cfg(feature = "data")]
    pub fn parse_data(&mut self, mut parser: DataP) {
        unsafe {
            let mut size: c_int = 0;
            let str_arg = (self.data.extract)(&mut parser, &mut size);
            if str_arg.is_null() {
                self.printf("[!] Running with no arguments\n\0");
            } else {
                (self.printf)(
                    0,
                    "[+] Running with args: %s\n\0".as_ptr() as *const c_char,
                    str_arg,
                );
            }
        };
    }
    #[cfg(feature = "data")]
    pub fn parse_args(&self, args: *mut c_char, args_len: c_int) -> DataP {
        let mut parser = DataP {
            original: core::ptr::null_mut(),
            buffer: core::ptr::null_mut(),
            length: 0,
            size: 0,
        };

        unsafe {
            (self.data.parse)(&mut parser, args, args_len);
        }

        parser
    }
    #[cfg(feature = "data")]
    pub fn get_int(&self, parser: &mut DataP) -> c_int {
        unsafe { (self.data.get_int)(parser) }
    }
    #[cfg(feature = "data")]
    pub fn get_short(&self, parser: &mut DataP) -> i16 {
        unsafe { (self.data.get_short)(parser) }
    }
    #[cfg(feature = "data")]
    pub fn get_string(&self, parser: &mut DataP) -> &'static str {
        unsafe {
            let mut size: c_int = 0;
            let ptr = (self.data.extract)(parser, &mut size);

            if ptr.is_null() || size == 0 {
                return "";
            }

            let slice = core::slice::from_raw_parts(ptr as *const u8, size as usize);
            core::str::from_utf8_unchecked(slice)
        }
    }
    #[cfg(feature = "data")]
    pub fn get_binary(&self, parser: &mut DataP) -> &'static [u8] {
        unsafe {
            let mut size: c_int = 0;
            let ptr = (self.data.extract)(parser, &mut size);

            if ptr.is_null() || size == 0 {
                return &[];
            }

            core::slice::from_raw_parts(ptr as *const u8, size as usize)
        }
    }
}

impl Drop for Beacon {
    fn drop(&mut self) {
        self.free();
    }
}
