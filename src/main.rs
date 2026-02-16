use markdown::{CompileOptions, Constructs, Options, ParseOptions};
use std::env::{current_exe, set_current_dir};
use std::path::{Path, PathBuf};

fn main() {
    let project_root = resolve_project_root();
    set_current_dir(&project_root).expect("Failed to change to project root directory");

    let source_post_dir = "src-post";
    let output_post_dir = "post";

    ensure_default_index();
    std::fs::create_dir_all(source_post_dir).expect("Failed to create source post directory");
    std::fs::create_dir_all(output_post_dir).expect("Failed to create output directory");

    println!("==> create home index...");
    convert_md_to_html("index.md", ".").expect("Failed to convert index.md to HTML");

    println!("==> create post index...");
    create_index_md(source_post_dir, output_post_dir);

    println!("==> create post...");
    for file in get_md_files(source_post_dir) {
        if is_listing_file(Path::new(&file), source_post_dir, output_post_dir) {
            continue;
        }
        convert_md_to_html(&file, output_post_dir).expect("Failed to convert markdown file to HTML");
    }
}

/// Project root: directory containing index.md and src-post/.
/// If exe lives in a folder named "md-to-html", use parent only when parent has index.md or src-post.
fn resolve_project_root() -> PathBuf {
    let exe_dir = current_exe()
        .expect("Failed to get executable path")
        .parent()
        .expect("Executable has no parent directory")
        .to_path_buf();

    let is_exe_in_md_to_html = exe_dir.file_name().and_then(|n| n.to_str()) == Some("md-to-html");

    if is_exe_in_md_to_html {
        let parent = exe_dir.parent().expect("Executable in md-to-html but no parent directory");
        let has_content = parent.join("index.md").exists() || parent.join("src-post").is_dir();
        if has_content {
            return parent.to_path_buf();
        }
    }

    exe_dir
}

fn ensure_default_index() {
    const DEFAULT_INDEX_MD: &str = "🚀 *[Go Post](post/post.html)*\n";
    if !Path::new("index.md").exists() {
        std::fs::write("index.md", DEFAULT_INDEX_MD).expect("Failed to create default index.md");
        println!("==> created default index.md");
    }
}

fn is_listing_file(file_path: &Path, source_post_dir: &str, output_post_dir: &str) -> bool {
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

fn create_index_md(cur_dir: &str, output_post_dir: &str) {
    let cur_path = Path::new(cur_dir);
    let listing_file_name = listing_filename_for_dir(cur_path, output_post_dir);
    let mut content = String::new();

    for entry in std::fs::read_dir(cur_dir).expect("can't read dir") {
        let path = entry.expect("can't read dir path").path();
        if path.is_dir() {
            let dir_name = path.file_name().expect("path is empty").to_str().expect("dir name to str");
            let dir_path = path.to_str().expect("path to str");
            create_index_md(dir_path, output_post_dir);
            content.push_str(&format!("- [{}]({}/{}.html)\n", dir_name, dir_name, dir_name));
        } else if path.extension().map(|e| e == "md").unwrap_or(false) {
            let file_name = path.file_name().expect("path is empty").to_str().expect("file name to str");
            if file_name == listing_file_name {
                continue;
            }
            let stem = file_name.replace(".md", "");
            let html_name = file_name.replace(".md", ".html");
            content.push_str(&format!("- [{}]({})\n", stem, html_name));
        }
    }

    let source_listing_path = cur_path.join(&listing_file_name);
    let content = if source_listing_path.exists() {
        println!("{} (use existing)", source_listing_path.display());
        std::fs::read_to_string(&source_listing_path).expect("Failed to read listing file")
    } else {
        println!("{}", source_listing_path.display());
        std::fs::write(&source_listing_path, &content).expect("Failed to write listing file");
        content
    };

    let output_path = Path::new(output_post_dir).join(cur_path.strip_prefix("src-post").unwrap_or(cur_path));
    std::fs::create_dir_all(&output_path).expect("Failed to create output directory for index");

    let output_dir_name = output_path.file_name().and_then(|n| n.to_str()).unwrap_or("index");
    let output_index_html = output_path.join(format!("{}.html", output_dir_name));
    let html_content = generate_index_html(&content, &output_index_html);
    std::fs::write(&output_index_html, html_content).expect("Failed to write index HTML file");
}

/// Listing file name = output directory name (e.g. post.md in src-post, rust.md in src-post/rust).
fn listing_filename_for_dir(cur_path: &Path, output_post_dir: &str) -> String {
    let relative = cur_path.strip_prefix("src-post").unwrap_or(cur_path);
    let output_path = Path::new(output_post_dir).join(relative);
    output_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| format!("{}.md", n))
        .unwrap_or_else(|| "post.md".to_string())
}

