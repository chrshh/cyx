#ifndef TYPES_H
#define TYPES_H

#include <stddef.h>
#include <stdint.h>

// Unsigned Ints
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

// Signed Ints
typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;

// Floats
typedef float f32;
typedef double f64;

// Pointer sized types
// Use for sizes, counts, pointer arithmetic, file offsets
// Never use raw int for these
typedef size_t usize;    // unsigned: sizes, array indices, strlen results
typedef ptrdiff_t isize; // signed: pointer diffs, signed offsets
typedef uintptr_t uptr;  // unigned int large enough to hold a pointer
typedef intptr_t iptr;   // signed version of the above

#endif
