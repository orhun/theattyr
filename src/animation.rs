use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Cursor},
    sync::OnceLock,
};

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

pub fn descriptions() -> &'static HashMap<&'static str, &'static str> {
    static MEM: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MEM.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("bambi.vt", "Bambi vs. Godzilla");
        m.insert("bambi_godzila", "Bambi Versus Godzilla, from Dave Brett");
        m.insert("barney.vt", "Barney Being Crushed by a Rock");
        m.insert("beer.vt", "Time for a Beer Break, Folks!");
        m.insert("bevis.butthead.vt", "Beavis and Butthead");
        m.insert("blinkeyes.vt", "Blinking Eyes");
        m.insert("bomb.vt", "The Bomb Test");
        m.insert("bugsbunny.vt", "Bugs Bunny: That's All, Folks");
        m.insert("cartwhee.vt", "Doing a Cartwheel");
        m.insert("castle.vt", "Disney's Fantasy in the Sky, by Don Bertino");
        m.insert("cert18.vt", "Make Money Fast: The Revenge, by GtB (1993)");
        m.insert("cow.vt", "Exploding Cow, Hauled off by U-Mass Food Service");
        m.insert("cowboom.vt", "Cow Explodes, Gets Hauled Off");
        m.insert("crash.vt", "Shuttle Blows Up");
        m.insert("cursor.vt", "Cursor Control Examples in VT100");
        m.insert("delay.vt", "A Small Delay");
        m.insert("demo.vt", "Alan's Impressive Demonstration");
        m.insert("dirty.vt", "Someone Having an Awful Amount of Fun");
        m.insert("dogs.vt", "Fucking Dogs");
        m.insert(
            "dont-wor.vt",
            "George Custer's Last Stand: Don't Worry, be Happy",
        );
        m.insert(
            "dontworry.vt",
            "Man Being Shot with Arrows: Don't Worry, Be Happy",
        );
        m.insert("duckpaint.vt", "Duck Painting");
        m.insert("firework.vt", "Fireworks by Chen Lin");
        m.insert("fireworks.vt", "Guy Setting Off Fireworks");
        m.insert("fishy-fishy.vt", "3-D Fishy Fishy");
        m.insert("fishy.vt", "Fish Swiming By, Glug Glug");
        m.insert("fishy2.vt", "Shamus the Fish by David Rybolt (1994)");
        m.insert("flatmap.vt", "Shifting Flat World Map");
        m.insert("frogs.vt", "Hopping Frog");
        m.insert("glass.vt", "Filling Glass of Liquid");
        m.insert("globe.vt", "ABSOLUTELY EXCELLENT Spinning Globe");
        m.insert("hallow.vt", "Happy Halloween");
        m.insert("hello.vt", "HELLO!");
        m.insert("juanspla.vt", "Plan File in the Form of a Typewriter");
        m.insert("july.4.vt", "July 4th Animation");
        m.insert("jumble.vt", "Now Is the Time for All Good Men");
        m.insert("maingate.vt", "The Disneyland Main Gate, by Don Bertino");
        m.insert("mark_twain.vt", "The Mark Twain Ferry, by Don Bertino");
        m.insert("monkey.vt", "The Monkey Gives You The Finger");
        m.insert("monorail.vt", "Disneyland's Monorail, by Don Bertino");
        m.insert("moon.animation", "Winking Moon Says Good Evening");
        m.insert("movglobe.vt", "Incredible Spinning, Moving Globe");
        m.insert("mr_pumpkin", "Happy Halloween Pumpkin by Mike Kamlet");
        m.insert("nasa.vt", "NASA: Keep Reaching for the Stars, by A.J.L.");
        m.insert("new_year.vt", "Happy New Year to You");
        m.insert("newbeer.vt", "Working on a VT100");
        m.insert("nifty.vt", "Small Animated Word NIFTY");
        m.insert("outerlimits.vt", "The Outer Limits");
        m.insert("pac3d.vt", "Pac Man in 3-D Chomping a Ghost");
        m.insert("paradise.vt", "A Bomb in Paradise by Gonad the Barbarian");
        m.insert("peace.vt", "Imagine World Peace by John G. Poupore");
        m.insert("prey.vt", "Klingon Bird of Prey");
        m.insert("prey_col.vt", "Klingon Bird of Prey");
        m.insert("safesex.vt", "Safe Sex (Literally)");
        m.insert("shuttle.vt", "Technology, Who Needs It");
        m.insert("skyway.vt", "Disneyland's Skyway, by Don Bertino");
        m.insert("snowing", "Merry Christmas from Woodrow");
        m.insert("snowing.vt", "Tis the Season: Merry Christmas");
        m.insert("spinweb.vt", "Spinning Web by R.L. Samuell (April 6, 1994)");
        m.insert("sship.vt", "Space Ship Warps and Fires");
        m.insert(
            "startrek.vt",
            "Star Trek Enterprise Blows up Politically Correct New Enterprise",
        );
        m.insert("strike.vt", "Bowling a Strike");
        m.insert("sun.vt", "A Happy Sun");
        m.insert("surf.vt", "Surfing Wave (In 3-D)");
        m.insert("tetris.vt", "Tetris Game");
        m.insert("tomorrw.vt", "Disneyland's Tomorrowland, by Don Bertino");
        m.insert(
            "torturet.vt",
            "VT100 FONT: The VT-100 Torture Test by Joe Smith (May 8, 1985)",
        );
        m.insert("treadmill.vt", "The Treadmill, by GtB Productions (1993)");
        m.insert("trek.vt", "The Enterprise Blows up an RCA Satellite");
        m.insert("trekvid.vt", "Politically Incorrect Star Trek");
        m.insert("turkey.vt", "Happy Thanksgiving");
        m.insert("tv.vt", "The Outer Limits Television Show");
        m.insert("twilight.vt", "The Twilight Zone");
        m.insert("twilightzone.vt", "Twilight Zone Opener");
        m.insert("valentin.vt", "Happy Valentine's Day, Beth and Dave");
        m.insert("valentine.vt", "Happy Valentine's Day, Jane and Chris");
        m.insert("van_halen.vt", "Van Halen's Song 5150, Animated");
        m.insert("wineglas.vt", "Wine Glass Filling");
        m.insert(
            "xmas-00.vt",
            "Santa Holds Moving Sign: Merry Christmas, Happy New Year",
        );
        m.insert("xmas-01.vt", "Merry Christmas");
        m.insert("xmas-02.vt", "Bird Flies By, Tree Grows, Merry Christmas");
        m.insert("xmas-03.vt", "Merry Christmas (Tree, Train, Presents)");
        m.insert(
            "xmas-04.vt",
            "Merry Christmas, Champagne Glass Filling, Jack-in-the-Box",
        );
        m.insert(
            "xmas-05.vt",
            "Happy Holidays, Starry Night, Christmas Tree, by Peter",
        );
        m.insert("xmas-06.vt", "Merry Christmas: Hearth and Tree");
        m.insert("xmas-07.vt", "A Christmas Card: Merry Christmas, from MIS");
        m.insert("xmas-08.vt", "Christmas Eve, 1992 (1992)");
        m.insert("xmas-09.vt", "Merry Christmas: Reindeer Land on Roof");
        m.insert("xmas.large", "Compilation of Several Christmas Animations");
        m.insert("xmas.vt", "Merry Christmas");
        m.insert("xmas2.vt", "Large Collection of Christmas Animations");
        m.insert("xmasshort.vt", "Merry Christmas, Tree, Train, Present");
        m.insert("zorro.vt", "The Story of Zorro by Cian O'Kiersey");
        m
    })
}
