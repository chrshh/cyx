#pragma once

#include "lexer.h"
#include "parser.h"
#ifndef CJSH_COMMON_H
#define CJSH_COMMON_H

int isAlphaNum(char c);
int isShChar(char c);
void PrintDebugTokensLXR(LexerState *lxr);
void PrintDebugPSR(ParserState *psr, Command *cmd);

#endif
