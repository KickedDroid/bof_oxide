# bof_oxide

A POC or Template whatever for developing BOFs for Sliver, Havoc, Cobalt Strike or most COFFLoaders.

### Build
```
./build.sh
```



# Usage Example

```rust
pub fn rust_bof(beacon: &mut Beacon) {
    beacon.printf("[+] Running Rust BOF...\n\n\0");
    beacon.printf("   This is where you can write your own custom functionality\n\n\0");

    beacon.output(
        BeaconOutputType::Standard,
        "[+] Rust BOF Completed successfully\0",
    );
}
```

![image](https://github.com/user-attachments/assets/5d62446c-a2a1-4fa0-9e37-37fd03fd7975)


### How it works
This is just a wrapper around the existing Beacon Fns provided. The difference is we pass the function pointers to a Rust wrapper. 

```
C -> Rust -> BeaconApi 
```
The bof entry point is still `go` and it's still handled in C.

```c
// Extern Rust initialize fn
extern void initialize(
    void (*beacon_output)(int, const char*, int),
    void (*beacon_format_alloc)(formatp*, int),
    void (*beacon_format_free)(formatp*),
    void (*beacon_printf)(int, const char * fmt, ...)
);

void go(char* args, int alen) {
    // Pass the fn pointers to the rust wrapper
    initialize(BeaconOutput, BeaconFormatAlloc, BeaconFormatFree, BeaconPrintf);
}
```

The rust intialize fn

```rust
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
```
