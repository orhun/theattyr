use std::time::{Duration, Instant};

use crate::{
    animation::{Animation, Animations},
    event::{handle_key_events, Event, EventHandler},
    fps::Fps,
};
use color_eyre::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, List, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    DefaultTerminal, Frame,
};

pub struct App {
    /// Is the application running?
    pub is_running: bool,
    // Is the animation being rendered?
    pub is_rendering: bool,
    /// Is the UI toggled?
    pub is_toggled: bool,
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
    /// FPS counter.
    pub fps: Fps,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new(event_handler: EventHandler) -> Self {
        Self {
            is_running: true,
            is_rendering: false,
            is_toggled: true,
            event_handler,
            list_state: ListState::default().with_selected(Some(0)),
            animations: Animations::iter().map(|a| a.to_string()).collect(),
            animation: Animation::default(),
            animation_area: Rect::default(),
            fps: Fps::default(),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let target_fps: f32 = 60.0;
        let frame_interval: Duration = Duration::from_secs_f32(1.0 / target_fps);
        let mut accumulator = Duration::new(0, 0);
        let mut last_tick = Instant::now();
        let list_width = self
            .animations
            .iter()
            .map(|a| a.len())
            .max()
            .unwrap_or_default();
        while self.is_running {
            terminal.draw(|frame| self.draw(frame, list_width))?;
            let event = self.event_handler.next()?;
            match event {
                Event::Tick => {
                    self.fps.tick();
                    accumulator += last_tick.elapsed();
                    while accumulator >= frame_interval {
                        self.event_handler.sender.send(Event::Tick)?;
                        accumulator -= frame_interval;
                    }
                    last_tick = Instant::now();
                }
                Event::Key(key_event) => handle_key_events(key_event, &mut self)?,
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {
                    self.animation
                        .parser
                        .set_size(self.animation_area.height, self.animation_area.width);
                }
            }
        }
        Ok(())
    }

    /// Renders the user interface.
    fn draw(&mut self, frame: &mut Frame, list_width: usize) {
        let area = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Min((list_width as u16 + 4) * self.is_toggled as u16),
                Constraint::Percentage(100),
            ],
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
        if self.is_toggled {
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
        }
        frame.render_widget(
            Block::bordered()
                .title(
                    Title::from("Animation")
                        .alignment(Alignment::Center)
                        .position(Position::Top),
                )
                .title(
                    Title::from(format!("fps: {}", self.fps.to_string()))
                        .alignment(Alignment::Right)
                        .position(Position::Bottom),
                ),
            area[1],
        );
        self.animation_area = area[1].inner(Margin {
            vertical: 1,
            horizontal: 1,
        });
        frame.render_widget(&mut self.animation, self.animation_area);
    }
}
