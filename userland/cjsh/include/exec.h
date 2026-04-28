#include <parser.h>
#include <signal.h>

int execute(ASTNode *node);

extern volatile sig_atomic_t curr_ch_pid;
