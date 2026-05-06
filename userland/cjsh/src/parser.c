#include <ctype.h>
#include <str/string.h>
#include <parser.h>
#include <core/memory.h>
#include <core/panic.h>
#include <stdio.h>
#include <exec.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <core/types.h>

// PrintDebugPSR(node);

ParserState initParserState(size_t numTokens) {
  ParserState psr;
  psr.numTokens = numTokens;
  psr.tokens = NULL;
  psr.pos = 0;
  return psr;
}

void destroyParserState(ParserState *psr) {
  psr->numTokens = 0;
  psr->pos = 0;
}

void parse(ParserState *psr) {
  if (psr->numTokens == 0) return;

  while (psr->pos < psr->numTokens) {
    ASTNode *node = parseStatement(psr);
    if (node == NULL) return;
    execute(node);
    freeAstNodes(*node);
  }
}

Token *peekNextToken(ParserState *psr) {
  if (psr->pos + 1 >= psr->numTokens) {
    return NULL;
  }
  return &psr->tokens[psr->pos + 1];
}

// The entry point — looks at token[0] and token[1] to decide:
// if token[0] is WORD and token[1] is EQUALS -> parseAssignment
// otherwise -> parseSimpleCmd
ASTNode *parseStatement(ParserState *psr) {
  if (psr->pos >= psr->numTokens) return NULL;

  Token currToken = psr->tokens[psr->pos];

  if (strcmp(currToken.literal, "export") == 0 || strcmp(currToken.literal, "expt") == 0 || strcmp(currToken.literal, "local") == 0 || strcmp(currToken.literal, "readonly") == 0) {
    if (peekNextToken(psr) == NULL) {
      printf("NOTHING TO EXPORT");
      return NULL;
    }
    char *name = peekNextToken(psr)->literal;
    psr->pos++;
    while (psr->tokens[psr->pos].lexeme != EQUALS && psr->pos < psr->numTokens) {
      psr->pos++;
    }
    psr->pos++;
    int exported = 1;
    return parseAssignment(psr, name, exported);
    return NULL;
  }

  if (currToken.lexeme == WORD && peekNextToken(psr) != NULL) {
    if (peekNextToken(psr)->lexeme == EQUALS) {
      char *name = currToken.literal;
      psr->pos++;
      psr->pos++;
      int exported = 0;
      return parseAssignment(psr, name, exported);
    }
  }

  return parsePipelineCmd(psr);
}

// Looks at current token, consumes tokens until it hits a boundary
// (pipe, equals at statement level, end of input), returns the WordPart list
WordPart *parseWrd(ParserState *psr) {
  Token currToken = psr->tokens[psr->pos];
  WordPart *word = cmalloc(sizeof(WordPart));
  word->next = NULL;

  if (currToken.lexeme == DOLLAR) {
    if (peekNextToken(psr) != NULL) {
      word->literal = peekNextToken(psr)->literal;
      word->type = WP_VAR;
      psr->pos++;
    } else {
      printf("EMTPY AFTER $");
      return NULL;
    }
  } else if (currToken.lexeme == EQUALS) {
    word->literal = "=";
    word->type = WP_LITERAL;
  } else if (currToken.lexeme == STRING) {
    WordPart *head = NULL;
    WordPart *tail = NULL;
    char *str = currToken.literal;
    int i = 0;
    int start = 0;

    while (str[i] != '\0') {
      if (str[i] == '$') {
        // emit literal chunk before the $
        if (i > start) {
          WordPart *lit = cmalloc(sizeof(WordPart));
          lit->type = WP_LITERAL;
          lit->literal = strndup(str + start, (usize)(i - start));
          lit->next = NULL;
          if (head == NULL) head = lit;
          else
            tail->next = lit;
          tail = lit;
        }
        i++; // skip the $
        start = i;
        // consume var name: letters, digits, underscore
        while (isalnum(str[i]) || str[i] == '_') i++;
        // emit var chunk
        WordPart *var = cmalloc(sizeof(WordPart));
        var->type = WP_VAR;
        var->literal = strndup(str + start, (usize)(i - start));
        var->next = NULL;
        if (head == NULL) head = var;
        else
          tail->next = var;
        tail = var;
        start = i;
      } else {
        i++;
      }
    }
    // emit any remaining literal after the last var
    if (i > start) {
      WordPart *lit = cmalloc(sizeof(WordPart));
      lit->type = WP_LITERAL;
      lit->literal = strndup(str + start, (usize)(i - start));
      lit->next = NULL;
      if (head == NULL) head = lit;
      else
        tail->next = lit;
      tail = lit;
    }

    cfree(word);
    psr->pos++;
    return head;
  } else if (currToken.lexeme == WORD) {
    word->literal = currToken.literal;
    word->type = WP_LITERAL;
  }
  psr->pos++;
  return word;
}

