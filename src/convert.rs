use std::path::Path;

use crate::html;
use crate::md;
use crate::path_utils;

pub(crate) fn convert_md_to_html(
    markdown_file: &str,
    output_base: &str,
) -> Result<(), markdown::message::Message> {
    let md_path = Path::new(markdown_file);
    let html_path = path_utils::resolve_html_output_path(md_path, output_base);

    if let Some(parent) = html_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create parent directory for HTML file");
    }

    println!("{} -> {}", markdown_file, html_path.display());

    let content = std::fs::read_to_string(markdown_file).expect("Failed to read markdown file");
    let content = md::replace_math_expr(&content);
    let body = markdown::to_html_with_options(&content, &md::options_full())?;

    let title = md_path
        .file_stem()
        .expect("Markdown file path has no stem")
        .to_str()
        .expect("File stem contains invalid UTF-8");

    let css_path = path_utils::calculate_css_path(&html_path);
    let html = html::wrap_html_body_public(&body, title, &css_path);

    std::fs::write(&html_path, html).expect("Failed to write HTML file");
    Ok(())
}

pub(crate) fn create_index_md(cur_dir: &str, output_post_dir: &str) {
    let cur_path = Path::new(cur_dir);
    let listing_file_name = path_utils::listing_filename_for_dir(cur_path, output_post_dir);
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

    let output_path =
        Path::new(output_post_dir).join(cur_path.strip_prefix("src-post").unwrap_or(cur_path));
    std::fs::create_dir_all(&output_path).expect("Failed to create output directory for index");

    let output_dir_name = output_path.file_name().and_then(|n| n.to_str()).unwrap_or("index");
    let output_index_html = output_path.join(format!("{}.html", output_dir_name));
    let html_content = html::generate_index_html(&content, &output_index_html);
    std::fs::write(&output_index_html, html_content).expect("Failed to write index HTML file");
}
