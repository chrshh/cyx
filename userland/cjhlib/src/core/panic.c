#include <core/ansi.h>
#include <stdio.h>
#include <stdlib.h>

void panic(char *msg) {
  printf(RED "PANIC: %s" RESET "\n", msg);
  exit(1);
}
