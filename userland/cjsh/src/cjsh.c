#include "common.h"
#include "core/memory.h"
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
size_t lxrbufsz = BUFFER_SIZE;

int main(void) {
  signal(SIGINT, fatal_error_signal);
  using_history();
  setenv("PATH", "usr/bin/", 0);

  int interactive = isatty(STDIN_FILENO);

  while (1) {
    LexerState lxr = initLexerState(lxrbufsz);
    if (lxr.source == NULL) {
      panic("allocation failed");
    }

    if (sigsetjmp(prompt_jmp, 1)) {
      // crtl c jumped here, immediately clean up and reset
      destroyLexerState(&lxr);
      continue;
    }

    char *input;
    if (interactive) {
      String shPrompt = GetShPrompt();
      printf(BOLD CYAN "%s " RESET, shPrompt.chars);
      input = readline(BOLD RED "~ " RESET);
      FreeShPrompt(shPrompt);
    } else {
      char *line = NULL;
      size_t len = 0;
      if (getline(&line, &len, stdin) == -1) {
        cfree(line);
        break;
      }
      line[strcspn(line, "\n")] = '\0';
      input = line;
    }

    if (!input)
      break;
    if (!(StrEmpty(input))) {
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
  }
  return EXIT_SUCCESS;
}
