use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};

pub enum Command {
	Up,
	Down,
	Back,
	Next,
	Exit,
	Resize(u16, u16)
}

impl Command {
	pub fn new(timeout: Duration) -> Option<Command> {
		if poll(timeout).ok()? {
			match read().ok()? {
				Event::Key(key_event) => match key_event.code {
					KeyCode::Up | KeyCode::Char('w') => Some(Command::Up),
					KeyCode::Down | KeyCode::Char('s') => Some(Command::Down),
					KeyCode::Char('b') => Some(Command::Back),
					KeyCode::Char('n') | KeyCode::Enter => Some(Command::Next),
					KeyCode::Esc => Some(Command::Exit),
					_ => None
				},
				Event::Resize(width, height) => Some(Command::Resize(width, height)),
				_ => None
			}
		} else { None }
	}
}