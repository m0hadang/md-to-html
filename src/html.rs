use std::path::Path;

use crate::path_utils;

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

pub(crate) fn generate_index_html(markdown_content: &str, html_path: &Path) -> String {
    let body = markdown::to_html_with_options(markdown_content, &crate::md::options_index())
        .expect("Failed to convert markdown to HTML for index");
    let css_path = path_utils::calculate_css_path(html_path);
    let page_title = html_path.file_stem().and_then(|n| n.to_str()).unwrap_or("index");
    wrap_html_body(&body, page_title, &css_path)
}

pub(crate) fn wrap_html_body_public(body: &str, title: &str, css_path: &str) -> String {
    wrap_html_body(body, title, css_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_html_body_includes_title_and_css() {
        let html = wrap_html_body_public("<p>Hi</p>", "My Title", "css/style.css");
        assert!(html.contains("<title>My Title</title>"));
        assert!(html.contains("href=\"css/style.css\""));
        assert!(html.contains("<h1 id=\"post-title\">My Title</h1>"));
        assert!(html.contains("<p>Hi</p>"));
    }
}
