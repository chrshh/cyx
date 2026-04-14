#include <core/panic.h>
#include <core/warn.h>
#include <stdlib.h>

void *cmalloc(size_t size) {
  void *p = malloc(size);
  if (!p) {
    panic("out of memory");
  }
  return p;
}

void cfree(void *ptr) {
  if (!ptr) {
    warn("free called on a NULL pointer, skipping free");
    return;
  }
  free(ptr);
}