ASTNode *parsePipelineCmd(ParserState *psr) {
  SimpleCmd **cmds = cmalloc(256 * sizeof(SimpleCmd *));
  usize count = 0;

  ASTNode *node = parseSimpleCmd(psr);
  cmds[count++] = &node->simpleCmd;

  while (psr->tokens[psr->pos].lexeme == PIPE) {
    psr->pos++;
    node = parseSimpleCmd(psr);
    cmds[count++] = &node->simpleCmd;
  }

  if (count == 1) return node;

  return makePipelineCmd(cmds, count);
}

// Called when we know we have a command, consumes the command name
// then calls parseWord() in a loop for each argument until end of input or pipe
ASTNode *parseSimpleCmd(ParserState *psr) {
  if (psr->pos >= psr->numTokens) return NULL;
  ASTNode *node = cmalloc(sizeof(ASTNode));
  SimpleCmd cmd;
  cmd.numArgs = 0;
  cmd.args = cmalloc(psr->numTokens * sizeof(WordPart *));

  while (psr->pos < psr->numTokens) {
    if (psr->tokens[psr->pos].lexeme == PIPE || psr->tokens[psr->pos].lexeme == SEMICOLON) break;
    WordPart *word = parseWrd(psr);
    if (word == NULL) {
      return NULL;
    }

    // Merge adjacent tokens (no whitespace gap) into the same arg
    while (psr->pos < psr->numTokens &&
           psr->tokens[psr->pos].lexeme != PIPE &&
           psr->tokens[psr->pos].lexeme != SEMICOLON) {
      Token *prev = &psr->tokens[psr->pos - 1];
      Token *curr = &psr->tokens[psr->pos];
      // Compute where the previous token ends in the source
      int prevEnd = prev->pos;
      if (prev->lexeme == STRING) prevEnd += (int)strlen(prev->literal) + 2;
      else if (prev->literal)
        prevEnd += (int)strlen(prev->literal);
      else
        prevEnd += 1;
      // If there's a gap, these are separate args
      if (curr->pos != prevEnd) break;

      WordPart *next = parseWrd(psr);
      if (next == NULL) break;

      // Append to tail of current word chain
      WordPart *tail = word;
      while (tail->next != NULL) tail = tail->next;
      tail->next = next;
    }

    cmd.args[cmd.numArgs] = word;
    cmd.numArgs++;
  }

  node->type = SIMPLE_CMD;
  node->simpleCmd.args = cmd.args;
  node->simpleCmd.numArgs = cmd.numArgs;
  return node;
}

// Called when we know we have NAME=value — name is passed in because
// parseStatement already consumed it to figure out which branch to take
ASTNode *parseAssignment(ParserState *psr, char *name, int exported) {
  WordPart *word = cmalloc(sizeof(WordPart));
  word->next = NULL;

  word = parseWrd(psr);
  if (word == NULL) {
    printf("NULL FROM ASSIGNMENT");
    return NULL;
  }

  ASTNode *node = cmalloc(sizeof(ASTNode));
  node->type = ASSIGNMENT;
  node->assignment.name = name;
  node->assignment.value = word;
  node->assignment.export = exported;
  return node;
}
