use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
};

use devbox::process::session::ProcessSession;
use devbox::workspace::{TerminalPane, Workspace};

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
    let mut workspace = Workspace::new();
    let mut sessions = Vec::new();
    let mut error_message: Option<String> = None;
    let mut last_refresh = Instant::now() - Duration::from_secs(1);

    // Create initial tabs from running sessions or start a default shell
    match fetch_sessions() {
        Ok(initial_sessions) if !initial_sessions.is_empty() => {
            sessions = initial_sessions;
            // TODO: Create tabs for each session
        }
        _ => {
            // Start a default shell if no sessions exist
            let (program, args) = devbox::terminal::default_shell();
            let pane = TerminalPane::new(
                "Shell".to_string(),
                program,
                &args,
                std::env::current_dir()?,
                24,
                80,
            )?;
            workspace.add_tab("Shell".to_string(), pane);
        }
    }

    loop {
        // Refresh session list periodically
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

        // Update all terminal outputs
        workspace.update_all();

        // Render UI
        terminal.draw(|frame| {
            let areas = Layout::vertical([
                Constraint::Length(3), // Tab bar
                Constraint::Min(0),     // Terminal content
                Constraint::Length(2),  // Footer
            ])
            .split(frame.area());

            // Render tab bar
            let tab_titles: Vec<String> = workspace
                .tabs
                .iter()
                .map(|tab| tab.name.clone())
                .collect();

            let tabs = Tabs::new(tab_titles)
                .block(Block::default().title(" Devbox Workspace ").borders(Borders::ALL))
                .select(workspace.active_tab)
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );

            frame.render_widget(tabs, areas[0]);

            // Render current tab's terminal content
            if let Some(tab) = workspace.current_tab() {
                if let Some(pane) = tab.panes.first() {
                    let terminal_text = pane.screen_contents();
                    let terminal_pane = Paragraph::new(terminal_text).block(
                        Block::default()
                            .title(format!(" {} ", pane.name))
                            .borders(Borders::ALL),
                    );

                    frame.render_widget(terminal_pane, areas[1]);
                }
            }

            // Render footer with help text
            let footer = match &error_message {
                Some(error) => vec![Line::from(vec![
                    Span::styled("Error: ", Style::default().fg(Color::Red)),
                    Span::raw(error),
                ])],
                None => vec![Line::from(vec![
                    Span::raw("Tab/Shift+Tab: Switch tabs • "),
                    Span::raw("Ctrl+T: New tab • "),
                    Span::raw("Ctrl+W: Close tab • "),
                    Span::raw("q: Quit • "),
                    Span::styled(
                        format!("{} sessions", sessions.len()),
                        Style::default().fg(Color::Green),
                    ),
                ])],
            };

            frame.render_widget(Paragraph::new(footer), areas[2]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    // Handle global shortcuts
                    match (key.code, key.modifiers) {
                        // Quit
                        (KeyCode::Char('q'), KeyModifiers::NONE) => break,

                        // Tab navigation
                        (KeyCode::Tab, KeyModifiers::NONE) => {
                            workspace.next_tab();
                            continue;
                        }
                        (KeyCode::BackTab, _) => {
                            workspace.prev_tab();
                            continue;
                        }

                        // New tab (Ctrl+T)
                        (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                            let (program, args) = devbox::terminal::default_shell();
                            let tab_number = workspace.tabs.len() + 1;
                            let pane = TerminalPane::new(
                                format!("Shell {}", tab_number),
                                program,
                                &args,
                                std::env::current_dir()?,
                                24,
                                80,
                            )?;
                            workspace.add_tab(format!("Shell {}", tab_number), pane);
                            workspace.active_tab = workspace.tabs.len() - 1;
                            continue;
                        }

                        // Close tab (Ctrl+W)
                        (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                            workspace.close_current_tab();
                            if workspace.tabs.is_empty() {
                                break; // Exit if no tabs left
                            }
                            continue;
                        }

                        _ => {}
                    }

                    // Send input to the focused terminal pane
                    if let Some(tab) = workspace.current_tab_mut() {
                        if let Some(pane) = tab.focused_pane_mut() {
                            let input: Option<Vec<u8>> = match key.code {
                                KeyCode::Char(character) => {
                                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                                        // Already handled above
                                        None
                                    } else {
                                        Some(character.to_string().into_bytes())
                                    }
                                }
                                KeyCode::Enter => Some(vec![b'\r']),
                                KeyCode::Backspace => Some(vec![0x7f]),
                                KeyCode::Esc => Some(vec![0x1b]),
                                KeyCode::Up => Some(vec![0x1b, b'[', b'A']),
                                KeyCode::Down => Some(vec![0x1b, b'[', b'B']),
                                KeyCode::Right => Some(vec![0x1b, b'[', b'C']),
                                KeyCode::Left => Some(vec![0x1b, b'[', b'D']),
                                _ => None,
                            };

                            if let Some(input) = input {
                                pane.send_input(&input)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
