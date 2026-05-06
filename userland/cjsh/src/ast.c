#include "ast.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <core/memory.h>

usize bufsz = 512;

// Initialize empty buffer
// walk through each word based on type
// either expand var or add just as-is
// return fully qualified string
char *expandWord(WordPart *words) {
  char *res = cmalloc(bufsz);
  usize offset = 0;

  // loop through list of words
  while (words != NULL) {

    // word is ENV VAR
    if (words->type == WP_VAR) {
      char *env = getenv(words->literal);
      if (env == NULL) {
        printf("FAILED TO GET ENV: %s", words->literal);
        return NULL;
      }
      if (bufsz <= offset + strlen(env)) {
        res = crealloc(res, bufsz * 2);
      }
      memcpy(res + offset, env, strlen(env));
      offset += strlen(env);

      // word is regular text
    } else {
      if (bufsz <= offset + strlen(words->literal)) {
        res = crealloc(res, bufsz * 2);
      }
      memcpy(res + offset, words->literal, strlen(words->literal));
      offset += strlen(words->literal);
    }

    words = words->next;
  }

  res[offset] = '\0';
  return res;
}

ASTNode *makePipelineCmd(SimpleCmd **cmds, usize numCmds) {
  ASTNode *pipelineCmd = cmalloc(sizeof(ASTNode));

  pipelineCmd->type = PIPELINE;
  pipelineCmd->pipeline.cmds = cmds;
  pipelineCmd->pipeline.numCmds = numCmds;
  return pipelineCmd;
}

void freeAstNodes(ASTNode node) {
  usize idx = 0;
  switch (node.type) {
  case SIMPLE_CMD:
    while (node.simpleCmd.args != NULL && idx < node.simpleCmd.numArgs) {
      freeWordParts(node.simpleCmd.args[idx]);
      idx++;
    }
    break;
  case ASSIGNMENT:
    break;
  case PIPELINE:;
    usize numCmds = node.pipeline.numCmds;
    usize i = 0;
    usize argIdx = 0;
    while (i < numCmds - 1) {
      while (node.pipeline.cmds[i]->args != NULL) {
        freeWordParts(node.pipeline.cmds[i]->args[argIdx]);
        node.pipeline.cmds[i]->args = NULL;
        argIdx++;
      }
      i++;
    }
    break;
  }
}

void freeWordParts(WordPart *words) {
  while (words != NULL) {
    WordPart *save = words;
    WordPart *next = words->next;
    FREE(save);
    words = next;
  }
}
