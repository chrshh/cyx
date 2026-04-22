#include <parser.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <core/panic.h>
#include <sys/wait.h>
#include <builtins.h>
#include <exec.h>

volatile sig_atomic_t curr_ch_pid = -1;

int execute(Command *cmd) {
  pid_t pid;
  int wstatus;
  int fds[2];
  int prev_rd_fd = 0;
  pid_t pids[64];
  int pid_count = 0;

  // SINGLE CMD
  // cmd->isEnv != 1 ADD THiS BACK IN AFTER TESTING
  if (cmd->next == NULL) {
    // Check Built in first
    for (size_t i = 0; i < builtins_len; i++) {
      if (strcmp(builtins[i].name, cmd->args[0]) == 0) {
        builtins[i].fn(cmd->args);
        return 0;
      }
    }

    pid = fork();

    // Child process returns 0: Child calls execvp
    if (pid == 0) {
      execvp(cmd->args[0], cmd->args);
      printf("cjsh: command not found\n");
      exit(1);
    } else {
      // Tell signal handler about the child
      curr_ch_pid = pid;
      waitpid(pid, &wstatus, WUNTRACED | WCONTINUED);
      curr_ch_pid = 0;
    }
    return 0;
  }

  // Handle ENV assignments
  // if (cmd->next == NULL && cmd->isEnv == 1) {
  //   return expt(cmd->args);
  // }

  // PIPE CHAIN
  while (cmd != NULL) {
    if (cmd->next != NULL) {
      if (pipe(fds) == -1) {
        perror("pipe failed");
        exit(1);
      }
    }
    pid = fork();
    // Child
    if (pid == 0) {
      if (prev_rd_fd != 0) {
        dup2(prev_rd_fd, STDIN_FILENO);
        close(prev_rd_fd);
      }
      if (cmd->next != NULL) {
        dup2(fds[1], STDOUT_FILENO);
        close(fds[1]);
        close(fds[0]);
      }
      execvp(cmd->args[0], cmd->args);
      printf("cjsh: command not found\n");
      exit(1);
    } else {
      if (cmd->next != NULL) {
        close(fds[1]);
        prev_rd_fd = fds[0];
      } else {
        close(prev_rd_fd);
      }
      pids[pid_count++] = pid;
      cmd = cmd->next;
    }
  }

  for (int i = 0; i < pid_count; i++) {
    waitpid(pids[i], &wstatus, WUNTRACED | WCONTINUED);
  }
  return 0;
}
