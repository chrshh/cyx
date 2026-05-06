#include "test.h"
#include <lexer.h>
#include <core/memory.h>
#include <string.h>
#include <stdlib.h>

// Helper: lex a string and return the LexerState with tokens populated
static LexerState lex(const char *input) {
  LexerState lxr = initLexerState(256);
  lxr.source = strdup(input);
  lxr.sourceLen = strlen(input);
  scanner(&lxr);
  return lxr;
}

// Single word produces one WORD token
static void test_single_word(void) {
  LexerState lxr = lex("echo");
  ASSERT_INT_EQ((int)lxr.numTokens, 1, "single word: token count");
  ASSERT(lxr.tokens[0].lexeme == WORD, "single word: lexeme is WORD");
  ASSERT_STR_EQ(lxr.tokens[0].literal, "echo", "single word: literal");
  destroyLexerState(&lxr);
}

// Multiple words separated by spaces
static void test_multiple_words(void) {
  LexerState lxr = lex("echo hello world");
  ASSERT_INT_EQ((int)lxr.numTokens, 3, "multi word: token count");
  ASSERT_STR_EQ(lxr.tokens[0].literal, "echo", "multi word: first");
  ASSERT_STR_EQ(lxr.tokens[1].literal, "hello", "multi word: second");
  ASSERT_STR_EQ(lxr.tokens[2].literal, "world", "multi word: third");
  destroyLexerState(&lxr);
}

// Pipe token is recognized between words
static void test_pipe_token(void) {
  LexerState lxr = lex("ls | wc");
  ASSERT_INT_EQ((int)lxr.numTokens, 3, "pipe: token count");
  ASSERT(lxr.tokens[0].lexeme == WORD, "pipe: first is WORD");
  ASSERT(lxr.tokens[1].lexeme == PIPE, "pipe: middle is PIPE");
  ASSERT(lxr.tokens[2].lexeme == WORD, "pipe: last is WORD");
  destroyLexerState(&lxr);
}

// Dollar sign produces DOLLAR token
static void test_dollar_token(void) {
  LexerState lxr = lex("$HOME");
  ASSERT_INT_EQ((int)lxr.numTokens, 2, "dollar: token count");
  ASSERT(lxr.tokens[0].lexeme == DOLLAR, "dollar: first is DOLLAR");
  ASSERT(lxr.tokens[1].lexeme == WORD, "dollar: second is WORD");
  ASSERT_STR_EQ(lxr.tokens[1].literal, "HOME", "dollar: var name");
  destroyLexerState(&lxr);
}

// Equals token
static void test_equals_token(void) {
  LexerState lxr = lex("FOO=bar");
  ASSERT_INT_EQ((int)lxr.numTokens, 3, "equals: token count");
  ASSERT(lxr.tokens[0].lexeme == WORD, "equals: first is WORD");
  ASSERT(lxr.tokens[1].lexeme == EQUALS, "equals: middle is EQUALS");
  ASSERT(lxr.tokens[2].lexeme == WORD, "equals: last is WORD");
  destroyLexerState(&lxr);
}

// Quoted string produces STRING token
static void test_string_token(void) {
  LexerState lxr = lex("echo \"hello world\"");
  ASSERT_INT_EQ((int)lxr.numTokens, 2, "string: token count");
  ASSERT(lxr.tokens[0].lexeme == WORD, "string: first is WORD");
  ASSERT(lxr.tokens[1].lexeme == STRING, "string: second is STRING");
  ASSERT_STR_EQ(lxr.tokens[1].literal, "hello world", "string: literal");
  destroyLexerState(&lxr);
}

// Semicolons produce SEMICOLON token
static void test_semicolon_token(void) {
  LexerState lxr = lex("echo hi; echo bye");
  ASSERT(lxr.numTokens == 5, "semicolon: token count is 5");
  ASSERT(lxr.tokens[2].lexeme == SEMICOLON, "semicolon: middle is SEMICOLON");
  destroyLexerState(&lxr);
}

// Redirection tokens
static void test_redirect_tokens(void) {
  LexerState lxr = lex("< >");
  ASSERT_INT_EQ((int)lxr.numTokens, 2, "redirect: token count");
  ASSERT(lxr.tokens[0].lexeme == LESS, "redirect: < is LESS");
  ASSERT(lxr.tokens[1].lexeme == GREATER, "redirect: > is GREATER");
  destroyLexerState(&lxr);
}

// Append token >>
static void test_append_token(void) {
  LexerState lxr = lex(">>");
  ASSERT_INT_EQ((int)lxr.numTokens, 1, "append: token count");
  ASSERT(lxr.tokens[0].lexeme == GREATER_GREATER, "append: >> is GREATER_GREATER");
  destroyLexerState(&lxr);
}

// Token position tracking
static void test_token_positions(void) {
  LexerState lxr = lex("echo hello");
  ASSERT_INT_EQ(lxr.tokens[0].pos, 0, "position: echo at 0");
  ASSERT_INT_EQ(lxr.tokens[1].pos, 5, "position: hello at 5");
  destroyLexerState(&lxr);
}

// Multi-pipe pipeline: "cat file | grep foo | wc"
// Tokens: cat(0) file(1) |(2) grep(3) foo(4) |(5) wc(6)
static void test_multi_pipe(void) {
  LexerState lxr = lex("cat file | grep foo | wc");
  ASSERT(lxr.tokens[2].lexeme == PIPE, "multi-pipe: first pipe");
  ASSERT(lxr.tokens[5].lexeme == PIPE, "multi-pipe: second pipe");
  ASSERT_INT_EQ((int)lxr.numTokens, 7, "multi-pipe: 7 tokens total");
  destroyLexerState(&lxr);
}

int main(void) {
  RUN_SUITE("lexer: single word", test_single_word);
  RUN_SUITE("lexer: multiple words", test_multiple_words);
  RUN_SUITE("lexer: pipe token", test_pipe_token);
  RUN_SUITE("lexer: dollar token", test_dollar_token);
  RUN_SUITE("lexer: equals token", test_equals_token);
  RUN_SUITE("lexer: string token", test_string_token);
  RUN_SUITE("lexer: semicolon token", test_semicolon_token);
  RUN_SUITE("lexer: redirect tokens", test_redirect_tokens);
  RUN_SUITE("lexer: append token", test_append_token);
  RUN_SUITE("lexer: token positions", test_token_positions);
  RUN_SUITE("lexer: multi pipe", test_multi_pipe);
  REPORT();
}
