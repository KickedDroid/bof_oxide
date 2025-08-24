use crate::Beacon;

// This will be the main file we edit to write out BOFs.
pub fn rust_bof(mut beacon: Beacon) {
    let str_arg = beacon.get_arg();
    if str_arg.is_null() {
        beacon.output("NO ARGS");
    }
}
