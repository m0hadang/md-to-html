# md-to-html

A small Rust CLI that converts Markdown to HTML for a static site or blog. It processes a root `index.md`, recursively converts all `.md` files under `src-post/`, and generates directory index pages.

## Features

- Converts Markdown to HTML with [markdown](https://crates.io/crates/markdown) (GFM tables, math).
- Inline/block math: `$...$` and `$$...$$` (via MathJax in output).
- Output includes highlight.js (code) and a link to `css/style.css`.
- Generates `index.html` per directory from the list of `.md` files in that directory.

## Requirements

- [Rust](https://www.rust-lang.org/) (2021 edition; install via rustup).

## Build & Run

To build the release binary into the **project root** (e.g. `md-to-html.exe` next to `Cargo.toml`), run the script (works on stable Cargo; `--out-dir` is nightly-only):

clear output post directory(post) and indexing md file in src-post

```ps1
.\clear.cmd
```

build

```ps1
.\build-release.cmd
```

Run the program from **inside** the `md-to-html` crate directory. The program changes the current directory to the **parent** of the crate, so your content must live there:

```
your-site/           <- project root (cwd after program runs)
├── index.md         <- converted to index.html at root
├── src-post/        <- source Markdown (recursive)
│   ├── foo.md
│   └── bar/
│       └── baz.md
├── post/            <- generated HTML (created/updated by tool)
├── css/
│   └── style.css    <- referenced by generated HTML
└── md-to-html/      <- this crate; run the binary from here
    ├── Cargo.toml
    └── src/main.rs
```

From the project root’s parent (or from `your-site`):

```bash
cd your-site
.\md-to-html.exe
```

## What It Does

1. **Home index** — Converts `../index.md` to `../index.html`.
2. **Post index** — For each directory under `src-post/`, writes an `index.md` (source) and builds `post/.../index.html` with a list of links to that directory’s pages.
3. **Posts** — Converts every `.md` under `src-post/` to `.html` under `post/`, preserving directory structure.

Generated HTML uses UTF-8, relative paths to `css/style.css`, and CDN links for highlight.js and MathJax.