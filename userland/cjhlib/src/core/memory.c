#include "core/types.h"
#include <core/panic.h>
#include <core/warn.h>
#include <stdlib.h>

void *cmalloc(usize size) {
  if (size == 0) {
    warn("cmalloc called with size 0");
    return NULL;
  }
  void *p = malloc(size);
  if (!p) {
    panic("out of memory");
  }
  return p;
}

void *ccalloc(usize count, usize size) {
  if (count == 0 || size == 0) {
    warn("ccalloc called with zero count or size");
    return NULL;
  }
  void *p = calloc(count, size);
  if (!p)
    panic("out of memory");
  return p;
}

void *crealloc(void *ptr, usize new_size) {
  if (new_size == 0) {
    warn("crealloc called with size 0");
    return NULL;
  }
  void *p = realloc(ptr, new_size);
  if (!p)
    panic("out of memory. realloc failed");
  return p;
}

void cfree(void *ptr) {
  if (!ptr) {
    warn("free called on a NULL pointer, skipping free");
    return;
  }
  free(ptr);
}
