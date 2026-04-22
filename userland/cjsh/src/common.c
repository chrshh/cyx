#include "common.h"
#include "core/ansi.h"
#include "parser.h"
#include <ctype.h>
#include <stdio.h>
#include <limits.h>
#include <string.h>
#include <unistd.h>
#include <core/memory.h>

int isAlphaNum(char c) { return isalpha(c) || isdigit(c); }
int isShChar(char c) {
  return c == '*' || c == '?' || c == '.' || c == '-' || c == '\\';
}

/* Print util used for handling a user error that is recoverable: eg.
 * unterminated string */
void PrintShWarning(char *msg) { printf(YELLOW "cjsh: %s" RESET, msg); }

String GetShPrompt() {
  String path = NewString();
  char buf[PATH_MAX];
  char *cwd = getcwd(buf, PATH_MAX);
  path = StringFromLiteral(cwd);
  StrArr str = Split(path, '/');
  path = str.strs[str.len - 1];
  cfree(str.strs);
  return path;
}

void FreeShPrompt(String shPrompt) { cfree(shPrompt.chars); }

void PrintDebugTokensLXR(LexerState *lxr) {
  printf("--- LXR --- \nNUM_TOKENS=%zu\n", lxr->numTokens);
  Token *tokens = lxr->tokens;
  for (usize i = 1; i < lxr->numTokens; i++) {
    printf("Token %zu: %s\n\n", i, tokens[i].literal);
    for (usize j = 0; j < strlen(tokens[i].literal); j++) {
      printf("Character %zu: %c\n", j, tokens[i].literal[j]);
    }
  }
}

void PrintDebugPSR(Command *head) {
  printf("--- PSR CMD ---\n");
  int cmdIdx = 0;
  while (head != NULL) {
    printf("CMD %d: %s\n", cmdIdx, head->args[0]);
    for (size_t i = 0; i < head->numArgs; i++) {
      printf("  ARG[%zu]: %s\n", i, head->args[i]);
    }
    head = head->next;
    cmdIdx++;
  }
}
