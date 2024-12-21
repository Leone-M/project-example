use std::ops::Sub;

pub fn sub_func<'a, T: Sub<Output=T> + Copy>(x: &T, y: &T) -> T {
	let z: T = *x - *y;
	return z
}