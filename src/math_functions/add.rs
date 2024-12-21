use std::ops::Add;

pub fn add_func<'a, T: Add<Output=T> + Copy>(x: &T, y: &T) -> T {
	let z: T = *x + *y;
	return z
}