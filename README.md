# markd

<p align="center">
  <img src="assets/markd-logo.png" alt="markd logo" width="180">
</p>

<p align="center">
  <b>Universal Terminal Document & Markdown Viewer</b><br>
  Fast, modern document viewing directly in your terminal.
</p>

---

## What is markd?

`markd` is a Rust-based terminal viewer for Markdown and common document formats.

It renders Markdown directly and uses [Microsoft MarkItDown](https://github.com/microsoft/markitdown) to convert supported documents such as PDF, Word, PowerPoint, and Excel files into Markdown before displaying them.

## How it works

```text
Your document
     │
     ├── Markdown ──────────► markd
     │
     └── PDF / DOCX / PPTX
         / XLSX / etc.
              │
              ▼
        MarkItDown
              │
              ▼
          Markdown
              │
              ▼
            markd
              │
              ▼
          Terminal TUI
```

For non-Markdown files, `markd` can automatically create an isolated environment for MarkItDown when it is needed.

## What it does

- View Markdown files in the terminal
- View PDFs
- View Word documents
- View Excel spreadsheets
- View PowerPoint presentations
- Switch between developer-focused themes
- Use the mouse for the theme menu
- Use Vim-style keyboard navigation
- Print converted content with `-p`

### Usage

```bash
markd README.md
markd report.pdf
markd document.docx
markd data.xlsx
markd slides.pptx
```

Print mode:

```bash
markd -p document.pdf
```

## Controls

| Key | Action |
|---|---|
| `t` / `m` | Open theme menu |
| `1`–`9` | Select themes 1–9 |
| `0` | Select theme 10 |
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `f` / `PageDown` | Page down |
| `b` / `PageUp` | Page up |
| `Space` | Page down |
| `g` / `Home` | Go to top |
| `G` | Go to bottom |
| `Esc` | Exit |

### Mouse

- Hover `⚙` to open the theme menu
- Hover a theme to highlight it
- Click a theme to apply it
- Click outside the menu to close it

## Themes

- VS Code Dark+
- GitHub Dark
- GitHub Light
- Dracula
- Catppuccin
- Nord
- Gruvbox
- Tokyo Night
- One Dark
- Omarchy

## Installation

Prebuilt `.deb`, `.rpm`, and Linux binary releases are available from the GitHub Releases page.

Build from source:

```bash
git clone https://github.com/Rktim/markd.git
cd markd
cargo build --release
```

Run:

```bash
./target/release/markd README.md
```

## License

MIT
