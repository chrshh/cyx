#include "textr.h"
#include <stdarg.h>
#include <stdio.h>
#include <string.h>

char *getModeStr(void) {
  switch (E.mode) {
  case MODE_INSERT:
    return "-- INSERT -- ";
    break;
  case MODE_COMMAND:
    return "-- COMMAND -- ";
    break;
  case MODE_VISUAL:
    return "-- VISUAL -- ";
    break;
  case MODE_NORMAL:
    return "-- NORMAL -- ";
    break;
  default:
    return "-- NORMAL -- ";
    break;
  }
}

void editorDrawStatusBar(WriteBuf *wb) {
  writeBufAppend(wb, "\x1b[7m", 4);
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
      E.buffer.filename ? E.buffer.filename : "[No Name]",
      E.buffer.num_rows,
      E.buffer.dirty ? "*" : "");

  int rlen = snprintf(
      rstatus,
      sizeof(rstatus),
      "%s | %d/%d",
      E.syntax ? E.syntax->filetype : "no ft",
      E.cursor.y + 1,
      E.buffer.num_rows);

  if (modelen > E.viewport.width) modelen = E.viewport.width;
  if (modelen + statuslen > E.viewport.width) statuslen = E.viewport.width - modelen;
  if (modelen + statuslen + rlen > E.viewport.width) rlen = E.viewport.width - modelen - statuslen;
  if (statuslen < 0) statuslen = 0;
  if (rlen < 0) rlen = 0;

  writeBufAppend(wb, mdstatus, modelen);
  writeBufAppend(wb, status, statuslen);

  int written = modelen + statuslen;
  while (written < E.viewport.width - rlen) {
    writeBufAppend(wb, " ", 1);
    written++;
  }
  writeBufAppend(wb, rstatus, rlen);
  writeBufAppend(wb, "\x1b[m", 3);
}

void editorSetStatusMsg(const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  vsnprintf(E.ui.msg, sizeof(E.ui.msg), fmt, ap);
  va_end(ap);
  E.ui.msg_time = time(NULL);
}

void editorDrawMsgBar(WriteBuf *wb) {
  writeBufAppend(wb, "\x1b[K", 3);
  int msglen = strlen(E.ui.msg);
  if (msglen > E.viewport.width) msglen = E.viewport.width;
  if (msglen && time(NULL) - E.ui.msg_time < 5)
    writeBufAppend(wb, E.ui.msg, msglen);
}
