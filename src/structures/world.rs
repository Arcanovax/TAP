use std::io;

use ratatui::{DefaultTerminal, layout::{self, Constraint::{Fill, Length}}, widgets::Widget};

use crate::enums::{exits::Exits, rooms::Rooms, states::States};

pub struct World {
	room: Rooms,
	quit: bool,
	message: String,
	exits: Option<Exits>,
	state: States,
}

impl World {
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
	}

	fn new() -> Self {
		Self {
			room: Rooms::START,
			quit: false,
			message: String::from("Choose your username: "),
			exits: None,
			state: States::Login
		}
	}

}

impl Widget for World {
	fn render:(self, area: Rect, buf: &mut buffer) {
		let layout = Layout::vertical([Length(3), Length(1), Fill(0)]);
		let [tabs, axis, demo] = area.layout(&layout);
		self.tabs().render(tabs, buf);
		let scroll_needed = self.render_demo(demo, buf);
        let axis_width = if scroll_needed {
            axis.width.saturating_sub(1)
        } else {
            axis.width
        };
        Self::axis(axis_width, self.spacing).render(axis, buf);
    }
	}
}