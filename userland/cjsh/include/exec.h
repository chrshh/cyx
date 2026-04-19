#include <parser.h>
#include <signal.h>

int execute(Command *cmd);

extern volatile sig_atomic_t curr_ch_pid;
