
Things I did to prepare file for translation to rust.

Removed all asserts manually.

Changed some #if directives to #ifdefs manually to get coan to process them correctly.

The OS_VXWORKS macro was causing issues with coan so I removed it manually in the spots it was causing issues.

Manually removed OSTRACE
manually removed max_worker_threads and default_worker_threads
manually removed SQLITE_DEPRECATED
manually removed SQLITE_EXPERIMENTAL
manually removed # define setDefaultSyncFlag and usages
manually removed # define vdbeSorterWorkDebug and usages
manually removed # define vdbeSorterRewindDebug and usages
manually removed # define vdbeSorterPopulateDebug and usages
manually removed # define vdbeSorterBlockDebug and usages
manually removed # define vdbeSorterJoinAll(x,rcin) (rcin) and usages
manually removed  # define vdbeSorterJoinThread(pTask) SQLITE_OK and usages
manually removed # defind sqlite3_mutex_free and usages
manually removed # define sqlite3_mutex_enter and usages
manually removed #define sqlite3_mutex_leave(X) and usages
manually removed #define sqlite3MutexEnd() and usages
manually removed #define MUTEX_LOGIC( ) and usages
manually removed #define sqlite3_mutex_alloc(X)  and usages replaced with  ((sqlite3_mutex*)8)
manually removed #define sqlite3_mutex_try(X) and usages and replaced with      SQLITE_OK
manually removed sqlite3_mutex_held(X) and usages
manually removed sqlite3_mutex_notheld(X) and usages
manually removed #define sqlite3MutexAlloc(X) and usages and replaced with     ((sqlite3_mutex*)8)
manually removed #define sqlite3MutexInit(X) and usages and replaced with SQLITE_OK
manually removed #define TESTONLY(X) and usages
manually removed memdbCheckReservedLock since it is never called
manually removed memdbSectorSize since it is never called
manually removed memdbDelete since it is unused
removed memdbCurrentTime(sqlite3_vfs *pVfs, double *pTimeOut) since it is unused
removed #define PAGERTRACE and usages
removed # define PAGER_INCR(v) and usages
remove #defind TRACE(X) and usages
remove unused ptrmapCheckPages 
remove unused sqlite3VdbeSerialType 
removed unused # define SQLITE_BIGENDIAN    0
removed unused # define SQLITE_LITTLEENDIAN 1



