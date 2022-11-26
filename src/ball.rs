use crate::{position::Position, velocity::Velocity, paddle::Paddle};
use std::io::{Stdout, stdout};
use crossterm::{style::{StyledContent, self, Stylize}, ExecutableCommand, cursor};

pub struct Ball {
	stdout: Stdout,
	pub position: Position,
	velocity: Velocity,
	style: StyledContent<String>,
}

impl Ball {
	pub fn new(position: Position, velocity: Velocity, style: StyledContent<String>) -> Self {
		Self { stdout: stdout(), position, velocity, style }
	}

	pub fn handle_move(&mut self, height: u16, leftpaddle: &Paddle, rightpaddle: &Paddle) {
		self._clear();
		self._handle_collision(height, leftpaddle, rightpaddle);
		self._handle_movement();
		self._render();
	}

	pub fn check_inside_paddle(&self, paddle: &Paddle) -> bool {
		return self.position.y <= paddle.position.y + paddle.height - 1 && self.position.y >= paddle.position.y;
	}

	pub fn reset(&mut self, width: u16, height: u16) {
		self._clear();
		self.position.x = width/2;
		self.position.y = height/2;
		self.velocity.x *= -1;
		self.velocity.y *= Velocity::get();
	}

	fn _clear(&mut self) {
		self.stdout
		.execute(cursor::MoveTo(self.position.x, self.position.y)).unwrap()
		.execute(style::PrintStyledContent("  ".black())).unwrap();
	}

	fn _render(&mut self) {
		self.stdout
		.execute(cursor::MoveTo(self.position.x, self.position.y)).unwrap()
		.execute(style::PrintStyledContent(self.style.clone())).unwrap();
	}

	fn _handle_movement(&mut self) {
		self.position.x = (self.position.x as i16 + self.velocity.x) as u16;
		self.position.y = (self.position.y as i16 + self.velocity.y) as u16;
	}	

	fn _handle_collision(&mut self, height: u16, leftpaddle: &Paddle, rightpaddle: &Paddle) {
		if self.position.y > height - 3 || self.position.y < 2 {
			self.velocity.y *= -1;
		}
		else if self.position.x == leftpaddle.position.x + 2 {
			if self.check_inside_paddle(leftpaddle) {
				self.velocity.x *= -1;
			}
		}
		else if self.position.x == rightpaddle.position.x - 2 {
			if self.check_inside_paddle(rightpaddle) {
				self.velocity.x *= -1;
			}
		}
	}
}