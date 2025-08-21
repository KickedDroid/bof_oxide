#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
mod beacon;
mod rust_bof;
use beacon::{Beacon, BeaconOutputFn, BeaconPrintfFn};
use core::ffi::{c_char, c_int};
use rust_bof::rust_bof;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize(
    beacon_output: BeaconOutputFn,
    beacon_printf: BeaconPrintfFn,

    // Arguments from Beacon
    args: *mut c_char,
    alen: c_int,
) {
    let mut beacon = Beacon::new(beacon_output, beacon_printf, args, alen);
    rust_bof(beacon);
}

#[panic_handler]
#[unsafe(no_mangle)]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
