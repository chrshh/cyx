use std::io::Write;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{json, Value};

use crate::editor::Editor;

pub struct Diagnostic {
    pub line: i32,     /* 0-based buffer row */
    pub severity: i64, /* 1=error 2=warning 3=info 4=hint */
    pub message: String,
}

enum LspState {
    Initializing, /* initialize sent, waiting on the server's response */
    Ready,
}

pub struct LspClient {
    child: Child,
    stdin: ChildStdin,
    stdout: ChildStdout, /* non-blocking; polled from the read_key idle loop */
    recv_buf: Vec<u8>,
    state: LspState,
    uri: String,
    language_id: &'static str,
    version: i64,
    pending_open: Option<String>, /* buffer text queued until initialize completes */
    pub diagnostics: Vec<Diagnostic>,
}

fn uri_from_path(path: &Path) -> String {
    format!("file://{}", path.display())
}

/* walk up from the file's directory looking for a project marker so the
 * server indexes the right workspace; falls back to the file's directory */
fn find_root(file_dir: &Path, markers: &[&str]) -> PathBuf {
    let mut dir = file_dir.to_path_buf();
    loop {
        for m in markers {
            if dir.join(m).exists() {
                return dir;
            }
        }
        if !dir.pop() {
            return file_dir.to_path_buf();
        }
    }
}

impl LspClient {
    /* filetype comes from the HLDB entry that matched in set_syntax_highlight */
    pub fn spawn(filetype: &str, file_path: &Path) -> Result<LspClient, String> {
        let (server_bin, language_id, root_markers): (&str, &'static str, &[&str]) =
            match filetype {
                "c" => ("clangd", "c", &["compile_commands.json", ".clangd", ".git"]),
                "rust" => ("rust-analyzer", "rust", &["Cargo.toml"]),
                _ => return Err(format!("no lsp server for {}", filetype)),
            };

        let file_dir = file_path.parent().unwrap_or(Path::new("/"));
        let root = find_root(file_dir, root_markers);
        let root_uri = uri_from_path(&root);

        let mut child = Command::new(server_bin)
            .current_dir(&root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| format!("{} not found", server_bin))?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        /* the editor polls for server messages between keystrokes, so reads
         * must never block the input loop */
        unsafe {
            let fd = stdout.as_raw_fd();
            let fl = libc::fcntl(fd, libc::F_GETFL);
            libc::fcntl(fd, libc::F_SETFL, fl | libc::O_NONBLOCK);
        }

        let mut client = LspClient {
            child,
            stdin,
            stdout,
            recv_buf: Vec::new(),
            state: LspState::Initializing,
            uri: uri_from_path(file_path),
            language_id,
            version: 1,
            pending_open: None,
            diagnostics: Vec::new(),
        };

        client.send(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": std::process::id(),
                "rootUri": root_uri,
                "capabilities": {
                    "textDocument": {
                        "publishDiagnostics": {},
                        "synchronization": { "didSave": true }
                    },
                    "window": { "workDoneProgress": false }
                },
                "workspaceFolders": [{ "uri": root_uri, "name": "root" }]
            }
        }));

        Ok(client)
    }

    fn send(&mut self, msg: &Value) {
        let body = msg.to_string();
        let _ = write!(self.stdin, "Content-Length: {}\r\n\r\n{}", body.len(), body);
        let _ = self.stdin.flush();
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.send(&json!({ "jsonrpc": "2.0", "method": method, "params": params }));
    }

    fn respond(&mut self, id: Value, result: Value) {
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "result": result }));
    }

    pub fn did_open(&mut self, text: String) {
        match self.state {
            LspState::Ready => {
                let params = json!({
                    "textDocument": {
                        "uri": self.uri,
                        "languageId": self.language_id,
                        "version": self.version,
                        "text": text
                    }
                });
                self.notify("textDocument/didOpen", params);
            }
            /* server still initializing; open once the handshake finishes */
            LspState::Initializing => self.pending_open = Some(text),
        }
    }

    pub fn did_change(&mut self, text: String) {
        match self.state {
            LspState::Ready => {
                self.version += 1;
                let params = json!({
                    "textDocument": { "uri": self.uri, "version": self.version },
                    /* a change event without a range means "replace the whole
                     * document" -- the simplest sync mode that is always valid */
                    "contentChanges": [{ "text": text }]
                });
                self.notify("textDocument/didChange", params);
            }
            LspState::Initializing => self.pending_open = Some(text),
        }
    }

    pub fn did_save(&mut self) {
        if matches!(self.state, LspState::Ready) {
            let params = json!({ "textDocument": { "uri": self.uri } });
            self.notify("textDocument/didSave", params);
        }
    }

    /* drain and handle whatever the server has sent; returns true when the
     * diagnostics list changed and the screen should be redrawn */
    pub fn pump(&mut self) -> bool {
        loop {
            let mut chunk = [0u8; 4096];
            let n = unsafe {
                libc::read(self.stdout.as_raw_fd(), chunk.as_mut_ptr().cast(), chunk.len())
            };
            if n <= 0 {
                break; /* EAGAIN (nothing buffered) or server exited */
            }
            self.recv_buf.extend_from_slice(&chunk[..n as usize]);
        }

        let mut changed = false;
        while let Some(body) = take_message(&mut self.recv_buf) {
            let Ok(msg) = serde_json::from_slice::<Value>(&body) else {
                continue;
            };
            changed |= self.handle_message(msg);
        }
        changed
    }

    fn handle_message(&mut self, msg: Value) -> bool {
        /* server -> client request: must be answered or the server stalls */
        if let (Some(id), Some(method)) = (msg.get("id"), msg.get("method")) {
            let id = id.clone();
            let result = match method.as_str().unwrap_or("") {
                /* one entry per requested configuration item */
                "workspace/configuration" => {
                    let items = msg["params"]["items"].as_array().map_or(0, |a| a.len());
                    Value::Array(vec![Value::Null; items])
                }
                _ => Value::Null,
            };
            self.respond(id, result);
            return false;
        }

        /* notification */
        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
            if method == "textDocument/publishDiagnostics"
                && msg["params"]["uri"].as_str() == Some(&self.uri)
            {
                self.diagnostics = msg["params"]["diagnostics"]
                    .as_array()
                    .map(|diags| diags.iter().map(parse_diagnostic).collect())
                    .unwrap_or_default();
                return true;
            }
            return false;
        }

        /* response; the only one we track is initialize (id 1) */
        if msg.get("id").and_then(|i| i.as_i64()) == Some(1)
            && matches!(self.state, LspState::Initializing)
        {
            self.state = LspState::Ready;
            self.notify("initialized", json!({}));
            if let Some(text) = self.pending_open.take() {
                self.did_open(text);
            }
        }
        false
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn parse_diagnostic(d: &Value) -> Diagnostic {
    Diagnostic {
        line: d["range"]["start"]["line"].as_i64().unwrap_or(0) as i32,
        severity: d["severity"].as_i64().unwrap_or(1),
        message: d["message"].as_str().unwrap_or("").to_string(),
    }
}

