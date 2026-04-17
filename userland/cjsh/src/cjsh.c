#include <parser.h>
#include <stdio.h>
#include <stdlib.h>
#include <readline/history.h>
#include <readline/readline.h>
#include <core/ansi.h>
#include <core/panic.h>
#include <str/string.h>
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

    char *input = readline(GREEN "~ " RESET);
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
  }
  return EXIT_SUCCESS;
}
