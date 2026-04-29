//! PhantomDev TUI Dashboard
//!
//! This crate provides a terminal user interface for visualizing
//! stealth scores, detection results, and style profiles.
//!
//! Built with ❤️ by John Varghese (J0X)
//! GitHub: https://github.com/John-Varghese-EH
//! LinkedIn: https://linkedin.com/in/John--Varghese

use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{
        Block, Borders, Gauge, Paragraph, Wrap, BarChart, List, ListItem, Tabs,
    },
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;
use phantomdev_core::{StealthScore, DetectionResult};

/// Main TUI application
pub struct PhantomTui {
    /// Current tab
    current_tab: Tab,
    /// Stealth score
    stealth_score: Option<StealthScore>,
    /// Detection results
    detection_results: Vec<DetectionResult>,
    /// Should quit
    should_quit: bool,
}

/// Available tabs
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Overview,
    Detection,
    Style,
    Settings,
}

impl Tab {
    fn title(&self) -> &'static str {
        match self {
            Tab::Overview => "Overview",
            Tab::Detection => "Detection",
            Tab::Style => "Style",
            Tab::Settings => "Settings",
        }
    }

    fn all() -> Vec<Tab> {
        vec![Tab::Overview, Tab::Detection, Tab::Style, Tab::Settings]
    }
}

