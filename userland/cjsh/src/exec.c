#include "ast.h"
#include "core/memory.h"
#include <parser.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>
#include <builtins.h>
#include <exec.h>

volatile sig_atomic_t curr_ch_pid = -1;

int execute(ASTNode *node) {
  AstNodeType nodeType = node->type;
  char **argv = NULL;

  switch (nodeType) {
  // Simple Command
  case SIMPLE_CMD:
    return execSimpleCmd(&node->simpleCmd);
    break;

  // Assignment Command (Export)
  case ASSIGNMENT: {
    char *val = expandWord(node->assignment.value);

    if (node->assignment.export) {
      argv = cmalloc(3 * sizeof(char *));
      argv[0] = "expt";
      argv[1] = node->assignment.name;
      argv[2] = val;
      return expt(argv);
    }
    return setenv(node->assignment.name, val, 1);
    break;
  }

  // Pipeline Commands (ls | wc)
  case PIPELINE:;
    execPipelineCmd(&node->pipeline);
    break;
  }
  return 0;
}

int execSimpleCmd(SimpleCmd *node) {
  char **argv;
  pid_t pid;
  int wstatus;
  argv = cmalloc((node->numArgs + 1) * sizeof(char *));

  for (usize i = 0; i < node->numArgs; i++) {
    argv[i] = expandWord(node->args[i]);
  }

  argv[node->numArgs] = NULL;
  for (size_t i = 0; i < builtins_len; i++) {
    if (strcmp(builtins[i].name, argv[0]) == 0) {
      return builtins[i].fn(argv);
    }
  }

  pid = fork();
  if (pid == 0) {
    execvp(argv[0], argv);
    printf("cjsh: command not found\n");
    return 1;
  } else {
    curr_ch_pid = pid;
    waitpid(pid, &wstatus, WUNTRACED | WCONTINUED);
    curr_ch_pid = 0;
  }
  FREE(argv);
  return 0;
}

int execPipelineCmd(Pipeline *node) {
  char **argv;
  pid_t pid;
  int wstatus;
  pid_t pids[64];
  int pid_count = 0;
  int fds[2];
  int prev_rd_fd = 0;

  if (pipe(fds) == -1) {
    perror("pipe failed");
    exit(1);
  }

  usize i = 0;
  while (i < node->numCmds) {
    SimpleCmd *cmd = node->cmds[i];
    argv = cmalloc((cmd->numArgs + 1) * sizeof(char *));

    for (usize j = 0; j < cmd->numArgs; j++) {
      argv[j] = expandWord(cmd->args[j]);
      argv[cmd->numArgs] = NULL;
    }

    pid = fork();
    // Child Process
    if (pid == 0) {
      if (prev_rd_fd != 0) {
        dup2(prev_rd_fd, STDIN_FILENO);
        close(prev_rd_fd);
      }
      if (i < node->numCmds - 1) {
        dup2(fds[1], STDOUT_FILENO);
        close(fds[0]);
        close(fds[1]);
      }

      execvp(argv[0], argv);
      printf("cjsh: command not found\n");
      exit(1);
    } else {
      // Parent Process
      if (i < node->numCmds - 1) {
        close(fds[1]);
        prev_rd_fd = fds[0];
      } else {
        close(prev_rd_fd);
      }
    }

    cfree(argv);
    pids[pid_count++] = pid;
    i++;
  }

  for (int i = 0; i < pid_count; i++) {
    waitpid(pids[i], &wstatus, WUNTRACED | WCONTINUED);
  }
  return 0;
}
