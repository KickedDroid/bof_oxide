#![no_main]
#![no_std]
#![allow(non_upper_case_globals)]
mod rust_bof;
use core::ffi::{c_char, c_int};
use rust_bof::rust_bof;
mod beacon;
mod data;
use beacon::*;
use data::Data;
#[repr(C, align(8))]
pub struct FormatP {
    original: *mut c_char,
    buffer: *mut c_char,
    length: c_int,
    size: c_int,
}

// This is the Entrypoint for the Rust portion
// Initialize and call rust_bof
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize(
    beacon_output: BeaconOutputFn,
    beacon_printf: BeaconPrintfFn,

    #[cfg(feature = "format")] beacon_format_alloc: BeaconFormatAllocFn,
    #[cfg(feature = "format")] beacon_format_free: BeaconFormatFreeFn,
    #[cfg(feature = "process_injection")] get_spawn_to: BeaconGetSpawnToFn,
    #[cfg(feature = "process_injection")] inject_process: BeaconInjectProcessFn,
    #[cfg(feature = "process_injection")] inject_temporary_process: BeaconInjectTemporaryProcessFn,
    #[cfg(feature = "process_injection")] cleanup_process: BeaconCleanupProcessFn,
    #[cfg(feature = "data")] beacon_data_parse: BeaconDataParseFn,
    #[cfg(feature = "data")] beacon_data_int: BeaconDataIntFn,
    #[cfg(feature = "data")] beacon_data_short: BeaconDataShortFn,
    #[cfg(feature = "data")] beacon_data_length: BeaconDataLengthFn,
    #[cfg(feature = "data")] beacon_data_extract: BeaconDataExtractFn,

    // Arguments from Beacon
    args: *mut c_char,
    alen: c_int,
) {
    #[cfg(feature = "data")]
    let data = BeaconDataFunctions {
        parse: beacon_data_parse,
        get_int: beacon_data_int,
        get_short: beacon_data_short,
        get_length: beacon_data_length,
        extract: beacon_data_extract,
    };
    // Pass the fn pointers to the Beacon wrapper
    let mut beacon = Beacon::new(
        beacon_output,
        beacon_printf,
        #[cfg(feature = "process_injection")]
        get_spawn_to,
        #[cfg(feature = "process_injection")]
        inject_process,
        #[cfg(feature = "process_injection")]
        inject_temporary_process,
        #[cfg(feature = "process_injection")]
        cleanup_process,
        args,
        alen,
    );

    #[cfg(feature = "data")]
    let mut data = Data::new(
        beacon_format_alloc,
        beacon_format_free,
        beacon_data_parse,
        beacon_data_int,
        beacon_data_short,
        beacon_data_length,
        beacon_data_extract,
        args,
        alen,
    );
    

    // Call rust_bof
    rust_bof(&mut beacon,&mut data);

    drop(beacon);
}

#[panic_handler]
#[unsafe(no_mangle)]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
} 