/* split one "Content-Length: N\r\n...\r\n\r\n<body>" frame off the buffer */
fn take_message(buf: &mut Vec<u8>) -> Option<Vec<u8>> {
    let hdr_end = buf.windows(4).position(|w| w == b"\r\n\r\n")?;

    let len = std::str::from_utf8(&buf[..hdr_end]).ok().and_then(|hdrs| {
        hdrs.lines()
            .find_map(|l| l.strip_prefix("Content-Length:"))
            .and_then(|v| v.trim().parse::<usize>().ok())
    });
    let Some(len) = len else {
        buf.clear(); /* malformed header; drop it rather than loop forever */
        return None;
    };

    let body_start = hdr_end + 4;
    if buf.len() < body_start + len {
        return None; /* body not fully received yet */
    }
    let body = buf[body_start..body_start + len].to_vec();
    buf.drain(..body_start + len);
    Some(body)
}

impl Editor {
    /* (re)start the language server for the current file; called from open() */
    pub fn lsp_start(&mut self) {
        self.lsp = None; /* Drop kills any previous server */

        let Some(filetype) = self.syntax.map(|s| s.filetype) else {
            return;
        };
        let Some(filename) = self.buffer.filename.clone() else {
            return;
        };
        let Ok(abs) = std::fs::canonicalize(&filename) else {
            return;
        };

        match LspClient::spawn(filetype, &abs) {
            Ok(mut lsp) => {
                let text = String::from_utf8_lossy(&self.rows_to_string()).into_owned();
                lsp.did_open(text);
                self.lsp = Some(lsp);
            }
            Err(e) => self.set_status_msg(format!("lsp: {}", e)),
        }
    }

    /* push the full buffer to the server; runs once per refresh when the
     * buffer changed, alongside rehighlight() */
    pub fn lsp_sync(&mut self) {
        if self.lsp.is_none() {
            return;
        }
        let text = String::from_utf8_lossy(&self.rows_to_string()).into_owned();
        if let Some(lsp) = self.lsp.as_mut() {
            lsp.did_change(text);
        }
    }

    /* poll for server messages while read_key waits on input */
    pub fn lsp_idle(&mut self) {
        let changed = match self.lsp.as_mut() {
            Some(lsp) => lsp.pump(),
            None => return,
        };
        if changed {
            self.refresh_screen();
        }
    }
}
