#include <windows.h>
#include "beacon.h"

// Extern Rust initialize fn
extern void initialize(
    #ifdef OUT
    void (*beacon_output)(int, const char *, int),
    void (*beacon_printf)(int, const char * fmt, ...),
    #endif
    
    void (*beacon_format_alloc)(formatp*, int),
    void (*beacon_format_free)(formatp*),

    #ifdef PROCESS_INJECTION
    void (*beacon_get_spawn_to)(BOOL, char*, int),
    void (*beacon_inject_process)(HANDLE, int, char*, int, int, char*, int),
    void (*beacon_inject_temporary_process)(PROCESS_INFORMATION*, char*, int, int, char*, int),
    void (*beacon_cleanup_process)(PROCESS_INFORMATION*),
    #endif


    void (*beacon_data_parse)(datap*, char*, int),
    int (*beacon_data_int)(datap*),
    short (*beacon_data_short)(datap*),
    int (*beacon_data_length)(datap*),
    char* (*beacon_data_extract)(datap*, int*),
    
    char* args, 
    int alen
);

void go(char* args, int alen) {
    // Pass the fn pointers to the rust wrapper
    initialize(
        BeaconOutput, 
        BeaconPrintf,
        
        BeaconFormatAlloc, 
        BeaconFormatFree, 

        #ifdef PROCESS_INJECTION
        BeaconGetSpawnTo,
        BeaconInjectProcess,
        BeaconInjectTemporaryProcess,
        BeaconCleanupProcess,
        #endif

        BeaconDataParse,
        BeaconDataInt,
        BeaconDataShort,
        BeaconDataLength,
        BeaconDataExtract,
        args,
        alen
    );
}
