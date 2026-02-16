use markdown::{CompileOptions, Constructs, Options, ParseOptions};

pub(crate) fn options_full() -> Options {
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

pub(crate) fn options_index() -> Options {
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

/// Replaces $expr$ with \(expr\) for MathJax inline math.
pub(crate) fn replace_math_expr(body: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_math_expr_empty() {
        assert_eq!(replace_math_expr(""), "");
    }

    #[test]
    fn replace_math_expr_single_inline() {
        assert_eq!(replace_math_expr("$x$"), r"\(x\)");
    }

    #[test]
    fn replace_math_expr_two_inline() {
        assert_eq!(replace_math_expr("$a$ and $b$"), r"\(a\) and \(b\)");
    }
}
