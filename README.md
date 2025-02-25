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

Structure of BOF

```
➜  rust_bof git:(main) ✗ objdump -t bof_oxide.o

bof_oxide.o:     file format pe-x86-64

SYMBOL TABLE:
[  0](sec  1)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .text
AUX scnlen 0x9b nreloc 3 nlnno 0 checksum 0x8f8752ca assoc 1 comdat 0
[  2](sec  5)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .xdata
AUX scnlen 0xc nreloc 0 nlnno 0 checksum 0x7f2842f8 assoc 4 comdat 0
[  4](sec  2)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .rdata
AUX scnlen 0x7d nreloc 0 nlnno 0 checksum 0x7fd708e1 assoc 5 comdat 0
[  6](sec  4)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .pdata
AUX scnlen 0xc nreloc 3 nlnno 0 checksum 0x30cfafda assoc 7 comdat 0
[  8](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 0) 0x0000000000000000 initialize
[  9](sec  1)(fl 0x00)(ty   20)(scl   2) (nx 1) 0x00000000000000a0 go
AUX tagndx 0 ttlsiz 0x0 lnnos 0 next 0
[ 11](sec  1)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x00000000000000a0 .text
AUX scnlen 0x40 nreloc 5 nlnno 0
[ 13](sec  5)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x000000000000000c .xdata
AUX scnlen 0xc nreloc 0 nlnno 0
[ 15](sec  4)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x000000000000000c .pdata
AUX scnlen 0xc nreloc 3 nlnno 0
[ 17](sec  3)(fl 0x00)(ty    0)(scl   3) (nx 1) 0x0000000000000000 .rdata$zzz
AUX scnlen 0x1d nreloc 0 nlnno 0
[ 19](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_BeaconOutput
[ 20](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_BeaconFormatFree
[ 21](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_BeaconPrintf
[ 22](sec  0)(fl 0x00)(ty    0)(scl   2) (nx 0) 0x0000000000000000 __imp_BeaconFormatAlloc
```
---
### References 

Header file `beacon.h` from https://github.com/Cobalt-Strike/bof_template/blob/main/beacon.h
