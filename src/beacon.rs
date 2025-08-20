use core::ffi::{c_char, c_int, c_short, c_void};
pub type BeaconOutputFn = extern "C" fn(i32, *const c_char, i32);
pub type BeaconPrintfFn = extern "C" fn(i32, *const c_char, *mut c_char);


pub struct Beacon {
    pub format: FormatP,
    pub data: DataP,
    pub output_addr: BeaconOutputFn,
    pub printf_addr: BeaconPrintfFn,
    pub args: *const c_char,
    pub alen: c_int,
}
impl Beacon {
    pub fn new(output: BeaconOutputFn, printf: BeaconPrintfFn, args: *const c_char, alen: c_int) -> Self {
        let mut beacon = Beacon {
            format: FormatP::new(),
            data: DataP::new(),
            output_addr: output,
            printf_addr: printf,
            args,
            alen,
        };

        return beacon;
    }

    pub fn output(&mut self, data: &str) {
        unsafe {(self.output_addr)(0, data.as_ptr() as *const c_char, data.len() as i32)};
    }
}


#[repr(C, align(8))]
pub struct FormatP {
    pub original: *mut c_char,
    pub buffer: *mut c_char,
    pub length: c_int,
    pub size: c_int,
}

impl FormatP {
    pub fn new() -> Self {
        FormatP {
            original: core::ptr::null_mut(),
            buffer: core::ptr::null_mut(),
            length: 0,
            size: 0,
        }
    }
}

#[repr(C)]
pub struct DataP {
    pub original: *mut c_char,
    pub buffer: *mut c_char,
    pub length: c_int,
    pub size: c_int,
}

impl DataP {
    pub fn new() -> Self {
        DataP {
            original: core::ptr::null_mut(),
            buffer: core::ptr::null_mut(),
            length: 0,
            size: 0,
        }
    }
}
