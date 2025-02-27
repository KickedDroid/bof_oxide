#include <windows.h>
#include "beacon.h"

// Extern Rust initialize fn
extern void initialize(
    #ifdef OUT
    void (*beacon_output)(int, const char*, int),
    void (*beacon_printf)(int, const char * fmt, ...),
    #endif
    
    #ifdef FORMAT 
    void (*beacon_format_alloc)(formatp*, int),
    void (*beacon_format_free)(formatp*),
    #endif
    #ifdef PROCESS_INJECTION
    void (*beacon_get_spawn_to)(BOOL, char*, int),
    void (*beacon_inject_process)(HANDLE, int, char*, int, int, char*, int),
    void (*beacon_inject_temporary_process)(PROCESS_INFORMATION*, char*, int, int, char*, int),
    void (*beacon_cleanup_process)(PROCESS_INFORMATION*),
    #endif

    #ifdef DATA
    void (*beacon_data_parse)(datap*, char*, int),
    int (*beacon_data_int)(datap*),
    short (*beacon_data_short)(datap*),
    int (*beacon_data_length)(datap*),
    char* (*beacon_data_extract)(datap*, int*),
    #endif
    
    char* args, 
    int alen
);

void go(char* args, int alen) {
    // Pass the fn pointers to the rust wrapper
    initialize(
        #ifdef OUT
        BeaconOutput, 
        BeaconFormatAlloc, 
        BeaconFormatFree, 
        BeaconPrintf,
        #endif 

        #ifdef PROCESS_INJECTION
        BeaconGetSpawnTo,
        BeaconInjectProcess,
        BeaconInjectTemporaryProcess,
        BeaconCleanupProcess,
        #endif

        #ifdef DATA
        BeaconDataParse,
        BeaconDataInt,
        BeaconDataShort,
        BeaconDataLength,
        BeaconDataExtract,
        #endif
        args,
        alen
    );
}
