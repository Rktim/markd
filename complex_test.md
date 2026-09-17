# markd — Complex Markdown Test

A stress-test document for the **markd** terminal document viewer.

## 1. Application Architecture

```text
┌─────────────────────────────────────────────────────────────────────┐
│                             markd CLI                               │
└──────────────────────────────────┬──────────────────────────────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    │                             │
              Is Markdown?                    Other format
                    │                             │
              ┌─────▼─────┐              ┌────────▼─────────┐
              │ Read file │              │ Ensure MarkItDown│
              │ directly  │              │    available     │
              └─────┬─────┘              └────────┬─────────┘
                    │                             │
                    │                    ┌────────▼─────────┐
                    │                    │ Convert document  │
                    │                    │   → Markdown      │
                    │                    └────────┬─────────┘
                    │                             │
                    └──────────────┬──────────────┘
                                   │
                            ┌──────▼──────┐
                            │  Markdown   │
                            │   content   │
                            └──────┬──────┘
                                   │
                            ┌──────▼──────┐
                            │   Termimad  │
                            │   renderer  │
                            └──────┬──────┘
                                   │
                            ┌──────▼──────┐
                            │    TUI      │
                            │  + Themes   │
                            └─────────────┘
```

## 2. End-to-End Runtime Flow

```text
User
 │
 │ markd report.pdf
 ▼
CLI argument parser
 │
 ▼
File existence check
 │
 ├── .md / .markdown ────────────────┐
 │                                    │
 └── other extension                  │
          │                           │
          ▼                           │
   ensure_markitdown()                │
          │                           │
          ├── PATH executable? ───────┤
          │                           │
          └── private venv?           │
                  │                   │
                  ├── exists ─────────┤
                  │                   │
                  └── missing         │
                       │              │
                       ▼              │
                  python3 -m venv     │
                       │              │
                       ▼              │
             pip install markitdown   │
                       │              │
                       └──────────────┤
                                      ▼
                              MarkItDown conversion
                                      │
                                      ▼
                              Markdown string
                                      │
                                      ▼
                                MadView / TUI
```

## 3. Component Breakdown

| Component | Responsibility | Technology |
|---|---|---|
| CLI | Parse arguments and flags | Rust `std::env` |
| Loader | Read or convert documents | Rust + MarkItDown |
| Converter | Convert Office/PDF/etc. | MarkItDown |
| Renderer | Render Markdown | Termimad |
| Terminal I/O | Keyboard, screen, colors | Crossterm |
| Theme system | Visual styles | `MadSkin` |
| Viewer | Scrolling and navigation | `MadView` |
| Packaging | Debian/RPM distribution | `cargo-deb`, `generate-rpm` |

## 4. Supported Input Flow

### Markdown

```text
README.md
   │
   ▼
fs::read_to_string()
   │
   ▼
Markdown
   │
   ▼
MadView
```

### PDF

```text
report.pdf
   │
   ▼
MarkItDown
   │
   ▼
Markdown
   │
   ▼
MadView
```

### DOCX

```text
document.docx
   │
   ▼
MarkItDown
   │
   ▼
Markdown
   │
   ▼
MadView
```

### XLSX

```text
data.xlsx
   │
   ▼
MarkItDown
   │
   ▼
Markdown
   │
   ▼
MadView
```

## 5. Theme System

`markd` provides three themes:

1. **VS Code Dark+**
   - Cyan/blue headings
   - Dark code blocks
   - Blue bullets

2. **GitHub Light**
   - Blue headings
   - Light code blocks
   - High contrast text

3. **Omarchy Minimal**
   - Cyan/purple headings
   - Dark muted backgrounds
   - Minimal terminal aesthetic

### Theme state machine

```text
                   ┌───────────────┐
                   │   VS Code     │
                   └───────┬───────┘
                           │ 2
                           ▼
                   ┌───────────────┐
              ┌────│ GitHub Light  │
              │    └───────┬───────┘
              │            │ 3
              │            ▼
              │    ┌───────────────┐
              └────│    Omarchy    │
                 1  └───────────────┘
```

## 6. Viewer Controls

| Key | Action |
|---|---|
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `f` / `Space` | Page down |
| `b` / `PageUp` | Page up |
| `g` / `Home` | Jump to top |
| `t` / `m` | Open theme menu |
| `1` | VS Code theme |
| `2` | GitHub Light |
| `3` | Omarchy |
| `Enter` | Select highlighted theme |
| `Esc` | Close menu / quit |
| `q` | Quit |

## 7. Nested Content

### Level 1

- First item
  - Nested item
    - Deeply nested item
      - Very deeply nested item

### Level 2

1. First step
   1. Sub-step A
   2. Sub-step B
      1. Deep step
      2. Another deep step
2. Second step
3. Third step

## 8. Text Formatting

This paragraph contains **bold**, *italic*, ***bold italic***, `inline code`,
and ~~strikethrough~~ formatting.

> This is a blockquote.
>
> It spans multiple lines and should remain visually distinct.

---

## 9. Code Block

```rust
use std::process::Command;

fn convert_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("markitdown")
        .arg(path)
        .output()?;

    if !output.status.success() {
        return Err("conversion failed".into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

fn main() {
    match convert_file("document.pdf") {
        Ok(markdown) => println!("{markdown}"),
        Err(error) => eprintln!("error: {error}"),
    }
}
```

## 10. Long Paragraph Test

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent vitae
ligula eget neque porta vulputate. Integer tincidunt, lacus at consequat
ultricies, lorem justo facilisis neque, vitae posuere magna erat vitae
nibh. Suspendisse potenti. Donec commodo, neque at consectetur malesuada,
urna libero aliquet justo, sed efficitur purus lorem sed nisl.

## 11. Links

- Project repository: <https://github.com/yourusername/markd>
- Rust: <https://www.rust-lang.org/>
- MarkItDown: <https://github.com/microsoft/markitdown>
- Termimad: <https://docs.rs/termimad/>

## 12. Final Integration Test

```text
┌───────────┐
│ sample.md │
└─────┬─────┘
      │
      ▼
┌─────────────┐
│ markd CLI   │
└─────┬───────┘
      │
      ▼
┌─────────────────────┐
│ Markdown detection  │
└─────┬───────────────┘
      │
      ▼
┌─────────────────────┐
│ Termimad MadView    │
└─────┬───────────────┘
      │
      ▼
┌─────────────────────┐
│ Interactive TUI     │
│ Scroll + Themes     │
└─────────────────────┘
```

## 13. Expected Result

The entire document should:

- render without crashing;
- preserve heading hierarchy;
- render lists and blockquotes;
- display code blocks;
- preserve tables as well as Termimad supports them;
- allow scrolling through the complete document;
- allow theme switching without losing the current scroll position.

**End of stress test.**
