/// Prepares markdown content for the terminal viewer by:
/// Syntax-highlighting fenced code blocks (`html`, `xml`, `css`, `svg`, etc.) with ANSI colors.
pub fn prepare_markdown_view(input: &str) -> String {
    let mut output = String::with_capacity(input.len() + 256);
    let mut in_fence = false;
    let mut fence_lang = String::new();
    let mut code_block_buffer = String::new();

    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            let fence_marker = &trimmed[..3];
            if !in_fence {
                // Starting a code block
                in_fence = true;
                fence_lang = trimmed[3..].trim().to_lowercase();
                code_block_buffer.clear();
                i += 1;
                continue;
            } else if trimmed.starts_with(fence_marker) {
                // Ending a code block
                in_fence = false;
                output.push_str("```\n");
                let highlighted = highlight_code(&code_block_buffer, &fence_lang);
                output.push_str(&highlighted);
                if !highlighted.ends_with('\n') {
                    output.push('\n');
                }
                output.push_str("```\n");
                i += 1;
                continue;
            }
        }

        if in_fence {
            code_block_buffer.push_str(line);
            code_block_buffer.push('\n');
        } else {
            output.push_str(line);
            output.push('\n');
        }

        i += 1;
    }

    if in_fence {
        output.push_str("```\n");
        let highlighted = highlight_code(&code_block_buffer, &fence_lang);
        output.push_str(&highlighted);
        output.push_str("```\n");
    }

    output
}

/// Syntax highlight code blocks based on the language tag.
pub fn highlight_code(code: &str, lang: &str) -> String {
    match lang {
        "html" | "htm" | "xml" | "svg" | "xhtml" => highlight_html_xml(code),
        "css" | "scss" | "less" => highlight_css(code),
        _ => code.to_string(),
    }
}

// ANSI Escape Helpers (24-bit TrueColor)
const RESET: &str = "\x1b[0m";

// Colors:
const CLR_TAG: &str = "\x1b[38;2;86;156;214m";       // Cyan / Blue (#569CD6)
const CLR_ATTR: &str = "\x1b[38;2;156;220;254m";     // Light Cyan (#9CDCFE)
const CLR_STR: &str = "\x1b[38;2;206;145;120m";      // Orange / Coral (#CE9178)
const CLR_COMMENT: &str = "\x1b[38;2;106;153;85m";   // Muted Green (#6A9955)
const CLR_PUNCT: &str = "\x1b[38;2;128;128;128m";    // Gray (#808080)
const CLR_SELECTOR: &str = "\x1b[38;2;220;220;170m"; // Gold / Pale Yellow (#DCDCAA)
const CLR_PROPERTY: &str = "\x1b[38;2;156;220;254m"; // Light Blue (#9CDCFE)
const CLR_VALUE: &str = "\x1b[38;2;206;145;120m";    // Orange (#CE9178)
const CLR_UNIT: &str = "\x1b[38;2;181;206;168m";     // Mint Green (#B5CEA8)

/// Fast, robust streaming tokenizer for HTML/XML syntax highlighting
pub fn highlight_html_xml(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Comments <!-- ... -->
        if i + 3 < len && chars[i] == '<' && chars[i + 1] == '!' && chars[i + 2] == '-' && chars[i + 3] == '-' {
            out.push_str(CLR_COMMENT);
            while i < len {
                out.push(chars[i]);
                if i >= 2 && chars[i] == '>' && chars[i - 1] == '-' && chars[i - 2] == '-' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push_str(RESET);
            continue;
        }

        // Tags <tag ...> or </tag>
        if chars[i] == '<' {
            out.push_str(CLR_TAG);
            out.push('<');
            i += 1;

            if i < len && chars[i] == '/' {
                out.push('/');
                i += 1;
            }

            // Tag name
            while i < len && !chars[i].is_whitespace() && chars[i] != '>' && chars[i] != '/' {
                out.push(chars[i]);
                i += 1;
            }
            out.push_str(RESET);

            // Attributes inside tag
            while i < len && chars[i] != '>' && chars[i] != '/' {
                if chars[i].is_whitespace() {
                    out.push(chars[i]);
                    i += 1;
                    continue;
                }

                // Attribute name
                out.push_str(CLR_ATTR);
                while i < len && !chars[i].is_whitespace() && chars[i] != '=' && chars[i] != '>' && chars[i] != '/' {
                    out.push(chars[i]);
                    i += 1;
                }
                out.push_str(RESET);

                // Equal sign
                if i < len && chars[i] == '=' {
                    out.push_str(CLR_PUNCT);
                    out.push('=');
                    out.push_str(RESET);
                    i += 1;

                    // Attribute value
                    if i < len && (chars[i] == '"' || chars[i] == '\'') {
                        let quote = chars[i];
                        out.push_str(CLR_STR);
                        out.push(quote);
                        i += 1;
                        while i < len && chars[i] != quote {
                            out.push(chars[i]);
                            i += 1;
                        }
                        if i < len && chars[i] == quote {
                            out.push(quote);
                            i += 1;
                        }
                        out.push_str(RESET);
                    }
                }
            }

            if i < len && chars[i] == '/' {
                out.push_str(CLR_TAG);
                out.push('/');
                out.push_str(RESET);
                i += 1;
            }

            if i < len && chars[i] == '>' {
                out.push_str(CLR_TAG);
                out.push('>');
                out.push_str(RESET);
                i += 1;
            }

            continue;
        }

        out.push(chars[i]);
        i += 1;
    }

    out
}

