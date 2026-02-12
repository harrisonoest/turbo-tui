# Scratchpad

## 2026-02-12 - Initial Analysis

**Objective:** Save query results to `./results` directory and allow user to name the file in the TUI.

**Finding:** The objective is already fully implemented in the codebase:

1. `save_results()` in `src/main.rs` (line 174-203):
   - Creates `results/` directory via `fs::create_dir_all("results")`
   - Saves both `.json` and `.csv` files to `results/{name}.ext`
   - Falls back to timestamp-based naming if no filename provided

2. `SaveDialog` in `src/app.rs` (line 33-68):
   - Text input with cursor support for typing a filename
   - `open()`, `close()`, `insert_char()`, `delete_char()` methods

3. `draw_save_dialog()` in `src/ui.rs` (line 355-385):
   - Renders a centered dialog with title "Save Results (Enter to save, Esc to cancel)"
   - Shows input field for filename

4. F8 key binding in `src/main.rs` opens the save dialog when results exist.

**Build status:** `cargo check` passes, `cargo test` passes (0 tests, 0 failures).

**Decision:** No implementation work needed. Emitting REFACTOR_COMPLETE.
