#ifndef CJSH_TEST_H
#define CJSH_TEST_H

#include <stdio.h>
#include <string.h>
#include <setjmp.h>

// Provided by cjsh.c normally — tests need to supply the symbol
sigjmp_buf prompt_jmp;

static int tests_run = 0;
static int tests_passed = 0;
static int tests_failed = 0;

#define ASSERT(cond, msg)                                                      \
  do {                                                                         \
    tests_run++;                                                               \
    if (cond) {                                                                \
      tests_passed++;                                                          \
    } else {                                                                   \
      tests_failed++;                                                          \
      printf("  FAIL: %s (line %d)\n", msg, __LINE__);                         \
    }                                                                          \
  } while (0)

#define ASSERT_STR_EQ(actual, expected, msg)                                   \
  do {                                                                         \
    tests_run++;                                                               \
    if ((actual) != NULL && (expected) != NULL &&                               \
        strcmp((actual), (expected)) == 0) {                                    \
      tests_passed++;                                                          \
    } else {                                                                   \
      tests_failed++;                                                          \
      printf("  FAIL: %s (line %d)\n    expected: \"%s\"\n    got:      "      \
             "\"%s\"\n",                                                        \
             msg, __LINE__, (expected) ? (expected) : "(null)",                 \
             (actual) ? (actual) : "(null)");                                   \
    }                                                                          \
  } while (0)

#define ASSERT_INT_EQ(actual, expected, msg)                                   \
  do {                                                                         \
    tests_run++;                                                               \
    if ((actual) == (expected)) {                                               \
      tests_passed++;                                                          \
    } else {                                                                   \
      tests_failed++;                                                          \
      printf("  FAIL: %s (line %d)\n    expected: %d\n    got:      %d\n",     \
             msg, __LINE__, (expected), (actual));                              \
    }                                                                          \
  } while (0)

#define RUN_SUITE(name, fn)                                                    \
  do {                                                                         \
    printf("--- %s ---\n", name);                                              \
    fn();                                                                       \
  } while (0)

#define REPORT()                                                               \
  do {                                                                         \
    printf("\n=== %d tests: %d passed, %d failed ===\n", tests_run,            \
           tests_passed, tests_failed);                                         \
    return tests_failed > 0 ? 1 : 0;                                           \
  } while (0)

#endif
