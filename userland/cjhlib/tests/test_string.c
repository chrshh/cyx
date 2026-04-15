#include <str/string.h>
#include <stdio.h>
#include <string.h>

#define ASSERT(condition, msg)                                                 \
  if (condition) {                                                             \
    printf("PASS✅: %s\n", msg);                                               \
  } else {                                                                     \
    printf("FAIL❌: %s\n", msg);                                               \
  }

int main() {
  printf("=== String Tests ===\n\n");

  // --- NewString ---
  printf("--- NewString ---\n");
  String s = NewString();
  ASSERT(s.capacity == 24, "NewString capacity is 24");
  ASSERT(s.len == 0, "NewString len is 0");
  ASSERT(s.chars != NULL, "NewString chars is not NULL");
  FreeString(s);

  // --- StringFromLiteral ---
  printf("--- StringFromLiteral ---\n");
  String hello = StringFromLiteral("hello");
  ASSERT(hello.len == 5, "StringFromLiteral sets correct len");
  ASSERT(hello.chars[0] == 'h', "first char is correct");
  ASSERT(hello.chars[4] == 'o', "last char is correct");
  ASSERT(hello.chars[5] == '\0', "null terminator set after last char");
  FreeString(hello);

  String empty = StringFromLiteral("");
  ASSERT(empty.len == 0, "StringFromLiteral with empty string has len 0");
  FreeString(empty);

  // --- StringEquals ---
  printf("--- StringEquals ---\n");
  String a = StringFromLiteral("hello");
  String b = StringFromLiteral("hello");
  String c = StringFromLiteral("world");
  String shorter = StringFromLiteral("hell");
  ASSERT(StringEquals(a, b) == 0, "equal strings return 0");
  ASSERT(StringEquals(a, c) != 0, "different strings return non-zero");
  ASSERT(StringEquals(a, shorter) == 0, "different length strings return 0");
  FreeString(a);
  FreeString(b);
  FreeString(c);
  FreeString(shorter);

  // --- StringAppend ---
  printf("--- StringAppend ---\n");
  String base = StringFromLiteral("hello");
  String appended = StringAppend(base, " world");
  ASSERT(appended.len == 11, "appended string has correct len");
  ASSERT(appended.chars[0] == 'h', "appended string starts correctly");
  ASSERT(appended.chars[5] == ' ', "space at index 5");
  ASSERT(appended.chars[6] == 'w', "appended chars start at correct offset");
  ASSERT(appended.chars[10] == 'd', "appended string ends correctly");
  ASSERT(appended.chars[11] == '\0', "appended string is null terminated");
  FreeString(base);
  FreeString(appended);

  String emptyBase = NewString();
  String fromEmpty = StringAppend(emptyBase, "hello");
  ASSERT(fromEmpty.len == 5, "appending to empty string has correct len");
  ASSERT(strncmp(fromEmpty.chars, "hello", 5) == 0, "appending to empty string is correct");
  FreeString(emptyBase);
  FreeString(fromEmpty);

  // --- StringSlice ---
  printf("--- StringSlice ---\n");
  String src = StringFromLiteral("hello world");
  String slice = StringSlice(src, 0, 5);
  ASSERT(slice.len == 5, "slice len is correct");
  ASSERT(strncmp(slice.chars, "hello", 5) == 0, "slice chars are correct");
  FreeString(slice);

  String midSlice = StringSlice(src, 6, 11);
  ASSERT(midSlice.len == 5, "mid-slice len is correct");
  ASSERT(strncmp(midSlice.chars, "world", 5) == 0, "mid-slice chars are correct");
  FreeString(midSlice);

  String emptySlice = StringSlice(src, 3, 3);
  ASSERT(emptySlice.len == 0, "slice with equal start/end has len 0");
  FreeString(emptySlice);
  FreeString(src);

  // --- StringPop ---
  printf("--- StringPop ---\n");
  String popStr = StringFromLiteral("hello");
  popStr = StringPop(popStr);
  ASSERT(popStr.len == 4, "StringPop decrements len");
  ASSERT(popStr.chars[0] == 'h', "StringPop preserves remaining chars");

  String singleChar = StringFromLiteral("x");
  singleChar = StringPop(singleChar);
  ASSERT(singleChar.len == 0, "StringPop on single char gives len 0");

  String emptyPop = NewString();
  emptyPop = StringPop(emptyPop);
  ASSERT(emptyPop.len == 0, "StringPop on empty string is a no-op");
  FreeString(popStr);
  FreeString(singleChar);
  FreeString(emptyPop);

  // --- StringTrim ---
  printf("--- StringTrim ---\n");
  String padded = StringFromLiteral("  hello  ");
  String trimmed = StringTrim(padded);
  ASSERT(trimmed.len == 5, "StringTrim removes leading and trailing spaces");
  ASSERT(strncmp(trimmed.chars, "hello", 5) == 0, "StringTrim result is correct");
  FreeString(padded);
  FreeString(trimmed);

  String leadingOnly = StringFromLiteral("   hi");
  String trimmedLeading = StringTrim(leadingOnly);
  ASSERT(trimmedLeading.len == 2, "StringTrim removes leading spaces only");
  ASSERT(strncmp(trimmedLeading.chars, "hi", 2) == 0, "StringTrim leading result is correct");
  FreeString(leadingOnly);
  FreeString(trimmedLeading);

  String trailingOnly = StringFromLiteral("hi   ");
  String trimmedTrailing = StringTrim(trailingOnly);
  ASSERT(trimmedTrailing.len == 2, "StringTrim removes trailing spaces only");
  ASSERT(strncmp(trimmedTrailing.chars, "hi", 2) == 0, "StringTrim trailing result is correct");
  FreeString(trailingOnly);
  FreeString(trimmedTrailing);

  return 0;
}
