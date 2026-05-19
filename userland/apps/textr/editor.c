#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include "textr.h"

void initEditor(EditorConfig *cfg) {
  cfg->mode = MODE_NORMAL;
  cfg->x = 0;
  cfg->y = 0;
  cfg->rowoff = 0;
  cfg->coloff = 0;
  cfg->numrows = 0;
  cfg->er = NULL;
  cfg->dirty = false;
  cfg->rx = 0;
  cfg->filename = NULL;
  cfg->statusmsg[0] = '\0';
  cfg->statusmsg_time = 0;
  cfg->cmdline[0] = '\0';
  if (getWindowSize(&cfg->rows, &cfg->cols) == -1) die("getWindowSize");
  cfg->rows -= 2;
}

void repositionCursorTL(wBuf *wb) {
  wBufAppend(wb, CURSOR_TL, 3);
}

void drawRows(wBuf *wb) {
  int y;
  for (y = 0; y < cfg.rows - 1; y++) {
    int filerow = y + cfg.rowoff;
    if (filerow >= cfg.numrows) {
      if (cfg.numrows == 0 && y == cfg.rows / 3) {
        char welcome[80];
        int welcomelen = snprintf(welcome, sizeof(welcome), "textr -- 0.1");
        if (welcomelen > cfg.cols) welcomelen = cfg.cols;
        int padding = (cfg.cols - welcomelen) / 2;
        if (padding) {
          wBufAppend(wb, "~", 1);
          padding--;
        }
        while (padding--) wBufAppend(wb, " ", 1);
        wBufAppend(wb, welcome, welcomelen);
      } else {
        wBufAppend(wb, "~", 1);
      }

    } else {
      int len = cfg.er[filerow].rsize - cfg.coloff;
      if (len < 0) len = 0;
      if (len > cfg.cols) len = cfg.cols;
      wBufAppend(wb, &cfg.er[filerow].render[cfg.coloff], len);
    }

    wBufAppend(wb, "\x1b[K", 3);
    wBufAppend(wb, "\r\n", 2);
  }
}

void updateCursorType(wBuf *wb) {
  switch (cfg.mode) {
  case MODE_NORMAL:
    wBufAppend(wb, CURSOR_BLOCK, 5);
    break;
  case MODE_COMMAND:
    wBufAppend(wb, CURSOR_HIDE, 6);
    break;
  case MODE_INSERT:
    wBufAppend(wb, CURSOR_BAR, 5);
    break;
  case MODE_VISUAL:
    wBufAppend(wb, CURSOR_BLOCK, 5);
    break;
  default:
    return;
  }
}

void refreshScreen(void) {
  editorScroll();
  wBuf wb = initWBuf();

  wBufAppend(&wb, CURSOR_HIDE, 6);
  wBufAppend(&wb, SCREEN_CLEAR, 4);
  repositionCursorTL(&wb);
  drawRows(&wb);

  char buf[32];
  int n;

  n = snprintf(buf, sizeof(buf), "\x1b[%d;1H", cfg.rows + 1);
  wBufAppend(&wb, buf, n);
  editorDrawStatusBar(&wb);

  n = snprintf(buf, sizeof(buf), "\x1b[%d;1H", cfg.rows + 2);
  wBufAppend(&wb, buf, n);
  if (cfg.cmdline[0] != '\0') {
    editorDrawCmdline(&wb);
  } else {
    editorDrawMsgBar(&wb);
  }

  n = snprintf(buf, sizeof(buf), "\x1b[%d;%dH", (cfg.y - cfg.rowoff) + 1, (cfg.rx - cfg.coloff) + 1);
  wBufAppend(&wb, buf, n);

  /* Enabled cursor and render cursor based on EDITOR MODE */
  wBufAppend(&wb, CURSOR_SHOW, 6);
  updateCursorType(&wb);
  write(STDOUT_FILENO, wb.b, wb.len);
  wBFree(&wb);
}

