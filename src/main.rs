// Basic OOP example with getters, setter, and to_string equivalent
// by Hevanafa, 28-11-2022

use std::io::{ Write, stdout, stdin };
mod circle;
use circle::Circle;

fn main() {
	let mut c = Circle::new(None);
    println!("Radius: {}", c.get_radius());

	print!("Input new radius: ");
	Write::flush(&mut stdout()).unwrap();
	
	// get new radius
	let mut temp = String::new();
	stdin().read_line(&mut temp).unwrap();
	let new_radius = temp.trim().parse::<f32>().unwrap();

	c.set_radius(new_radius);
	println!("New radius: {}", c.get_radius());
	println!("Circumference: {}", c.get_circumference());
	println!("Area: {}", c.get_area());
	println!("Circle: {}", c);

	print!("Press Enter to exit");
	stdin().read_line(&mut String::new()).unwrap();
}
