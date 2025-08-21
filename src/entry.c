#include <windows.h>
#include "beacon.h"

// Extern Rust initialize fn
extern void initialize(
    void (*beacon_output)(int, const char *, int),
    void (*beacon_printf)(int, const char * fmt, ...),

    char* args,
    int alen
);

void go(char* args, int alen) {
    // Pass the fn pointers to the rust wrapper
    initialize(
        BeaconOutput,
        BeaconPrintf,

        args,
        alen
    );
}
