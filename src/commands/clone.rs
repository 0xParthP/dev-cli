//! Clone command implementation.

use anyhow::{Result, anyhow};
use std::{
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};

use crate::cli::CloneArgs;
use crate::config::Config;

/// Helper to extract repository directory name from a Git URL or file path.
pub fn extract_repo_name(url: &str) -> String {
    let trimmed = url.trim().trim_end_matches('/').trim_end_matches('\\');
    let last_segment = trimmed.rsplit(['/', '\\', ':']).next().unwrap_or("repository");

    let name = last_segment.strip_suffix(".git").unwrap_or(last_segment);
    if name.is_empty() { "repository".to_string() } else { name.to_string() }
}

/// Core git clone helper function.
pub fn clone_repository(
    url: &str,
    target_root: &Path,
    custom_name: Option<&str>,
    inherit_stdio: bool,
) -> Result<PathBuf> {
    if !target_root.exists() {
        std::fs::create_dir_all(target_root)?;
    }

    let repo_name = match custom_name {
        Some(name) if !name.trim().is_empty() => name.trim().to_string(),
        _ => extract_repo_name(url),
    };

    let target_path = target_root.join(&repo_name);

    let mut cmd = Command::new("git");
    cmd.arg("clone");
    cmd.arg(url);
    cmd.arg(&target_path);

    if inherit_stdio {
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        cmd.stdin(Stdio::inherit());
        let status = cmd.status().map_err(|e| anyhow!("Failed to spawn git process: {}", e))?;
        if status.success() {
            Ok(target_path)
        } else {
            Err(anyhow!("git clone failed with exit code: {}", status))
        }
    } else {
        cmd.env("GIT_TERMINAL_PROMPT", "0");
        let output = cmd.output().map_err(|e| anyhow!("Failed to spawn git process: {}", e))?;
        if output.status.success() {
            Ok(target_path)
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            let clean_msg = err_msg
                .lines()
                .find(|l| l.contains("fatal:") || l.contains("error:"))
                .unwrap_or_else(|| err_msg.lines().next().unwrap_or("git clone failed"));
            Err(anyhow!("{}", clean_msg.trim()))
        }
    }
}

/// Channels returned by [`clone_repository_streamed`].
pub struct CloneChannels {
    /// Receives individual log lines from git stderr as they arrive.
    pub log_rx: mpsc::Receiver<String>,
    /// Receives the final result — the cloned path on success or an error message.
    pub done_rx: mpsc::Receiver<Result<PathBuf, String>>,
}

impl std::fmt::Debug for CloneChannels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CloneChannels").finish_non_exhaustive()
    }
}

/// Spawn a background thread that runs `git clone --progress` and streams stderr
/// lines back via channels so the TUI can render live progress without blocking.
pub fn clone_repository_streamed(
    url: String,
    target_root: PathBuf,
    custom_name: Option<String>,
) -> CloneChannels {
    let (log_tx, log_rx) = mpsc::channel::<String>();
    let (done_tx, done_rx) = mpsc::channel::<Result<PathBuf, String>>();

    thread::spawn(move || {
        if let Err(e) = std::fs::create_dir_all(&target_root) {
            let _ = done_tx.send(Err(e.to_string()));
            return;
        }

        let repo_name = match custom_name.as_deref() {
            Some(n) if !n.trim().is_empty() => n.trim().to_string(),
            _ => extract_repo_name(&url),
        };

        let target_path = target_root.join(&repo_name);

        let mut cmd = Command::new("git");
        // --progress forces git to emit progress even when stderr is not a TTY.
        cmd.args(["clone", "--progress", &url, &target_path.to_string_lossy()]);
        cmd.env("GIT_TERMINAL_PROMPT", "0");
        // Pipe stderr so we can read it; stdout is unused for clone.
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::null());

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let _ = done_tx.send(Err(format!("Failed to spawn git: {e}")));
                return;
            }
        };

        // Read stderr byte by byte, splitting on both \r and \n.
        // git --progress uses \r to overwrite progress lines in a terminal;
        // we treat each \r-terminated segment as a separate log entry so the
        // TUI can show them sequentially.
        if let Some(stderr) = child.stderr.take() {
            use std::io::Read;
            let mut reader = BufReader::new(stderr);
            let mut buf = Vec::new();
            let mut byte = [0u8; 1];
            loop {
                match reader.read(&mut byte) {
                    Ok(0) | Err(_) => {
                        // Flush any trailing buffer.
                        if let Ok(s) = std::str::from_utf8(&buf) {
                            let clean = strip_ansi(s);
                            for part in clean.split('\n') {
                                let t = part.trim();
                                if !t.is_empty() {
                                    let _ = log_tx.send(t.to_string());
                                }
                            }
                        }
                        break;
                    }
                    Ok(_) => {
                        if byte[0] == b'\r' || byte[0] == b'\n' {
                            if let Ok(s) = std::str::from_utf8(&buf) {
                                let clean = strip_ansi(s);
                                let t = clean.trim().to_string();
                                if !t.is_empty() {
                                    let _ = log_tx.send(t);
                                }
                            }
                            buf.clear();
                        } else {
                            buf.push(byte[0]);
                        }
                    }
                }
            }
        }

        match child.wait() {
            Ok(status) if status.success() => {
                let _ = done_tx.send(Ok(target_path));
            }
            Ok(status) => {
                let _ = done_tx.send(Err(format!("git clone failed (exit {})", status)));
            }
            Err(e) => {
                let _ = done_tx.send(Err(e.to_string()));
            }
        }
    });

    CloneChannels { log_rx, done_rx }
}

