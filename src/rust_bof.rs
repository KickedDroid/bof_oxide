use crate::data::Data;
use crate::{beacon, Beacon};
use beacon::BeaconOutputType;
use core::arch::asm;




// This will be the main file we edit to write out BOFs.
pub fn rust_bof(beacon: &mut Beacon, data: &mut Data) {
    
    let str_arg = data.extract_str();
    if str_arg.is_null() {
        beacon.output(
            BeaconOutputType::Error,
            "[!] Str_arg argument is required\n",
        );
        return;
    }
    
    
    beacon.printf("Hello %s from rust-bof\0", str_arg);
    
    data.free();
    beacon.output(
        BeaconOutputType::Standard,
        "[+] Rust BOF Completed successfully",
    );
}
