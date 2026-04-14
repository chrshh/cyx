#include <core/ansi.h>
#include <stdio.h>

void warn(char *msg) { printf(YELLOW "WARN: %s" RESET "\n", msg); }