impl PhantomTui {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Overview,
            stealth_score: None,
            detection_results: Vec::new(),
            should_quit: false,
        }
    }

    /// Run the TUI application
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main loop
        while !self.should_quit {
            terminal.draw(|f| self.draw(f))?;

            // Handle events
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        Ok(())
    }

    /// Handle keyboard input
    fn handle_key(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Left => {
                let tabs = Tab::all();
                let current = tabs.iter().position(|&t| t == self.current_tab).unwrap_or(0);
                if current > 0 {
                    self.current_tab = tabs[current - 1];
                }
            }
            KeyCode::Right => {
                let tabs = Tab::all();
                let current = tabs.iter().position(|&t| t == self.current_tab).unwrap_or(0);
                if current < tabs.len() - 1 {
                    self.current_tab = tabs[current + 1];
                }
            }
            KeyCode::Char('1') => self.current_tab = Tab::Overview,
            KeyCode::Char('2') => self.current_tab = Tab::Detection,
            KeyCode::Char('3') => self.current_tab = Tab::Style,
            KeyCode::Char('4') => self.current_tab = Tab::Settings,
            _ => {}
        }
    }

    /// Draw the UI
    fn draw(&self, f: &mut Frame) {
        let size = f.size();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Content
                Constraint::Length(1),  // Footer
            ])
            .split(size);

        // Draw header
        self.draw_header(f, chunks[0]);

        // Draw content based on current tab
        match self.current_tab {
            Tab::Overview => self.draw_overview(f, chunks[1]),
            Tab::Detection => self.draw_detection(f, chunks[1]),
            Tab::Style => self.draw_style(f, chunks[1]),
            Tab::Settings => self.draw_settings(f, chunks[1]),
        }

        // Draw footer
        self.draw_footer(f, chunks[2]);
    }

    /// Draw header with tabs
    fn draw_header(&self, f: &mut Frame, area: Rect) {
        let tabs = Tab::all();
        let titles: Vec<Line> = tabs
            .iter()
            .map(|t| {
                let style = if *t == self.current_tab {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(Span::styled(t.title(), style))
            })
            .collect();

        let tabs_widget = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("PhantomDev Dashboard")
                    .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::UNDERLINED))
            .divider(Span::raw(" | "));

        f.render_widget(tabs_widget, area);
    }

    /// Draw overview tab
    fn draw_overview(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),
                Constraint::Min(0),
            ])
            .split(area);

        // Draw stealth score gauge
        self.draw_stealth_score(f, chunks[0]);

        // Draw summary
        let summary = vec![
            Line::from("PhantomDev Status:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  • Detection Engine: "),
                Span::styled("Active", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("  • Humanizer: "),
                Span::styled("Ready", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("  • Jitter Engine: "),
                Span::styled("Disabled", Style::default().fg(Color::Yellow)),
            ]),
            Line::from(""),
            Line::from("Press 'q' to quit, '1-4' to switch tabs"),
        ];

        let paragraph = Paragraph::new(summary)
            .block(Block::default().borders(Borders::ALL).title("Summary"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);
    }

    /// Draw stealth score gauge
    fn draw_stealth_score(&self, f: &mut Frame, area: Rect) {
        let score = self.stealth_score.as_ref().unwrap_or(&StealthScore {
            overall: 0.75,
            ai_probability: 0.25,
            pattern_score: 0.3,
            style_score: 0.8,
        });

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Stealth Score"))
            .gauge_style(
                Style::default()
                    .fg(if score.overall > 0.7 {
                        Color::Green
                    } else if score.overall > 0.4 {
                        Color::Yellow
                    } else {
                        Color::Red
                    })
                    .bg(Color::DarkGray)
            )
            .label(Span::styled(
                format!("{:.0}%", score.overall * 100.0),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ))
            .ratio(score.overall as f64);

        f.render_widget(gauge, area);
    }

    /// Draw detection tab
    fn draw_detection(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(0),
            ])
            .split(area);

        // Draw pattern breakdown
        let patterns = vec![
            ("Watermarks", 0.2),
            ("Emojis", 0.1),
            ("Comments", 0.3),
            ("Naming", 0.15),
            ("Entropy", 0.25),
        ];

        let bars: Vec<(&str, u64)> = patterns
            .iter()
            .map(|(name, value)| (*name, (*value * 100.0) as u64))
            .collect();

        let barchart = BarChart::default()
            .block(Block::default().borders(Borders::ALL).title("Pattern Detection"))
            .bar_width(8)
            .bar_gap(2)
            .data(&bars)
            .style(Style::default().fg(Color::Cyan))
            .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

        f.render_widget(barchart, chunks[0]);

        // Draw recent detections
        let items: Vec<ListItem> = vec![
            ListItem::new("src/main.rs - 85% stealth"),
            ListItem::new("src/lib.rs - 92% stealth"),
            ListItem::new("tests/integration.rs - 78% stealth"),
            ListItem::new("README.md - 95% stealth"),
        ];

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Recent Scans"))
            .style(Style::default().fg(Color::White));

        f.render_widget(list, chunks[1]);
    }

    /// Draw style tab
    fn draw_style(&self, f: &mut Frame, area: Rect) {
        let style_info = vec![
            Line::from("Style Profile:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  Naming Convention: "),
                Span::styled("snake_case", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  Comment Style: "),
                Span::styled("Inline Only", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  Indentation: "),
                Span::styled("4 spaces", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  Max Line Length: "),
                Span::styled("100 chars", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  Commit Format: "),
                Span::styled("Conventional", Style::default().fg(Color::Cyan)),
            ]),
            Line::from(""),
            Line::from("Style learned from 127 commits and 45 files"),
        ];

        let paragraph = Paragraph::new(style_info)
            .block(Block::default().borders(Borders::ALL).title("Current Style Profile"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    /// Draw settings tab
    fn draw_settings(&self, f: &mut Frame, area: Rect) {
        let settings = vec![
            Line::from("Configuration:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  [x] Use Local Models"),
            ]),
            Line::from(vec![
                Span::raw("  [x] Cloud Fallback"),
            ]),
            Line::from(vec![
                Span::raw("  [ ] Auto Humanize"),
            ]),
            Line::from(vec![
                Span::raw("  [ ] Jitter Enabled"),
            ]),
            Line::from(""),
            Line::from("Detection Threshold: 0.15"),
            Line::from("Entropy Level: 0.50"),
            Line::from(""),
            Line::from("Press 'q' to quit"),
        ];

        let paragraph = Paragraph::new(settings)
            .block(Block::default().borders(Borders::ALL).title("Settings"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    /// Draw footer
    fn draw_footer(&self, f: &mut Frame, area: Rect) {
        let footer = Line::from(vec![
            Span::raw(" "),
            Span::styled("PhantomDev", Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to quit"),
        ]);

        let paragraph = Paragraph::new(footer)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }

    /// Update stealth score
    pub fn update_stealth_score(&mut self, score: StealthScore) {
        self.stealth_score = Some(score);
    }

    /// Add detection result
    pub fn add_detection_result(&mut self, result: DetectionResult) {
        self.detection_results.push(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_creation() {
        let tui = PhantomTui::new();
        assert_eq!(tui.current_tab, Tab::Overview);
        assert!(!tui.should_quit);
    }

    #[test]
    fn test_tab_navigation() {
        let mut tui = PhantomTui::new();
        assert_eq!(tui.current_tab, Tab::Overview);

        // Simulate right key press
        tui.handle_key(event::KeyEvent {
            code: KeyCode::Right,
            modifiers: event::KeyModifiers::empty(),
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE,
        });
        assert_eq!(tui.current_tab, Tab::Detection);
    }

    #[test]
    fn test_quit() {
        let mut tui = PhantomTui::new();
        assert!(!tui.should_quit);

        tui.handle_key(event::KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: event::KeyModifiers::empty(),
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::NONE,
        });
        assert!(tui.should_quit);
    }
}
