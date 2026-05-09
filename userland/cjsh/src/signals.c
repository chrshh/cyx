#include <signal.h>
#include <unistd.h>
#include <setjmp.h>
#include <sys/reboot.h>
#include <exec.h>

extern sigjmp_buf prompt_jmp;

void fatal_error_signal(int sig) {
  if (curr_ch_pid > 0) {
    kill(curr_ch_pid, sig);
    return;
  }

  /*
   * TEMPORARY DEV KILL SWITCH
   * Ctrl+C at an idle prompt powers off the guest so we can get back to
   * the host fast. reboot(RB_POWER_OFF) works whether cjsh is PID 1 or
   * running under cinit (cinit would otherwise respawn us). _exit is the
   * fallback if reboot returns EPERM (non-root, shouldn't happen yet).
   * Replace with proper SIGINT handling once interactive editing matters.
   */
  (void)sig;
  reboot(RB_POWER_OFF);
  _exit(1);
}