/// Fast streaming tokenizer for CSS syntax highlighting
pub fn highlight_css(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_rule = false;

    while i < len {
        // Comments /* ... */
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            out.push_str(CLR_COMMENT);
            while i < len {
                out.push(chars[i]);
                if i >= 1 && chars[i] == '/' && chars[i - 1] == '*' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push_str(RESET);
            continue;
        }

        if chars[i] == '{' {
            in_rule = true;
            out.push_str(CLR_PUNCT);
            out.push('{');
            out.push_str(RESET);
            i += 1;
            continue;
        }

        if chars[i] == '}' {
            in_rule = false;
            out.push_str(CLR_PUNCT);
            out.push('}');
            out.push_str(RESET);
            i += 1;
            continue;
        }

        if !in_rule {
            // CSS Selectors: .class, #id, tag, :hover
            if !chars[i].is_whitespace() {
                out.push_str(CLR_SELECTOR);
                while i < len && chars[i] != '{' && chars[i] != ',' {
                    out.push(chars[i]);
                    i += 1;
                }
                out.push_str(RESET);
                if i < len && chars[i] == ',' {
                    out.push_str(CLR_PUNCT);
                    out.push(',');
                    out.push_str(RESET);
                    i += 1;
                }
                continue;
            }
        } else {
            // Inside CSS declaration: property: value;
            if chars[i] == ':' {
                out.push_str(CLR_PUNCT);
                out.push(':');
                out.push_str(RESET);
                i += 1;
                continue;
            }

            if chars[i] == ';' {
                out.push_str(CLR_PUNCT);
                out.push(';');
                out.push_str(RESET);
                i += 1;
                continue;
            }

            // String inside CSS
            if chars[i] == '"' || chars[i] == '\'' {
                let quote = chars[i];
                out.push_str(CLR_STR);
                out.push(quote);
                i += 1;
                while i < len && chars[i] != quote {
                    out.push(chars[i]);
                    i += 1;
                }
                if i < len && chars[i] == quote {
                    out.push(quote);
                    i += 1;
                }
                out.push_str(RESET);
                continue;
            }

            // Property or value token
            if !chars[i].is_whitespace() {
                let mut token = String::new();
                let start_idx = i;
                while i < len && !chars[i].is_whitespace() && chars[i] != ':' && chars[i] != ';' && chars[i] != '}' {
                    token.push(chars[i]);
                    i += 1;
                }

                let is_after_colon = chars[..start_idx].iter().rev().find(|&&c| !c.is_whitespace()) == Some(&':');

                if is_after_colon {
                    // Check if value (number/color/unit/keyword)
                    if token.starts_with('#') || token.ends_with("px") || token.ends_with("em") || token.ends_with("rem") || token.ends_with('%') {
                        out.push_str(CLR_UNIT);
                    } else {
                        out.push_str(CLR_VALUE);
                    }
                } else {
                    out.push_str(CLR_PROPERTY);
                }
                out.push_str(&token);
                out.push_str(RESET);
                continue;
            }
        }

        out.push(chars[i]);
        i += 1;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_code_block_highlighting() {
        let code = r#"<div class="test">Hello</div>"#;
        let highlighted = highlight_html_xml(code);
        assert!(highlighted.contains("\x1b[38;2;86;156;214m<div"));
        assert!(highlighted.contains("\x1b[38;2;156;220;254mclass"));
        assert!(highlighted.contains("\x1b[38;2;206;145;120m\"test\""));
    }

    #[test]
    fn test_css_highlighting() {
        let css = ".card { color: #fff; }";
        let highlighted = highlight_css(css);
        assert!(highlighted.contains("\x1b[38;2;220;220;170m.card"));
        assert!(highlighted.contains("\x1b[38;2;156;220;254mcolor"));
        assert!(highlighted.contains("\x1b[38;2;181;206;168m#fff"));
    }
}
