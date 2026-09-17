 `markd` 🚀

> **Universal Terminal Document & Markdown Viewer** powered by Rust and Microsoft MarkItDown.

`markd` is a fast, modern terminal user interface (TUI) for viewing documents directly inside your command line. It natively renders Markdown (`.md`) files with rich formatting and themes, and automatically converts non-Markdown files (`.pdf`, `.docx`, `.pptx`, `.xlsx`, etc.) on the fly into clean Markdown.

---

## ✨ Features

- **Dual Mode Viewing**:
  - **Mode 1 (Native Markdown)**: Zero-overhead rendering for `.md` and `.markdown` files.
  - **Mode 2 (MarkItDown Auto-Conversion)**: Seamlessly converts PDFs, Word documents, PowerPoint presentations, and Excel spreadsheets using `markitdown` in memory.
- **Top-Right Theme Selector (`[ ⚙ ]`)**: Press `t` or `m` inside the TUI to open an interactive theme menu:
  - **VS Code Dark+** *(Default)*
  - **GitHub Light**
  - **Omarchy Minimal** (Catppuccin Pastel Dark)
- **Vim & Standard Navigation**: Full keyboard navigation (`j`/`k`, arrow keys, `PageUp`/`PageDown`, `Home`/`g`).
- **Clean Output Mode**: Use `markd -p <file>` to render markdown directly to stdout.

---

## 💻 Usage

```bash
# View native Markdown file
markd README.md

# View a PDF report
markd research_paper.pdf

# View a PowerPoint presentation
markd slides.pptx

# View an Excel spreadsheet
markd budget.xlsx

# Print directly to stdout (non-interactive)
markd -p document.pdf
```

### Keyboard Controls inside TUI
| Key | Action |
| :--- | :--- |
| **`t`** / **`m`** | Open / Close the top-right **`[ ⚙ ]`** Theme Menu |
| **`1`**, **`2`**, **`3`** | Switch directly to VS Code, Light, or Omarchy theme |
| **`j`** / **`Down`** | Scroll down 1 line |
| **`k`** / **`Up`** | Scroll up 1 line |
| **`Space`** / **`PageDown`** | Scroll down 1 page |
| **`b`** / **`PageUp`** | Scroll up 1 page |
| **`g`** / **`Home`** | Reset scroll position to top |
| **`q`** / **`Esc`** | Exit viewer |
```
