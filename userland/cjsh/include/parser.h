#ifndef CJSH_PARSE_H_
#define CJSH_PARSE_H_

#include "lexer.h"
#include <stddef.h>
typedef struct Command Command;
typedef struct ParserState ParserState;

struct Command {
  char *cmd;
  char **args;
  size_t numArgs;
  Command *next; // for pipes
  char *outFile;
  char *inFile;
  char *appendFile;
  char *errFile;
};

struct ParserState {
  Token *tokens;
  size_t numTokens;
  size_t pos;
};

ParserState initParserState(size_t numTokens);
void destroyParserState(ParserState *psr);

void parse(ParserState *psr);
// int match(ParserState *psr);

void parseStr(ParserState *psr, Command *cmd);
void parseNum(ParserState *psr, Command *cmd);
void parseWrd(ParserState *psr, Command *cmd);

#endif
