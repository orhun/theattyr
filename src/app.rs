use crate::{
    animation::{Animation, Animations},
    event::{handle_key_events, Event, EventHandler},
};
use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, List, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState},
    DefaultTerminal, Frame,
};

pub struct App {
    /// Is the application running?
    pub is_running: bool,
    // Is the animation being rendered?
    pub is_rendering: bool,
    /// Event handler.
    pub event_handler: EventHandler,
    /// List state.
    pub list_state: ListState,
    /// Animations.
    pub animations: Vec<String>,
    /// Animation widget.
    pub animation: Animation,
    /// Animation area.
    pub animation_area: Rect,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new(event_handler: EventHandler) -> Self {
        Self {
            is_running: false,
            is_rendering: false,
            event_handler,
            list_state: ListState::default(),
            animations: Animations::iter().map(|a| a.to_string()).collect(),
            animation: Animation::default(),
            animation_area: Rect::default(),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.is_running = true;
        while self.is_running {
            terminal.draw(|frame| self.draw(frame))?;
            let event = self.event_handler.next()?;
            match event {
                Event::Tick => {}
                Event::Key(key_event) => handle_key_events(key_event, &mut self)?,
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
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
                    .map(Line::from)
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
        self.animation_area = area[1];
        if !self.animation.is_rendered {
            frame.render_widget(&mut self.animation, self.animation_area);
            self.event_handler.sender.send(Event::Tick).unwrap();
        }
    }
}
