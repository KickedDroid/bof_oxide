#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_void;
use core::i128;

mod rust_bof;
use rust_bof::rust_bof;
mod beacon;
use beacon::{Beacon, BeaconFormatAllocFn, BeaconFormatFreeFn, BeaconOutputFn, BeaconPrintfFn};
#[repr(C, align(8))]
struct FormatP {
    original: *mut c_char,
    buffer: *mut c_char,
    length: c_int,
    size: c_int,
}

// This is the Entrypoint for the Rust portion
// Initialize and call rust_bof
#[no_mangle]
pub extern "C" fn initialize(
    beacon_output: BeaconOutputFn,
    beacon_format_alloc: BeaconFormatAllocFn,
    beacon_format_free: BeaconFormatFreeFn,
    beacon_printf: BeaconPrintfFn,
) {
    // Pass the fn pointers to the Beacon wrapper
    let mut beacon = Beacon::new(
        beacon_output,
        beacon_format_alloc,
        beacon_format_free,
        beacon_printf,
    );

    // Call rust_bof
    rust_bof(&mut beacon);
}

#[panic_handler]
#[no_mangle]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
