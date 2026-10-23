//! Standalone helpers: formatting, paths and filesystem reads.

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use home::home_dir;

use crate::config::Config;

/// Format a byte count as `B`/`KB`/`MB`/`GB`.
pub(crate) fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

/// File name of a path, lossily converted for display.
pub(crate) fn file_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy()
        .into_owned()
}

/// Display width of `text` in terminal cells.
///
/// ASCII counts as one cell, everything else as two: good enough for CJK, but
/// over-counts accented Latin (e.g. `é`).
pub(crate) fn text_width(text: &str) -> u16 {
    let width: usize = text.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum();
    width.min(u16::MAX as usize) as u16
}

/// Shorten a path under the home directory to `~/...`.
pub(crate) fn display_path(path: &Path) -> String {
    if let Some(home) = home_dir()
        && let Ok(relative) = path.strip_prefix(&home)
    {
        if relative.as_os_str().is_empty() {
            return "~".to_string();
        }
        return format!("~/{}", relative.to_string_lossy());
    }

    path.to_string_lossy().into_owned()
}

pub(crate) enum FileType {
    Directory,
    Executable,
    Symlink,
    File,
}

pub(crate) fn load_dir(path: &Path) -> Vec<PathBuf> {
    let mut items = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            items.push(entry.path());
        }
    }
    items.sort_by(|a, b| {
        let a_is_dir = a.is_dir();
        let b_is_dir = b.is_dir();
        if a_is_dir && !b_is_dir {
            std::cmp::Ordering::Less
        } else if !a_is_dir && b_is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.cmp(b)
        }
    });
    items
}

pub(crate) fn file_type(path: &Path) -> FileType {
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return FileType::File,
    };

    if meta.is_dir() {
        return FileType::Directory;
    }
    if meta.is_symlink() {
        return FileType::Symlink;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if meta.permissions().mode() & 0o111 != 0 {
            return FileType::Executable;
        }
    }

    #[cfg(windows)]
    {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext = ext.to_lowercase();
            if ext == "exe" || ext == "bat" || ext == "cmd" || ext == "ps1" {
                return FileType::Executable;
            }
        }
    }

    FileType::File
}

/// One-line summary of a path's type, size and permissions.
pub(crate) fn file_summary(path: &Path) -> String {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return " failed load metadata ".to_string(),
    };
    let ftype = if meta.is_dir() {
        "Directory"
    } else if meta.is_symlink() {
        "Symlink"
    } else {
        "File"
    };
    let size = format_size(meta.len());
    let perm = if meta.permissions().readonly() {
        "RO"
    } else {
        "RW"
    };
    format!(" Type: {}  |  Size: {}  |  Perm: {} ", ftype, size, perm)
}

/// Icon for a path: explicit file name, then extension, then file type.
pub(crate) fn icon_for<'a>(config: &'a Config, file_type: &FileType, path: &Path) -> &'a str {
    if let Some(name) = path.file_name().and_then(|n| n.to_str())
        && let Some(icon) = config.icons.by_name.get(&name.to_lowercase())
    {
        return icon;
    }

    if let Some(extension) = path.extension().and_then(|e| e.to_str())
        && let Some(icon) = config.icons.by_extension.get(&extension.to_lowercase())
    {
        return icon;
    }

    match file_type {
        FileType::Directory => &config.icons.dir,
        FileType::Symlink => &config.icons.symlink,
        FileType::Executable => &config.icons.executable,
        FileType::File => &config.icons.file,
    }
}

/// Index of `select` in `items`, falling back to the first entry.
pub(crate) fn selection_index(items: &[PathBuf], select: Option<&Path>) -> usize {
    select
        .and_then(|select| items.iter().position(|item| item == select))
        .unwrap_or(0)
}

/// Next index for a `delta` move over `len` items, clamped or wrapped by `cycle`.
pub(crate) fn compute_new_index(
    current: Option<usize>,
    len: usize,
    delta: i32,
    cycle: bool,
) -> usize {
    if len == 0 {
        return 0;
    }
    match current {
        Some(i) => {
            let new = i as i32 + delta;
            if new < 0 {
                if cycle { len - 1 } else { 0 }
            } else if new >= len as i32 {
                if cycle { 0 } else { len - 1 }
            } else {
                new as usize
            }
        }
        None => 0,
    }
}

/// File contents plus the last scrollable line index.
pub(crate) struct PreviewText {
    pub(crate) text: String,
    pub(crate) max_scroll: u16,
}

