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
  Command cmd;
  cmd.args = cmalloc((psr->numTokens + 1) * (sizeof(char *)));

  Token arg1 = psr->tokens[0];
  cmd.cmd = arg1.literal;
  cmd.args[0] = arg1.literal;
  psr->pos++;

  for (size_t i = 1; i < psr->numTokens; i++) {
    if (psr->pos >= psr->numTokens) {
      break;
    }
    switch (psr->tokens[i].lexeme) {
    case WORD:
      parseWrd(psr, &cmd);
      continue;
    // case NUMBER:
    //   parseNum(psr);
    //   break;
    // case STRING:
    //   parseStr(psr, cmd);
    //   break;
    default:
      printf("command not recognized");
    }
  }
  cmd.args[psr->pos + 1] = NULL;

  execute(cmd.args[0], cmd.args);
}

// int match(ParserState *psr) {}

void parseWrd(ParserState *psr, Command *cmd) {
  cmd->args[psr->pos] = psr->tokens[psr->pos].literal;
}
