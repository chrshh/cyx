#include <stdnoreturn.h>
void panic(char *msg);

noreturn void panic_impl(const char *file, int line, const char *func,
                         const char *fmt, ...);

/**
 * Main Panic
 * @return Captured file, line, & function name
 *
 * Usage: PANIC("unexpected token: %s", token->val);
 */
#define PANIC(fmt, ...)                                                        \
  panic_impl(__FILE__, __LINE__, __func__, fmt, ##__VA_ARGS__)

// Panics if condition is FALSE
#define PANIC_IF(cond, fmt, ...)                                               \
  do {                                                                         \
    if (cond)                                                                  \
      PANIC(fmt, ##__VA_ARGS__);                                               \
  } while (0)

// Panics if pointer is NULL
#define PANIC_IF_NULL(ptr, fmt, ...) PANIC_IF((ptr) == NULL, fmt, ##__VA_ARGS__)
