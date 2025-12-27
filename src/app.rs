use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    _running: bool,
    _active_tab: usize,
    _tabs: Vec<&'static str>,
}

impl App {
    pub fn new() -> Self {
        Self {
            _running: true,
            _active_tab: 0,
            _tabs: vec!["Articles", "Videos", "Radio"],
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self._running = true;
        while self._running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(frame.area());

        // Tabs
        let titles: Vec<Line> = self
            ._tabs
            .iter()
            .map(|t| Line::from(Span::styled(*t, Style::default())))
            .collect();

        let tabs = Tabs::new(titles)
            .select(self._active_tab)
            .block(
                Block::default()
                    .title(Line::from("Media").bold())
                    .borders(Borders::ALL),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(tabs, layout[0]);

        // Content
        let content = Paragraph::new(format!(
            "You are viewing the {} tab.\n\nPress `h` / `l` to switch tabs and `q` to quit",
            self._tabs[self._active_tab]
        ))
        .block(
            Block::default()
                .title(Line::from(self._tabs[self._active_tab]).blue())
                .borders(Borders::ALL),
        )
        .centered();

        frame.render_widget(content, layout[1]);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Quit
            (_, KeyCode::Esc | KeyCode::Char('q')) => self.quit(),
            // Next tab
            (_, KeyCode::Char('l')) => {
                self._active_tab = (self._active_tab + 1) % self._tabs.len();
            }
            // Previous tab
            (_, KeyCode::Char('h')) => {
                if self._active_tab == 0 {
                    self._active_tab = self._tabs.len() - 1;
                } else {
                    self._active_tab -= 1;
                }
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self._running = false;
    }
}
