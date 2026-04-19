#pragma once

#include <stddef.h>

typedef struct {
  char *chars;
  size_t len;
  size_t capacity;
} String;

/**
 * String rules:
 *  - len = actual number of chars, NOT counting '\0'
 *  - capacity = allocated space, NOT counting '\0'
 *  - Always allocate capacity + 1 to house '\0'
 *  - Always set chars[len] = '\0' after writing characters
 *  - Dont check for '\0' in loops, let the bounds check handle that
 **/

String NewString();
void FreeString(String str);
String StringResize(String str);

String StringAppend(String str, char *c);
String StringCopy(String str);
String StringFromLiteral(char *c);
String StringPop(String str);
String StringTrim(String str);

String StringSlice(String str, size_t start, size_t end);
int StringContains(String str, char *c);
int StringIndexOf(String str);

int StringEquals(String str1, String str2);
int IsEmpty(char *s);
