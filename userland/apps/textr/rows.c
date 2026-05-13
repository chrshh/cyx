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

  cfg.er[pos].rsize = 0;
  cfg.er[pos].render = NULL;
  editorUpdateRow(&cfg.er[pos]);

  cfg.numrows++;
}

void editorUpdateRow(erow *er) {
  int tabs = 0;
  int j;
  for (j = 0; j < er->size; j++)
    if (er->chars[j] == '\t') tabs++;

  free(er->render);
  er->render = malloc(er->size + tabs * (TAB_STOP - 1) + 1);

  int idx = 0;
  for (j = 0; j < er->size; j++) {
    if (er->chars[j] == '\t') {
      er->render[idx++] = ' ';
      while (idx % TAB_STOP != 0) er->render[idx++] = ' ';
    } else {
      er->render[idx++] = er->chars[j];
    }
  }

  er->render[idx] = '\0';
  er->rsize = idx;
}

int editorRowXtoRx(erow *er, int x) {
  int rx = 0;
  int j = 0;

  for (j = 0; j < x; j++) {
    if (er->chars[j] == '\t')
      rx += (TAB_STOP - 1) - (rx % TAB_STOP);
    rx++;
  }
  return rx;
}

void editorRowInsertChar(erow *er, int pos, int c) {
  if (pos < 0 || pos > er->size) pos = er->size;
  er->chars = realloc(er->chars, er->size + 2);
  memmove(&er->chars[pos + 1], &er->chars[pos], er->size - pos + 1);
  er->size++;
  er->chars[pos] = c;
  editorUpdateRow(er);
}

void editorInsertChar(int c) {
  if (cfg.y == cfg.numrows) {
    editorAppendRow("", 0);
  }
  editorRowInsertChar(&cfg.er[cfg.y], cfg.x, c);
  cfg.x++;
  cfg.dirty = true;
}