fn convert_md_to_html(markdown_file: &str, output_base: &str) -> Result<(), markdown::message::Message> {
    let md_path = Path::new(markdown_file);
    let html_path = resolve_html_output_path(md_path, output_base);

    if let Some(parent) = html_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create parent directory for HTML file");
    }

    println!("{} -> {}", markdown_file, html_path.display());

    let markdown = std::fs::read_to_string(markdown_file).expect("Failed to read markdown file");
    let markdown = replace_math_expr(&markdown);
    let body = markdown::to_html_with_options(&markdown, &markdown_options_full())?;

    let title = md_path
        .file_stem()
        .expect("Markdown file path has no stem")
        .to_str()
        .expect("File stem contains invalid UTF-8");

    let css_path = calculate_css_path(&html_path);
    let html = wrap_html_body(&body, title, &css_path);

    std::fs::write(&html_path, html).expect("Failed to write HTML file");
    Ok(())
}

fn resolve_html_output_path(md_path: &Path, output_base: &str) -> PathBuf {
    if md_path.starts_with("src-post") {
        let relative = md_path.strip_prefix("src-post").unwrap_or(md_path);
        return Path::new(output_base).join(relative).with_extension("html");
    }
    let file_name = md_path.file_name().expect("Markdown file path has no filename");
    if output_base == "." {
        Path::new(file_name).with_extension("html").to_path_buf()
    } else {
        Path::new(output_base).join(file_name).with_extension("html")
    }
}

fn markdown_options_full() -> Options {
    Options {
        compile: CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..CompileOptions::default()
        },
        parse: ParseOptions {
            constructs: Constructs {
                math_text: true,
                math_flow: true,
                gfm_table: true,
                character_escape: false,
                ..Constructs::default()
            },
            ..ParseOptions::default()
        },
    }
}

fn markdown_options_index() -> Options {
    Options {
        compile: CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..CompileOptions::default()
        },
        parse: ParseOptions {
            constructs: Constructs { gfm_table: true, ..Constructs::default() },
            ..ParseOptions::default()
        },
    }
}

fn calculate_css_path(html_path: &Path) -> String {
    let depth = html_path.parent().map(|p| p.components().count()).unwrap_or(0);
    let mut css_path = String::new();
    for _ in 0..depth {
        css_path.push_str("../");
    }
    css_path.push_str("css/style.css");
    css_path
}

fn wrap_html_body(body: &str, title: &str, css_path: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<HTML>
<HEAD>
<meta charset="UTF-8">
  <link rel="stylesheet" href="{css_path}">
  <!-- code highlight -->
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/school-book.css">
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/go.min.js"></script>
  <script>hljs.highlightAll();</script>
  <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
  <title>{title}</title>
</HEAD>

<BODY>
<h1 id="post-title">{title}</h1>

{body}

</BODY>

</HTML>"#
    )
}

fn generate_index_html(markdown_content: &str, html_path: &Path) -> String {
    let body = markdown::to_html_with_options(markdown_content, &markdown_options_index())
        .expect("Failed to convert markdown to HTML for index");
    let css_path = calculate_css_path(html_path);
    let page_title = html_path.file_stem().and_then(|n| n.to_str()).unwrap_or("index");
    wrap_html_body(&body, page_title, &css_path)
}

/// Replaces $expr$ with \(expr\) for MathJax inline math.
fn replace_math_expr(body: &str) -> String {
    let mut result = String::new();
    let mut in_math = false;
    for c in body.chars() {
        if c == '$' {
            result.push_str(if in_math { r"\)" } else { r"\(" });
            in_math = !in_math;
        } else {
            result.push(c);
        }
    }
    result
}

fn get_md_files(dir: &str) -> Vec<String> {
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

// -----------------------------------------------------------------------------
// Tests (no mocks; pure logic and path checks only)
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn replace_math_expr_empty() {
        assert_eq!(replace_math_expr(""), "");
    }

    #[test]
    fn replace_math_expr_single_inline() {
        assert_eq!(replace_math_expr("$x$"), r"\(x\)");
    }

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
        // src-post/post.md is the listing for output dir "post"
        assert!(is_listing_file(
            Path::new("src-post/post.md"),
            "src-post",
            "post"
        ));
    }
}
