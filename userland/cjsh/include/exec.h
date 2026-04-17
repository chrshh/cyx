#include <parser.h>
#include <signal.h>

int execute(Command *cmd);

volatile sig_atomic_t curr_ch_pid;
