#include <str/string.h>

typedef struct {
  String *strs;
  size_t len;
  size_t capacity;
} StrArr;

StrArr NewStrArr();
void FreeStrArr(StrArr strarr);

StrArr Split(String str, char delim);
