#include <core/memory.h>
#include <ctype.h>
#include <lexer.h>
#include <stdio.h>
#include <stdlib.h>
#include <core/panic.h>
#include <string.h>
#include <common.h>
#include <glob.h>

LexerState initLexerState(size_t bufsz) {
  LexerState lxr = {.tokens = NULL,
                    .numTokens = 0,
                    .capcity = INIT_CAPACITY,
                    .source = (char *)cmalloc(bufsz),
                    .curr = 0,
                    .sourceLen = 0};
  return lxr;
}

void destroyLexerState(LexerState *lxr) {
  freeTokens(lxr);
  lxr->capcity = 0;
  lxr->source = NULL;
  lxr->curr = 0;
  lxr->sourceLen = 0;
  lxr->numTokens = 0;
}

void freeTokens(LexerState *lxr) {
  if (lxr->tokens == NULL) {
    return;
  }
  for (size_t i = 0; i < lxr->numTokens; i++) {
    cfree(lxr->tokens[i].literal);
  }

  cfree(lxr->tokens);
  lxr->tokens = NULL;
  lxr->numTokens = 0;
}

int isEnd(LexerState *lxr) {
  if (lxr->curr >= lxr->sourceLen) {
    return 1;
  }
  return 0;
}

// returns curr char but does not increment/decrement
char peek(LexerState *lxr) {
  if (isEnd(lxr)) {
    return '\0';
  }
  return lxr->source[lxr->curr];
}

char peekNextChar(LexerState *lxr) {
  if (lxr->curr + 1 >= lxr->sourceLen) {
    return '\0';
  }
  return (lxr->source[lxr->curr + 1]);
}

char peekPrevious(LexerState *lxr) {
  if (lxr->curr == 0) {
    return '\0';
  }
  return lxr->source[lxr->curr - 1];
}

// returns next char in the string
char advance(LexerState *lxr) { return lxr->source[lxr->curr++]; }

void addToken(LexerState *lxr, Lexeme type, char *literal, double val) {
  if (lxr->numTokens == lxr->capcity) {
    lxr->capcity *= 2;
    lxr->tokens = (Token *)realloc(lxr->tokens, sizeof(Token) * lxr->capcity);
  }

  Token token;
  token.lexeme = type;
  token.literal = literal ? strdup(literal) : NULL;
  token.val = val;
  token.pos = lxr->start;

  lxr->tokens[lxr->numTokens++] = token;
}

// scan through full word, build array of tokens
void scanner(LexerState *lxr) {
  lxr->tokens = (Token *)cmalloc(sizeof(Token) * lxr->capcity);

  while (!isEnd(lxr)) {
    lxr->start = lxr->curr;
    scanToken(lxr);
  }
  // PrintDebugTokensLXR(lxr);
}

void string(LexerState *lxr) {
  while (!isEnd(lxr)) {
    if (peek(lxr) == '"' && peekPrevious(lxr) != '\\') {
      break;
    }
    advance(lxr);
  }

  if (isEnd(lxr)) {
    PrintShWarning("unterminated string");
    return;
  }

  advance(lxr);

  // -2 accounts for quotations / escape chars
  size_t len = lxr->curr - lxr->start - 2;
  char *val = (char *)cmalloc(len + 1);
  size_t j = 0;

  for (size_t i = lxr->start + 1; i < lxr->curr - 1; i++) {
    if (lxr->source[i] == '\\' && lxr->source[i + 1] == '"') {
      continue; // skipping escape character
    }
    val[j++] = lxr->source[i];
  }
  val[j++] = '\0';

  addToken(lxr, STRING, val, 0.0);
}

void word(LexerState *lxr) {
  while (isAlphaNum(peek(lxr)) || isShChar(peek(lxr))) {
    advance(lxr);
  }

  char *rawval = strndup(lxr->source + lxr->start, lxr->curr - lxr->start);

  if (strchr(rawval, '*') || strchr(rawval, '?')) {
    glob_t glb;
    memset(&glb, 0, sizeof(glob));
    int val = glob(rawval, GLOB_TILDE, NULL, &glb);
    if (val != 0) {
      panic("error while globbing");
    }

    for (size_t i = 0; i < glb.gl_pathc; i++) {
      char *glbval = glb.gl_pathv[i];
      addToken(lxr, WORD, glbval, 0.0);
      if (i > 0) {
        lxr->tokens[lxr->numTokens - 1].pos =
            lxr->tokens[lxr->numTokens - 1].pos + strlen(glbval) + 1;
      }
    }
    globfree(&glb);
  } else {
    addToken(lxr, WORD, rawval, 0.0);
  }
  free(rawval);
}

void number(LexerState *lxr) {
  while (isdigit(peek(lxr))) {
    advance(lxr);
  }

  if (peek(lxr) == '.' && isdigit(peek(lxr))) {
    advance(lxr);

    while (isdigit(peek(lxr))) {
      advance(lxr);
    }
  }

  char numsubstr[lxr->curr - lxr->start + 1];
  strncpy(numsubstr, &lxr->source[lxr->start], lxr->curr - lxr->start);
  numsubstr[lxr->curr - lxr->start] = '\0';
  double val = strtod(numsubstr, NULL);
  addToken(lxr, NUMBER, numsubstr, val);
}

void scanToken(LexerState *lxr) {
  char c = advance(lxr);

  switch (c) {
  case '<':
    addToken(lxr, LESS, NULL, 0);
    break;
  case '>':
    if (peek(lxr) == '>') {
      advance(lxr);
      addToken(lxr, GREATERGREATER, NULL, 0);
    } else {
      addToken(lxr, GREATER, NULL, 0);
    }
    break;
  case ';':
    addToken(lxr, SEMICOLON, NULL, 0);
    break;
  case '$':
    addToken(lxr, DOLLAR, "$", 0);
    break;
  case '=':
    addToken(lxr, EQUALS, "=", 0);
    break;
  case '|':
    addToken(lxr, PIPE, "|", 0);
    break;
  case '\\':
    addToken(lxr, BACKSLASH, NULL, 0);
    break;
  case '/':
  case ':':
  case ' ':
  case '\r':
  case '\t':
  case '\n':
    break;

  case '"':
    string(lxr);
    break;

  default:
    if (isdigit(c)) {
      number(lxr);
    } else if (isAlphaNum(c) || isShChar(c)) {
      word(lxr);
    } else {
      printf("unknown command\n");
      break;
    }
  }
}

const char *lexemeToString(Lexeme type) {
  switch (type) {
  case NUMBER:
    return "NUMBER";
  case STRING:
    return "STRING";
  case PIPE:
    return "|";
  case LESS:
    return "<";
  case GREATER:
    return ">";
  case GREATERGREATER:
    return ">>";
  case BACKSLASH:
    return "\\";
  case DOLLAR:
    return "$";
  case EQUALS:
    return "=";
  default:
    return "UNKNOWN";
  }
}
