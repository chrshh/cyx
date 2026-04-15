#include <core/memory.h>
#include <core/panic.h>
#include <ctype.h>
#include <stdlib.h>
#include <str/string.h>
#include <string.h>

String NewString() {
  String str;
  str.capacity = 24;
  str.chars = cmalloc(str.capacity + 1);
  str.len = 0;
  memset(str.chars, 0, str.capacity);
  return str;
}

void FreeString(String str) { cfree(str.chars); }

String StringResize(String str) {
  str.capacity = str.capacity * 2;
  str.chars = realloc(str.chars, str.capacity + 1);
  if (!str.chars) {
    panic("failed to realloc");
  }
  return str;
}

String StringFromLiteral(char *chars) {
  String str = NewString();
  for (size_t i = 0; i < strlen(chars); i++) {
    if (str.len + 1 >= str.capacity) {
      str = StringResize(str);
    }
    str.chars[i] = chars[i];
    str.len += 1;
  }
  str.chars[str.len] = '\0';
  return str;
}

String StringAppend(String str, char *c) {
  if (!c) {
    return str;
  }
  size_t clen = strlen(c);
  size_t len = str.len + clen;
  String newStr = NewString();
  newStr.len = len;
  newStr.capacity = len;
  newStr.chars = cmalloc(len + 1);
  memcpy(newStr.chars, str.chars, str.len);
  memcpy(newStr.chars + str.len, c, clen);
  newStr.chars[len] = '\0';
  return newStr;
}

int StringEquals(String str1, String str2) {
  if (str1.len != str2.len) {
    return 0;
  }
  return strncmp(str1.chars, str2.chars, str1.len) == 0;
}

int len(String str) { return (strlen(str.chars)); }

// could be useful for removing null terminiator
String StringPop(String str) {
  if (len(str) == 0) {
    return str;
  }
  str.len = str.len - 1;
  str.chars[str.len] = '\0';
  return str;
}

String StringTrim(String str) {
  size_t start = 0;
  size_t end = str.len;
  for (size_t i = 0; i < end; i++) {
    if (isspace(str.chars[i])) {
      start++;
    } else {
      break;
    }
  }

  for (size_t i = str.len - 1; i > 0; i--) {
    if (isspace(str.chars[i])) {
      end--;
    } else {
      break;
    }
  }

  return StringSlice(str, start, end);
}

String StringSlice(String str, size_t start, size_t end) {
  String newStr = NewString();

  for (size_t i = 0; i < end; i++) {
    if (newStr.len >= newStr.capacity) {
      newStr = StringResize(newStr);
    }
    if (i >= start) {
      newStr.chars[newStr.len] = str.chars[i];
      newStr.len++;
    }
  }
  newStr.chars[newStr.len] = '\0';

  return newStr;
}
