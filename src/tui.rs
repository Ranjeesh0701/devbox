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

use devbox::process::session::{ProcessSession, ProcessStatus};
use devbox::service_log_view::ServiceLogView;
use devbox::storage;
use devbox::workspace::TerminalPane;

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
    let mut service_views: Vec<ServiceLogView> = Vec::new();
    let mut shell_tabs: Vec<TerminalPane> = Vec::new();
    let mut active_tab: usize = 0;
    let mut error_message: Option<String> = None;
    let mut last_refresh = Instant::now() - Duration::from_secs(1);

    // Initial load of services
    match fetch_sessions() {
        Ok(sessions) => {
            for session in sessions {
                match storage::get_log_file(&session.id) {
                    Ok(log_path) => {
                        match ServiceLogView::new(session.clone(), log_path) {
                            Ok(view) => service_views.push(view),
                            Err(e) => {
                                eprintln!("Failed to create log view for {}: {}", session.name, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get log path for {}: {}", session.name, e);
                    }
                }
            }

            if service_views.is_empty() {
                error_message = Some("No services running. Start services first.".to_string());
            }
        }
        Err(e) => {
            error_message = Some(format!("Failed to fetch sessions: {}", e));
        }
    }

    loop {
        // Refresh session list and update logs periodically
        if last_refresh.elapsed() >= Duration::from_millis(500) {
            match fetch_sessions() {
                Ok(sessions) => {
                    // Update existing service views
                    for view in &mut service_views {
                        if let Err(e) = view.update() {
                            eprintln!("Failed to update log for {}: {}", view.name(), e);
                        }

                        // Update session status
                        if let Some(updated_session) = sessions.iter().find(|s| s.id == view.session.id) {
                            view.session = updated_session.clone();
                        }
                    }

                    // Add new services that appeared
                    for session in sessions {
                        if !service_views.iter().any(|v| v.session.id == session.id) {
                            match storage::get_log_file(&session.id) {
                                Ok(log_path) => {
                                    match ServiceLogView::new(session.clone(), log_path) {
                                        Ok(view) => service_views.push(view),
                                        Err(e) => {
                                            eprintln!("Failed to create log view for {}: {}", session.name, e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to get log path for {}: {}", session.name, e);
                                }
                            }
                        }
                    }

                    error_message = None;
                }
                Err(error) => {
                    error_message = Some(error);
                }
            }

            last_refresh = Instant::now();
        }

        // Update shell tabs if any
        for shell in &mut shell_tabs {
            shell.update();
        }

        // Render UI
        terminal.draw(|frame| {
            let areas = Layout::vertical([
                Constraint::Length(3), // Tab bar
                Constraint::Min(0),     // Content
                Constraint::Length(2),  // Footer
            ])
            .split(frame.area());

            // Build tab titles
            let mut tab_titles: Vec<String> = service_views
                .iter()
                .map(|view| {
                    let status_icon = match view.status() {
                        ProcessStatus::Running => "●",
                        ProcessStatus::Exited => "○",
                    };
                    format!("{} {}", status_icon, view.name())
                })
                .collect();

            // Add shell tabs
            for shell in shell_tabs.iter() {
                tab_titles.push(format!("🐚 {}", shell.name));
            }

            let tabs = Tabs::new(tab_titles)
                .block(Block::default().title(" Devbox Workspace ").borders(Borders::ALL))
                .select(active_tab)
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );

            frame.render_widget(tabs, areas[0]);

            // Render current tab's content
            let total_tabs = service_views.len() + shell_tabs.len();
            if active_tab < service_views.len() {
                // Service log view
                let view = &service_views[active_tab];
                let content = view.content();

                let status_text = match view.status() {
                    ProcessStatus::Running => "Running",
                    ProcessStatus::Exited => "Exited",
                };

                let content_pane = Paragraph::new(content).block(
                    Block::default()
                        .title(format!(" {} - {} ", view.name(), status_text))
                        .borders(Borders::ALL),
                );

                frame.render_widget(content_pane, areas[1]);
            } else if active_tab < total_tabs {
                // Shell tab
                let shell_index = active_tab - service_views.len();
                if let Some(shell) = shell_tabs.get(shell_index) {
                    let terminal_text = shell.screen_contents();
                    let terminal_pane = Paragraph::new(terminal_text).block(
                        Block::default()
                            .title(format!(" {} ", shell.name))
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
                    Span::raw("Tab/Shift+Tab: Switch • "),
                    Span::raw("Ctrl+T: New shell • "),
                    Span::raw("Ctrl+W: Close tab • "),
                    Span::raw("Ctrl+R: Refresh • "),
                    Span::raw("q: Quit"),
                ])],
            };

            frame.render_widget(Paragraph::new(footer), areas[2]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    let total_tabs = service_views.len() + shell_tabs.len();

                    match (key.code, key.modifiers) {
                        // Quit
                        (KeyCode::Char('q'), KeyModifiers::NONE) => break,

                        // Tab navigation
                        (KeyCode::Tab, KeyModifiers::NONE) => {
                            if total_tabs > 0 {
                                active_tab = (active_tab + 1) % total_tabs;
                            }
                            continue;
                        }
                        (KeyCode::BackTab, _) => {
                            if total_tabs > 0 {
                                active_tab = if active_tab == 0 {
                                    total_tabs - 1
                                } else {
                                    active_tab - 1
                                };
                            }
                            continue;
                        }

                        // New shell tab (Ctrl+T)
                        (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                            let (program, args) = devbox::terminal::default_shell();
                            let shell_number = shell_tabs.len() + 1;
                            match TerminalPane::new(
                                format!("Shell {}", shell_number),
                                program,
                                &args,
                                std::env::current_dir()?,
                                24,
                                80,
                            ) {
                                Ok(pane) => {
                                    shell_tabs.push(pane);
                                    active_tab = service_views.len() + shell_tabs.len() - 1;
                                }
                                Err(e) => {
                                    error_message = Some(format!("Failed to create shell: {}", e));
                                }
                            }
                            continue;
                        }

                        // Close tab (Ctrl+W) - only for shells
                        (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                            if active_tab >= service_views.len() {
                                let shell_index = active_tab - service_views.len();
                                if shell_index < shell_tabs.len() {
                                    shell_tabs.remove(shell_index);
                                    if active_tab > 0 {
                                        active_tab -= 1;
                                    }
                                }
                            }
                            continue;
                        }

                        // Force refresh (Ctrl+R)
                        (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                            last_refresh = Instant::now() - Duration::from_secs(1);
                            continue;
                        }

                        _ => {}
                    }

                    // Send input to shell tabs only
                    if active_tab >= service_views.len() {
                        let shell_index = active_tab - service_views.len();
                        if let Some(shell) = shell_tabs.get_mut(shell_index) {
                            let input: Option<Vec<u8>> = match key.code {
                                KeyCode::Char(character) => {
                                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                                        None // Already handled above
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
                                let _ = shell.send_input(&input);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
