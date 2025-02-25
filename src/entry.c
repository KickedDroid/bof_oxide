#include <windows.h>
#include "beacon.h"

// Extern Rust initialize fn
extern void initialize(
    void (*beacon_output)(int, const char*, int),
    void (*beacon_format_alloc)(formatp*, int),
    void (*beacon_format_free)(formatp*),
    void (*beacon_printf)(int, const char * fmt, ...),
    void (*beacon_get_spawn_to)(BOOL, char*, int),
    void (*beacon_inject_process)(HANDLE, int, char*, int, int, char*, int),
    void (*beacon_inject_temporary_process)(PROCESS_INFORMATION*, char*, int, int, char*, int),
    void (*beacon_cleanup_process)(PROCESS_INFORMATION*)   
);

void go(char* args, int alen) {
    // Pass the fn pointers to the rust wrapper
   initialize(
        BeaconOutput, 
        BeaconFormatAlloc, 
        BeaconFormatFree, 
        BeaconPrintf,
        
        BeaconGetSpawnTo,
        BeaconInjectProcess,
        BeaconInjectTemporaryProcess,
        BeaconCleanupProcess
    );
}
