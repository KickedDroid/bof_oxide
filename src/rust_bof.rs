use crate::{Beacon, FormatP};
use beacon::BeaconOutputType;
// This will be the main file we edit to write out BOFs.
pub fn rust_bof(beacon: &mut Beacon) {
    beacon.printf("   This is where you can write your own custom functionality\n\n\0");

    beacon.output(
        BeaconOutputType::Standard,
        "[+] Rust BOF Completed successfully\0",
    );
}
