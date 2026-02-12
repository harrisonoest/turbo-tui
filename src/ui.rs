use crate::app::{App, Focus};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    if app.show_help {
        draw_help(f);
        return;
    }

    if app.file_browser.active {
        draw_file_browser(f, app);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(60),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_editor(f, app, chunks[0]);
    draw_results(f, app, chunks[1]);
    draw_status(f, app, chunks[2]);

    if app.save_dialog.active {
        draw_save_dialog(f, app);
    }
}

fn draw_editor(f: &mut Frame, app: &App, area: Rect) {
    let style = if app.focus == Focus::Editor {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Blue)
    };

    let block = Block::default()
        .title("SQL Query Editor")
        .borders(Borders::ALL)
        .border_style(style);

    let inner_area = block.inner(area);
    let visible_height = inner_area.height as usize;

    // Get lines and apply scroll offset
    let lines: Vec<&str> = app.query.lines().collect();
    let visible_lines: Vec<&str> = lines
        .iter()
        .skip(app.editor_scroll)
        .take(visible_height)
        .copied()
        .collect();

    let text = visible_lines.join("\n");
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);

    // Show cursor when editor is focused
    if app.focus == Focus::Editor {
        let lines_before_cursor = app.query[..app.cursor_position].matches('\n').count();
        let line_start = app.query[..app.cursor_position]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let col = app.cursor_position - line_start;

        // Adjust cursor position for scroll
        if lines_before_cursor >= app.editor_scroll
            && lines_before_cursor < app.editor_scroll + visible_height
        {
            f.set_cursor_position((
                area.x + col as u16 + 1,
                area.y + (lines_before_cursor - app.editor_scroll) as u16 + 1,
            ));
        }
    }
}

fn draw_results(f: &mut Frame, app: &App, area: Rect) {
    let style = if app.focus == Focus::Results {
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Blue)
    };

    let block = Block::default()
        .title(format!(
            "Results{}",
            app.result
                .as_ref()
                .map(|r| format!(" ({} rows)", r.row_count))
                .unwrap_or_default()
        ))
        .borders(Borders::ALL)
        .border_style(style);

    if let Some(result) = &app.result {
        let header_cells = result.columns.iter().map(|h| {
            Cell::from(h.as_str()).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        });
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows = result
            .rows
            .iter()
            .skip(app.scroll_offset)
            .take(area.height.saturating_sub(4) as usize)
            .enumerate()
            .map(|(i, row)| {
                let cells = row.iter().map(|c| Cell::from(c.as_str()));
                let color = if i % 2 == 0 {
                    Color::White
                } else {
                    Color::Cyan
                };
                Row::new(cells).height(1).style(Style::default().fg(color))
            });

        let widths = vec![
            Constraint::Percentage(100 / result.columns.len().max(1) as u16);
            result.columns.len()
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(block)
            .column_spacing(1);

        f.render_widget(table, area);
    } else {
        f.render_widget(block, area);
    }
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let status_text = vec![
        Line::from(Span::styled(
            app.status_message.as_str(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled(
                "F1",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":Help "),
            Span::styled(
                "F5",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":Execute "),
            Span::styled(
                "F6",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":Format "),
            Span::styled(
                "F7",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":Load "),
            Span::styled(
                "F8",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":Save "),
            Span::styled(
                "Tab",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(":Switch "),
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(":Quit"),
        ]),
    ];

    let paragraph = Paragraph::new(status_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)),
    );

    f.render_widget(paragraph, area);
}

fn draw_help(f: &mut Frame) {
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Keyboard Shortcuts:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  F1",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("         - Toggle this help screen"),
        ]),
        Line::from(vec![
            Span::styled(
                "  F5",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("         - Execute SQL query"),
        ]),
        Line::from(vec![
            Span::styled(
                "  F6",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("         - Format SQL query"),
        ]),
        Line::from(vec![
            Span::styled(
                "  F7",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("         - Load query from file"),
        ]),
        Line::from(vec![
            Span::styled(
                "  F8",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("         - Save results to file"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Tab",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("        - Switch focus between editor and results"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw("        - Quit application"),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↑/↓",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("        - Scroll results (when focused)"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Backspace",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  - Delete character (in editor)"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press F1 or Esc to close this help screen",
            Style::default().fg(Color::Green),
        )),
    ];

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        );

    let paragraph = Paragraph::new(help_text).block(block);

    let area = centered_rect(60, 60, f.area());
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_save_dialog(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 30, f.area());

    f.render_widget(Clear, area);

    let block = Block::default()
        .title("Save Results (Enter to save, Esc to cancel)")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Rgb(20, 20, 30)));

    let input_display = if app.save_dialog.input.is_empty() {
        " ".to_string()
    } else {
        app.save_dialog.input.clone()
    };

    let text = vec![
        Line::from(""),
        Line::from("Filename (leave empty for timestamp):"),
        Line::from(""),
        Line::from(Span::styled(
            format!(" {:<width$}", input_display, width = (area.width as usize).saturating_sub(4)),
            Style::default().fg(Color::White).bg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::reset().fg(Color::Cyan).bg(Color::Rgb(20, 20, 30)));

    f.render_widget(paragraph, area);

    // Position cursor in the input field
    let inner = Rect::new(area.x + 2, area.y + 4, area.width - 3, 1);
    f.set_cursor_position((inner.x + app.save_dialog.cursor as u16, inner.y));
}

fn draw_file_browser(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    // File list
    let file_items: Vec<Line> = app
        .file_browser
        .files
        .iter()
        .enumerate()
        .map(|(i, name)| {
            if i == app.file_browser.selected_index {
                Line::from(format!("> {}", name)).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Line::from(format!("  {}", name)).style(Style::default().fg(Color::Cyan))
            }
        })
        .collect();

    let file_list = Paragraph::new(file_items).block(
        Block::default()
            .title("SQL Files")
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
    );

    f.render_widget(file_list, chunks[0]);

    // Preview
    let preview = Paragraph::new(app.file_browser.preview.as_str())
        .block(
            Block::default()
                .title("Preview")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: false });

    f.render_widget(preview, chunks[1]);
}
