#include "ast.h"
#include "core/memory.h"
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

int execute(ASTNode *node) {
  AstNodeType nodeType = node->type;
  char **argv;
  pid_t pid;
  int wstatus;

  pid_t pids[64];
  int pid_count = 0;

  switch (nodeType) {
  case SIMPLE_CMD:
    argv = cmalloc((node->simpleCmd.numArgs + 1) * sizeof(char *));

    for (usize i = 0; i < node->simpleCmd.numArgs; i++) {
      argv[i] = expandWord(node->simpleCmd.args[i]);
    }

    argv[node->simpleCmd.numArgs] = NULL;
    for (size_t i = 0; i < builtins_len; i++) {
      if (strcmp(builtins[i].name, argv[0]) == 0) {
        return builtins[i].fn(argv);
      }
    }

    pid = fork();
    if (pid == 0) {
      execvp(argv[0], argv);
      printf("cjsh: command not found\n");
      exit(1);
    } else {
      curr_ch_pid = pid;
      waitpid(pid, &wstatus, WUNTRACED | WCONTINUED);
      curr_ch_pid = 0;
    }
    FREE(argv);
    return 0;

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
  }
  case PIPELINE:;
    int fds[2];
    int prev_rd_fd = 0;

    if (pipe(fds) == -1) {
      perror("pipe failed");
      exit(1);
    }

    usize i = 0;
    while (i < node->pipeline.numCmds) {
      SimpleCmd *cmd = node->pipeline.cmds[i];
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
        if (i < node->pipeline.numCmds - 1) {
          dup2(fds[1], STDOUT_FILENO);
          close(fds[0]);
          close(fds[1]);
        }

        execvp(argv[0], argv);
        printf("cjsh: command not found\n");
        exit(1);
      } else {
        // Parent Process
        if (i < node->pipeline.numCmds - 1) {
          close(fds[1]);
          prev_rd_fd = fds[0];
        } else {
          close(prev_rd_fd);
        }
      }

      // free(argv);
      pids[pid_count++] = pid;
      i++;
    }

    for (int i = 0; i < pid_count; i++) {
      waitpid(pids[i], &wstatus, WUNTRACED | WCONTINUED);
    }
    break;
  }
  // PIPE CHAIN
  //   while (cmd != NULL) {
  //     if (cmd->next != NULL) {
  //       if (pipe(fds) == -1) {
  //         perror("pipe failed");
  //         exit(1);
  //       }
  //     }
  //     pid = fork();
  //     // Child
  //     if (pid == 0) {
  //       if (prev_rd_fd != 0) {
  //         dup2(prev_rd_fd, STDIN_FILENO);
  //         close(prev_rd_fd);
  //       }
  //       if (cmd->next != NULL) {
  //         dup2(fds[1], STDOUT_FILENO);
  //         close(fds[1]);
  //         close(fds[0]);
  //       }
  //       execvp(cmd->args[0], cmd->args);
  //       printf("cjsh: command not found\n");
  //       exit(1);
  //     } else {
  //       if (cmd->next != NULL) {
  //         close(fds[1]);
  //         prev_rd_fd = fds[0];
  //       } else {
  //         close(prev_rd_fd);
  //       }
  //       pids[pid_count++] = pid;
  //       cmd = cmd->next;
  //     }
  //   }
  //
  //   for (int i = 0; i < pid_count; i++) {
  //     waitpid(pids[i], &wstatus, WUNTRACED | WCONTINUED);
  //   }
  //   return 0;
  // }
  // }
  return 0;
}
