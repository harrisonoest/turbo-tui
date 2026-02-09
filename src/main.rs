mod app;
mod config;
mod database;
mod ui;

use app::{App, Focus};
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use database::Database;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{fs, io};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load("config.toml")?;
    let db = Database::new(config.database.clone());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app, &db, &config).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    db: &Database,
    config: &Config,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            if app.show_help {
                match key.code {
                    KeyCode::F(1) | KeyCode::Esc => app.show_help = false,
                    _ => {}
                }
                continue;
            }

            if app.file_browser.active {
                let queries_dir = config
                    .queries
                    .as_ref()
                    .map(|q| q.directory.as_str())
                    .unwrap_or("queries");
                match key.code {
                    KeyCode::Up => app.file_browser_up(queries_dir),
                    KeyCode::Down => app.file_browser_down(queries_dir),
                    KeyCode::Enter => app.load_selected_file(queries_dir),
                    KeyCode::Esc => app.close_file_browser(),
                    _ => {}
                }
                continue;
            }

            match app.focus {
                Focus::Editor => match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.should_quit = true
                    }
                    KeyCode::Char(c) => app.insert_char(c),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Up => app.editor_scroll_up(),
                    KeyCode::Down => app.editor_scroll_down(),
                    KeyCode::Enter => app.insert_char('\n'),
                    KeyCode::Tab => app.focus = Focus::Results,
                    KeyCode::F(1) => app.show_help = true,
                    KeyCode::F(5) => {
                        app.status_message = "Executing query...".to_string();
                        terminal.draw(|f| ui::draw(f, app))?;
                        match db.execute_query(&app.query).await {
                            Ok(result) => app.set_result(result),
                            Err(e) => app.set_error(e.to_string()),
                        }
                    }
                    KeyCode::F(6) => app.format_query(),
                    KeyCode::F(7) => {
                        let queries_dir = config
                            .queries
                            .as_ref()
                            .map(|q| q.directory.as_str())
                            .unwrap_or("queries");
                        app.open_file_browser(queries_dir);
                    }
                    KeyCode::F(8) => {
                        if let Some(result) = &app.result {
                            match save_results(result) {
                                Ok(path) => {
                                    app.status_message = format!("Results saved to {}", path)
                                }
                                Err(e) => app.status_message = format!("Error saving: {}", e),
                            }
                        }
                    }
                    KeyCode::Esc => app.should_quit = true,
                    _ => {}
                },
                Focus::Results => match key.code {
                    KeyCode::Up => app.scroll_up(),
                    KeyCode::Down => app.scroll_down(),
                    KeyCode::Tab => app.focus = Focus::Editor,
                    KeyCode::F(1) => app.show_help = true,
                    KeyCode::F(8) => {
                        if let Some(result) = &app.result {
                            match save_results(result) {
                                Ok(path) => {
                                    app.status_message = format!("Results saved to {}", path)
                                }
                                Err(e) => app.status_message = format!("Error saving: {}", e),
                            }
                        }
                    }
                    KeyCode::Esc => app.should_quit = true,
                    _ => {}
                },
                Focus::FileBrowser => {
                    // Handled above
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn save_results(result: &database::QueryResult) -> anyhow::Result<String> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let json_path = format!("results_{}.json", timestamp);
    let csv_path = format!("results_{}.csv", timestamp);

    let json_data = serde_json::json!({
        "columns": result.columns,
        "rows": result.rows,
        "row_count": result.row_count,
    });
    fs::write(&json_path, serde_json::to_string_pretty(&json_data)?)?;

    let mut wtr = csv::Writer::from_path(&csv_path)?;
    wtr.write_record(&result.columns)?;
    for row in &result.rows {
        wtr.write_record(row)?;
    }
    wtr.flush()?;

    Ok(format!("{} and {}", json_path, csv_path))
}
