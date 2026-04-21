#include <core/ansi.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdnoreturn.h>

void panic(char *msg) {
  printf(RED "PANIC: %s" RESET "\n", msg);
  exit(1);
}

static void (*panic_hook)(void) = NULL;

void panic_set_hook(void (*hook)(void)) { panic_hook = hook; }

noreturn void panic_impl(const char *file, int line, const char *func,
                         const char *fmt, ...) {
  // Print to stderr, panic output should never go to stdout
  // since stdout may be a pipe your shell is writing into
  fprintf(stderr,
          COLOR_ERROR BOLD "PANIC" RESET " at " CYAN "%s" RESET ":" YELLOW
                           "%d" RESET " in " CYAN "%s()" RESET "\n",
          file, line, func);

  fprintf(stderr, "  " COLOR_ERROR "→ " RESET);
  va_list args;
  va_start(args, fmt);
  vfprintf(stderr, fmt, args);
  va_end(args);
  fprintf(stderr, "\n");

  if (panic_hook)
    panic_hook();

  // abort() instead of exit(1):
  abort();
}
