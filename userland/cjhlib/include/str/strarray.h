#pragma once
#include <str/string.h>

typedef struct {
  String *strs;
  size_t len;
  size_t capacity;
} StrArr;

StrArr NewStrArr();
void FreeStrArr(StrArr strarr);

/**
 * @brief  Splits a String at given delimiter
 * @note  Input characters must be of type type String
 * @param  str  Raw input
 * @param  delim char to split str by
 * @return  Array of strings
 */
StrArr Split(String str, char delim);
