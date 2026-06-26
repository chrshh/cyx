#include "asemics.h"
#include <stdarg.h>

void initDbg(void) {
    dbg = fopen("/tmp/asemics.log", "w");
    if (!dbg) die("fopen");
    setvbuf(dbg, NULL, _IOLBF, 0);
    fprintf(dbg, "%s", SCREEN_CLEAR);
    fprintf(dbg, "-- ENTRY --\n");
}

void addDbgLog(const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    fprintf(dbg, " (%d, %d) MODE=%d\n", E.cursor.rx, E.cursor.y, E.mode);
    vfprintf(dbg, fmt, ap);
    va_end(ap);
}