/// Remove ANSI escape sequences from a string.
/// Handles `ESC[...m` colour codes and carriage-return progress lines.
fn strip_ansi(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\x1b' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            // Skip until we hit the final byte of the escape sequence (a letter or '~').
            i += 2;
            while i < bytes.len() && !bytes[i].is_ascii_alphabetic() && bytes[i] != b'~' {
                i += 1;
            }
            i += 1; // skip the terminating letter
        } else if bytes[i] == b'\r' {
            // Carriage return — git uses these for in-place progress; treat as newline.
            out.push('\n');
            i += 1;
        } else {
            // Safety: we only push valid UTF-8 subslices.
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i] != b'\x1b' && bytes[i] != b'\r' {
                i += 1;
            }
            if let Ok(chunk) = std::str::from_utf8(&bytes[start..i]) {
                out.push_str(chunk);
            }
        }
    }
    // Trim trailing whitespace/newlines on each virtual line.
    out.lines().map(str::trim_end).collect::<Vec<_>>().join("\n")
}

/// Executes the CLI clone command.
pub fn execute(args: CloneArgs) -> Result<()> {
    let cfg = Config::load()?;
    if cfg.projects_root.is_empty() {
        return Err(anyhow!("No project roots configured. Please add a root first."));
    }

    // Determine target root
    let target_root = match args.root {
        Some(ref root_arg) => {
            // Check if it's an index
            if let Ok(idx) = root_arg.parse::<usize>() {
                if idx < cfg.projects_root.len() {
                    cfg.projects_root[idx].clone()
                } else {
                    return Err(anyhow!(
                        "Root index {} is out of bounds. You have {} configured roots.",
                        idx,
                        cfg.projects_root.len()
                    ));
                }
            } else {
                // Otherwise treat as path string
                PathBuf::from(root_arg)
            }
        }
        None => cfg.projects_root[0].clone(),
    };

    println!("Cloning '{}' into '{}'...", args.url, target_root.display());
    let target_path = clone_repository(&args.url, &target_root, args.name.as_deref(), true)?;
    println!("Successfully cloned into {}", target_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_extract_repo_name() {
        assert_eq!(extract_repo_name("https://github.com/user/my-repo.git"), "my-repo");
        assert_eq!(extract_repo_name("git@github.com:user/another-repo.git"), "another-repo");
        assert_eq!(extract_repo_name("https://example.com/simple"), "simple");
        assert_eq!(extract_repo_name("repo.git"), "repo");
        assert_eq!(extract_repo_name("https://example.com/"), "example.com");
        assert_eq!(extract_repo_name("///"), "repository");
        assert_eq!(extract_repo_name(""), "repository");
    }

    #[test]
    fn test_strip_ansi() {
        let text = "\x1b[31mError Message\x1b[0m";
        assert_eq!(strip_ansi(text), "Error Message");

        let cr_text = "Progress: 50%\rProgress: 100%";
        assert_eq!(strip_ansi(cr_text), "Progress: 50%\nProgress: 100%");
    }

    #[test]
    fn test_clone_repository_and_streamed_local() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        std::fs::create_dir_all(&source).unwrap();

        let status =
            Command::new("git").args(["init", &source.to_string_lossy()]).output().unwrap();
        assert!(status.status.success());

        let _ = Command::new("git")
            .args(["-C", &source.to_string_lossy(), "config", "user.name", "Test"])
            .output();
        let _ = Command::new("git")
            .args(["-C", &source.to_string_lossy(), "config", "user.email", "test@test.com"])
            .output();
        std::fs::write(source.join("file.txt"), "hello").unwrap();
        let _ = Command::new("git").args(["-C", &source.to_string_lossy(), "add", "."]).output();
        let _ = Command::new("git")
            .args(["-C", &source.to_string_lossy(), "commit", "-m", "initial"])
            .output();

        let target_root = temp.path().join("roots");

        // Test non-streamed clone
        let cloned =
            clone_repository(&source.to_string_lossy(), &target_root, Some("custom_name"), false)
                .unwrap();
        assert!(cloned.exists());
        assert_eq!(cloned.file_name().unwrap(), "custom_name");

        // Test streamed clone
        let channels = clone_repository_streamed(
            source.to_string_lossy().to_string(),
            target_root.clone(),
            Some("streamed_name".to_string()),
        );

        let res = channels.done_rx.recv().unwrap();
        assert!(res.is_ok());
        let streamed_path = res.unwrap();
        assert!(streamed_path.exists());
        assert_eq!(streamed_path.file_name().unwrap(), "streamed_name");
    }

    #[test]
    fn test_clone_repository_streamed_failure() {
        let temp = TempDir::new().unwrap();
        let target_root = temp.path().join("roots");

        let channels = clone_repository_streamed(
            "https://invalid-url-that-does-not-exist-12345.com/repo.git".to_string(),
            target_root,
            None,
        );

        let res = channels.done_rx.recv().unwrap();
        assert!(res.is_err());
    }
}
