#ifndef MEMORY_H
#define MEMORY_H

#include "core/types.h"
#include <stddef.h>

void *cmalloc(usize size);
void *crealloc(void *ptr, usize new_size);
void *ccalloc(usize count, size_t size);
void cfree(void *ptr);

#define ALLOC(type) ((type *)cmalloc(sizeof(type)))
#define ALLOC_N(type, count) ((type *)cmalloc(sizeof(type) * (count)))
#define ALLOC_ZERO(type) ((type *)ccalloc(1, sizeof(type)))
// Frees and NULLs in one step, no dangling pointers
#define FREE(ptr)                                                              \
  do {                                                                         \
    cfree(ptr);                                                                \
    (ptr) = NULL;                                                              \
  } while (0)

#endif
