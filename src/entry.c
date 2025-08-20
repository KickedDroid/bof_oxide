#include <windows.h>
#include "beacon.h"

// Extern Rust initialize fn
extern void initialize(
    #ifdef OUT
    void (*beacon_output)(int, const char *, int),
    void (*beacon_printf)(int, const char * fmt, ...),
    #endif


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
