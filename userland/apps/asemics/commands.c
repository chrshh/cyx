#include "asemics.h"
#include <string.h>

int parseCommands(char *cmd, int len) {
  int cmds = 0;
  if (len == 1) {
    return 0;
  }

  int i = 1;

  while (cmd[i] != '\0') {
    switch (cmd[i]) {
    case 'w':
      cmds |= SAVE;
      break;
    case 'q':
      cmds |= QUIT;
      break;
    case '!':
      cmds |= FORCE;
      break;
    default:
      return -1;
      break;
    }
    i++;
  }
  return cmds;
}

void execCommands() {
  char *cmd = E.ui.cmdline;
  int len = strlen(cmd);

  int cmds = parseCommands(cmd, len);
  bool force = false;

  if (cmds == -1) {
    // TODO: unknown cmd
  }

  if (cmds == 0) {
    // TODO: no-op
  }

  if (cmds & SAVE) {
    editorSave();
  }
  if (cmds & QUIT) {
    if (cmds & FORCE) {
      force = true;
      editorQuit(force);
    } else {
      editorQuit(force);
    }
  }

  E.mode = MODE_NORMAL;
}

void commandInsertChar(int c) {
  int n = strlen(E.ui.cmdline);
  if (n >= 80) {
    return;
  }
  E.ui.cmdline[n] = c;
  E.ui.cmdline[n + 1] = '\0';
  return;
}

void commandDelChar(void) {
  int n = strlen(E.ui.cmdline);
  if (n == 1) return;
  E.ui.cmdline[n - 1] = '\0';
  return;
}

void editorDrawCmdline(WriteBuf *wb) {
  writeBufAppend(wb, "\x1b[K", 3);
  int cmdlen = strlen(E.ui.cmdline);
  if (cmdlen > E.viewport.width) cmdlen = E.viewport.width;
  writeBufAppend(wb, E.ui.cmdline, cmdlen);
}
