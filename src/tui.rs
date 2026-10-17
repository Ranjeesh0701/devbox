use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use devbox::process::session::{ProcessSession, ProcessStatus};

pub fn run<F>(mut fetch_sessions: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut() -> Result<Vec<ProcessSession>, String>,
{
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    if let Err(error) = execute!(stdout, EnterAlternateScreen) {
        let _ = disable_raw_mode();
        return Err(error.into());
    }

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut fetch_sessions);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<F>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    fetch_sessions: &mut F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut() -> Result<Vec<ProcessSession>, String>,
{
    let (program, args) = devbox::terminal::default_shell();
    let mut shell = devbox::terminal::TerminalSession::spawn(
        program,
        &args,
        std::path::Path::new("."),
        24,
        80,
    )?;
    let mut sessions = Vec::new();
    let mut error_message: Option<String> = None;
    let mut last_refresh = Instant::now() - Duration::from_secs(1);
    let mut terminal_output = String::new();

    loop {
        if last_refresh.elapsed() >= Duration::from_secs(1) {
            match fetch_sessions() {
                Ok(updated_sessions) => {
                    sessions = updated_sessions;
                    error_message = None;
                }
                Err(error) => {
                    error_message = Some(error);
                }
            }

            last_refresh = Instant::now();
        }

        loop {
            match shell.try_read_output() {
                Ok(bytes) => {
                    terminal_output.push_str(&String::from_utf8_lossy(&bytes));
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            }
        }

        terminal.draw(|frame| {
            let areas = Layout::vertical([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
                Constraint::Length(2),
            ])
            .split(frame.area());

            let rows = sessions.iter().map(|session| {
                let status = match session.status {
                    ProcessStatus::Running => "Running",
                    ProcessStatus::Exited => "Exited",
                };

                Row::new(vec![
                    Cell::from(session.name.clone()),
                    Cell::from(session.pid.to_string()),
                    Cell::from(status),
                    Cell::from(session.command.clone()),
                ])
            });

            let table = Table::new(
                rows,
                [
                    Constraint::Length(20),
                    Constraint::Length(10),
                    Constraint::Length(12),
                    Constraint::Min(10),
                ],
            )
            .header(
                Row::new(["SERVICE", "PID", "STATUS", "COMMAND"])
                    .style(ratatui::style::Style::default().bold()),
            )
            .block(
                Block::default()
                    .title(" Devbox Workspace ")
                    .borders(Borders::ALL),
            );

            frame.render_widget(table, areas[0]);

            let terminal_pane = Paragraph::new(terminal_output.as_str())
                .block(Block::default().title(" Terminal ").borders(Borders::ALL));

            frame.render_widget(terminal_pane, areas[1]);

            let footer = match &error_message {
                Some(error) => format!("Refresh failed: {error}"),
                None => format!(
                    "{} sessions · Refreshes every second · Press q to quit",
                    sessions.len()
                ),
            };

            frame.render_widget(Paragraph::new(footer), areas[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }

                    let input: Option<Vec<u8>> = match key.code {
                        KeyCode::Char(character) => Some(character.to_string().into_bytes()),
                        KeyCode::Enter => Some(vec![b'\r']),
                        KeyCode::Backspace => Some(vec![0x7f]),
                        KeyCode::Tab => Some(vec![b'\t']),
                        KeyCode::Esc => Some(vec![0x1b]),
                        KeyCode::Up => Some(vec![0x1b, b'[', b'A']),
                        KeyCode::Down => Some(vec![0x1b, b'[', b'B']),
                        KeyCode::Right => Some(vec![0x1b, b'[', b'C']),
                        KeyCode::Left => Some(vec![0x1b, b'[', b'D']),
                        _ => None,
                    };

                    if let Some(input) = input {
                        shell.send_input(&input)?;
                    }
                }
            }
        }
    }

    Ok(())
}
