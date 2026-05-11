#include <stdlib.h>
#include <string.h>

#include "textr.h"

void editorAppendRow(char *s, size_t len) {
  cfg.er = realloc(cfg.er, sizeof(erow) * (cfg.numrows + 1));

  int pos = cfg.numrows;

  cfg.er[pos].size = len;
  cfg.er[pos].chars = malloc(len + 1);
  memcpy(cfg.er[pos].chars, s, len);
  cfg.er[pos].chars[len] = '\0';
  cfg.numrows++;
}
