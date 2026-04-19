#pragma once

#include "lexer.h"
#include "parser.h"
#include <str/strarray.h>
#ifndef CJSH_COMMON_H
#define CJSH_COMMON_H

int isAlphaNum(char c);

/**
 * @brief  Creates full shell prompt (<cwd> ~)
 * @note   This needs to be recalculated everytime a builtin cmd is executed
 * @return Fully qualified shell prompt
 */
String GetShPrompt();
void FreeShPrompt(String str);
int isShChar(char c);

void PrintShWarning(char *msg);
void PrintDebugTokensLXR(LexerState *lxr);
void PrintDebugPSR(Command *cmd);

#endif
