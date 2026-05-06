#include "test.h"
#include <lexer.h>
#include <parser.h>
#include <ast.h>
#include <core/memory.h>
#include <string.h>
#include <stdlib.h>

// Helper: lex + parse a single statement, return the ASTNode
static ASTNode *parse_one(const char *input) {
  LexerState lxr = initLexerState(256);
  lxr.source = strdup(input);
  lxr.sourceLen = strlen(input);
  scanner(&lxr);

  ParserState psr = initParserState(lxr.numTokens);
  psr.tokens = lxr.tokens;
  ASTNode *node = parseStatement(&psr);
  return node;
}

// Simple command: "echo hello" -> SIMPLE_CMD with 2 args
static void test_simple_cmd(void) {
  ASTNode *node = parse_one("echo hello");
  ASSERT(node != NULL, "simple cmd: node not NULL");
  ASSERT_INT_EQ((int)node->type, SIMPLE_CMD, "simple cmd: type is SIMPLE_CMD");
  ASSERT_INT_EQ((int)node->simpleCmd.numArgs, 2, "simple cmd: 2 args");

  // First arg should be "echo"
  ASSERT(node->simpleCmd.args[0] != NULL, "simple cmd: arg[0] exists");
  ASSERT(node->simpleCmd.args[0]->type == WP_LITERAL, "simple cmd: arg[0] is literal");
  ASSERT_STR_EQ(node->simpleCmd.args[0]->literal, "echo", "simple cmd: arg[0] value");

  // Second arg should be "hello"
  ASSERT(node->simpleCmd.args[1]->type == WP_LITERAL, "simple cmd: arg[1] is literal");
  ASSERT_STR_EQ(node->simpleCmd.args[1]->literal, "hello", "simple cmd: arg[1] value");
}

// Single command with no pipe still returns SIMPLE_CMD (not PIPELINE)
static void test_no_pipe_is_simple(void) {
  ASTNode *node = parse_one("ls");
  ASSERT(node != NULL, "no pipe: node not NULL");
  ASSERT_INT_EQ((int)node->type, SIMPLE_CMD, "no pipe: type is SIMPLE_CMD");
  ASSERT_INT_EQ((int)node->simpleCmd.numArgs, 1, "no pipe: 1 arg");
}

// Pipeline: "echo hello | cat" -> PIPELINE with 2 commands
static void test_pipeline(void) {
  ASTNode *node = parse_one("echo hello | cat");
  ASSERT(node != NULL, "pipeline: node not NULL");
  ASSERT_INT_EQ((int)node->type, PIPELINE, "pipeline: type is PIPELINE");
  ASSERT_INT_EQ((int)node->pipeline.numCmds, 2, "pipeline: 2 commands");

  // First command: echo hello (2 args)
  SimpleCmd *first = node->pipeline.cmds[0];
  ASSERT_INT_EQ((int)first->numArgs, 2, "pipeline: first cmd has 2 args");
  ASSERT_STR_EQ(first->args[0]->literal, "echo", "pipeline: first cmd is echo");

  // Second command: cat (1 arg)
  SimpleCmd *second = node->pipeline.cmds[1];
  ASSERT_INT_EQ((int)second->numArgs, 1, "pipeline: second cmd has 1 arg");
  ASSERT_STR_EQ(second->args[0]->literal, "cat", "pipeline: second cmd is cat");
}

// Triple pipeline: "a | b | c" -> PIPELINE with 3 commands
static void test_triple_pipeline(void) {
  ASTNode *node = parse_one("cat file | grep foo | wc");
  ASSERT(node != NULL, "triple pipe: node not NULL");
  ASSERT_INT_EQ((int)node->type, PIPELINE, "triple pipe: type is PIPELINE");
  ASSERT_INT_EQ((int)node->pipeline.numCmds, 3, "triple pipe: 3 commands");
}

