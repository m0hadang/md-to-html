use markdown::{CompileOptions, Constructs, Options, ParseOptions};
use std::env::{current_exe, set_current_dir};
use std::path::Path;

fn main() {
    // Project root: directory containing index.md and src-post/
    // - If exe is in a folder named "md-to-html", use parent only when parent has index.md or src-post.
    // - Otherwise use the exe's directory (run .\md-to-html.exe from project root, or from inside repo).
    let exe_path = current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Executable has no parent directory");
    let project_root = if exe_dir.file_name().and_then(|n| n.to_str()) == Some("md-to-html") {
        let parent = exe_dir
            .parent()
            .expect("Executable in md-to-html but no parent directory");
        let parent_has_content = parent.join("index.md").exists() || parent.join("src-post").is_dir();
        if parent_has_content {
            parent
        } else {
            exe_dir
        }
    } else {
        exe_dir
    };
    set_current_dir(project_root).expect("Failed to change to project root directory");

    let source_post_dir = "src-post";
    let output_post_dir = "post";

    // Create default index.md if it does not exist
    const DEFAULT_INDEX_MD: &str = "🚀 *[Go Post](post/post.html)*\n";
    if !Path::new("index.md").exists() {
        std::fs::write("index.md", DEFAULT_INDEX_MD).expect("Failed to create default index.md");
        println!("==> created default index.md");
    }

    // Create output directory structure (src-post may not exist yet)
    std::fs::create_dir_all(source_post_dir)
        .expect("Failed to create source post directory");
    std::fs::create_dir_all(output_post_dir)
        .expect("Failed to create output directory");

    println!("==> create home index...");
    convert_md_to_html("index.md", ".")
        .expect("Failed to convert index.md to HTML");

    println!("==> create post index...");
    create_index_md(source_post_dir, output_post_dir);

    println!("==> create post...");
    get_md_files(source_post_dir).iter().for_each(|file| {
        // Skip listing files ({dirname}.md); already generated as {dirname}.html by create_index_md
        if is_listing_file(Path::new(file), source_post_dir, output_post_dir) {
            return;
        }
        convert_md_to_html(file, output_post_dir)
            .expect("Failed to convert markdown file to HTML");
    });
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
    let output_dir_name = match output_path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return false,
    };
    let listing_name = format!("{}.md", output_dir_name);
    file_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == listing_name)
        .unwrap_or(false)
}

fn create_index_md(cur_dir: &str, output_post_dir: &str) {
    let mut content = String::new();
    let cur_path = Path::new(cur_dir);

    // Listing file name = output directory name (e.g. post.md in src-post, rust.md in src-post/rust)
    let relative_path = cur_path.strip_prefix("src-post").unwrap_or(cur_path);
    let output_path = Path::new(output_post_dir).join(relative_path);
    let listing_file_name = output_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| format!("{}.md", n))
        .unwrap_or_else(|| "post.md".to_string());

    for entry in std::fs::read_dir(cur_dir).expect("can't read dir") {
        let path = entry.expect("can't read dir path").path();
        if path.is_dir() {
            let dir_name = path
                .file_name()
                .expect("path is empty")
                .to_str()
                .expect("can't not convert from file name to str");
            let dir_path = path.to_str().expect("can't not convert from path to str");

            create_index_md(dir_path, output_post_dir);

            content.push_str(&format!("- [{}]({}/{}.html)\n", dir_name, dir_name, dir_name));
        } else if path
            .extension()
            .expect("File path has no extension")
            == "md"
        {
            let file_name = path
                .file_name()
                .expect("path is empty")
                .to_str()
                .expect("can't not convert from file name to str");

            if file_name == listing_file_name {
                continue;
            }

            content.push_str(&format!(
                "- [{}]({})\n",
                file_name.replace(".md", ""),
                file_name.replace(".md", ".html")
            ));
        }
    }

    // Use existing listing file if present; otherwise write generated content
    let source_listing_path = cur_path.join(&listing_file_name);
    let content = if source_listing_path.exists() {
        println!("{} (use existing)", source_listing_path.display());
        std::fs::read_to_string(&source_listing_path).expect("Failed to read listing file")
    } else {
        println!("{}", source_listing_path.display());
        std::fs::write(&source_listing_path, &content).expect("Failed to write listing file");
        content
    };

    // Write index HTML to output directory (filename = directory name, e.g. post/post.html)
    // Strip src-post prefix to get relative path
    let relative_path = cur_path.strip_prefix("src-post").unwrap_or(cur_path);
    let output_path = Path::new(output_post_dir).join(relative_path);
    std::fs::create_dir_all(&output_path)
        .expect("Failed to create output directory for index");

    let output_dir_name = output_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("index");
    let output_index_html = output_path.join(format!("{}.html", output_dir_name));
    let html_content = generate_index_html(&content, &output_index_html);
    std::fs::write(&output_index_html, html_content)
        .expect("Failed to write index HTML file");
}

