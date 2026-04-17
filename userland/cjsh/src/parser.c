#include "common.h"
#include <parser.h>
#include <core/memory.h>
#include <core/panic.h>
#include <stdio.h>
#include <exec.h>
#include <stdlib.h>
#include <string.h>

ParserState initParserState(size_t numTokens) {
  ParserState psr;
  psr.numTokens = numTokens;
  psr.tokens = NULL;
  psr.pos = 0;
  return psr;
}

void destroyParserState(ParserState *psr) {
  psr->numTokens = 0;
  psr->pos = 0;
}

void parse(ParserState *psr) {
  if (psr->numTokens < 1) {
    panic("0 tokens");
  }

  /**
   * Create Space for args[], set main command to the very first token
   **/
  Command *cmd = cmalloc(sizeof(Command));
  cmd->args = cmalloc((psr->numTokens + 1) * (sizeof(char *)));
  memset(cmd->args, 0, (psr->numTokens + 1) * sizeof(char *));
  cmd->cmd = psr->tokens[0].literal;
  cmd->args[0] = psr->tokens[0].literal;
  psr->pos = 1;
  cmd->numArgs = 1;
  Command *head = cmd;

  while (psr->pos < psr->numTokens) {
    if (psr->pos > psr->numTokens) {
      return;
    }
    switch (psr->tokens[psr->pos].lexeme) {
    case WORD:
      cmd = parseWrd(psr, cmd);
      break;
    case NUMBER:
      cmd = parseNum(psr, cmd);
      break;
    case STRING:
      cmd = parseStr(psr, cmd);
      break;
    case PIPE:
      cmd = parsePipe(psr, cmd);
      break;
    default:
      printf("command not recognized\n");
      break;
    }
  }
  cmd->args[cmd->numArgs] = NULL;

  // PrintDebugPSR(head);
  execute(head);
}

// int match(ParserState *psr) {}

Token *peekNextToken(ParserState *psr) {
  if (psr->pos + 1 >= psr->numTokens) {
    return NULL;
  }
  return &psr->tokens[psr->pos + 1];
}

Command *parseWrd(ParserState *psr, Command *cmd) {
  cmd->args[cmd->numArgs] = psr->tokens[psr->pos].literal;
  cmd->numArgs++;
  psr->pos++;
  return cmd;
}

Command *parseStr(ParserState *psr, Command *cmd) {
  cmd->args[cmd->numArgs] = psr->tokens[psr->pos].literal;
  psr->pos++;
  cmd->numArgs++;
  return cmd;
}

Command *parseNum(ParserState *psr, Command *cmd) {
  cmd->args[cmd->numArgs] = psr->tokens[psr->pos].literal;
  psr->pos++;
  cmd->numArgs++;
  return cmd;
}

Command *parsePipe(ParserState *psr, Command *cmd) {
  Token *nxtToken = peekNextToken(psr);
  if (nxtToken == NULL) {
    printf("cjsh: broken pipe");
    exit(1);
  }
  cmd->args[cmd->numArgs] = NULL;
  Command *newCmd = cmalloc(sizeof(Command));
  newCmd->args = cmalloc((psr->numTokens + 1) * (sizeof(char *)));
  memset(newCmd->args, 0, (psr->numTokens + 1) * sizeof(char *));
  newCmd->cmd = nxtToken->literal;
  newCmd->args[0] = nxtToken->literal;
  newCmd->numArgs = 1;
  newCmd->next = NULL;
  cmd->next = newCmd;
  psr->pos++;
  psr->pos++;
  return newCmd;
}
