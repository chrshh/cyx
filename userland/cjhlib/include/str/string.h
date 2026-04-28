#ifndef STRING_H
#define STRING_H

#include <stddef.h>
#include <core/types.h>
#include <stdbool.h>

#define INIT_CAPACITY 24

/** Return value for aborting string methods that return a string */
#define STR_INVALID ((String){0})

typedef struct {
  char *chars;
  usize len;
  usize capacity;
} String;

typedef enum {
  STR_OK = 0,
  STR_ERR_NULL_INPUT,
  STR_ERR_INVALID_ARG,
  STR_ERR_OUT_OF_BOUNDS,
} StrError;

/**
 *  Global error number for local threads
 */
extern _Thread_local StrError str_errno;

/**
 * @brief Provides human-readable error message
 */
const char *StrErrorMessage(StrError err);

/**
 * @brief Prints out a human readable
 */

static inline bool StrOk(String s) { return s.chars != NULL; }
static inline bool StrErr(String s) { return s.chars == NULL; }

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

void StrAppend(String *str, const char *c);
String StrDup(String str);
String StrFromChar(const char *c);
String StrPop(String str);
String StrTrim(String str);

String StrSlice(String str, usize start, usize end);
bool StrContains(String str, char *c);
bool StrStartsWith(String str, char *prefix);
String StrReplaceChar(String str, char target, char replacement);

/**
 * brief Adds *str2 -> end of *str1 and returns a malloc'd String
 * warning Caller is responsible for freeing the old *str1 & *str2
 */
String StrConcat(const String *str1, const String *str2);

bool StrEq(String str1, String str2);
bool StrEmpty(char *s);
int StrLen(String str);

#endif
