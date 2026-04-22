#ifndef CJSH_AST_H
#define CJSH_AST_H

#include "core/types.h"
#include <stdbool.h>

typedef enum {
  WP_LITERAL,
  WP_VAR
} WordPartType;

typedef struct WordPart {
  WordPartType type;
  char *literal;
  struct WordPart *next;
} WordPart;

typedef enum {
  SIMPLE_CMD,
  ASSIGNMENT,
  PIPELINE
} AstNodeType;

typedef struct {
  WordPart **args;
  usize numArgs;
  char *inFile;
  char *outFile;
  char *appendFile;
  bool background;
} SimpleCmd;

typedef struct {
  char *name;      // left side of =
  WordPart *value; // right side of =
  bool export;     // was export keyword used
} Assignment;

typedef struct {
  SimpleCmd **cmds;
  usize numCmds;
} Pipeline;

typedef struct {
  AstNodeType type;
  union {
    SimpleCmd simpleCmd;
    Assignment assignment;
    Pipeline pipeline;
  };
} ASTNode;

WordPart *makeWordPart(WordPart kind, char *val);
ASTNode *makeSimpleCmd(void);
ASTNode *makeAssignmentCmd(char *name, WordPart *val, bool export);
ASTNode *makePipelineCmd(SimpleCmd **cmds, usize numCmds);

void freeAstNodes(ASTNode node);
void freeWordParts(WordPart *words);

/**
 * @brief This is the function that exec.c calls
 * It walks the word list, calls getenv on WP_VAR, concats all values, and returns a malloc'd string
 * All words go through this function
 */
char *expandWord(WordPart *words);

#endif
