use crate::consts::*;
use crate::editor::Editor;

pub const HL_NORMAL: u8 = 0;
pub const HL_NUMBER: u8 = 1;
pub const HL_STRING: u8 = 2;
pub const HL_COMMENT: u8 = 3;
pub const HL_MLCOMMENT: u8 = 4;
pub const HL_KEYWORD1: u8 = 5;
pub const HL_KEYWORD2: u8 = 6;
pub const HL_OPERATOR: u8 = 7;
pub const HL_MATCH: u8 = 8;

/* highlighting rules */
pub const HL_HIGHLIGHT_NUMBERS: i32 = 1 << 0;
pub const HL_HIGHLIGHT_STRINGS: i32 = 1 << 1;

pub struct EditorSyntax {
    pub filetype: &'static str,
    pub filematch: &'static [&'static str],
    pub keywords: &'static [&'static str],
    #[allow(dead_code)] // carried over from the C struct; is_operator hardcodes the set
    pub operators: &'static str,
    pub singleline_comment_start: &'static str,
    pub multiline_comment_start: &'static str,
    pub multiline_comment_end: &'static str,
    pub flags: i32,
}

static C_HL_EXTENSIONS: &[&str] = &[".c", ".h", ".cpp"];

static C_HL_KEYWORDS: &[&str] = &[
    "switch", "if", "while", "for", "break",
    "continue", "return", "else", "struct", "union",
    "typedef", "static", "enum", "class", "case",
    "int|", "long|", "double|", "float|", "char|",
    "unsigned|", "signed|", "void|", "#include",
];

pub static HLDB: &[EditorSyntax] = &[EditorSyntax {
    filetype: "c",
    filematch: C_HL_EXTENSIONS,
    keywords: C_HL_KEYWORDS,
    operators: "=+&><-*",
    singleline_comment_start: "//",
    multiline_comment_start: "/*",
    multiline_comment_end: "*/",
    flags: HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS,
}];

pub fn is_separator(c: u8) -> bool {
    b" \t\n\x0b\x0c\r".contains(&c) || c == 0 || b",.()+-/*=~%<>[];".contains(&c)
}

pub fn is_operator(c: u8) -> bool {
    b"=+&><-*".contains(&c)
}

pub fn syntax_to_color(hl: u8) -> &'static str {
    match hl {
        HL_NUMBER => BLUE,
        HL_STRING => GREEN,
        HL_COMMENT => COMMENT,
        HL_MLCOMMENT => MLCOMMENT,
        HL_KEYWORD1 => KEYWORD1,
        HL_KEYWORD2 => KEYWORD2,
        HL_OPERATOR => OPERATOR,
        HL_MATCH => MATCH,
        _ => DEF_COLOR,
    }
}

impl Editor {
    pub fn update_syntax(&mut self, idx: i32) {
        let rsize = self.buffer.rows[idx as usize].rsize() as usize;
        let mut hl = vec![HL_NORMAL; rsize];

        let Some(syntax) = self.syntax else {
            self.buffer.rows[idx as usize].hl = hl;
            return;
        };

        let keywords = syntax.keywords;

        let scs = syntax.singleline_comment_start.as_bytes();
        let mcs = syntax.multiline_comment_start.as_bytes();
        let mce = syntax.multiline_comment_end.as_bytes();

        let mut prev_sep = true;
        let mut in_str: u8 = 0;
        let mut in_comment =
            idx > 0 && self.buffer.rows[idx as usize - 1].hl_open_comment;

        let render = self.buffer.rows[idx as usize].render.clone();
        /* the C keyword-boundary check reads render[i + klen], which can be the
         * terminating NUL one past the end; emulate that with 0 */
        let at = |i: usize| -> u8 {
            if i < render.len() { render[i] } else { 0 }
        };

        let mut i = 0usize;
        while i < rsize {
            let c = render[i];
            let prev_hl = if i > 0 { hl[i - 1] } else { HL_NORMAL };

            /* single line comment highlighting */
            if !scs.is_empty() && in_str == 0 && !in_comment && render[i..].starts_with(scs) {
                hl[i..].fill(HL_COMMENT);
                break;
            }

            /* multiline comment highlighting */
            if !mcs.is_empty() && !mce.is_empty() && in_str == 0 {
                if in_comment {
                    hl[i] = HL_MLCOMMENT;
                    if render[i..].starts_with(mce) {
                        hl[i..i + mce.len()].fill(HL_MLCOMMENT);
                        i += mce.len();
                        in_comment = false;
                        prev_sep = true;
                        continue;
                    } else {
                        i += 1;
                        continue;
                    }
                } else if render[i..].starts_with(mcs) {
                    hl[i..i + mcs.len()].fill(HL_MLCOMMENT);
                    i += mcs.len();
                    in_comment = true;
                    continue;
                }
            }

            /* string highlighting */
            if syntax.flags & HL_HIGHLIGHT_STRINGS != 0 {
                if in_str != 0 {
                    hl[i] = HL_STRING;
                    if c == b'\\' && i + 1 < rsize {
                        hl[i + 1] = HL_STRING;
                        i += 2;
                        continue;
                    }
                    if c == in_str {
                        in_str = 0;
                    }
                    i += 1;
                    prev_sep = true;
                    continue;
                } else if c == b'"' || c == b'\'' {
                    in_str = c;
                    hl[i] = HL_STRING;
                    i += 1;
                    continue;
                }
            }

            /* number highlighting */
            if syntax.flags & HL_HIGHLIGHT_NUMBERS != 0
                && ((c.is_ascii_digit() && (prev_sep || prev_hl == HL_NUMBER))
                    || (c == b'.' && prev_hl == HL_NUMBER))
            {
                hl[i] = HL_NUMBER;
                i += 1;
                prev_sep = false;
                continue;
            }

            /* keyword highlighting */
            if prev_sep {
                let mut matched = false;
                for kw in keywords {
                    let kwb = kw.as_bytes();
                    let kw2 = kwb[kwb.len() - 1] == b'|';
                    let klen = if kw2 { kwb.len() - 1 } else { kwb.len() };

                    if render[i..].starts_with(&kwb[..klen]) && is_separator(at(i + klen)) {
                        hl[i..i + klen].fill(if kw2 { HL_KEYWORD2 } else { HL_KEYWORD1 });
                        i += klen;
                        matched = true;
                        break;
                    }
                }
                if matched {
                    prev_sep = false;
                    continue;
                }
            }

            /* operator highlighting */
            if is_operator(c) {
                hl[i] = HL_OPERATOR;
                i += 1;
                continue;
            }

            prev_sep = is_separator(c);
            i += 1;
        }

        let row = &mut self.buffer.rows[idx as usize];
        let changed = row.hl_open_comment != in_comment;
        row.hl = hl;
        row.hl_open_comment = in_comment;
        if changed && idx + 1 < self.buffer.num_rows {
            self.update_syntax(idx + 1);
        }
    }

    pub fn set_syntax_highlight(&mut self) {
        self.syntax = None;
        let Some(filename) = self.buffer.filename.clone() else {
            return;
        };

        let ext = filename.find('.').map(|i| &filename[i..]);

        for syntax in HLDB {
            for &fm in syntax.filematch {
                let is_ext = fm.starts_with('.');
                if (is_ext && ext == Some(fm)) || (!is_ext && filename.contains(fm)) {
                    self.syntax = Some(syntax);

                    for filerow in 0..self.buffer.num_rows {
                        self.update_syntax(filerow);
                    }
                    return;
                }
            }
        }
    }
}
