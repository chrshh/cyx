#include "common.h"
#include "parser.h"
#include <ctype.h>
#include <stdio.h>

int isAlphaNum(char c) { return isalpha(c) || isdigit(c); }
int isShChar(char c) {
  return c == '*' || c == '?' || c == '.' || c == '/' || c == '-' || c == '\\';
}

void PrintDebugTokensLXR(LexerState *lxr) {
  printf("--- LXR --- \nNUM_TOKENS=%zu\n", lxr->numTokens);
  Token *tokens = lxr->tokens;
  for (size_t i = 0; i < lxr->numTokens; i++) {
    printf("Token %zu: %s\n\n", i + 1, tokens[i].literal);
  }
}

void PrintDebugPSR(Command *head) {
  printf("--- PSR CMD ---\n");
  int cmdIdx = 0;
  while (head != NULL) {
    printf("CMD %d: %s\n", cmdIdx, head->cmd);
    for (size_t i = 0; i < head->numArgs; i++) {
      printf("  ARG[%zu]: %s\n", i, head->args[i]);
    }
    head = head->next;
    cmdIdx++;
  }
}
