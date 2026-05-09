#include <str/string.h>
#include <stdio.h>
#include <string.h>

#define ASSERT(condition, msg)                                                 \
  if (condition) {                                                             \
    printf("PASS✅: %s\n", msg);                                               \
  } else {                                                                     \
    printf("FAIL❌: %s\n", msg);                                               \
  }

int main(void) {
  printf("=== String Tests ===\n\n");

  // --- NewStr ---
  printf("--- NewStr ---\n");
  String s = NewStr();
  ASSERT(s.capacity == 24, "NewStr capacity is 24");
  ASSERT(s.len == 0, "NewStr len is 0");
  ASSERT(s.chars != NULL, "NewStr chars is not NULL");
  FreeStr(&s);

  // --- StrFromChar ---
  printf("--- StrFromChar ---\n");
  String hello = StrFromChar("hello");
  ASSERT(hello.len == 5, "StrFromChar sets correct len");
  ASSERT(hello.chars[0] == 'h', "first char is correct");
  ASSERT(hello.chars[4] == 'o', "last char is correct");
  ASSERT(hello.chars[5] == '\0', "null terminator set after last char");
  FreeStr(&hello);

  String empty = StrFromChar("");
  ASSERT(empty.len == 0, "StrFromChar with empty string has len 0");
  FreeStr(&empty);

  // --- StrEq ---
  printf("--- StrEq ---\n");
  String a = StrFromChar("hello");
  String b = StrFromChar("hello");
  String c = StrFromChar("world");
  String shorter = StrFromChar("hell");
  ASSERT(StrEq(a, b), "equal strings return true");
  ASSERT(!StrEq(a, c), "different strings return false");
  ASSERT(!StrEq(a, shorter), "different length strings return false");
  FreeStr(&a);
  FreeStr(&b);
  FreeStr(&c);
  FreeStr(&shorter);

  // --- StrAppend ---
  printf("--- StrAppend ---\n");
  String base = StrFromChar("hello");
  StrAppend(&base, " world");
  ASSERT(base.len == 11, "appended string has correct len");
  ASSERT(base.chars[0] == 'h', "appended string starts correctly");
  ASSERT(base.chars[5] == ' ', "space at index 5");
  ASSERT(base.chars[6] == 'w', "appended chars start at correct offset");
  ASSERT(base.chars[10] == 'd', "appended string ends correctly");
  ASSERT(base.chars[11] == '\0', "appended string is null terminated");
  FreeStr(&base);

  // Repeated StrAppend on the same string -- regression test for the
  // FormatPermsOctal NULL-free warnings (StrAppend used to drop the result
  // on the floor, so the second append saw chars==NULL).
  String multi = NewStr();
  StrAppend(&multi, "rwx");
  StrAppend(&multi, "r-x");
  StrAppend(&multi, "r-x");
  ASSERT(multi.len == 9, "three appends produce len 9");
  ASSERT(strncmp(multi.chars, "rwxr-xr-x", 9) == 0, "three appends preserve content");
  FreeStr(&multi);

  String emptyBase = NewStr();
  StrAppend(&emptyBase, "hello");
  ASSERT(emptyBase.len == 5, "appending to empty string has correct len");
  ASSERT(strncmp(emptyBase.chars, "hello", 5) == 0, "appending to empty string is correct");
  FreeStr(&emptyBase);

  // --- StrSlice ---
  printf("--- StrSlice ---\n");
  String src = StrFromChar("hello world");
  String slice = StrSlice(src, 0, 5);
  ASSERT(slice.len == 5, "slice len is correct");
  ASSERT(strncmp(slice.chars, "hello", 5) == 0, "slice chars are correct");
  FreeStr(&slice);

  String midSlice = StrSlice(src, 6, 11);
  ASSERT(midSlice.len == 5, "mid-slice len is correct");
  ASSERT(strncmp(midSlice.chars, "world", 5) == 0, "mid-slice chars are correct");
  FreeStr(&midSlice);

  String emptySlice = StrSlice(src, 3, 3);
  ASSERT(emptySlice.len == 0, "slice with equal start/end has len 0");
  FreeStr(&emptySlice);
  FreeStr(&src);

  // --- StrPop ---
  printf("--- StrPop ---\n");
  String popStr = StrFromChar("hello");
  popStr = StrPop(popStr);
  ASSERT(popStr.len == 4, "StrPop decrements len");
  ASSERT(popStr.chars[0] == 'h', "StrPop preserves remaining chars");

  String singleChar = StrFromChar("x");
  singleChar = StrPop(singleChar);
  ASSERT(singleChar.len == 0, "StrPop on single char gives len 0");

  String emptyPop = NewStr();
  emptyPop = StrPop(emptyPop);
  ASSERT(emptyPop.len == 0, "StrPop on empty string is a no-op");
  FreeStr(&popStr);
  FreeStr(&singleChar);
  FreeStr(&emptyPop);

  // --- StrTrim ---
  printf("--- StrTrim ---\n");
  String padded = StrFromChar("  hello  ");
  String trimmed = StrTrim(padded);
  ASSERT(trimmed.len == 5, "StrTrim removes leading and trailing spaces");
  ASSERT(strncmp(trimmed.chars, "hello", 5) == 0, "StrTrim result is correct");
  FreeStr(&padded);
  FreeStr(&trimmed);

  String leadingOnly = StrFromChar("   hi");
  String trimmedLeading = StrTrim(leadingOnly);
  ASSERT(trimmedLeading.len == 2, "StrTrim removes leading spaces only");
  ASSERT(strncmp(trimmedLeading.chars, "hi", 2) == 0, "StrTrim leading result is correct");
  FreeStr(&leadingOnly);
  FreeStr(&trimmedLeading);

  String trailingOnly = StrFromChar("hi   ");
  String trimmedTrailing = StrTrim(trailingOnly);
  ASSERT(trimmedTrailing.len == 2, "StrTrim removes trailing spaces only");
  ASSERT(strncmp(trimmedTrailing.chars, "hi", 2) == 0, "StrTrim trailing result is correct");
  FreeStr(&trailingOnly);
  FreeStr(&trimmedTrailing);

  return 0;
}
