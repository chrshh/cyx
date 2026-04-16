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

void PrintDebugPSR(ParserState *psr, Command *cmd) {
  printf("--- PSR --- \nNUM_TOKENS=%zu\n", psr->numTokens);
  Token *tokens = psr->tokens;
  for (size_t i = 0; i < psr->numTokens; i++) {
    printf("Token %zu: %s\n\n", i + 1, tokens[i].literal);
  }

  printf("--- PSR CMD ---\n");
  printf("ARG[0]: %s\n", cmd->cmd);
  for (size_t i = 0; i < cmd->numArgs; i++) {
    printf("ARG[%zu]: %s\n\n", i, cmd->args[i]);
  }
}
