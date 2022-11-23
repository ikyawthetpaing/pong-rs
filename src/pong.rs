use std::{io::{Stdout, stdout}, time::{Duration, Instant}};
use crossterm::{
	ExecutableCommand,
	cursor,
	style::{self, Stylize, StyledContent},
	terminal::{size, self}
};

use crate::{paddle::Paddle, position::Position, command::Command, ball::Ball, velocity::Velocity};

pub struct Pong {
	stdout: Stdout,
	width: u16,
	height: u16,
	symbol: String,
	padding: u16,
	timeout: Duration,
	ball: Ball,
	left_paddle: Paddle,
	right_paddle: Paddle
}

impl Pong {
	pub fn new() -> Self {
		let (width, height) = size().unwrap();
		let symbol = String::from("██"); //█
		let paddle_height = height/4;
		let paddle_center_y = height/2 - paddle_height/2;
		let padding = 5;

		Self {
			stdout: stdout(),
			width: width - 1,
			height: height,
			symbol: symbol.clone(),
			padding: padding,
			timeout: Duration::from_millis(25),
			ball: Ball::new(
				Position::new(width/2, height/2),
				Velocity::new(1, 1),
				symbol.clone().cyan()),
			left_paddle: Paddle::new(
				Position::new(padding, paddle_center_y),
				paddle_height,
				symbol.clone().green()),
			right_paddle: Paddle::new(
				Position::new(width - padding - 2, paddle_center_y),
				paddle_height, 
				symbol.clone().red())
		}
	}

	pub fn run(&mut self) {
		self._hide_cursor();
		self._clear_terminal();
		self._render_border();
		self._render_score();

		self.left_paddle.render();
		self.right_paddle.render();

		let mut ball_dealy = Instant::now();
		let mut is_ball_dealy = true;
		loop {
			let now = Instant::now();
			if !is_ball_dealy {
				ball_dealy = now.checked_add(self.timeout).unwrap();
				is_ball_dealy = true;
			
			}
			if now > ball_dealy {
				self.ball.handle_move(self.height, &self.left_paddle, &self.right_paddle);
				is_ball_dealy = false;
			}

			if let Some(command) = Command::new(self.timeout) {
				match command {
					Command::Up => {
						if self.left_paddle.position.y > 1 {
							self.left_paddle.try_move_up();
						}
					},
					Command::Down => {
						if self.left_paddle.position.y < self.height - self.left_paddle.height - 1 {
							self.left_paddle.try_move_down();
						}
					},
					Command::Resize(width, height) => self._resize(width, height),
					Command::Exit => break,
					_ => ()
				}
			}

			// Auto move right paddle
			if self.ball.position.x > self.width - self.width/5 {
				if !self.ball.check_inside_paddle(&self.right_paddle) {
					if self.right_paddle.position.y < self.ball.position.y {
						self.right_paddle.try_move_down();
					} else {
						self.right_paddle.try_move_up();
					}
				}
			}
			self._handle_winner();
			self.ball.check_will_through_paddle(&mut self.left_paddle);
			self.ball.check_inside_paddle(&mut self.right_paddle);
		}
		self._clear_terminal();
		let label = format!("Your score: {} | Computer score: {}", self.left_paddle.score, self.right_paddle.score);
		self._render_at(self.width/2 - label.len() as u16 / 2, self.height/2, label.bold().dark_magenta());
		self._show_cursor();
	}

	fn _clear_terminal(&mut self) {
		self.stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
	}
	fn _hide_cursor(&mut self) {
		self.stdout.execute(cursor::Hide).unwrap();
	}

	fn _show_cursor(&mut self) {
		self.stdout.execute(cursor::Show).unwrap();
	}

	fn _render_at(&mut self, x: u16, y: u16, style: StyledContent<String>) {
		self.stdout
		.execute(cursor::MoveTo(x, y)).unwrap()
		.execute(style::PrintStyledContent(style)).unwrap();
	}

	fn _render_border(&mut self) {
		for x in 0..self.width {
			for y in 0..self.height {
				if (x == 0 || y == 0) || (x == self.width - 1 || y == self.height - 1) {
					self._render_at(x, y, self.symbol.clone().blue());
				}
			}
		}
	}

	fn _render_score(&mut self) {
		let user_label = format!("Your score: {}", self.left_paddle.score);
		let other_laber = format!("Computer score: {}", self.right_paddle.score);

		self._render_at(self.width/4 - user_label.len() as u16/2, 0, user_label.green());
		self._render_at(self.width - self.width/4 - other_laber.len() as u16/2, 0, other_laber.red());
	}

	fn _get_position(&self, pre_length: u16, new_length: u16, pre_obj_pos: u16) -> u16 {
		return ((((pre_obj_pos as f64/pre_length as f64) * 100.0)/100.0) * new_length as f64).ceil() as u16;
	}

	fn _resize(&mut self, width: u16, height: u16) {
		self.left_paddle.height = height/4;
		self.right_paddle.height = height/4;

		self.left_paddle.position = Position::new(
			self.padding,
			self._get_position(self.height, height, self.left_paddle.position.y));

		self.right_paddle.position = Position::new(
			width - self.padding - 1,
			self._get_position(self.height, height, self.right_paddle.position.y));

		self.ball.position = Position::new(
			self._get_position(self.width, width, self.ball.position.x),
			self._get_position(self.height, height, self.ball.position.y));

		self.width = width;
		self.height = height + 1;

		self._clear_terminal();
		self._render_border();
		self._render_score();
		self.left_paddle.render();
		self.right_paddle.render();
	}

	fn _handle_winner(&mut self) {
		if self.ball.position.x < 3 {
			self.ball.reset(self.width, self.height);
			self.right_paddle.score += 1;
			self._render_score();
		}
		else if self.ball.position.x > self.width - 4 {
			self.ball.reset(self.width, self.height);
			self.left_paddle.score += 1;
			self._render_score();
		}
	}
}