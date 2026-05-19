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

  int modelen = snprintf(
      mdstatus,
      sizeof(mdstatus),
      "%-10s",
      getModeStr());

  int statuslen = snprintf(
      status,
      sizeof(status),
      "%.20s - %d lines %s",
      cfg.filename ? cfg.filename : "[No Name]",
      cfg.numrows,
      cfg.dirty ? "*" : "");

  int rlen = snprintf(
      rstatus,
      sizeof(rstatus),
      "%d/%d",
      cfg.y + 1,
      cfg.numrows);

  if (modelen > cfg.cols) modelen = cfg.cols;
  if (modelen + statuslen > cfg.cols) statuslen = cfg.cols - modelen;
  if (modelen + statuslen + rlen > cfg.cols) rlen = cfg.cols - modelen - statuslen;
  if (statuslen < 0) statuslen = 0;
  if (rlen < 0) rlen = 0;

  wBufAppend(wb, mdstatus, modelen);
  wBufAppend(wb, status, statuslen);

  int written = modelen + statuslen;
  while (written < cfg.cols - rlen) {
    wBufAppend(wb, " ", 1);
    written++;
  }
  wBufAppend(wb, rstatus, rlen);
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
