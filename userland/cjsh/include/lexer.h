#ifndef LEXER_H_
#define LEXER_H_

#define BUFFER_SIZE 256
#define INIT_LXR_CAPACITY 8

#include <stddef.h>

typedef enum {
  STRING,
  NUMBER,
  LESS,
  GREATER,
  GREATER_GREATER,
  SEMICOLON,
  BACKSLASH,
  PIPE,
  DOLLAR,
  EQUALS,
  WORD,
  AND_AND,
  OR_OR,
  AMPERSAND,
} Lexeme;

typedef struct {
  Lexeme lexeme;
  char *literal;
  double val;
  int pos;
} Token;

typedef struct {
  Token *tokens;
  size_t numTokens;
  size_t capcity;
  char *source;
  size_t curr;
  size_t sourceLen;
  size_t start;
} LexerState;

LexerState initLexerState(size_t bufferSize);
void destroyLexerState(LexerState *lxr);
void freeTokens(LexerState *lxr);

void scanner(LexerState *lxr);
char advance(LexerState *lxr);
char peek(LexerState *lxr);
char peekNextChar(LexerState *lxr);
void scanToken(LexerState *lxr);

void addToken(LexerState *lxr, Lexeme type, char *literal, double val);
int isEnd(LexerState *lxr);

void string(LexerState *lxr);
void number(LexerState *lxr);
void word(LexerState *lxr);

const char *lexemeToString(Lexeme type);

#endif