void clearScreen(void) {
  write(STDOUT_FILENO, SCREEN_CLEAR, 4);
  write(STDOUT_FILENO, CURSOR_TL, 3);
}

wBuf initWBuf() {
  wBuf wb;
  wb.b = NULL;
  wb.len = 0;
  return wb;
}

void wBufAppend(wBuf *wb, const char *s, int len) {
  char *new = realloc(wb->b, wb->len + len);

  if (new == NULL) return;
  memcpy(&new[wb->len], s, len);
  wb->b = new;
  wb->len += len;
}

void wBFree(wBuf *wb) {
  free(wb->b);
}

void moveCursor(int key) {
  erow *row = (cfg.y >= cfg.numrows) ? NULL : &cfg.er[cfg.y];
  switch (key) {

  case 'h':
  case ARROW_LEFT:
    if (cfg.x != 0) {
      cfg.x--;
    }
    break;

  case 'j':
  case ARROW_DOWN:
    if (cfg.y < cfg.numrows) {
      cfg.y++;
    }
    break;

  case 'k':
  case ARROW_UP:
    if (cfg.y != 0) {
      cfg.y--;
    }
    break;

  case 'l':
  case ARROW_RIGHT:
    if (row && cfg.x < row->size - 1) {
      cfg.x++;
    }
    break;
  }

  row = (cfg.y >= cfg.numrows) ? NULL : &cfg.er[cfg.y];
  int rowlen = row ? row->size : 0;
  if (cfg.x > rowlen) {
    cfg.x = rowlen;
  }
}

void editorOpen(char *filename) {
  free(cfg.filename);
  cfg.filename = strdup(filename);
  FILE *fp = fopen(filename, "r");
  if (!fp) die("fopen");

  char *line = NULL;
  size_t linecap = 0;
  ssize_t linelen;
  while ((linelen = getline(&line, &linecap, fp)) != -1) {
    while (linelen > 0 && (line[linelen - 1] == '\n' || line[linelen - 1] == '\r')) linelen--;

    editorInsertRow(cfg.numrows, line, linelen);
  }
  free(line);
  fclose(fp);
  cfg.dirty = false;
}

void editorScroll() {
  cfg.rx = 0;
  if (cfg.y < cfg.numrows) {
    cfg.rx = editorRowXtoRx(&cfg.er[cfg.y], cfg.x);
  }

  if (cfg.y < cfg.rowoff) {
    cfg.rowoff = cfg.y;
  }
  if (cfg.y >= cfg.rowoff + cfg.rows) {
    cfg.rowoff = cfg.y - cfg.rows + 1;
  }
  if (cfg.rx < cfg.coloff) {
    cfg.coloff = cfg.rx;
  }
  if (cfg.rx >= cfg.coloff + cfg.cols) {
    cfg.coloff = cfg.rx - cfg.cols + 1;
  }
}

char *editorRowsToStr(int *buflen) {
  int totallen = 0;
  int j;

  for (j = 0; j < cfg.numrows; j++)
    totallen += cfg.er[j].size + 1;
  *buflen = totallen;

  char *buf = malloc(totallen);
  char *p = buf;

  for (j = 0; j < cfg.numrows; j++) {
    memcpy(p, cfg.er[j].chars, cfg.er[j].size);
    p += cfg.er[j].size;
    *p = '\n';
    p++;
  }

  return buf;
}

void editorSave() {
  if (cfg.filename == NULL) return;

  int len;
  char *buf = editorRowsToStr(&len);

  int fd = open(cfg.filename, O_RDWR | O_CREAT, 0644);
  if (fd != -1) {
    if (ftruncate(fd, len) != -1) {
      if (write(fd, buf, len) == len) {
        close(fd);
        free(buf);
        cfg.dirty = false;
        cfg.cmdline[0] = '\0';
        editorSetStatusMsg("%d: bytes written to disk", len);
        return;
      }
    }
    close(fd);
  }
  free(buf);
  editorSetStatusMsg("Failed to save. I/O error: %s", strerror(errno));
}

void editorQuit() {
  clearScreen();
  exit(0);
}
