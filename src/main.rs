mod velocity;
mod command;
mod position;
mod paddle;
mod ball;
mod pong;

use crate::pong::Pong;

fn main() {
	Pong::new().run();
}
