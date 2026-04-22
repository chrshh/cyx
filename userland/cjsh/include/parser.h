#ifndef CJSH_PARSE_H
#define CJSH_PARSE_H

#include "lexer.h"
#include <stddef.h>
typedef struct Command Command;
typedef struct ParserState ParserState;

typedef enum {
  SIMPLE,
  ASSIGNMENT,
  PIPELINE,
} CmdType;

struct Command {
  char *cmd;
  char **args;
  size_t numArgs;
  Command *next; // for pipes
  CmdType cmdType;
  int isEnv; // for env vars
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
Token *peekNext(ParserState *psr);

Command *parseStr(ParserState *psr, Command *cmd);
Command *parseNum(ParserState *psr, Command *cmd);
Command *parseWrd(ParserState *psr, Command *cmd);
Command *parsePipe(ParserState *psr, Command *cmd);
Command *parseEq(ParserState *psr, Command *cmd);
Command *parseVar(ParserState *psr, Command *cmd);
char *expandVar(char *rawval);
char *expandVarInStr(char *rawval);
char *concatVar(char *rawval, char *expandedVar);

#endif
