use core::fmt;
use std::f32::consts::PI;

pub struct Circle {
	radius: f32
}

impl Circle {
	pub fn new(radius: Option<f32>) -> Circle {
		Circle{ radius: radius.unwrap_or(1.0) }
	}

	pub fn get_radius(&self) -> f32 {
		self.radius
	}

	pub fn set_radius(&mut self, radius: f32) {
		self.radius = radius;
	}

	pub fn get_circumference(&self) -> f32 {
		self.radius * 2.0 * PI
	}

	pub fn get_area(&self) -> f32 {
		self.radius.powf(2.0) * PI
	}
}

// to_string implementation
impl fmt::Display for Circle {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.write_str(format!("Circle[radius={}]", self.radius).as_str())?;
		Ok(())
	}
}
