#include "textr.h"
#include <stdarg.h>
#include <stdio.h>
#include <string.h>

char *getModeStr(void) {
  switch (cfg.mode) {
  case MODE_INSERT:
    return "INSERT";
    break;
  case MODE_COMMAND:
    return "COMMAND";
    break;
  case MODE_VISUAL:
    return "VISUAL";
    break;
  case MODE_NORMAL:
    return "NORMAL";
    break;
  default:
    return "NORMAL";
    break;
  }
}

void editorDrawStatusBar(wBuf *wb) {
  wBufAppend(wb, "\x1b[7m", 4);
  char status[80], rstatus[80], mdstatus[20];

  int mdlen = snprintf(mdstatus, sizeof(mdstatus), "%-10s", getModeStr());

  int len = snprintf(status, sizeof(status), "%.20s - %d lines %s",
                     cfg.filename ? cfg.filename : "[No Name]", cfg.numrows,
                     cfg.dirty ? "*" : "");
  int rlen = snprintf(rstatus, sizeof(rstatus), "%d/%d", cfg.y + 1, cfg.numrows);
  if (len > cfg.cols) len = cfg.cols;
  wBufAppend(wb, mdstatus, mdlen);
  wBufAppend(wb, status, len);
  while (mdlen + len < cfg.cols) {
    if (cfg.cols - len == rlen) {
      wBufAppend(wb, rstatus, rlen);
      break;
    } else {

      wBufAppend(wb, " ", 1);
      len++;
    }
  }
  wBufAppend(wb, "\r\n", 2);
  wBufAppend(wb, "\x1b[m", 3);
}

void editorSetStatusMsg(const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  vsnprintf(cfg.statusmsg, sizeof(cfg.statusmsg), fmt, ap);
  va_end(ap);
  cfg.statusmsg_time = time(NULL);
}

void editorDrawMsgBar(wBuf *wb) {
  wBufAppend(wb, "\x1b[K", 3);
  int msglen = strlen(cfg.statusmsg);
  if (msglen > cfg.cols) msglen = cfg.cols;
  if (msglen && time(NULL) - cfg.statusmsg_time < 5)
    wBufAppend(wb, cfg.statusmsg, msglen);
}
