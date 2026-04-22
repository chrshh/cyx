#include "ast.h"
#include <string.h>

char *expandWord(WordPart *words) {
  char *fullWord = "";

  while (words != NULL) {
    if (words->type == WP_VAR) {
      strncat();
    }
    words = words->next;
  }

  return fullWord;
}
