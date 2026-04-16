#include <ctype.h>

int isAlphaNum(char c) { return isalpha(c) || isdigit(c); }
int isShChar(char c) {
  return c == '*' || c == '?' || c == '.' || c == '/' || c == '-' || c == '\\';
}
