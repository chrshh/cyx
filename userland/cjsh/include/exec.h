#include <parser.h>
#include <signal.h>

int execute(ASTNode *node);
int execSimpleCmd(SimpleCmd *node);
int execPipelineCmd(Pipeline *node);

extern volatile sig_atomic_t curr_ch_pid;
