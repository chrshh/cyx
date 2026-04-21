#ifndef CORE_UTIL_H
#define CORE_UTIL_H

// Min, Max macros
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) < (b) ? (a) : (b))

#define ARR_LEN(arr) (sizeof(arr) / sizeof((arr)[0]))

// Byte macros (ULL = Unsigned Long Long)
#define KB(n) ((usize)(n) * 1024ULL)
#define MB(n) (KB(n) * 1024ULL)
#define GB(n) (MB(n) * 1024ULL)

// Function Annotations

/**
 * INLINE = suggests inlining to compiler, but compiler can ignore it if
 * function is too large
 */
#define INLINE static inline

/**
 * FORCE_INLINE = forces compiler to inline. Use sparingly only when
 * optimization is needed after profiling
 */
#define FORCE_INLINE __attribute__((always_inline)) static inline

/**
 * LIKELY = this condition is almost always true
 * UNLIKELY = this condition is almost never true
 */
#define LIKELY(x) __builtin_expect(!!(x), 1)
#define UNLIKELY(x) __builtin_expect(!!(x), 0)

/**
 * Compiler warns if caller ignores return value
 * Use on any function where ignoring the result is a bug
 */
#define MUST_USE __attribute__((warn_unused_result))

#endif