// Assignment: "FOO=bar" -> ASSIGNMENT node
static void test_assignment(void) {
  ASTNode *node = parse_one("FOO=bar");
  ASSERT(node != NULL, "assignment: node not NULL");
  ASSERT_INT_EQ((int)node->type, ASSIGNMENT, "assignment: type is ASSIGNMENT");
  ASSERT_STR_EQ(node->assignment.name, "FOO", "assignment: name is FOO");
  ASSERT(node->assignment.value != NULL, "assignment: value not NULL");
  ASSERT(node->assignment.export == 0, "assignment: not exported");
}

// Export assignment: "export FOO=bar" -> ASSIGNMENT with export flag
static void test_export_assignment(void) {
  ASTNode *node = parse_one("export FOO=bar");
  ASSERT(node != NULL, "export: node not NULL");
  ASSERT_INT_EQ((int)node->type, ASSIGNMENT, "export: type is ASSIGNMENT");
  ASSERT_STR_EQ(node->assignment.name, "FOO", "export: name is FOO");
  ASSERT(node->assignment.export == 1, "export: exported flag set");
}

// Variable reference: "$HOME" -> SIMPLE_CMD with WP_VAR arg
static void test_var_reference(void) {
  ASTNode *node = parse_one("echo $HOME");
  ASSERT(node != NULL, "var ref: node not NULL");
  ASSERT_INT_EQ((int)node->type, SIMPLE_CMD, "var ref: type is SIMPLE_CMD");
  ASSERT_INT_EQ((int)node->simpleCmd.numArgs, 2, "var ref: 2 args");

  WordPart *var_arg = node->simpleCmd.args[1];
  ASSERT(var_arg->type == WP_VAR, "var ref: arg[1] is WP_VAR");
  ASSERT_STR_EQ(var_arg->literal, "HOME", "var ref: var name is HOME");
}

// String with interpolation: echo "hello $USER" -> STRING arg with WP_LITERAL + WP_VAR
static void test_string_interpolation(void) {
  ASTNode *node = parse_one("echo \"hello $USER\"");
  ASSERT(node != NULL, "interp: node not NULL");
  ASSERT_INT_EQ((int)node->type, SIMPLE_CMD, "interp: type is SIMPLE_CMD");
  ASSERT_INT_EQ((int)node->simpleCmd.numArgs, 2, "interp: 2 args");

  WordPart *str_arg = node->simpleCmd.args[1];
  ASSERT(str_arg != NULL, "interp: string arg exists");
  ASSERT(str_arg->type == WP_LITERAL, "interp: first part is literal");
  ASSERT_STR_EQ(str_arg->literal, "hello ", "interp: literal is 'hello '");

  ASSERT(str_arg->next != NULL, "interp: has second part");
  ASSERT(str_arg->next->type == WP_VAR, "interp: second part is var");
  ASSERT_STR_EQ(str_arg->next->literal, "USER", "interp: var is USER");
}

// Many arguments: "echo a b c d" -> 5 args
static void test_many_args(void) {
  ASTNode *node = parse_one("echo a b c d");
  ASSERT(node != NULL, "many args: node not NULL");
  ASSERT_INT_EQ((int)node->simpleCmd.numArgs, 5, "many args: 5 args");
}

int main(void) {
  RUN_SUITE("parser: simple command", test_simple_cmd);
  RUN_SUITE("parser: no pipe is simple", test_no_pipe_is_simple);
  RUN_SUITE("parser: pipeline", test_pipeline);
  RUN_SUITE("parser: triple pipeline", test_triple_pipeline);
  RUN_SUITE("parser: assignment", test_assignment);
  RUN_SUITE("parser: export assignment", test_export_assignment);
  RUN_SUITE("parser: var reference", test_var_reference);
  RUN_SUITE("parser: string interpolation", test_string_interpolation);
  RUN_SUITE("parser: many arguments", test_many_args);
  REPORT();
}