fn convert_md_to_html(
    markdown_file: &str,
    output_base: &str,
) -> Result<(), markdown::message::Message> {
    let md_path = Path::new(markdown_file);

    // Determine output path
    let output_path = if md_path.starts_with("src-post") {
        // For src-post files, maintain directory structure in post/
        let relative_path = md_path.strip_prefix("src-post").unwrap_or(md_path);
        Path::new(output_base).join(relative_path)
    } else {
        // For root files like index.md, output to current directory (root)
        // If output_base is ".", we want to output to root, so use empty path
        if output_base == "." {
            Path::new(
                md_path
                    .file_name()
                    .expect("Markdown file path has no filename"),
            )
            .to_path_buf()
        } else {
            Path::new(output_base).join(
                md_path
                    .file_name()
                    .expect("Markdown file path has no filename"),
            )
        }
    };

    // Change extension from .md to .html
    let html_path = output_path.with_extension("html");

    // Create output directory if needed
    if let Some(parent) = html_path.parent() {
        std::fs::create_dir_all(parent)
            .expect("Failed to create parent directory for HTML file");
    }

    println!("{} -> {}", markdown_file, html_path.display());

    let markdown = std::fs::read_to_string(markdown_file)
        .expect("Failed to read markdown file");
    let markdown = replace_math_expr(&markdown);
    let body = markdown::to_html_with_options(
        &markdown,
        &Options {
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
                // math_text_single_dollar: true,
                ..ParseOptions::default()
            },
        },
    )?;

    let title = md_path
        .file_stem()
        .expect("Markdown file path has no stem")
        .to_str()
        .expect("File stem contains invalid UTF-8");

    // Calculate CSS path relative to output HTML file
    let css_path = calculate_css_path(&html_path);

    let html = format!(
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
    );

    std::fs::write(&html_path, html)
        .expect("Failed to write HTML file");
    Ok(())
}

fn calculate_css_path(html_path: &Path) -> String {
    // Calculate relative path from HTML file to root css/ directory
    // Example: post/algorithm/big-o.html -> ../../css/style.css
    // Count directory components (excluding filename)
    let depth = if let Some(parent) = html_path.parent() {
        parent.components().count()
    } else {
        0
    };

    let mut css_path = String::new();
    for _ in 0..depth {
        css_path.push_str("../");
    }
    css_path.push_str("css/style.css");
    css_path
}

fn generate_index_html(markdown_content: &str, html_path: &Path) -> String {
    // Convert markdown list to HTML
    let body = markdown::to_html_with_options(
        markdown_content,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..CompileOptions::default()
            },
            parse: ParseOptions {
                constructs: Constructs {
                    gfm_table: true,
                    ..Constructs::default()
                },
                ..ParseOptions::default()
            },
        },
    )
    .expect("Failed to convert markdown to HTML for index");

    let css_path = calculate_css_path(html_path);
    let page_title = html_path
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("index");

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
  <title>{page_title}</title>
</HEAD>

<BODY>
<h1 id="post-title">{page_title}</h1>

{body}

</BODY>

</HTML>"#
    )
}

fn replace_math_expr(body: &str) -> String {
    // $expr$ -> \\(expr)\\)
    let mut result = String::new();
    let mut in_math = false;
    for c in body.chars() {
        if c == '$' {
            if in_math {
                result.push_str(r#"\)"#);
            } else {
                result.push_str(r#"\("#);
            }
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
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            file_list.append(&mut get_md_files(
                path.to_str()
                    .expect("Path contains invalid UTF-8"),
            ));
        } else if path
            .extension()
            .expect("File path has no extension")
            == "md"
        {
            file_list.push(
                path.to_str()
                    .expect("Path contains invalid UTF-8")
                    .to_string(),
            );
        }
    }
    file_list
}
