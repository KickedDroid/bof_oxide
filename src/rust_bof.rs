use crate::{Beacon, FormatP};
use beacon::BeaconOutputType;
use core::ffi::c_char;
use core::arch::asm;
// This will be the main file we edit to write out BOFs.
pub fn rust_bof(beacon: &mut Beacon) {
    
    // BeaconOutput is a buffer we can add to
    /* beacon.output(
        BeaconOutputType::Error,
        "[!] Uh oh error 123\n",
    );
    beacon.output(
        BeaconOutputType::Utf8,
        "[+] Testing 123\n",
    ); */
    beacon.output(
        BeaconOutputType::Standard,
        "[+] Rust BOF Completed successfully",
    );
}

fn peb() -> u64 {
    let rax:u64;
    unsafe {
        asm!(
            "push rbx",
            "xor rbx, rbx",
            "xor rax, rax",
            "mov rbx, qword ptr gs:[0x60]",
            "mov rax,rbx",
            "pop rbx",
            out("rax") rax,
        );
    }
    rax
}