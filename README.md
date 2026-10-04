# markd

<p align="center">
  <img src="assets/markd-logo.png" alt="markd logo" width="180">
</p>

<p align="center">
  <b>Universal Terminal Document & Markdown Viewer & Editor</b><br>
  Fast, modern document viewing and editing directly in your terminal.
</p>

---

## What is markd?

`markd` is a fast, keyboard-driven terminal document viewer and interactive text editor written in Rust.

It combines two seamless modes:
1. **Document Viewer**: Fast Markdown rendering with 10 developer-focused themes, Vim-style navigation, and automatic conversion of office documents (PDF, Word, Excel, PowerPoint) into formatted Markdown via [Microsoft MarkItDown](https://github.com/microsoft/markitdown).
2. **Built-in Interactive Editor**: Full-featured text editor with live cursor, line-number gutter, undo/redo, smart auto-indentation, line duplication/deletion, and instant disk saving.

## How it works

```text
Your document
     │
     ├── Markdown (.md, .txt) ────► markd
     │                                │
     └── PDF / DOCX / PPTX            ├───► Viewer Mode (Themes, Scrolling, Search)
         / XLSX / etc.                │
              │                       └───► Editor Mode (Undo, Auto-Indent, Save)
              ▼
         MarkItDown
              │
              ▼
          Markdown
```

For non-Markdown files, `markd` automatically invokes an isolated MarkItDown environment to convert documents on the fly.

## What it does

- **Dual-Mode Workflow**: Switch seamlessly between document viewing and full-screen editing with a single keystroke (`e` to edit, `Esc` to view).
- **Built-in Interactive Editor**: Live cursor positioning, line numbers, undo/redo history (`Ctrl+Z` / `Ctrl+Y`), line deletion (`Ctrl+K`), and line duplication (`Ctrl+D`).
- **Smart Auto-Indent**: Automatically preserves indentation whitespace and continues Markdown list bullets (`- `, `* `, `1. `) upon pressing `Enter`.
- **Universal Document Conversion**: View PDFs, Word documents, Excel spreadsheets, and PowerPoint presentations.
- **Code Syntax Highlighting**: Fenced code blocks (`html`, `css`, `xml`, etc.) rendered with TrueColor ANSI syntax highlighting.
- **10 Built-in Themes**: Switch themes live via mouse or keyboard shortcuts.
- **Mouse & Vim Navigation**: Scroll with mouse wheel, click to position cursor in editor, or navigate with Vim keys (`j`/`k`, `g`/`G`).
- **Print Mode**: Use `-p` to pipe converted Markdown directly to stdout.

## Installation

### From Debian Package (`.deb`)

Download or use the generated package:

```bash
sudo dpkg -i markd.deb
```

### From Source via Cargo

```bash
git clone https://github.com/Rktim/markd.git
cd markd
cargo install --path . --force
```

### Build Binary

```bash
cargo build --release
./target/release/markd --help
```

## Usage

```bash
# View documents in the terminal
markd README.md
markd report.pdf
markd notes.docx
markd budget.xlsx

# Open directly into the built-in editor
markd -e notes.md

# Print converted markdown directly to stdout
markd -p document.pdf

# Check version
markd -v
```

## Controls

### Viewer Mode

| Key | Action |
|---|---|
| `e` | Open built-in interactive editor |
| `t` / `m` | Open theme selector menu |
| `1`–`9` | Select theme by index |
| `0` | Select theme 10 |
| `j` / `↓` | Scroll down one line |
| `k` / `↑` | Scroll up one line |
| `f` / `PageDown` / `Space` | Page down |
| `b` / `PageUp` | Page up |
| `g` / `Home` | Go to top of document |
| `G` | Go to bottom of document |
| `q` / `Esc` | Quit markd |

### Editor Mode

| Key | Action |
|---|---|
| `Ctrl+S` | Save changes to disk |
| `Ctrl+Z` | Undo last change |
| `Ctrl+Y` | Redo change |
| `Ctrl+K` | Delete current line |
| `Ctrl+D` | Duplicate current line |
| `Ctrl+A` / `Home` | Jump to line start |
| `Ctrl+E` / `End` | Jump to line end |
| `Tab` | Indent (4 spaces) |
| `Enter` | Smart auto-indent (preserves indent & bullet lists) |
| `Esc` / `Ctrl+Q` | Return to document viewer |

### Mouse Controls

- Click **`[ ✎ Edit ]`** in header to enter editor mode
- Click any line inside the editor to position cursor
- Hover **`⚙`** to open the theme menu
- Click any theme in the menu to apply it
- Click outside the menu to dismiss

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

## License

MIT
