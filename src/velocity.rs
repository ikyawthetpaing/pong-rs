pub struct Velocity {
	pub x: i16,
	pub y: i16
}

impl Velocity {
	pub fn new(x: i16, y: i16) -> Self {
		Self { x, y }
	}
}