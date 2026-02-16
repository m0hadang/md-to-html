mod config;
mod convert;
mod html;
mod md;
mod path_utils;

use std::env::set_current_dir;
use std::path::Path;

fn main() {
    let project_root = path_utils::resolve_project_root();
    set_current_dir(&project_root).expect("Failed to change to project root directory");

    let source_post_dir = config::SOURCE_POST_DIR;
    let output_post_dir = config::OUTPUT_POST_DIR;

    path_utils::ensure_default_index();
    std::fs::create_dir_all(source_post_dir).expect("Failed to create source post directory");
    std::fs::create_dir_all(output_post_dir).expect("Failed to create output directory");

    println!("==> create home index...");
    convert::convert_md_to_html(config::INDEX_MD, ".")
        .expect("Failed to convert index to HTML");

    println!("==> create post index...");
    convert::create_index_md(source_post_dir, output_post_dir);

    println!("==> create post...");
    for file in path_utils::get_md_files(source_post_dir) {
        if path_utils::is_listing_file(Path::new(&file), source_post_dir, output_post_dir) {
            continue;
        }
        convert::convert_md_to_html(&file, output_post_dir)
            .expect("Failed to convert markdown file to HTML");
    }
}
