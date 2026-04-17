#include <setjmp.h>

void fatal_error_signal(int sig);
extern sigjmp_buf prompt_jmp;
