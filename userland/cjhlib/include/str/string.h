#ifndef STRING_H
#define STRING_H

#include <stddef.h>
#include <core/types.h>
#include <stdbool.h>

#define INIT_CAPACITY 24

typedef struct {
  char *chars;
  usize len;
  usize capacity;
} String;

/**
 * String rules:
 *  - len = actual number of chars, NOT counting '\0'
 *  - capacity = allocated space, NOT counting '\0'
 *  - Always allocate capacity + 1 to house '\0'
 *  - Always set chars[len] = '\0' after writing characters
 *  - Dont check for '\0' in loops, let the bounds check handle that
 **/

String NewStr(void);
void FreeStr(String *str);
String StrResize(String str);

String StrAppend(String str, const char *c);
String StrDup(String str);
String StrFromChar(const char *c);
String StrPop(String str);
String StrTrim(String str);

String StrSlice(String str, usize start, usize end);
bool StrContains(String str, char *c);
bool StrStartsWith(String str, char *prefix);
String StrReplaceChar(String str, char *target, char *replacement);

bool StrEq(String str1, String str2);
bool StrEmpty(char *s);
int StrLen(String str);

#endif
