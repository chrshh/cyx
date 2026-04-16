#include "common.h"
#include <parser.h>
#include <core/memory.h>
#include <core/panic.h>
#include <stdio.h>
#include <exec.h>

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
  Command cmd;
  cmd.args = cmalloc((psr->numTokens + 1) * (sizeof(char *)));
  cmd.cmd = psr->tokens[0].literal;

  for (size_t i = 0; i < psr->numTokens; i++) {
    if (psr->pos > psr->numTokens) {
      break;
    }
    switch (psr->tokens[i].lexeme) {
    case WORD:
      parseWrd(psr, &cmd);
      continue;
    case NUMBER:
      parseNum(psr, &cmd);
      break;
    case STRING:
      parseStr(psr, &cmd);
      break;
    default:
      printf("command not recognized");
    }
  }
  cmd.args[psr->pos++] = NULL;

  // PrintDebugPSR(psr, &cmd);
  execute(cmd.args[0], cmd.args);
}

// int match(ParserState *psr) {}

void parseWrd(ParserState *psr, Command *cmd) {
  cmd->args[psr->pos++] = psr->tokens[psr->pos].literal;
}

void parseStr(ParserState *psr, Command *cmd) {
  cmd->args[psr->pos++] = psr->tokens[psr->pos].literal;
}

void parseNum(ParserState *psr, Command *cmd) {
  cmd->args[psr->pos++] = psr->tokens[psr->pos].literal;
}
