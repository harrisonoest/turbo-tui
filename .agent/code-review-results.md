# Code Review: TurboTUI - All src/ Modules

## Files Reviewed
- [x] src/config.rs
- [x] src/database.rs
- [x] src/app.rs
- [x] src/ui.rs
- [x] src/main.rs
- [x] Cargo.toml
- [x] config.toml

## Summary
**REQUEST_CHANGES** — One show-stopping input bug, several correctness issues that will cause panics or wrong behavior under normal usage.

## Critical Issues (Must Fix)

- [ ] **src/main.rs:87** — `KeyCode::Char('c')` sets `should_quit = true` without checking for Ctrl modifier. This means **users cannot type the letter 'c'** in the query editor. The handler should check `key.modifiers.contains(KeyModifiers::CONTROL)` before quitting. This is a blocking usability bug.

- [ ] **src/app.rs:55-59** — `insert_char` / `delete_char` / `move_cursor_left` / `move_cursor_right` treat `cursor_position` as a byte index (used with `String::insert` and `String::remove` which take byte indices), but increment/decrement by 1. For multi-byte UTF-8 characters, this will **panic** with "byte index N is not a char boundary". Should use char-aware cursor movement (e.g., `char_indices()`).

- [ ] **src/app.rs:80-85** — `format_query()` replaces `self.query` with reformatted text but does not update `cursor_position`. If the formatted text is shorter than the original, the cursor will point past the end of the string, causing a panic on the next `insert_char` call. Fix: set `self.cursor_position = self.query.len()` after formatting.

- [ ] **src/config.rs + src/database.rs** — `DatabaseConfig` is missing `trusted_connection` and `trust_server_certificate` fields that are documented in the README and present in `config.toml`. The `trust_server_certificate` value is silently ignored — `database.rs:38` always calls `trust_cert()` unconditionally. These config fields should be added to the struct and respected in the connection logic.

## Suggestions (Should Consider)

- **src/database.rs:25-50** — A new TCP connection is established for every single query execution. For a TUI app where users run many queries in a session, consider holding a persistent connection or using connection pooling to reduce latency.

- **src/database.rs:65-85** — The type fallback chain for column values is missing common SQL Server types: `datetime`/`NaiveDateTime`, `Decimal`, `Uuid`, `i16`/`smallint`, `u8`/`tinyint`. The final `&[u8]` fallback produces ugly debug output like `[104, 101, 108, 108, 111]`. Consider adding more type handlers or using a display-friendly fallback.

- **src/ui.rs:48-73** — The cursor positioning in `draw_editor` calculates the visual cursor row by counting `\n` characters, but the editor uses `Wrap { trim: false }`. If a line is longer than the editor width, it wraps visually but the cursor calculation doesn't account for this, placing the cursor on the wrong row.

- **src/ui.rs:107** — Column width calculation `100 / result.columns.len().max(1) as u16` doesn't distribute remainder. For 3 columns: 33% × 3 = 99%, leaving 1% unused. For >100 columns, each gets 0%. Consider using `Constraint::Ratio` or `Constraint::Min` instead.

- **src/main.rs** — No panic hook is installed to restore terminal state. If any code panics, the terminal will be left in raw mode with the alternate screen active, requiring the user to run `reset`. Add a `std::panic::set_hook` that calls `disable_raw_mode` and `LeaveAlternateScreen`.

- **src/app.rs:130,148,163** — Path construction uses `format!("{}/{}", directory, filename)` instead of `std::path::PathBuf::join()`. While functional on Unix, this is not cross-platform idiomatic Rust.

## Nitpicks (Optional)

- **src/main.rs:107-115** — The `Focus::FileBrowser` arm in the main match is dead code (comment says "Handled above"). Could remove the arm and add `Focus::FileBrowser => unreachable!()` or restructure the control flow.

- **src/main.rs** — `save_results` duplicates the F8 handling logic in both `Focus::Editor` and `Focus::Results` match arms. Could extract to a helper or handle F8 before the focus match.

- **config.toml** — Contains real database credentials (`password = "sp3c1@l"`). Should use environment variable substitution or ensure this file is in `.gitignore`.

## Positive Notes

- Clean module separation with clear responsibilities (config, database, app state, UI rendering, event loop)
- Good use of ratatui patterns — proper layout splitting, styled widgets, alternating row colors
- Solid error handling throughout with `anyhow::Result` — errors surface to the user via status bar rather than crashing
- File browser with live preview is excellent UX
- SQL formatting via F6 is a nice productivity feature
- The `App` struct keeps all mutable state in one place, making the data flow easy to follow
