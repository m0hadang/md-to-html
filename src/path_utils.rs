use std::env::current_exe;
use std::path::{Path, PathBuf};

use crate::config;

/// Project root: directory containing INDEX_MD and SOURCE_POST_DIR.
/// If exe lives in a folder named "md-to-html", use parent only when parent has index or source dir.
pub(crate) fn resolve_project_root() -> PathBuf {
    let exe_dir = current_exe()
        .expect("Failed to get executable path")
        .parent()
        .expect("Executable has no parent directory")
        .to_path_buf();

    let is_exe_in_md_to_html = exe_dir.file_name().and_then(|n| n.to_str()) == Some("md-to-html");

    if is_exe_in_md_to_html {
        let parent = exe_dir.parent().expect("Executable in md-to-html but no parent directory");
        let has_content =
            parent.join(config::INDEX_MD).exists() || parent.join(config::SOURCE_POST_DIR).is_dir();
        if has_content {
            return parent.to_path_buf();
        }
    }

    exe_dir
}

pub(crate) fn ensure_default_index() {
    if !Path::new(config::INDEX_MD).exists() {
        std::fs::write(config::INDEX_MD, config::default_index_content())
            .expect("Failed to create default index");
        println!("==> created default {}", config::INDEX_MD);
    }
}

pub(crate) fn is_listing_file(file_path: &Path, source_post_dir: &str, output_post_dir: &str) -> bool {
    let parent = match file_path.parent() {
        Some(p) => p,
        None => return false,
    };
    let relative = match parent.strip_prefix(source_post_dir) {
        Ok(r) => r,
        Err(_) => return false,
    };
    let output_path = Path::new(output_post_dir).join(relative);
    let output_dir_name = output_path.file_name().and_then(|n| n.to_str());
    let listing_name = match output_dir_name {
        Some(n) => format!("{}.md", n),
        None => return false,
    };
    file_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == listing_name)
        .unwrap_or(false)
}

/// Listing file name = output directory name (e.g. post.md in source dir, rust.md in source/rust).
pub(crate) fn listing_filename_for_dir(cur_path: &Path, output_post_dir: &str) -> String {
    let relative = cur_path
        .strip_prefix(config::SOURCE_POST_DIR)
        .unwrap_or(cur_path);
    let output_path = Path::new(output_post_dir).join(relative);
    output_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| format!("{}.md", n))
        .unwrap_or_else(|| output_post_dir.to_string())
}

pub(crate) fn resolve_html_output_path(md_path: &Path, output_base: &str) -> PathBuf {
    if md_path.starts_with(config::SOURCE_POST_DIR) {
        let relative = md_path
            .strip_prefix(config::SOURCE_POST_DIR)
            .unwrap_or(md_path);
        return Path::new(output_base).join(relative).with_extension("html");
    }
    let file_name = md_path.file_name().expect("Markdown file path has no filename");
    if output_base == "." {
        Path::new(file_name).with_extension("html").to_path_buf()
    } else {
        Path::new(output_base).join(file_name).with_extension("html")
    }
}

pub(crate) fn calculate_css_path(html_path: &Path) -> String {
    let depth = html_path.parent().map(|p| p.components().count()).unwrap_or(0);
    let mut css_path = String::new();
    for _ in 0..depth {
        css_path.push_str("../");
    }
    css_path.push_str("css/style.css");
    css_path
}

pub(crate) fn get_md_files(dir: &str) -> Vec<String> {
    let mut file_list = Vec::new();
    for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
        let path = entry.expect("Failed to read directory entry").path();
        if path.is_dir() {
            let sub = path.to_str().expect("Path contains invalid UTF-8");
            file_list.extend(get_md_files(sub));
        } else if path.extension().map(|e| e == "md").unwrap_or(false) {
            file_list.push(path.to_str().expect("Path contains invalid UTF-8").to_string());
        }
    }
    file_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_css_path_at_root() {
        let path = Path::new("index.html");
        assert_eq!(calculate_css_path(path), "css/style.css");
    }

    #[test]
    fn calculate_css_path_nested() {
        let path = Path::new("post/rust/foo.html");
        assert_eq!(calculate_css_path(path), "../../css/style.css");
    }

    #[test]
    fn is_listing_file_true_when_name_matches_dir() {
        let p = format!("{}/post.md", config::SOURCE_POST_DIR);
        assert!(is_listing_file(
            Path::new(&p),
            config::SOURCE_POST_DIR,
            config::OUTPUT_POST_DIR
        ));
    }

    #[test]
    fn is_listing_file_false_when_not_listing() {
        let p = format!("{}/other.md", config::SOURCE_POST_DIR);
        assert!(!is_listing_file(
            Path::new(&p),
            config::SOURCE_POST_DIR,
            config::OUTPUT_POST_DIR
        ));
    }

    #[test]
    fn resolve_html_output_path_src_post_keeps_structure() {
        let path_str = format!("{}/rust/foo.md", config::SOURCE_POST_DIR);
        let out = resolve_html_output_path(Path::new(&path_str), config::OUTPUT_POST_DIR);
        let expected = format!("{}/rust/foo.html", config::OUTPUT_POST_DIR);
        assert_eq!(out, Path::new(&expected));
    }
}
