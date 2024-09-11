use std::io::{BufRead, BufReader, Cursor};

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use rust_embed::Embed;
use tui_term::widget::PseudoTerminal;
use vt100::Parser;

#[derive(Embed)]
#[folder = "vt100"]
pub struct Animations;

pub struct Animation {
    /// Is the animation rendered?
    pub is_rendered: bool,
    // Reader for the file.
    pub reader: BufReader<Cursor<Vec<u8>>>,
    /// VT100 parser.
    pub parser: Parser,
    /// Buffer.
    pub buffer: String,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            is_rendered: false,
            reader: BufReader::new(Cursor::new(Vec::new())),
            parser: Parser::default(),
            buffer: String::new(),
        }
    }
}

impl Widget for &mut Animation {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut line_buffer = String::new();
        let bytes_read = self.reader.read_line(&mut line_buffer).unwrap();
        self.buffer += &line_buffer;
        self.parser.process(self.buffer.as_bytes());
        let pseudo_term = PseudoTerminal::new(self.parser.screen());
        pseudo_term.render(area, buf);
        if bytes_read == 0 {
            self.is_rendered = true;
        }
    }
}
