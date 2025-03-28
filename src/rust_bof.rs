use crate::{Beacon, FormatP};
use beacon::BeaconOutputType;
use core::ffi::c_char;
use core::arch::asm;
use Data;
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
    data.free();

    unsafe {
        (beacon.printf)(
            0,
            "Hello %s from rust-bof\n\n\0".as_ptr() as *const c_char,
            str_arg,
        );
    }
    
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