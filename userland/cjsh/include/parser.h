#ifndef CJSH_PARSE_H
#define CJSH_PARSE_H

#include "ast.h"
#include "core/types.h"
#include "lexer.h"
#include <stddef.h>
typedef struct ParserState ParserState;

struct ParserState {
  Token *tokens;
  usize numTokens;
  usize pos;
};

ParserState initParserState(usize numTokens);
void destroyParserState(ParserState *psr);

void parse(ParserState *psr);
Token *peekNext(ParserState *psr);
/**
 * This function decides between parseStatement OR parseSimpleCmd by
 * looking at wether token[0] == WORD && token[1] == '='
 */
ASTNode *parseStatement(ParserState *psr);
ASTNode *parseSimpleCmd(ParserState *psr);
ASTNode *parseAssignment(ParserState *psr, char *name, int exported);
ASTNode *parsePipelineCmd(ParserState *psr);

/**
 * This function builds the list of WordParts*
 * Each time a '$' token is reached, WordPartType = WP_VAR, otherwise it is always a LITERAL
 */
WordPart *parseWrd(ParserState *psr);

#endif
