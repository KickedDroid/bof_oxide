#include <windows.h>
#include "beacon.h"

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
