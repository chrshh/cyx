#include "test.h"
#include <ast.h>
#include <core/memory.h>
#include <string.h>
#include <stdlib.h>

// expandWord: single literal word
static void test_expand_literal(void) {
  WordPart word = {.type = WP_LITERAL, .literal = "hello", .next = NULL};
  char *result = expandWord(&word);
  ASSERT_STR_EQ(result, "hello", "expand literal: hello");
  cfree(result);
}

// expandWord: single var lookup
static void test_expand_var(void) {
  setenv("TEST_VAR_CJSH", "expanded_value", 1);
  WordPart word = {.type = WP_VAR, .literal = "TEST_VAR_CJSH", .next = NULL};
  char *result = expandWord(&word);
  ASSERT_STR_EQ(result, "expanded_value", "expand var: TEST_VAR_CJSH");
  cfree(result);
  unsetenv("TEST_VAR_CJSH");
}

// expandWord: literal + var concatenation (simulates "hello $NAME")
static void test_expand_concat(void) {
  setenv("TEST_NAME", "world", 1);
  WordPart var_part = {.type = WP_VAR, .literal = "TEST_NAME", .next = NULL};
  WordPart lit_part = {.type = WP_LITERAL, .literal = "hello ", .next = &var_part};
  char *result = expandWord(&lit_part);
  ASSERT_STR_EQ(result, "hello world", "expand concat: hello + $TEST_NAME");
  cfree(result);
  unsetenv("TEST_NAME");
}

// expandWord: multiple vars chained
static void test_expand_multi_var(void) {
  setenv("TEST_A", "foo", 1);
  setenv("TEST_B", "bar", 1);
  WordPart b = {.type = WP_VAR, .literal = "TEST_B", .next = NULL};
  WordPart a = {.type = WP_VAR, .literal = "TEST_A", .next = &b};
  char *result = expandWord(&a);
  ASSERT_STR_EQ(result, "foobar", "expand multi var: $A$B");
  cfree(result);
  unsetenv("TEST_A");
  unsetenv("TEST_B");
}

// expandWord: undefined var returns NULL
static void test_expand_undefined_var(void) {
  unsetenv("SURELY_UNDEFINED_VAR_XYZ");
  WordPart word = {.type = WP_VAR, .literal = "SURELY_UNDEFINED_VAR_XYZ", .next = NULL};
  char *result = expandWord(&word);
  ASSERT(result == NULL, "expand undefined: returns NULL");
}

// expandWord: empty literal
static void test_expand_empty_literal(void) {
  WordPart word = {.type = WP_LITERAL, .literal = "", .next = NULL};
  char *result = expandWord(&word);
  ASSERT_STR_EQ(result, "", "expand empty literal: empty string");
  cfree(result);
}

// makePipelineCmd: creates correct node
static void test_make_pipeline(void) {
  SimpleCmd cmd1 = {.args = NULL, .numArgs = 0};
  SimpleCmd cmd2 = {.args = NULL, .numArgs = 0};
  SimpleCmd **cmds = cmalloc(2 * sizeof(SimpleCmd *));
  cmds[0] = &cmd1;
  cmds[1] = &cmd2;

  ASTNode *node = makePipelineCmd(cmds, 2);
  ASSERT(node != NULL, "make pipeline: not NULL");
  ASSERT_INT_EQ((int)node->type, PIPELINE, "make pipeline: type is PIPELINE");
  ASSERT_INT_EQ((int)node->pipeline.numCmds, 2, "make pipeline: 2 cmds");
  ASSERT(node->pipeline.cmds[0] == &cmd1, "make pipeline: first cmd pointer");
  ASSERT(node->pipeline.cmds[1] == &cmd2, "make pipeline: second cmd pointer");
  cfree(node);
}

int main(void) {
  RUN_SUITE("ast: expand literal", test_expand_literal);
  RUN_SUITE("ast: expand var", test_expand_var);
  RUN_SUITE("ast: expand concat", test_expand_concat);
  RUN_SUITE("ast: expand multi var", test_expand_multi_var);
  RUN_SUITE("ast: expand undefined var", test_expand_undefined_var);
  RUN_SUITE("ast: expand empty literal", test_expand_empty_literal);
  RUN_SUITE("ast: make pipeline", test_make_pipeline);
  REPORT();
}
