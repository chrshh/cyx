#include <core/memory.h>
#include <core/panic.h>
#include <stdlib.h>
#include <str/strarray.h>

StrArr NewStrArr() {
  StrArr arr;
  arr.capacity = 8;
  arr.len = 0;
  arr.strs = cmalloc(8 * sizeof(String));
  return arr;
}

void FreeStrArr(StrArr strarr) { cfree(strarr.strs); }

StrArr StrArrResize(StrArr arr) {
  arr.capacity = arr.capacity * 2;
  arr.strs = realloc(arr.strs, arr.capacity * sizeof(String));
  if (!arr.strs) {
    panic("failed to realloc");
  }
  return arr;
}

StrArr Split(String str, char delim) {
  if (str.len == 0) {
    panic("cannot split empty string");
  }

  StrArr arr = NewStrArr();
  size_t delimIdx = 0;
  size_t arrIdx = 0;

  for (size_t i = 0; i < str.len; i++) {
    if (arr.len >= arr.capacity) {
      arr = StrArrResize(arr);
    }
    if (str.chars[i] == delim) {
      String newStr = StrSlice(str, delimIdx, i);
      arr.strs[arrIdx] = newStr;
      arrIdx++;
      delimIdx = i + 1;
      arr.len++;
    }
  }

  String newStr = StrSlice(str, delimIdx, str.len);
  arr.strs[arrIdx] = newStr;
  arr.len++;

  return arr;
}
