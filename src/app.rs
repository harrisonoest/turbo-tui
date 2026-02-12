use crate::database::QueryResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Editor,
    Results,
    FileBrowser,
}

pub struct App {
    pub query: String,
    pub cursor_position: usize,
    pub result: Option<QueryResult>,
    pub status_message: String,
    pub focus: Focus,
    pub scroll_offset: usize,
    pub editor_scroll: usize,
    pub should_quit: bool,
    pub show_help: bool,
    pub file_browser: FileBrowser,
    pub save_dialog: SaveDialog,
}

#[derive(Default)]
pub struct FileBrowser {
    pub files: Vec<String>,
    pub selected_index: usize,
    pub preview: String,
    pub active: bool,
}

#[derive(Default)]
pub struct SaveDialog {
    pub active: bool,
    pub input: String,
    pub cursor: usize,
}

impl SaveDialog {
    pub fn open(&mut self) {
        self.active = true;
        self.input.clear();
        self.cursor = 0;
    }

    pub fn close(&mut self) {
        self.active = false;
        self.input.clear();
        self.cursor = 0;
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    pub fn delete_char(&mut self) {
        if self.cursor > 0 {
            let prev = self.input[..self.cursor]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.input.remove(prev);
            self.cursor = prev;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            query: String::new(),
            cursor_position: 0,
            result: None,
            status_message: "Ready. Press F1 for help.".to_string(),
            focus: Focus::Editor,
            scroll_offset: 0,
            editor_scroll: 0,
            should_quit: false,
            show_help: false,
            file_browser: FileBrowser::default(),
            save_dialog: SaveDialog::default(),
        }
    }
}

impl App {
    pub fn insert_char(&mut self, c: char) {
        self.query.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let prev = self.query[..self.cursor_position]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.query.remove(prev);
            self.cursor_position = prev;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position = self.query[..self.cursor_position]
                .char_indices()
                .next_back()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.query.len() {
            self.cursor_position = self.query[self.cursor_position..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.cursor_position + i)
                .unwrap_or(self.query.len());
        }
    }

    pub fn format_query(&mut self) {
        let formatted = sqlformat::format(
            &self.query,
            &sqlformat::QueryParams::None,
            sqlformat::FormatOptions::default(),
        );
        self.query = formatted;
        self.cursor_position = self.query.len();
        self.status_message = "Query formatted".to_string();
    }

    pub fn set_result(&mut self, result: QueryResult) {
        self.status_message = format!("Query executed: {} rows returned", result.row_count);
        self.result = Some(result);
        self.focus = Focus::Results;
        self.scroll_offset = 0;
    }

    pub fn set_error(&mut self, error: String) {
        self.status_message = format!("Error: {}", error);
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if let Some(result) = &self.result {
            if self.scroll_offset < result.row_count.saturating_sub(1) {
                self.scroll_offset += 1;
            }
        }
    }

    pub fn editor_scroll_up(&mut self) {
        if self.editor_scroll > 0 {
            self.editor_scroll -= 1;
        }
    }

    pub fn editor_scroll_down(&mut self) {
        let line_count = self.query.lines().count();
        if self.editor_scroll < line_count.saturating_sub(1) {
            self.editor_scroll += 1;
        }
    }

    pub fn open_file_browser(&mut self, directory: &str) {
        use std::fs;

        self.file_browser.files.clear();
        self.file_browser.selected_index = 0;
        self.file_browser.preview.clear();

        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "sql" {
                        if let Some(name) = entry.file_name().to_str() {
                            self.file_browser.files.push(name.to_string());
                        }
                    }
                }
            }
            self.file_browser.files.sort();

            if !self.file_browser.files.is_empty() {
                self.update_preview(directory);
            }
        }

        self.file_browser.active = true;
        self.focus = Focus::FileBrowser;
    }

    pub fn update_preview(&mut self, directory: &str) {
        use std::fs;
        use std::path::Path;

        if let Some(filename) = self
            .file_browser
            .files
            .get(self.file_browser.selected_index)
        {
            let path = Path::new(directory).join(filename);
            self.file_browser.preview =
                fs::read_to_string(&path).unwrap_or_else(|_| "Error reading file".to_string());
        }
    }

    pub fn file_browser_up(&mut self, directory: &str) {
        if self.file_browser.selected_index > 0 {
            self.file_browser.selected_index -= 1;
            self.update_preview(directory);
        }
    }

    pub fn file_browser_down(&mut self, directory: &str) {
        if self.file_browser.selected_index < self.file_browser.files.len().saturating_sub(1) {
            self.file_browser.selected_index += 1;
            self.update_preview(directory);
        }
    }

    pub fn load_selected_file(&mut self, directory: &str) {
        if let Some(filename) = self
            .file_browser
            .files
            .get(self.file_browser.selected_index)
        {
            let path = std::path::Path::new(directory).join(filename);
            if let Ok(content) = std::fs::read_to_string(&path) {
                self.query = content;
                self.cursor_position = self.query.len();
                self.status_message = format!("Loaded {}", filename);
            }
        }
        self.file_browser.active = false;
        self.focus = Focus::Editor;
    }

    pub fn close_file_browser(&mut self) {
        self.file_browser.active = false;
        self.focus = Focus::Editor;
    }
}
