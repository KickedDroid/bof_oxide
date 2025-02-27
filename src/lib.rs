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
use beacon::*;
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
        beacon_format_alloc,
        beacon_format_free,
        beacon_printf,
        #[cfg(feature = "process_injection")]
        get_spawn_to,
        #[cfg(feature = "process_injection")]
        inject_process,
        #[cfg(feature = "process_injection")]
        inject_temporary_process,
        #[cfg(feature = "process_injection")]
        cleanup_process,
        #[cfg(feature = "data")]
        data,
        args,
        alen,
    );

    #[cfg(feature = "data")]
    let data = beacon.parse_args(args, alen);
    #[cfg(feature = "data")]
    beacon.parse_data(data);
    // Call rust_bof
    rust_bof(&mut beacon);

    drop(beacon);
}

#[panic_handler]
#[no_mangle]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
