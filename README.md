# TurboTUI - Blazing Fast SQL Server Query Tool

A colorful, high-performance Terminal User Interface (TUI) application for querying SQL Server databases, built with Rust and Ratatui.

## Features

- **Interactive SQL Editor** - Multi-line query editor with cursor navigation
- **Auto-formatting** - Format SQL queries with F6
- **Real-time Execution** - Execute queries with F5 and view results instantly
- **Results Viewer** - Scrollable table view with pagination
- **File Operations** - Load queries from `.sql` files and save results to JSON/CSV
- **Config File** - TOML-based configuration for database credentials
- **Keyboard Shortcuts** - Intuitive navigation and commands
- **Error Handling** - User-friendly error messages in status bar

## Prerequisites

- Rust 1.70+ (install from https://rustup.rs)
- Access to SQL Server instance.
- Network connectivity to SQL Server

## Installation

```bash
# Build the project
cargo build --release

# The binary will be at target/release/turbotui
```

## Configuration

Edit `config.toml` in the project root:

```toml
[database]
server = "MyServer"
port = 1433                  # Use actual port (named instances use dynamic ports)
user = "your_username"       # Leave empty for Windows Authentication
password = "your_password"   # Leave empty for Windows Authentication
database = "my_database"
trusted_connection = false   # Set to true for Windows Authentication
trust_server_certificate = true

[queries]
directory = "queries"        # Directory containing .sql files
```

For SQL Server authentication, provide `user` and `password`. For Windows Authentication (Windows only), leave them empty and set `trusted_connection = true`.

**Note:** SQL Server named instances use dynamic ports. Use the actual port number, not the default 1433. You can find the port using `netstat` or SQL Server Configuration Manager.

## Usage

```bash
# Run the application
cargo run --release

# Or run the compiled binary
./target/release/turbotui
```

## Keyboard Shortcuts

### Global
- **F1** - Toggle help screen
- **Esc** - Quit application
- **Tab** - Switch focus between editor and results

### Query Editor (when focused)
- **F5** - Execute SQL query
- **F6** - Format SQL query
- **F7** - Open file browser to load query from files
- **F8** - Save results to timestamped JSON and CSV files
- **Enter** - New line
- **Backspace** - Delete character
- **←/→** - Move cursor left/right
- **↑/↓** - Scroll editor up/down
- **Ctrl+C** - Quit

### File Browser (when open)
- **↑/↓** - Navigate through .sql files
- **Enter** - Load selected file into editor
- **Esc** - Close file browser

### Results Viewer (when focused)
- **↑/↓** - Scroll results up/down
- **F8** - Save results to files

## File Operations

### Load Query
Press **F7** to open the file browser. The browser displays:
- **Left panel**: List of `.sql` files from the configured queries directory
- **Right panel**: Live preview of the selected file

Navigate with **↑/↓** arrows and press **Enter** to load the selected file into the editor.

Configure the queries directory in `config.toml`:
```toml
[queries]
directory = "queries"
```

### Save Results
Press **F8** to save query results. Creates two files:
- `results_YYYYMMDD_HHMMSS.json` - JSON format with metadata
- `results_YYYYMMDD_HHMMSS.csv` - CSV format for spreadsheets

## Architecture

### Modules
- **config.rs** - TOML configuration parser
- **database.rs** - SQL Server connectivity using Tiberius
- **app.rs** - Application state management
- **ui.rs** - Ratatui UI rendering
- **main.rs** - Event loop and async runtime

### Dependencies
- **ratatui** - Terminal UI framework
- **crossterm** - Cross-platform terminal manipulation
- **tiberius** - Native Rust SQL Server driver
- **tokio** - Async runtime
- **sqlformat** - SQL query formatting
- **serde/toml** - Configuration parsing
- **csv/serde_json** - Result export

## Troubleshooting

### Connection Issues
- Verify SQL Server is accessible: `telnet MyServer 1433`
- Check firewall rules allow the SQL Server port
- Ensure `trust_server_certificate = true` for self-signed certs
- For named instances, use the actual dynamic port (not 1433)

### Authentication Issues
- For Windows Authentication: leave `user` and `password` empty (Windows only)
- For SQL Server Authentication: provide valid credentials
- Verify user has appropriate database permissions

### Build Issues
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

## Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Check for errors
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## License

TurboTUI is a ratatui-based Rust UI for MySQL queries from the terminal. 
Copyright (C) 2025 Harrison Oest

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

More information can be found in the [LICENSE found here.](LICENSE.md)
