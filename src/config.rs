/// Single place to change directory and file names.

/// Source directory for markdown posts (e.g. src-post).
pub(crate) const SOURCE_POST_DIR: &str = "src-post";

/// Output directory for generated HTML (e.g. post).
pub(crate) const OUTPUT_POST_DIR: &str = "post";

/// Root index markdown file (e.g. index.md).
pub(crate) const INDEX_MD: &str = "index.md";

/// Default content for INDEX_MD when it is created (link to post index).
pub(crate) fn default_index_content() -> String {
    format!(
        "🚀 *[Go Post]({}/{}.html)*\n",
        OUTPUT_POST_DIR,
        OUTPUT_POST_DIR
    )
}
