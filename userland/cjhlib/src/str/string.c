#include <core/memory.h>
#include <core/panic.h>
#include <ctype.h>
#include <str/string.h>
#include <string.h>

_Thread_local StrError str_errno = STR_OK;

const char *StrErrMessage(StrError err) {
  switch (err) {
  case STR_OK:
    return "ok";
  case STR_ERR_INVALID_ARG:
    return "invalid argument";
  case STR_ERR_NULL_INPUT:
    return "null input";
  case STR_ERR_OUT_OF_BOUNDS:
    return "out of bounds";
  default:
    "unknown error";
  }
}

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

void StrAppend(String *str, const char *c) {
  if (!c) {
    return;
  }
  usize clen = strlen(c);
  usize len = str->len + clen;
  String newStr;
  newStr.len = len;
  newStr.capacity = len;
  newStr.chars = cmalloc(len + 1);
  memcpy(newStr.chars, str->chars, str->len);
  memcpy(newStr.chars + str->len, c, clen);
  FreeStr(str);
  newStr.chars[len] = '\0';
  *str = newStr;
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
  String newStr = NewStr();

  for (size_t i = 0; i < end; i++) {
    if (newStr.len >= newStr.capacity) {
      newStr = StrResize(newStr);
    }
    if (i >= start) {
      newStr.chars[newStr.len] = str.chars[i];
      newStr.len++;
    }
  }
  newStr.chars[newStr.len] = '\0';

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

String StrReplaceChar(String str, char t, char r) {
  for (usize i = 0; i < str.len; i++) {
    if (str.chars[i] == t) {
      str.chars[i] = r;
    }
  }
  return str;
}

String StrConcat(const String *str1, const String *str2) {
  if (str1 == NULL || str2 == NULL) {
    str_errno = STR_ERR_NULL_INPUT;
    return STR_INVALID;
  }
  String res;
  usize fullLen = str1->len + str2->len;

  /** The full string length is len(str1) + len(str2)
   *  We allocate the full string length + 1 for '\0'
   *  Copy str1 -> return buff, if str1 is 10 characters this means res[0-9] is written into
   *  Copy str2 -> return buff OFFSET by str1, otherwise we ovverwrite the same memory (bad bug)
   *  This is the standard safe pattern for combining all data (char, int, etc)
   */
  res.chars = cmalloc(fullLen + 1);
  memcpy(res.chars, str1->chars, str1->len);
  memcpy(res.chars + str1->len, str2->chars, str2->len);
  res.chars[fullLen] = '\0';

  return res;
}
