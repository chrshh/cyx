#include <core/ansi.h>
#include <readline/history.h>
#include <readline/readline.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  rl_bind_key('\t', rl_complete);
  using_history();

  // REPL LOOP
  while (1) {

    // 1) get line
    char *input = readline(GREEN "~ " RESET);
    // 2) get tokens gettok()
    if (!input)
      break;
    add_history(input);
    //  -> later swap this to lexing->parsing

    printf("fart\n");
    // 3) Exec

    free(input);
  }
  return EXIT_SUCCESS;
}
