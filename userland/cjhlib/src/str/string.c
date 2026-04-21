#include <core/memory.h>
#include <core/panic.h>
#include <ctype.h>
#include <str/string.h>
#include <string.h>

String NewStr() {
  String str;
  str.capacity = INIT_CAPACITY;
  str.chars = cmalloc(str.capacity + 1);
  str.len = 0;
  memset(str.chars, 0, str.capacity);
  return str;
}

void FreeStr(String *str) {
  FREE(str->chars);
  str->capacity = 0;
  str->len = 0;
}

String StrResize(String str) {
  str.capacity = str.capacity * 2;
  str.chars = crealloc(str.chars, str.capacity + 1);
  return str;
}

String StrFromChar(const char *chars) {
  String str = NewStr();
  for (usize i = 0; i < strlen(chars); i++) {
    if (str.len + 1 >= str.capacity) {
      str = StrResize(str);
    }
    str.chars[i] = chars[i];
    str.len += 1;
  }
  str.chars[str.len] = '\0';
  return str;
}

String StrAppend(String str, const char *c) {
  if (!c) {
    return str;
  }
  usize clen = strlen(c);
  usize len = str.len + clen;
  String newStr = NewStr();
  newStr.len = len;
  newStr.capacity = len;
  newStr.chars = cmalloc(len + 1);
  memcpy(newStr.chars, str.chars, str.len);
  memcpy(newStr.chars + str.len, c, clen);
  FreeStr(&str);
  newStr.chars[len] = '\0';
  return newStr;
}

bool StrEq(String str1, String str2) {
  if (str1.len != str2.len) {
    return false;
  }
  return strncmp(str1.chars, str2.chars, str1.len) == 0;
}

int StrLen(String str) { return str.len; }

// could be useful for removing null terminiator
String StrPop(String str) {
  if (StrLen(str) == 0) {
    return str;
  }
  str.len = str.len - 1;
  str.chars[str.len] = '\0';
  return str;
}

String StrTrim(String str) {
  usize start = 0;
  usize end = str.len;
  for (usize i = 0; i < end; i++) {
    if (isspace(str.chars[i])) {
      start++;
    } else {
      break;
    }
  }

  for (usize i = str.len - 1; i > 0; i--) {
    if (isspace(str.chars[i])) {
      end--;
    } else {
      break;
    }
  }

  return StrSlice(str, start, end);
}

String StrSlice(String str, usize start, usize end) {
  if (!(start >= end) || !(end > str.len)) panic("unable to slice string");
  String newStr = NewStr();

  for (usize i = 0; i < end; i++) {
    if (newStr.len >= newStr.capacity) {
      newStr = StrResize(newStr);
    }
    if (i >= start) {
      newStr.chars[newStr.len] = str.chars[i];
      newStr.len++;
    }
  }
  newStr.chars[newStr.len] = '\0';
  FreeStr(&str);

  return newStr;
}

// Returns 1 if empty
bool StrEmpty(char *s) {
  if (s[0] == '\0') return true;
  return false;
}

bool StrContains(String str, char *c) {
  if (c[0] == '\0') return false;
  usize len = str.len;
  usize matchIdx = 0;
  usize countIdx = 0;
  while (countIdx < len) {
    if (c[matchIdx] == '\0') return true;

    if (str.chars[countIdx] == c[matchIdx]) {
      matchIdx++;
    } else {
      matchIdx = 0;
    }

    countIdx++;
  }
  return false;
}

String StrDup(String str) {
  String dupStr = StrFromChar(str.chars);
  dupStr.capacity = str.capacity;
  dupStr.len = str.len;
  return dupStr;
}

bool StrStartsWith(String str, char *prefix) {
  usize i = 0;
  while (prefix[i] != '\0') {
    if (str.chars[i] != prefix[i]) {
      return false;
    }
    i++;
  }
  return true;
}

String StrReplaceChar(String str, char *t, char *r) {
  if (t[1] != '\0' || r[1] != '\0') {
    panic("cannot replace multiple characters at once");
  }

  for (usize i = 0; i < str.len; i++) {
    if (str.chars[i] == t[0]) {
      str.chars[i] = r[0];
    }
  }
  return str;
}
