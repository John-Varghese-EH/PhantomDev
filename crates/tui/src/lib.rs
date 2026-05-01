//! PhantomDev TUI Dashboard
//!
//! This crate provides a terminal user interface for visualizing
//! stealth scores, detection results, and style profiles.
//!
//! Built with ❤️ by John Varghese (J0X)
//! GitHub: <https://github.com/John-Varghese-EH>
//! LinkedIn: <https://linkedin.com/in/John-Varghese>

use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap, BarChart, List, ListItem, Tabs},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

/// Main TUI application
pub struct PhantomTui {
    /// Current tab
    current_tab: Tab,
    /// Should quit
    should_quit: bool,
}

/// Available tabs
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tab {
    Dashboard,
    Scan,
    Humanize,
    Score,
    Settings,
    Help,
}

impl Tab {
    fn title(&self) -> &'static str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Scan => "Scan",
            Tab::Humanize => "Humanize",
            Tab::Score => "Score",
            Tab::Settings => "Settings",
            Tab::Help => "Help",
        }
    }

    fn all() -> Vec<Tab> {
        vec![Tab::Dashboard, Tab::Scan, Tab::Humanize, Tab::Score, Tab::Settings, Tab::Help]
    }
}

impl PhantomTui {
    /// Create a new TUI application
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Dashboard,
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
            KeyCode::Char('d') => self.current_tab = Tab::Dashboard,
            KeyCode::Char('s') => self.current_tab = Tab::Scan,
            KeyCode::Char('h') => self.current_tab = Tab::Humanize,
            KeyCode::Char('c') => self.current_tab = Tab::Score,
            KeyCode::Char('g') => self.current_tab = Tab::Settings,
            KeyCode::Char('l') => self.current_tab = Tab::Help,
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
            Tab::Dashboard => self.draw_dashboard(f, chunks[1]),
            Tab::Scan => self.draw_scan(f, chunks[1]),
            Tab::Humanize => self.draw_humanize(f, chunks[1]),
            Tab::Score => self.draw_score(f, chunks[1]),
            Tab::Settings => self.draw_settings(f, chunks[1]),
            Tab::Help => self.draw_help(f, chunks[1]),
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
                let style = Style::default().fg(Color::Gray);
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
            .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .divider(Span::raw(" | "));

        f.render_widget(tabs_widget, area);
    }

    /// Draw dashboard tab
    fn draw_dashboard(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),
                Constraint::Min(0),
            ])
            .split(area);

        // Draw stealth score gauge
        self.draw_stealth_score(f, chunks[0]);

        // Draw quick actions
        let actions = vec![
            Line::from("Quick Actions:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[S]can", Style::default().fg(Color::Cyan)),
                Span::raw(" - Scan for AI patterns in staged files"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[H]umanize", Style::default().fg(Color::Cyan)),
                Span::raw(" - Humanize code to make it look human-written"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[C]heck Score", Style::default().fg(Color::Cyan)),
                Span::raw(" - Check your stealth score"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[G]o to Settings", Style::default().fg(Color::Cyan)),
                Span::raw(" - Configure PhantomDev"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[L]earn More", Style::default().fg(Color::Cyan)),
                Span::raw(" - View help and documentation"),
            ]),
            Line::from(""),
            Line::from("Press 'q' to quit, arrow keys to navigate"),
        ];

        let paragraph = Paragraph::new(actions)
            .block(Block::default().borders(Borders::ALL).title("Quick Actions"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);
    }

    /// Draw stealth score gauge
    fn draw_stealth_score(&self, f: &mut Frame, area: Rect) {
        let score = 0.75; // Mock score for now

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Stealth Score"))
            .gauge_style(
                Style::default()
                    .fg(if score > 0.7 {
                        Color::Green
                    } else if score > 0.4 {
                        Color::Yellow
                    } else {
                        Color::Red
                    })
                    .bg(Color::DarkGray)
            )
            .label(Span::styled(
                format!("{:.0}%", score * 100.0),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ))
            .ratio(score as f64);

        f.render_widget(gauge, area);
    }

    /// Draw scan tab
    fn draw_scan(&self, f: &mut Frame, area: Rect) {
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
            ListItem::new("No files scanned yet. Run 'phantomdev scan' to check your code.".to_string()),
        ];

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Recent Scans"))
            .style(Style::default().fg(Color::White));

        f.render_widget(list, chunks[1]);
    }

    /// Draw humanize tab
    fn draw_humanize(&self, f: &mut Frame, area: Rect) {
        let style_info = vec![
            Line::from("Humanization Options:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[F]ix", Style::default().fg(Color::Cyan)),
                Span::raw(" - Auto-fix AI patterns in staged files"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("[D]ry Run", Style::default().fg(Color::Cyan)),
                Span::raw(" - Show what would be changed without applying"),
            ]),
            Line::from(""),
            Line::from("Humanization will transform your code to match your personal style:"),
            Line::from("  • Variable and function naming"),
            Line::from("  • Comment phrasing and placement"),
            Line::from("  • Code structure and entropy"),
            Line::from("  • Commit message style"),
        ];

        let paragraph = Paragraph::new(style_info)
            .block(Block::default().borders(Borders::ALL).title("Humanize Code"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    /// Draw score tab
    fn draw_score(&self, f: &mut Frame, area: Rect) {
        let score_info = vec![
            Line::from("Stealth Score Information:"),
            Line::from(""),
            Line::from("Your stealth score measures how human-like your code appears:"),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("90-100%", Style::default().fg(Color::Green)),
                Span::raw(" - Excellent (indistinguishable from human code)"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("70-89%", Style::default().fg(Color::Yellow)),
                Span::raw(" - Good (mostly human-like)"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("50-69%", Style::default().fg(Color::Yellow)),
                Span::raw(" - Needs Work (some AI patterns detected)"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("0-49%", Style::default().fg(Color::Red)),
                Span::raw(" - Poor (clearly AI-generated)"),
            ]),
        ];

        let paragraph = Paragraph::new(score_info)
            .block(Block::default().borders(Borders::ALL).title("Stealth Score"))
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

    /// Draw help tab
    fn draw_help(&self, f: &mut Frame, area: Rect) {
        let help_text = vec![
            Line::from("PhantomDev - Adversarial Stylometry Framework"),
            Line::from(""),
            Line::from("Commands:"),
            Line::from("  phantomdev              - Launch this dashboard (default)"),
            Line::from("  phantomdev scan         - Scan for AI patterns"),
            Line::from("  phantomdev humanize      - Humanize code"),
            Line::from("  phantomdev score         - Check stealth score"),
            Line::from("  phantomdev fix           - Auto-fix AI patterns"),
            Line::from("  phantomdev dashboard      - Launch TUI dashboard"),
            Line::from("  phantomdev config        - Configure settings"),
            Line::from("  phantomdev init          - Initialize in current directory"),
            Line::from("  phantomdev install        - Install IDE integration"),
            Line::from(""),
            Line::from("Navigation:"),
            Line::from("  Use arrow keys or hotkeys to navigate the dashboard"),
            Line::from("  Press 'q' or ESC to quit"),
        ];

        let paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title("Help"))
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
    pub fn update_stealth_score(&mut self, _score: f64) {
        // In a real implementation, this would update the UI with the actual score
    }
}