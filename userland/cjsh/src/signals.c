#include <signal.h>
#include <unistd.h>
#include <setjmp.h>
#include <exec.h>

extern sigjmp_buf prompt_jmp;

void fatal_error_signal(int sig) {
  if (curr_ch_pid > 0) {
    kill(curr_ch_pid, sig);
  } else {
    // No child running, reset prompt
    write(STDOUT_FILENO, "\r", 1);
    siglongjmp(prompt_jmp, 1);
  }
}
