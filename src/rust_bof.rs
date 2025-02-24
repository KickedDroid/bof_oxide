use crate::{Beacon, FormatP};
use beacon::BeaconOutputType;
// This will be the main file we edit to write out BOFs.
pub fn rust_bof(beacon: &Beacon) {
    beacon.printf("[+] Running Rust BOF...\n\n\0");
    beacon.printf("   This is where you can write your own custom functionality\n\n\0");

    // This is just an example showing how you could handle output to the COFFLoader
    let example_res = 0;
    match example_res {
        0 => {
            beacon.output(
                BeaconOutputType::Standard,
                "[+] Rust BOF Completed successfully\0",
            );
        }
        1 => {
            beacon.output(BeaconOutputType::Error, "[x] Rust BOF FAILED\0");
        }
        _ => {
            beacon.output(
                BeaconOutputType::Error,
                "[x] Rust BOF FAILED for unknown reason wtf\0",
            );
        }
    }
}