/// Read up to [`MAX_LINES`] lines of a file for the preview pane.
pub(crate) fn read_file_preview(path: &Path) -> PreviewText {
    const MAX_LINES: usize = 1000;

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return PreviewText {
                text: format!("read file error: {e}"),
                max_scroll: 0,
            };
        }
    };

    let mut content = String::new();
    let mut lines = 0;
    for (i, line) in BufReader::new(file).lines().enumerate() {
        if i >= MAX_LINES {
            content.push_str("\n... file too large, truncated preview ...");
            lines += 1;
            break;
        }
        if let Ok(line) = line {
            content.push_str(&line);
            content.push('\n');
            lines += 1;
        }
    }
    PreviewText {
        text: content,
        max_scroll: (lines as u16).saturating_sub(1),
    }
}

/// Run the platform's grep for `query` under `target`, returning raw output lines.
pub(crate) async fn search_content(query: &str, target: &Path) -> Result<Vec<String>, String> {
    let output = build_grep_command(query, target)
        .output()
        .await
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect())
}

#[cfg(not(target_os = "windows"))]
fn build_grep_command(query: &str, target: &Path) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("grep");
    cmd.arg("-rnI").arg(query).arg(target);
    cmd
}

#[cfg(target_os = "windows")]
fn build_grep_command(query: &str, target: &Path) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("findstr");
    cmd.arg("/n").arg("/i").arg(format!("/c:{}", query));
    if target.is_dir() {
        cmd.arg("/s").arg(format!("{}\\*", target.display()));
    } else {
        cmd.arg(target);
    }
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_terminal_cells() {
        assert_eq!(text_width(""), 0);
        assert_eq!(text_width("abc"), 3);
        assert_eq!(text_width("中文"), 4);
        assert_eq!(text_width("a中"), 3);
    }

    #[test]
    fn text_width_saturates() {
        assert_eq!(text_width(&"a".repeat(70_000)), u16::MAX);
    }

    #[test]
    fn formats_byte_sizes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(1023), "1023 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn moves_within_bounds() {
        assert_eq!(compute_new_index(Some(1), 3, 1, false), 2);
        assert_eq!(compute_new_index(Some(1), 3, -1, false), 0);
    }

    #[test]
    fn clamps_at_ends_without_cycle() {
        assert_eq!(compute_new_index(Some(2), 3, 1, false), 2);
        assert_eq!(compute_new_index(Some(0), 3, -1, false), 0);
    }

    #[test]
    fn wraps_at_ends_with_cycle() {
        assert_eq!(compute_new_index(Some(2), 3, 1, true), 0);
        assert_eq!(compute_new_index(Some(0), 3, -1, true), 2);
    }

    #[test]
    fn empty_list_never_underflows() {
        assert_eq!(compute_new_index(Some(0), 0, -1, true), 0);
        assert_eq!(compute_new_index(None, 0, 1, true), 0);
    }
    #[test]
    fn selection_keeps_the_entry_we_came_from() {
        let items = vec![
            PathBuf::from("/a"),
            PathBuf::from("/b"),
            PathBuf::from("/c"),
        ];
        assert_eq!(selection_index(&items, Some(Path::new("/b"))), 1);
    }

    #[test]
    fn selection_falls_back_to_first_entry() {
        let items = vec![PathBuf::from("/a")];
        assert_eq!(selection_index(&items, None), 0);
        assert_eq!(selection_index(&items, Some(Path::new("/missing"))), 0);
        assert_eq!(selection_index(&[], Some(Path::new("/a"))), 0);
    }

    fn temp_file(name: &str, content: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("zap-{name}-{}.txt", std::process::id()));
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn reads_lines_and_reports_scroll() {
        let path = temp_file("preview", "a\nb\nc\n");
        let preview = read_file_preview(&path);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(preview.text, "a\nb\nc\n");
        assert_eq!(preview.max_scroll, 2);
    }

    #[test]
    fn caps_at_max_lines() {
        let path = temp_file("preview-cap", &"x\n".repeat(1002));
        let preview = read_file_preview(&path);
        std::fs::remove_file(&path).unwrap();
        assert!(
            preview
                .text
                .ends_with("... file too large, truncated preview ...")
        );
        assert_eq!(preview.max_scroll, 1000);
    }

    #[test]
    fn reports_open_errors() {
        let preview = read_file_preview(Path::new("/definitely/not/here.txt"));
        assert!(preview.text.starts_with("read file error:"));
        assert_eq!(preview.max_scroll, 0);
    }
}
