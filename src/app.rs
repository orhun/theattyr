use std::{
    io::{BufReader, Cursor},
    time::{Duration, Instant},
};
use vt100::Parser;

use crate::{
    animation::{descriptions, Animation, Animations},
    event::{handle_key_events, Event, EventHandler},
    fps::Fps,
    Args,
};
use color_eyre::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, List, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    DefaultTerminal, Frame,
};

pub struct App {
    /// Arguments.
    pub args: Args,
    /// Is the application running?
    pub is_running: bool,
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
    /// Frame interval for stable FPS.
    pub frame_interval: Duration,
    /// FPS counter widget.
    pub fps: Fps,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new(event_handler: EventHandler, args: Args) -> Self {
        Self {
            is_running: true,
            is_toggled: true,
            event_handler,
            list_state: ListState::default().with_selected(Some(0)),
            animations: Animations::iter().map(|a| a.to_string()).collect(),
            animation: Animation::default(),
            animation_area: Rect::default(),
            frame_interval: Duration::from_secs_f32(1.0 / args.fps),
            fps: Fps::default(),
            args,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut accumulator = Duration::new(0, 0);
        let mut last_tick = Instant::now();
        let list_width = self
            .animations
            .iter()
            .map(|a| a.len())
            .max()
            .unwrap_or_default();
        if let Some(position) = self
            .args
            .file
            .as_ref()
            .and_then(|file| self.animations.iter().position(|anim| file == anim))
        {
            self.is_toggled = false;
            self.list_state.select(Some(position));
        }
        while self.is_running {
            terminal.draw(|frame| self.draw(frame, list_width))?;
            let event = self.event_handler.next()?;
            match event {
                Event::Tick => {
                    self.fps.tick();
                    accumulator += last_tick.elapsed();
                    while accumulator >= self.frame_interval {
                        if !self.animation.is_rendered {
                            self.event_handler.sender.send(Event::Tick)?;
                        }
                        accumulator -= self.frame_interval;
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
            .block(Block::bordered().title("|VT100 Animations|"))
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
        let mut block = Block::bordered()
            .title(Title::from(
                env!("CARGO_PKG_NAME").bold().into_centered_line(),
            ))
            .title(
                Title::from(Line::from(vec![
                    "|".into(),
                    self.list_state
                        .selected()
                        .map(|i| self.animations[i].clone())
                        .unwrap_or_default()
                        .bold(),
                    ": ".into(),
                    self.list_state
                        .selected()
                        .and_then(|i| {
                            descriptions()
                                .get(self.animations[i].as_str())
                                .map(|v| v.to_string())
                        })
                        .unwrap_or_default()
                        .italic(),
                    "|".into(),
                ]))
                .alignment(Alignment::Left)
                .position(Position::Bottom),
            );

        if !self.animation.is_rendered {
            block = block.title(
                Title::from(Line::from(vec![
                    "|".into(),
                    "fps".italic(),
                    ": ".into(),
                    self.fps.to_string().into(),
                    "|".into(),
                ]))
                .alignment(Alignment::Right)
                .position(Position::Top),
            );
        }

        frame.render_widget(block, area[1]);
        self.animation_area = area[1].inner(Margin {
            vertical: 1,
            horizontal: 1,
        });
        frame.render_widget(&mut self.animation, self.animation_area);
    }

    pub fn start_animation(&mut self) {
        let selected = self.list_state.selected().unwrap_or_default();
        let data = Animations::get(&self.animations[selected].clone())
            .unwrap()
            .data
            .into_owned();
        self.animation = Animation {
            is_rendered: false,
            reader: BufReader::new(Cursor::new(data)),
            parser: Parser::new(self.animation_area.height, self.animation_area.width, 0),
            buffer: String::new(),
        };
    }
}
