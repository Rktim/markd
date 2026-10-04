use crossterm::{
    cursor::{MoveTo, Show},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::error::Error;
use std::fs;
use std::io::{Stdout, Write};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct HistoryState {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
}

pub struct Editor {
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_row: usize,
    pub scroll_col: usize,
    pub file_path: String,
    pub is_dirty: bool,
    pub status_message: Option<(String, Instant)>,
    undo_stack: Vec<HistoryState>,
    redo_stack: Vec<HistoryState>,
}

impl Editor {
    pub fn new(content: &str, file_path: String) -> Self {
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };

        Self {
            lines: if lines.is_empty() { vec![String::new()] } else { lines },
            cursor_row: 0,
            cursor_col: 0,
            scroll_row: 0,
            scroll_col: 0,
            file_path,
            is_dirty: false,
            status_message: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn get_content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some((msg.into(), Instant::now()));
    }

    fn push_history(&mut self) {
        if self.undo_stack.len() >= 100 {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(HistoryState {
            lines: self.lines.clone(),
            cursor_row: self.cursor_row,
            cursor_col: self.cursor_col,
        });
        self.redo_stack.clear();
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let content = self.get_content();
        fs::write(&self.file_path, &content)?;
        self.is_dirty = false;
        let filename = self.file_path.rsplit('/').next().unwrap_or(&self.file_path);
        self.set_status(format!("✓ Saved to {}", filename));
        Ok(())
    }

    pub fn insert_char(&mut self, c: char) {
        self.push_history();
        if self.cursor_row >= self.lines.len() {
            self.lines.push(String::new());
        }

        let line = &mut self.lines[self.cursor_row];
        let mut chars: Vec<char> = line.chars().collect();
        let col = self.cursor_col.min(chars.len());
        chars.insert(col, c);
        *line = chars.into_iter().collect();
        self.cursor_col = col + 1;
        self.is_dirty = true;
    }

    pub fn insert_tab(&mut self) {
        self.push_history();
        for _ in 0..4 {
            if self.cursor_row >= self.lines.len() {
                self.lines.push(String::new());
            }

            let line = &mut self.lines[self.cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            let col = self.cursor_col.min(chars.len());
            chars.insert(col, ' ');
            *line = chars.into_iter().collect();
            self.cursor_col = col + 1;
        }
        self.is_dirty = true;
    }

    pub fn insert_newline(&mut self) {
        self.push_history();
        if self.cursor_row >= self.lines.len() {
            self.lines.push(String::new());
        }

        let line = &self.lines[self.cursor_row];
        let chars: Vec<char> = line.chars().collect();
        let col = self.cursor_col.min(chars.len());

        let before: String = chars[..col].iter().collect();
        let after: String = chars[col..].iter().collect();

        // Calculate auto-indent prefix
        let indent_prefix = {
            let trimmed = before.trim_start();
            let leading_spaces = before.len() - trimmed.len();
            let mut prefix = " ".repeat(leading_spaces);

            if trimmed.starts_with("- [ ] ") || trimmed.starts_with("- [x] ") {
                prefix.push_str("- [ ] ");
            } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                prefix.push_str(&trimmed[..2]);
            } else if let Some(dot_pos) = trimmed.find(". ") {
                if dot_pos <= 3 && trimmed[..dot_pos].chars().all(|c| c.is_ascii_digit()) {
                    if let Ok(num) = trimmed[..dot_pos].parse::<usize>() {
                        prefix.push_str(&format!("{}. ", num + 1));
                    }
                }
            }
            prefix
        };

        // If current line is just an empty list bullet ("-", "*", "- [ ]"), clear it
        let trimmed_before = before.trim();
        if trimmed_before == "-" || trimmed_before == "*" || trimmed_before == "- [ ]" {
            self.lines[self.cursor_row] = String::new();
            self.cursor_col = 0;
            self.is_dirty = true;
            return;
        }

        self.lines[self.cursor_row] = before;
        let new_line = format!("{}{}", indent_prefix, after);
        let new_col = indent_prefix.chars().count();
        self.lines.insert(self.cursor_row + 1, new_line);

        self.cursor_row += 1;
        self.cursor_col = new_col;
        self.is_dirty = true;
    }

    pub fn backspace(&mut self) {
        if self.cursor_row >= self.lines.len() {
            return;
        }

        self.push_history();

        if self.cursor_col > 0 {
            let line = &mut self.lines[self.cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            let col = self.cursor_col.min(chars.len());
            if col > 0 {
                chars.remove(col - 1);
                *line = chars.into_iter().collect();
                self.cursor_col = col - 1;
                self.is_dirty = true;
            }
        } else if self.cursor_row > 0 {
            // Merge with previous line
            let current_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            let prev_len = self.lines[self.cursor_row].chars().count();
            self.lines[self.cursor_row].push_str(&current_line);
            self.cursor_col = prev_len;
            self.is_dirty = true;
        }
    }

    pub fn delete(&mut self) {
        if self.cursor_row >= self.lines.len() {
            return;
        }

        self.push_history();

        let line_len = self.lines[self.cursor_row].chars().count();
        if self.cursor_col < line_len {
            let line = &mut self.lines[self.cursor_row];
            let mut chars: Vec<char> = line.chars().collect();
            chars.remove(self.cursor_col);
            *line = chars.into_iter().collect();
            self.is_dirty = true;
        } else if self.cursor_row + 1 < self.lines.len() {
            // Merge next line into current line
            let next_line = self.lines.remove(self.cursor_row + 1);
            self.lines[self.cursor_row].push_str(&next_line);
            self.is_dirty = true;
        }
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(HistoryState {
                lines: self.lines.clone(),
                cursor_row: self.cursor_row,
                cursor_col: self.cursor_col,
            });
            self.lines = prev.lines;
            self.cursor_row = prev.cursor_row.min(self.lines.len().saturating_sub(1));
            self.cursor_col = prev.cursor_col.min(self.current_line_len());
            self.is_dirty = true;
            self.set_status("↶ Undo");
        } else {
            self.set_status("Already at oldest change");
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(HistoryState {
                lines: self.lines.clone(),
                cursor_row: self.cursor_row,
                cursor_col: self.cursor_col,
            });
            self.lines = next.lines;
            self.cursor_row = next.cursor_row.min(self.lines.len().saturating_sub(1));
            self.cursor_col = next.cursor_col.min(self.current_line_len());
            self.is_dirty = true;
            self.set_status("↷ Redo");
        } else {
            self.set_status("Already at newest change");
        }
    }

    pub fn delete_line(&mut self) {
        self.push_history();
        if self.lines.len() > 1 {
            self.lines.remove(self.cursor_row);
            if self.cursor_row >= self.lines.len() {
                self.cursor_row = self.lines.len() - 1;
            }
        } else {
            self.lines[0].clear();
            self.cursor_col = 0;
        }
        self.clamp_cursor_col();
        self.is_dirty = true;
        self.set_status("Line deleted");
    }

    pub fn duplicate_line(&mut self) {
        self.push_history();
        let current = self.lines[self.cursor_row].clone();
        self.lines.insert(self.cursor_row + 1, current);
        self.cursor_row += 1;
        self.is_dirty = true;
        self.set_status("Line duplicated");
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.clamp_cursor_col();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
            self.clamp_cursor_col();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].chars().count();
        }
    }

    pub fn move_cursor_right(&mut self) {
        let line_len = self.current_line_len();
        if self.cursor_col < line_len {
            self.cursor_col += 1;
        } else if self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
            self.cursor_col = 0;
        }
    }

    pub fn move_to_line_start(&mut self) {
        self.cursor_col = 0;
    }

    pub fn move_to_line_end(&mut self) {
        self.cursor_col = self.current_line_len();
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.cursor_row = self.cursor_row.saturating_sub(page_size);
        self.clamp_cursor_col();
    }

    pub fn page_down(&mut self, page_size: usize) {
        self.cursor_row = (self.cursor_row + page_size).min(self.lines.len().saturating_sub(1));
        self.clamp_cursor_col();
    }

    pub fn click_at(&mut self, screen_col: u16, screen_row: u16, gutter_w: usize) {
        if screen_row == 0 {
            return;
        }
        let target_line = self.scroll_row + (screen_row as usize).saturating_sub(1);
        if target_line < self.lines.len() {
            self.cursor_row = target_line;
            let text_col = (screen_col as usize).saturating_sub(gutter_w);
            self.cursor_col = (self.scroll_col + text_col).min(self.lines[target_line].chars().count());
        }
    }

    fn current_line_len(&self) -> usize {
        self.lines.get(self.cursor_row).map(|l| l.chars().count()).unwrap_or(0)
    }

    fn clamp_cursor_col(&mut self) {
        let max_col = self.current_line_len();
        if self.cursor_col > max_col {
            self.cursor_col = max_col;
        }
    }

    pub fn gutter_width(&self) -> usize {
        let digits = self.lines.len().to_string().len().max(2);
        digits + 3 // e.g. " 12 │ "
    }

    pub fn adjust_viewport(&mut self, view_width: usize, view_height: usize) {
        // Vertical viewport scroll
        if self.cursor_row < self.scroll_row {
            self.scroll_row = self.cursor_row;
        } else if self.cursor_row >= self.scroll_row + view_height {
            self.scroll_row = self.cursor_row.saturating_sub(view_height - 1);
        }

        // Horizontal viewport scroll
        if self.cursor_col < self.scroll_col {
            self.scroll_col = self.cursor_col;
        } else if self.cursor_col >= self.scroll_col + view_width {
            self.scroll_col = self.cursor_col.saturating_sub(view_width - 1);
        }
    }

    pub fn render(
        &mut self,
        stdout: &mut Stdout,
        term_width: u16,
        term_height: u16,
    ) -> Result<(), Box<dyn Error>> {
        let view_height = (term_height as usize).saturating_sub(2);
        let gutter_w = self.gutter_width();
        let view_width = (term_width as usize).saturating_sub(gutter_w);

        self.adjust_viewport(view_width, view_height);

        stdout.queue(Clear(ClearType::All))?;

        // 1. Render Header
        self.render_header(stdout, term_width)?;

        // 2. Render Editor Body
        let digits = self.lines.len().to_string().len().max(2);

        for row in 0..view_height {
            let line_index = self.scroll_row + row;
            let screen_y = 1 + row as u16;

            stdout.queue(MoveTo(0, screen_y))?;

            if line_index < self.lines.len() {
                // Line Number Gutter
                let is_current = line_index == self.cursor_row;
                if is_current {
                    stdout.queue(SetForegroundColor(Color::Rgb { r: 255, g: 215, b: 0 }))?; // Gold
                    stdout.queue(SetBackgroundColor(Color::Rgb { r: 40, g: 42, b: 54 }))?;
                } else {
                    stdout.queue(SetForegroundColor(Color::Rgb { r: 100, g: 110, b: 130 }))?;
                    stdout.queue(SetBackgroundColor(Color::Rgb { r: 24, g: 24, b: 32 }))?;
                }

                let gutter_str = format!(" {:>width$} │ ", line_index + 1, width = digits);
                stdout.queue(Print(gutter_str))?;

                // Line Content
                stdout.queue(ResetColor)?;
                let line = &self.lines[line_index];
                let chars: Vec<char> = line.chars().collect();
                let visible_chars: String = if self.scroll_col < chars.len() {
                    chars[self.scroll_col..]
                        .iter()
                        .take(view_width)
                        .collect()
                } else {
                    String::new()
                };

                let padding = view_width.saturating_sub(visible_chars.chars().count());
                stdout.queue(Print(visible_chars))?;
                stdout.queue(Print(" ".repeat(padding)))?;
            } else {
                // Empty lines past EOF
                stdout.queue(SetForegroundColor(Color::Rgb { r: 70, g: 70, b: 85 }))?;
                stdout.queue(Print(format!(" {:>width$} ~ ", "", width = digits)))?;
                stdout.queue(ResetColor)?;
                stdout.queue(Print(" ".repeat(view_width)))?;
            }
        }

        // 3. Render Status Bar
        self.render_status_bar(stdout, term_width, term_height)?;

        // 4. Place terminal cursor at active editor position
        let screen_x = (gutter_w + self.cursor_col.saturating_sub(self.scroll_col)) as u16;
        let screen_y = (1 + self.cursor_row.saturating_sub(self.scroll_row)) as u16;

        if screen_x < term_width && screen_y < term_height.saturating_sub(1) {
            stdout.queue(MoveTo(screen_x, screen_y))?;
            stdout.queue(Show)?;
        }

        stdout.flush()?;
        Ok(())
    }

    fn render_header(&self, stdout: &mut Stdout, width: u16) -> Result<(), Box<dyn Error>> {
        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(SetBackgroundColor(Color::Rgb { r: 45, g: 30, b: 60 }))?; // Dark Purple/Maroon
        stdout.queue(SetForegroundColor(Color::Rgb { r: 240, g: 240, b: 250 }))?;

        let filename = self.file_path.rsplit('/').next().unwrap_or(&self.file_path);
        let dirty_badge = if self.is_dirty { " [MODIFIED *]" } else { "" };
        let left = format!(" markd │ ✎ EDIT MODE │ {}{}", filename, dirty_badge);
        let right = "Ctrl+S: Save │ Esc: Return to View ";

        let left_len = left.chars().count();
        let right_len = right.chars().count();
        let padding = (width as usize).saturating_sub(left_len + right_len);

        stdout.queue(Print(format!("{}{}{}", left, " ".repeat(padding), right)))?;
        stdout.queue(ResetColor)?;
        Ok(())
    }

    fn render_status_bar(&self, stdout: &mut Stdout, width: u16, height: u16) -> Result<(), Box<dyn Error>> {
        let y = height.saturating_sub(1);
        stdout.queue(MoveTo(0, y))?;
        stdout.queue(SetBackgroundColor(Color::Rgb { r: 30, g: 30, b: 38 }))?;
        stdout.queue(SetForegroundColor(Color::Rgb { r: 180, g: 180, b: 195 }))?;

        let left = if let Some((ref msg, time)) = self.status_message {
            if time.elapsed() < Duration::from_secs(3) {
                format!(" {}", msg)
            } else {
                format!(
                    " Ln {}, Col {} │ {} lines │ UTF-8",
                    self.cursor_row + 1,
                    self.cursor_col + 1,
                    self.lines.len()
                )
            }
        } else {
            format!(
                " Ln {}, Col {} │ {} lines │ UTF-8",
                self.cursor_row + 1,
                self.cursor_col + 1,
                self.lines.len()
            )
        };

        let right = "^S Save  ^Z Undo  ^Y Redo  ^K Del  ^Q/Esc View  ";
        let left_len = left.chars().count();
        let right_len = right.chars().count();
        let padding = (width as usize).saturating_sub(left_len + right_len);

        let line = format!("{}{}{}", left, " ".repeat(padding), right);
        let truncated: String = line.chars().take(width as usize).collect();
        stdout.queue(Print(truncated))?;
        stdout.queue(ResetColor)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_typing_and_newline() {
        let mut editor = Editor::new("Hello", "test.md".to_string());
        editor.move_to_line_end();
        editor.insert_char(' ');
        editor.insert_char('W');
        editor.insert_char('o');
        editor.insert_char('r');
        editor.insert_char('l');
        editor.insert_char('d');
        assert_eq!(editor.lines[0], "Hello World");
        assert!(editor.is_dirty);

        editor.insert_newline();
        assert_eq!(editor.lines.len(), 2);
        assert_eq!(editor.cursor_row, 1);
        assert_eq!(editor.cursor_col, 0);

        editor.insert_char('!');
        assert_eq!(editor.lines[1], "!");
    }

    #[test]
    fn test_editor_backspace_and_merge() {
        let mut editor = Editor::new("Line 1\nLine 2", "test.md".to_string());
        editor.cursor_row = 1;
        editor.cursor_col = 0;
        editor.backspace();
        assert_eq!(editor.lines.len(), 1);
        assert_eq!(editor.lines[0], "Line 1Line 2");
        assert_eq!(editor.cursor_row, 0);
        assert_eq!(editor.cursor_col, 6);
    }

    #[test]
    fn test_editor_undo_redo() {
        let mut editor = Editor::new("Start", "test.md".to_string());
        editor.move_to_line_end();
        editor.insert_char('!');
        assert_eq!(editor.lines[0], "Start!");

        editor.undo();
        assert_eq!(editor.lines[0], "Start");

        editor.redo();
        assert_eq!(editor.lines[0], "Start!");
    }

    #[test]
    fn test_editor_delete_and_duplicate_line() {
        let mut editor = Editor::new("Line A\nLine B", "test.md".to_string());
        editor.cursor_row = 0;
        editor.duplicate_line();
        assert_eq!(editor.lines.len(), 3);
        assert_eq!(editor.lines[0], "Line A");
        assert_eq!(editor.lines[1], "Line A");

        editor.delete_line();
        assert_eq!(editor.lines.len(), 2);
    }

    #[test]
    fn test_editor_smart_auto_indent() {
        let mut editor = Editor::new("    indented text", "test.md".to_string());
        editor.move_to_line_end();
        editor.insert_newline();
        assert_eq!(editor.lines.len(), 2);
        assert_eq!(editor.lines[1], "    ");
        assert_eq!(editor.cursor_col, 4);

        let mut list_editor = Editor::new("- Item 1", "test.md".to_string());
        list_editor.move_to_line_end();
        list_editor.insert_newline();
        assert_eq!(list_editor.lines[1], "- ");
        assert_eq!(list_editor.cursor_col, 2);
    }
}
