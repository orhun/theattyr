use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor},
};

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, List, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    DefaultTerminal, Frame,
};
use rust_embed::Embed;
use tui_term::widget::PseudoTerminal;
use vt100::Parser;

#[derive(Embed)]
#[folder = "vt100"]
struct Animation;

pub struct App {
    /// Is the application running?
    is_running: bool,
    // Is the animation being rendered?
    is_rendering: bool,
    /// Animations.
    animations: Vec<String>,
    /// List state.
    list_state: ListState,
    // Reader for the file.
    reader: Option<BufReader<Cursor<Vec<u8>>>>,
    /// VT100 parser.
    parser: Option<Parser>,
    /// Buffer.
    buffer: String,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            is_running: false,
            is_rendering: false,
            animations: Animation::iter().map(|a| a.to_string()).collect(),
            list_state: ListState::default(),
            reader: None,
            parser: None,
            buffer: String::new(),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.is_running = true;
        while self.is_running {
            terminal.draw(|frame| self.draw(frame))?;
            if self.is_rendering {
                std::thread::sleep(std::time::Duration::from_millis(10));
            } else {
                self.handle_crossterm_events()?;
            }
        }
        Ok(())
    }

    /// Renders the user interface.
    fn draw(&mut self, frame: &mut Frame) {
        let area = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(20), Constraint::Fill(1)],
        )
        .split(frame.area());
        frame.render_stateful_widget(
            List::new(
                self.animations
                    .clone()
                    .into_iter()
                    .map(|v| Line::from(v))
                    .collect::<Vec<Line>>(),
            )
            .block(Block::bordered().title("Animations"))
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol(">"),
            area[0],
            &mut self.list_state,
        );
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        let mut scrollbar_state = ScrollbarState::new(self.animations.len())
            .position(self.list_state.selected().unwrap_or_default());
        frame.render_stateful_widget(
            scrollbar,
            area[0].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );

        if !self.is_rendering {
            frame.render_widget(Block::bordered(), area[1]);
        } else {
            match &mut self.parser {
                Some(parser) => {
                    let mut line_buffer = String::new();
                    let bytes_read = self
                        .reader
                        .as_mut()
                        .unwrap()
                        .read_line(&mut line_buffer)
                        .unwrap();
                    if bytes_read > 0 {
                        self.buffer += &line_buffer;
                        parser.process(self.buffer.as_bytes());
                        let pseudo_term = PseudoTerminal::new(parser.screen());
                        frame.render_widget(pseudo_term.block(Block::bordered()), area[1]);
                    } else {
                        self.is_rendering = false;
                        self.buffer.clear();
                        self.parser = None;
                    }
                }
                None => {
                    self.parser = Some(Parser::new(area[1].height, area[1].width, 0));
                }
            }
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                self.is_running = false;
            }
            (_, KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J')) => {
                self.list_state.select_next();
            }
            (_, KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K')) => {
                self.list_state.select_previous();
            }
            (_, KeyCode::Enter) => {
                if !self.is_rendering {
                    let selected = self.list_state.selected().unwrap_or_default();
                    let data = Animation::get(&self.animations[selected].clone())
                        .unwrap()
                        .data
                        .into_owned();
                    self.reader = Some(BufReader::new(Cursor::new(data)));
                    self.is_rendering = true;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
