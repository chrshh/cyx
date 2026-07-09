use tree_sitter::{Language, Parser, Query, QueryCursor, StreamingIterator};

use crate::consts::*;
use crate::editor::Editor;

pub const HL_NORMAL: u8 = 0;
pub const HL_NUMBER: u8 = 1;
pub const HL_STRING: u8 = 2;
pub const HL_COMMENT: u8 = 3;
pub const HL_KEYWORD1: u8 = 5;
pub const HL_KEYWORD2: u8 = 6;
pub const HL_OPERATOR: u8 = 7;
pub const HL_MATCH: u8 = 8;

pub struct EditorSyntax {
    pub filetype: &'static str,
    pub filematch: &'static [&'static str],
}

pub static HLDB: &[EditorSyntax] = &[
    EditorSyntax {
        filetype: "c",
        filematch: &[".c", ".h", ".cpp"],
    },
    EditorSyntax {
        filetype: "rust",
        filematch: &[".rs"],
    },
];

pub fn syntax_to_color(hl: u8) -> &'static str {
    match hl {
        HL_NUMBER => BLUE,
        HL_STRING => GREEN,
        HL_COMMENT => COMMENT,
        HL_KEYWORD1 => KEYWORD1,
        HL_KEYWORD2 => KEYWORD2,
        HL_OPERATOR => OPERATOR,
        HL_MATCH => MATCH,
        _ => DEF_COLOR,
    }
}

/* map a highlight-query capture name onto the editor's palette; query names
 * are dotted (e.g. "keyword.repeat"), so match on the leading word */
fn hl_for_capture(name: &str) -> u8 {
    let base = name.split('.').next().unwrap_or(name);
    match base {
        "comment" => HL_COMMENT,
        "string" | "character" | "escape" => HL_STRING,
        "number" | "constant" => HL_NUMBER,
        "keyword" | "repeat" | "conditional" | "include" | "preproc" | "storageclass" => {
            HL_KEYWORD1
        }
        "type" => HL_KEYWORD2,
        "operator" => HL_OPERATOR,
        _ => HL_NORMAL,
    }
}

pub struct TsHighlighter {
    parser: Parser,
    query: Query,
    capture_hl: Vec<u8>, /* capture index -> HL_* code */
}

impl TsHighlighter {
    pub fn new(filetype: &str) -> Option<TsHighlighter> {
        let (language, highlight_query): (Language, &str) = match filetype {
            "c" => (tree_sitter_c::LANGUAGE.into(), tree_sitter_c::HIGHLIGHT_QUERY),
            "rust" => (
                tree_sitter_rust::LANGUAGE.into(),
                tree_sitter_rust::HIGHLIGHTS_QUERY,
            ),
            _ => return None,
        };

        let mut parser = Parser::new();
        parser.set_language(&language).ok()?;
        let query = Query::new(&language, highlight_query).ok()?;
        let capture_hl = query
            .capture_names()
            .iter()
            .map(|name| hl_for_capture(name))
            .collect();

        Some(TsHighlighter {
            parser,
            query,
            capture_hl,
        })
    }
}

impl Editor {
    /* re-parse the whole buffer and repaint every row's hl array; runs once
     * per refresh when edits have set hl_dirty, not on every keystroke path */
    pub fn rehighlight(&mut self) {
        for row in &mut self.buffer.rows {
            row.hl = vec![HL_NORMAL; row.render.len()];
        }

        let Some(ts) = self.ts.as_mut() else {
            return;
        };

        /* flatten rows into the single source document tree-sitter expects;
         * row y starts at line y, so node Points map straight onto rows */
        let mut src: Vec<u8> = Vec::new();
        for row in &self.buffer.rows {
            src.extend_from_slice(&row.chars);
            src.push(b'\n');
        }

        let Some(tree) = ts.parser.parse(&src, None) else {
            return;
        };

        let mut cursor = QueryCursor::new();
        let mut captures = cursor.captures(&ts.query, tree.root_node(), src.as_slice());
        while let Some((mat, cap_idx)) = captures.next() {
            let cap = mat.captures[*cap_idx];
            let hl = ts.capture_hl[cap.index as usize];
            if hl == HL_NORMAL {
                continue;
            }

            /* paint the node's span; columns are byte offsets into chars, so
             * convert to render columns to account for tab expansion */
            let start = cap.node.start_position();
            let end = cap.node.end_position();
            for y in start.row..=end.row {
                let Some(row) = self.buffer.rows.get_mut(y) else {
                    break;
                };
                let cs = if y == start.row { start.column } else { 0 };
                let ce = if y == end.row {
                    end.column.min(row.chars.len())
                } else {
                    row.chars.len()
                };
                if cs >= ce {
                    continue;
                }
                let rs = row.x_to_rx(cs as i32) as usize;
                let re = (row.x_to_rx(ce as i32) as usize).min(row.hl.len());
                if rs < re {
                    row.hl[rs..re].fill(hl);
                }
            }
        }
    }

    pub fn set_syntax_highlight(&mut self) {
        self.syntax = None;
        self.ts = None;
        self.hl_dirty = true;

        let Some(filename) = self.buffer.filename.clone() else {
            return;
        };

        let ext = filename.rfind('.').map(|i| &filename[i..]);

        for syntax in HLDB {
            for &fm in syntax.filematch {
                let is_ext = fm.starts_with('.');
                if (is_ext && ext == Some(fm)) || (!is_ext && filename.contains(fm)) {
                    self.syntax = Some(syntax);
                    self.ts = TsHighlighter::new(syntax.filetype);
                    return;
                }
            }
        }
    }
}
