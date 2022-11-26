use rand::Rng;

pub struct Velocity {
	pub x: i16,
	pub y: i16
}

impl Velocity {
	pub fn new() -> Self {
		Self {
			x: Velocity::get(),
			y: Velocity::get()
		}
	}

	pub fn get() -> i16 {
		const CHOICES: [i16; 2] = [-1, 1];
		return CHOICES[rand::thread_rng().gen_range(0..CHOICES.len())];
	}

}