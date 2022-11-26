use crate::position::Position;
use std::io::{Stdout, stdout};
use crossterm::{style::{StyledContent, self, Stylize}, ExecutableCommand, cursor};

pub struct Paddle {
	stdout: Stdout,
	pub position: Position,
	pub height: u16,
	style: StyledContent<String>,
	pub score: u16
}

impl Paddle {
	pub fn new(position: Position, height: u16, style: StyledContent<String>) -> Self {
		Self { stdout: stdout() , position, height, style, score: 0 }
	}

	pub fn render(&mut self) {
		for i in 0..self.height {
			self._render_at(self.position.x, self.position.y + i);
		}
	}

	pub fn try_move_up(&mut self) {
		self._clear_at(self.position.x, self.position.y + self.height - 1);
		self.position.y -= 1;
		self._render_at(self.position.x, self.position.y);
	}

	pub fn try_move_down(&mut self) {
		self._clear_at(self.position.x, self.position.y);
		self.position.y += 1;
		self._render_at(self.position.x, self.position.y + self.height - 1)
	}

	pub fn try_move_auto(&mut self, ball_pos_y: u16) {
		if !(ball_pos_y <= self.position.y + self.height - 1 && ball_pos_y >= self.position.y) {
			if self.position.y < ball_pos_y {
				self.try_move_down();
			} else {
				self.try_move_up();
			}
		}
	}

	fn _clear_at(&mut self, x: u16, y: u16) {
		self.stdout
		.execute(cursor::MoveTo(x, y)).unwrap()
		.execute(style::PrintStyledContent("  ".black())).unwrap();
	}

	fn _render_at(&mut self, x: u16, y: u16) {
		self.stdout
		.execute(cursor::MoveTo(x, y)).unwrap()
		.execute(style::PrintStyledContent(self.style.clone())).unwrap();
	}
}