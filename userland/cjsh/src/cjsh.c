#include "common.h"
#include <parser.h>
#include <stdio.h>
#include <stdlib.h>
#include <readline/history.h>
#include <readline/readline.h>
#include <core/ansi.h>
#include <core/panic.h>
#include <lexer.h>
#include <string.h>
#include <signals.h>
#include <signal.h>
#include <setjmp.h>

sigjmp_buf prompt_jmp;
size_t bufsz = BUFFER_SIZE;

int main() {
  signal(SIGINT, fatal_error_signal);
  using_history();
  setenv("PATH", "usr/bin/", 0);

  while (1) {
    LexerState lxr = initLexerState(bufsz);
    if (lxr.source == NULL) {
      panic("allocation failed");
    }

    if (sigsetjmp(prompt_jmp, 1)) {
      // crtl c jumped here, immediately clean up and reset
      destroyLexerState(&lxr);
      continue;
    }

    String shPrompt = GetShPrompt();

    printf(CYAN "%s " RESET, shPrompt.chars);
    char *input = readline(RED "~ " RESET);
    if (!input)
      break;
    if (!(IsEmpty(input))) {
      add_history(input);
      lxr.sourceLen = strlen(input);
      lxr.source = input;
      scanner(&lxr);
      ParserState psr = initParserState(lxr.numTokens);
      psr.tokens = lxr.tokens;
      parse(&psr);
      destroyParserState(&psr);
    }
    destroyLexerState(&lxr);
    FreeShPrompt(shPrompt);
  }
  return EXIT_SUCCESS;
}
