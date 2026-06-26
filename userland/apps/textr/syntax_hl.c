#include <ctype.h>
#include <stdlib.h>
#include <string.h>

#include "textr.h"

char *C_HL_extensions[] = { ".c", ".h", ".cpp", NULL };

char *C_HL_keywords[] = { "switch",    "if",      "while",   "for",      "break",
                          "continue",  "return",  "else",    "struct",   "union",
                          "typedef",   "static",  "enum",    "class",    "case",
                          "int|",      "long|",   "double|", "float|",   "char|",
                          "unsigned|", "signed|", "void|",   "#include", NULL };

EditorSyntax HLDB[] = {
    { "c", C_HL_extensions, C_HL_keywords, "=+&><-*", "//", "/*", "*/",
      HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS },
};

void editorUpdateSyntax(Row *row) {
    row->hl = realloc(row->hl, row->rsize);
    memset(row->hl, HL_NORMAL, row->rsize);

    if (E.syntax == NULL) return;

    char **keywords = E.syntax->keywords;

    char *scs = E.syntax->singleline_comment_start;
    char *mcs = E.syntax->multiline_comment_start;
    char *mce = E.syntax->multiline_comment_end;

    int scs_len = scs ? strlen(scs) : 0;
    int mcs_len = mcs ? strlen(mcs) : 0;
    int mce_len = mce ? strlen(mce) : 0;

    int prev_sep   = 1;
    int in_str     = 0;
    int in_comment = (row->idx > 0 && E.buffer.rows[row->idx - 1].hl_open_comment);

    int i = 0;
    while (i < row->rsize) {
        char          c       = row->render[i];
        unsigned char prev_hl = (i > 0) ? row->hl[i - 1] : HL_NORMAL;

        /* single line comment highlighting */
        if (scs_len && !in_str && !in_comment) {
            if (!strncmp(&row->render[i], scs, scs_len)) {
                memset(&row->hl[i], HL_COMMENT, row->rsize - i);
                break;
            }
        }

        /* multiline comment highlighting */
        if (mcs_len && mce_len && !in_str) {
            if (in_comment) {
                row->hl[i] = HL_MLCOMMENT;
                if (!strncmp(&row->render[i], mce, mce_len)) {
                    memset(&row->hl[i], HL_MLCOMMENT, mce_len);
                    i += mce_len;
                    in_comment = 0;
                    prev_sep   = 1;
                    continue;
                } else {
                    i++;
                    continue;
                }
            } else if (!strncmp(&row->render[i], mcs, mcs_len)) {
                memset(&row->hl[i], HL_MLCOMMENT, mcs_len);
                i += mcs_len;
                in_comment = 1;
                continue;
            }
        }

        /* string highlighting */
        if (E.syntax->flags & HL_HIGHLIGHT_STRINGS) {
            if (in_str) {
                row->hl[i] = HL_STRING;
                if (c == '\\' && i + 1 < row->rsize) {
                    row->hl[i + 1] = HL_STRING;
                    i += 2;
                    continue;
                }
                if (c == in_str) in_str = 0;
                i++;
                prev_sep = 1;
                continue;
            } else {
                if (c == '"' || c == '\'') {
                    in_str     = c;
                    row->hl[i] = HL_STRING;
                    i++;
                    continue;
                }
            }
        }

        /* number highlighting */
        if (E.syntax->flags & HL_HIGHLIGHT_NUMBERS) {
            if ((isdigit(c) && (prev_sep || prev_hl == HL_NUMBER)) ||
                (c == '.' && prev_hl == HL_NUMBER)) {
                row->hl[i] = HL_NUMBER;
                i++;
                prev_sep = 0;
                continue;
            }
        }

        /* keyword highlighting */
        if (prev_sep) {
            int j;
            for (j = 0; keywords[j]; j++) {
                int klen = strlen(keywords[j]);
                int kw2  = keywords[j][klen - 1] == '|';
                if (kw2) klen--;

                if (!strncmp(&row->render[i], keywords[j], klen) &&
                    is_separator(row->render[i + klen])) {
                    memset(&row->hl[i], kw2 ? HL_KEYWORD2 : HL_KEYWORD1, klen);
                    i += klen;
                    break;
                }
            }
            if (keywords[j] != NULL) {
                prev_sep = 0;
                continue;
            }
        }

        /* operator highlighting */
        if (is_operator(c)) {
            row->hl[i] = HL_OPERATOR;
            i++;
            continue;
        }

        prev_sep = is_separator(c);
        i++;
    }

    int changed          = (row->hl_open_comment != in_comment);
    row->hl_open_comment = in_comment;
    if (changed && row->idx + 1 < E.buffer.num_rows) editorUpdateSyntax(&E.buffer.rows[row->idx + 1]);
}

char *editorSyntaxToColor(int hl) {
    switch (hl) {
    case HL_NUMBER: return BLUE;
    case HL_STRING: return GREEN;
    case HL_COMMENT: return COMMENT;
    case HL_MLCOMMENT: return MLCOMMENT;
    case HL_KEYWORD1: return KEYWORD1;
    case HL_KEYWORD2: return KEYWORD2;
    case HL_OPERATOR: return OPERATOR;
    case HL_MATCH: return MATCH;
    default: return DEF_COLOR;
    }
}

int is_separator(int c) {
    return isspace(c) || c == '\0' || strchr(",.()+-/*=~%<>[];", c) != NULL;
}

int is_operator(int c) {
    return strchr("=+&><-*", c) != NULL;
}

void editorSetSyntaxHighlight(void) {
    E.syntax = NULL;
    if (E.buffer.filename == NULL) return;

    char *ext = strchr(E.buffer.filename, '.');

    for (unsigned int j = 0; j < HLDB_ENTRIES; j++) {
        EditorSyntax *syntax = &HLDB[j];
        unsigned int  i      = 0;
        while (syntax->filematch[i]) {
            int is_ext = (syntax->filematch[i][0] == '.');
            if ((is_ext && ext && !strcmp(ext, syntax->filematch[i])) ||
                (!is_ext && strstr(E.buffer.filename, syntax->filematch[i]))) {
                E.syntax = syntax;

                int filerow;
                for (filerow = 0; filerow < E.buffer.num_rows; filerow++) {
                    editorUpdateSyntax(&E.buffer.rows[filerow]);
                }
                return;
            }
            i++;
        }
    }
}
