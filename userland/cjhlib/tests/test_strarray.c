#include <str/strarray.h>
#include <stdio.h>
#include <string.h>

// NOTE: header declares NewStrArray() but implementation defines NewStrArr().
// Tests use Split() which calls NewStrArr() internally, so no direct call needed.

#define ASSERT(condition, msg)                                                 \
  if (condition) {                                                             \
    printf("PASS✅: %s\n", msg);                                               \
  } else {                                                                     \
    printf("FAIL❌: %s\n", msg);                                               \
  }

int main() {
  printf("=== StrArr Tests ===\n\n");

  // --- NewStrArr ---
  printf("--- NewStrArr ---\n");
  StrArr arr = NewStrArr();
  ASSERT(arr.capacity == 8, "NewStrArr capacity is 8");
  ASSERT(arr.len == 0, "NewStrArr len is 0");
  ASSERT(arr.strs != NULL, "NewStrArr strs is not NULL");
  FreeStrArr(arr);

  // --- Split: basic ---
  printf("--- Split ---\n");
  String csv = StrFromChar("one,two,three");
  StrArr parts = Split(csv, ',');
  ASSERT(parts.len == 3, "Split produces correct number of parts");
  ASSERT(strncmp(parts.strs[0].chars, "one", 3) == 0, "first part is correct");
  ASSERT(strncmp(parts.strs[1].chars, "two", 3) == 0, "second part is correct");
  ASSERT(strncmp(parts.strs[2].chars, "three", 5) == 0, "third part is correct");
  FreeStrArr(parts);
  FreeStr(&csv);

  // --- Split: single segment (no delimiter) ---
  printf("--- Split: no delimiter ---\n");
  String noDelim = StrFromChar("hello");
  StrArr single = Split(noDelim, ',');
  ASSERT(single.len == 1, "Split with no delimiter returns one segment");
  ASSERT(strncmp(single.strs[0].chars, "hello", 5) == 0, "single segment is correct");
  FreeStrArr(single);
  FreeStr(&noDelim);

  // --- Split: two parts ---
  printf("--- Split: two parts ---\n");
  String pair = StrFromChar("hello world");
  StrArr twoParts = Split(pair, ' ');
  ASSERT(twoParts.len == 2, "Split on space produces two parts");
  ASSERT(strncmp(twoParts.strs[0].chars, "hello", 5) == 0, "first part correct");
  ASSERT(strncmp(twoParts.strs[1].chars, "world", 5) == 0, "second part correct");
  FreeStrArr(twoParts);
  FreeStr(&pair);

  // --- Split: many parts ---
  printf("--- Split: many parts ---\n");
  String path = StrFromChar("a/b/c/d/e");
  StrArr pathParts = Split(path, '/');
  ASSERT(pathParts.len == 5, "Split produces correct count for many parts");
  ASSERT(strncmp(pathParts.strs[0].chars, "a", 1) == 0, "first path part correct");
  ASSERT(strncmp(pathParts.strs[4].chars, "e", 1) == 0, "last path part correct");
  FreeStrArr(pathParts);
  FreeStr(&path);

  return 0;
}
